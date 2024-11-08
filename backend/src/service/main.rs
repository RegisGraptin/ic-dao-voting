use std::{cell::RefCell, time::Duration};

use crate::{create_icp_signer, get_rpc_service_base, get_rpc_service_sepolia};

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
                
                // Execute the tx on BTC side
                // TODO::

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

