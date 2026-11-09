// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

import {Script, console} from "forge-std/Script.sol";
import {AICID} from "../src/AICID.sol";
import {InheritanceVault} from "../src/InheritanceVault.sol";
import {TimeCapsule} from "../src/TimeCapsule.sol";

/// @notice Деплой всех контрактов AIC-OS за один запуск.
///
/// Прогон без отправки (симуляция):
///   forge script script/DeployAll.s.sol --rpc-url mainnet
///
/// Реальный деплой (например, в Sepolia):
///   forge script script/DeployAll.s.sol \
///     --rpc-url sepolia --broadcast --verify --account deployer
contract DeployAll is Script {
    function run() external {
        vm.startBroadcast();

        AICID aicid = new AICID();
        console.log(unicode"AICID (реестр имён)          :", address(aicid));

        InheritanceVault vault = new InheritanceVault();
        console.log(unicode"InheritanceVault (наследство):", address(vault));

        TimeCapsule capsule = new TimeCapsule();
        console.log(unicode"TimeCapsule (капсула времени):", address(capsule));

        vm.stopBroadcast();

        console.log("");
        console.log(unicode"--- ПАРАМЕТРЫ КОНТРАКТОВ ---");
        console.log(unicode"Срок регистрации имени (AICID), сек:", aicid.REGISTRATION_PERIOD());
        console.log(unicode"Мин. неактивность (Vault), сек     :", vault.MIN_INACTIVITY());
        console.log(unicode"Мин. блокировка (Capsule), сек     :", capsule.MIN_LOCK());
    }
}
