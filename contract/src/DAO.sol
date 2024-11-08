// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import {ERC20Votes} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol";
import {Nonces} from "@openzeppelin/contracts/utils/Nonces.sol";

contract DAO is ERC20, ERC20Permit {

    struct BTCProposal {
        string btcAddress;
        uint256 amount;
    }

    uint256 public proposalIds;

    event CreateBTCProposalEvent(uint256 proposalId);
    event AcceptedBTCProposalEvent(uint256 proposalId, string btcAddress, uint256 amount);

    mapping(uint256 => BTCProposal) public proposals;
    
    // Add the moment, we collect only yes vote. The no are just not collected
    // We need to take this modification into account on prod
    mapping(uint256 => uint256) votingPower; 


    constructor() ERC20("VotingToken", "VTK") ERC20Permit("VotingToken") {
        _mint(msg.sender, 1000);
    }

    function sendBTCProposal(string memory btcAddress, uint256 amount) public {
        BTCProposal memory proposal = BTCProposal({
            btcAddress: btcAddress,
            amount: amount
        });

        proposals[proposalIds] = proposal;

        emit CreateBTCProposalEvent(proposalIds);

        proposalIds++;
    }

    function voteOnProposal(uint256 proposalId) public {
        require(this.balanceOf(msg.sender) > 0, "No token for voting");
        require(proposalId < proposalIds, "invalid proposal id");
        // For prod version, check not already voted proposal and other 
        // edge cases missing for this implementation
        
        
        // Add voting power
        votingPower[proposalId] += this.balanceOf(msg.sender);
        

        // In case we have reached our threshold
        if (votingPower[proposalId] > (this.totalSupply() / 2 )) {
            emit AcceptedBTCProposalEvent(
                proposalId, 
                proposals[proposalId].btcAddress, 
                proposals[proposalId].amount
            );
        }
    }

}
