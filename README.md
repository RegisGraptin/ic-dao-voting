# IC-DAO-CONNECT - Cross-Chain DAO Voting Platform

DAO execution layer are limited due to the DAO smart contract. Indeed, usually all the voting and proposals are done on a unique chain as for instance ethereum. However, it is impossible for the DAO to execute action on other chains limiting the DAO power.

With `ic-dao-connect` we intent to change and improve the DAO capabilities by allowing the DAO to do specific actions on other chain while preserving the security of the DAO.

## Our Implementation

In our initial approach, we wanted to add the possibility for a DAO on ethereum to send a transaction on BTC. However, due to the current limitation of our knoledge and the code available, we could not find a proper way to use BTC. Instead, we decided to define a transfer on Optimism using USDC. Basically, a user have the possibility to create a particular proposal with the intent to transfer USDC on Optimism. Once the proposal created and accepted, the smart contract will emit an event indicating that the proposal has been accepted. 

In the meantime, we have an ICP services that montoring the logs from etherem. Once a log is detected from the given DAO, it will generate and execute a transfer of USDC on Optimism layer with the parameters defined in the event. Here we are limiting our approach to a USDC transfer, but this can be extend to other actions as voting on proposal on another chain, sending funds to another chain, extend the dapp...


## Links

Deployed DAO proposals on ethereum sepolia deployed using ICP
https://zkfwe-6yaaa-aaaab-qacca-cai.icp0.io/

Backend service listener
backend-icp: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=5stdt-mqaaa-aaaab-qac3q-cai

Oisy wallet principal: xlfna-5vht7-zjrj6-n6mum-frrfb-scwhx-b656x-rsese-fl234-iak7y-oae
