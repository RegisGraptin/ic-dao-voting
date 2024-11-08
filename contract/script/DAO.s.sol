// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {DAO} from "../src/DAO.sol";

contract DAOScript is Script {
    DAO public dao;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        dao = new DAO();

        vm.stopBroadcast();
    }
}
