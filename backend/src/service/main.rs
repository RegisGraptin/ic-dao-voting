use std::{cell::RefCell, str::FromStr, time::Duration};

use crate::{create_icp_signer, get_rpc_service_base, get_rpc_service_optimism_sepolia, get_rpc_service_sepolia};

use alloy::{
    network::EthereumWallet,
    eips::BlockNumberOrTag,
    primitives::{address, Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{Filter, Log},
    signers::Signer,
    sol,
    sol_types::SolEvent,
    transports::icp::IcpConfig,
};

use ic_cdk_timers::TimerId;

const POLL_LIMIT: usize = 3;

thread_local! {
    static NONCE: RefCell<Option<u64>> = const { RefCell::new(None) };
}

const OPTIMISM_CHAIN_ID: u64 = 11155420;

struct State {
    timer_id: Option<TimerId>,
    logs: Vec<String>,
    poll_count: usize,
}

impl State {
    fn default() -> State {
        State {
            // Store the id of the IC_CDK timer used for polling the EVM RPC periodically.
            // This id can be used to cancel the timer before the configured `POLL_LIMIT`
            // has been reached.
            timer_id: None,
            // The logs returned by the EVM are stored here for display in the frontend.
            logs: Vec::new(),
            // The number of polls made. Polls finish automatically, once the `POLL_LIMIT`
            // has been reached. This count is used to create a good interactive UI experience.
            poll_count: 0,
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}


// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs, clippy::too_many_arguments)]
    #[sol(rpc)]
    DAO,
    "src/abi/DAO.json"
);

sol!(
    #[allow(missing_docs, clippy::too_many_arguments)]
    #[sol(rpc)]
    USDC,
    "src/abi/USDC.json"
);


async fn send_tranfer_on_other_chain(
    target_address: Address,
    amount: U256
) -> Result<String, String> {
    
    // Setup signer
    let signer = create_icp_signer().await;
    let address = signer.address();

    // Setup provider
    let wallet = EthereumWallet::from(signer);
    let rpc_service = get_rpc_service_optimism_sepolia();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_gas_estimation()
        .wallet(wallet)
        .on_icp(config);

    // Attempt to get nonce from thread-local storage
    let maybe_nonce = NONCE.with_borrow(|maybe_nonce| {
        // If a nonce exists, the next nonce to use is latest nonce + 1
        maybe_nonce.map(|nonce| nonce + 1)
    });

    // If no nonce exists, get it from the provider
    let nonce = if let Some(nonce) = maybe_nonce {
        nonce
    } else {
        provider.get_transaction_count(address).await.unwrap_or(0)
    };

    // Mint a new NFT
    let contract = USDC::new(
        address!("63A0bfd6a5cdCF446ae12135E2CD86b908659568"),
        provider.clone(),
    );

    match contract
        .transfer(target_address, amount)
        .nonce(nonce)
        .chain_id(OPTIMISM_CHAIN_ID)
        .from(address)
        .send()
        .await
    {
        Ok(builder) => {
            let node_hash = *builder.tx_hash();
            let tx_response = provider.get_transaction_by_hash(node_hash).await.unwrap();

            match tx_response {
                Some(tx) => {
                    // The transaction has been mined and included in a block, the nonce
                    // has been consumed. Save it to thread-local storage. Next transaction
                    // for this address will use a nonce that is = this nonce + 1
                    NONCE.with_borrow_mut(|nonce| {
                        *nonce = Some(tx.nonce);
                    });
                    Ok(format!("{:?}", tx))
                }
                None => Err("Could not get transaction.".to_string()),
            }
        }
        Err(e) => Err(format!("{:?}", e)),
    }

}




/// Watch for BTC proposal accepted
#[ic_cdk::update]
async fn watch_btc_event_transfer_start() -> Result<String, String> {
    // Don't start a timer if one is already running
    STATE.with_borrow(|state| {
        if state.timer_id.is_some() {
            return Err("Already watching for logs.".to_string());
        }
        Ok(())
    })?;

    let rpc_service = get_rpc_service_base();
    let config = IcpConfig::new(rpc_service).set_max_response_size(100_000);
    let provider = ProviderBuilder::new().on_icp(config);

    // This callback will be called every time new logs are received
    let callback = |incoming_logs: Vec<Log>| {
        STATE.with_borrow_mut(|state| {
            for log in incoming_logs.iter() {
                let proposal: Log<DAO::AcceptedBTCProposalEvent> = log.log_decode().unwrap();
                let DAO::AcceptedBTCProposalEvent { proposalId, btcAddress, amount } = proposal.data();
                
                // Execute the tx on Optimisme side
                // FIXME: Suppose to be BTC, but not BTC wallet implementation yet
                send_tranfer_on_other_chain(Address::from_str(btcAddress).unwrap(), *amount);

                // FIXME:: If we need to sign and send a tx on BTC, the function might be async, leading
                // to a potential issue for the callbacke function...
            }

            state.poll_count += 1;
            if state.poll_count >= POLL_LIMIT {
                state.timer_id.take();
            }
        })
    };

    // Clear the logs and poll count when starting a new watch
    STATE.with_borrow_mut(|state| {
        state.logs.clear();
        state.poll_count = 0;
    });

    let dao_address = address!("A6E782af1b182329282CC67f1ce0f4680030E12F");
    let filter = Filter::new()
        .address(dao_address)
        .event(DAO::AcceptedBTCProposalEvent::SIGNATURE)
        .from_block(BlockNumberOrTag::Latest);

    // Initialize the poller and start watching
    // `with_limit` (optional) is used to limit the number of times to poll, defaults to 3
    // `with_poll_interval` (optional) is used to set the interval between polls, defaults to 7 seconds
    let poller = provider.watch_logs(&filter).await.unwrap();
    let timer_id = poller
        .with_limit(Some(POLL_LIMIT))
        .with_poll_interval(Duration::from_secs(10))
        .start(callback)
        .unwrap();

    // Save timer id to be able to stop watch before completion
    STATE.with_borrow_mut(|state| {
        state.timer_id = Some(timer_id);
    });

    Ok(format!("Watching for logs, polling {} times.", POLL_LIMIT))
}

/// Stop the watch before it reaches completion
#[ic_cdk::update]
async fn watch_btc_proposal_event_stop() -> Result<String, String> {
    STATE.with_borrow_mut(|state| {
        if let Some(timer_id) = state.timer_id.take() {
            ic_cdk_timers::clear_timer(timer_id);
            Ok(())
        } else {
            Err("No timer to clear.".to_string())
        }
    })?;

    Ok("Watching for logs stopped.".to_string())
}

/// Returns a boolean that is `true` when watching and `false` otherwise.
#[ic_cdk::query]
async fn watch_btc_proposal_event_is_polling() -> Result<bool, String> {
    STATE.with_borrow(|state| Ok(state.timer_id.is_some()))
}

/// Returns the number of polls made. Polls finish automatically, once the `POLL_LIMIT`
/// has been reached. This count is used to create a good interactive UI experience.
#[ic_cdk::query]
async fn watch_btc_proposal_event_poll_count() -> Result<usize, String> {
    STATE.with_borrow(|state| Ok(state.poll_count))
}

/// Returns the list of logs returned by the watch. Gets reset on each start.
#[ic_cdk::query]
async fn watch_btc_proposal_event_get() -> Result<Vec<String>, String> {
    STATE.with_borrow(|state| Ok(state.logs.iter().map(|log| format!("{log:?}")).collect()))
}

