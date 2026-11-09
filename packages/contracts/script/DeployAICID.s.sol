// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

import {Script, console} from "forge-std/Script.sol";
import {AICID} from "../src/AICID.sol";

/// @notice Deploy AICID to the target chain.
///
/// Usage (dry-run on mainnet fork, no broadcast):
///   forge script script/DeployAICID.s.sol --rpc-url <RPC>
///
/// Usage (real deploy):
///   forge script script/DeployAICID.s.sol \
///     --rpc-url <RPC> --broadcast --private-key $env:DEPLOYER_KEY
contract DeployAICID is Script {
    function run() external returns (AICID id) {
        vm.startBroadcast();
        id = new AICID();
        vm.stopBroadcast();

        console.log("AICID deployed at:", address(id));
        console.log("REGISTRATION_PERIOD (seconds):", id.REGISTRATION_PERIOD());
    }
}
