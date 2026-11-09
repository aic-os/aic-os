# PROJECT_TREE (auto-generated)

> Generated: 2026-11-10 00:52:28
> Regenerate: powershell -File tools/scripts/update-tree.ps1

```
aic-os/
|-- .ai
|   |-- MEMORY.md
|   \-- PROJECT_TREE.md
|-- config
|   \-- settings.yaml
|-- data
|   \-- chain.json
|-- docs
|   |-- ARCHITECTURE.md
|   |-- GLOSSARY.md
|   \-- TOKENOMICS.md
|-- packages
|   |-- contracts
|   |   |-- broadcast
|   |   |   |-- DeployAICID.s.sol
|   |   |   |   |-- 1
|   |   |   |   |   \-- dry-run
|   |   |   |   \-- 31337
|   |   |   |       |-- run-1794245449763.json
|   |   |   |       |-- run-latest.json
|   |   |   |       \-- run-latest.recovery.lock
|   |   |   \-- DeployAll.s.sol
|   |   |       \-- 1
|   |   |           \-- dry-run
|   |   |-- cache
|   |   |   |-- DeployAICID.s.sol
|   |   |   |   |-- 1
|   |   |   |   |   \-- dry-run
|   |   |   |   \-- 31337
|   |   |   |       |-- run-1794245449763.json
|   |   |   |       |-- run-latest.json
|   |   |   |       |-- run-latest.json.recovery.json
|   |   |   |       \-- run-latest.json.recovery.lock
|   |   |   |-- DeployAll.s.sol
|   |   |   |   \-- 1
|   |   |   |       \-- dry-run
|   |   |   \-- solidity-files-cache.json
|   |   |-- lib
|   |   |   \-- forge-std
|   |   |       |-- .github
|   |   |       |   |-- workflows
|   |   |       |   |-- CODEOWNERS
|   |   |       |   \-- dependabot.yml
|   |   |       |-- scripts
|   |   |       |   \-- vm.py
|   |   |       |-- src
|   |   |       |   |-- interfaces
|   |   |       |   |-- Base.sol
|   |   |       |   |-- Config.sol
|   |   |       |   |-- console.sol
|   |   |       |   |-- console2.sol
|   |   |       |   |-- LibVariable.sol
|   |   |       |   |-- safeconsole.sol
|   |   |       |   |-- Script.sol
|   |   |       |   |-- StdAssertions.sol
|   |   |       |   |-- StdChains.sol
|   |   |       |   |-- StdCheats.sol
|   |   |       |   |-- StdConfig.sol
|   |   |       |   |-- StdConstants.sol
|   |   |       |   |-- StdError.sol
|   |   |       |   |-- StdInvariant.sol
|   |   |       |   |-- StdJson.sol
|   |   |       |   |-- StdMath.sol
|   |   |       |   |-- StdSecp256k1.sol
|   |   |       |   |-- StdStorage.sol
|   |   |       |   |-- StdStyle.sol
|   |   |       |   |-- StdToml.sol
|   |   |       |   |-- StdUtils.sol
|   |   |       |   |-- Test.sol
|   |   |       |   \-- Vm.sol
|   |   |       |-- test
|   |   |       |   |-- compilation
|   |   |       |   |-- fixtures
|   |   |       |   |-- CommonBase.t.sol
|   |   |       |   |-- Config.t.sol
|   |   |       |   |-- LibVariable.t.sol
|   |   |       |   |-- StdAssertions.t.sol
|   |   |       |   |-- StdChains.t.sol
|   |   |       |   |-- StdCheats.t.sol
|   |   |       |   |-- StdConstants.t.sol
|   |   |       |   |-- StdError.t.sol
|   |   |       |   |-- StdJson.t.sol
|   |   |       |   |-- StdMath.t.sol
|   |   |       |   |-- StdSecp256k1.t.sol
|   |   |       |   |-- StdStorage.t.sol
|   |   |       |   |-- StdStyle.t.sol
|   |   |       |   |-- StdToml.t.sol
|   |   |       |   |-- StdUtils.t.sol
|   |   |       |   \-- Vm.t.sol
|   |   |       |-- .gitattributes
|   |   |       |-- .gitignore
|   |   |       |-- CONTRIBUTING.md
|   |   |       |-- foundry.toml
|   |   |       |-- LICENSE-APACHE
|   |   |       |-- LICENSE-MIT
|   |   |       |-- package.json
|   |   |       |-- README.md
|   |   |       \-- RELEASE_CHECKLIST.md
|   |   |-- out
|   |   |   |-- AICID.sol
|   |   |   |   \-- AICID.json
|   |   |   |-- AICID.t.sol
|   |   |   |   |-- AICIDTest.json
|   |   |   |   \-- VmContractHelper915f18e2e9e7b6093ad2882b7cd1420960e01b928bd062bbd6d77434d0e7b2ec.json
|   |   |   |-- Base.sol
|   |   |   |   |-- CommonBase.json
|   |   |   |   |-- ScriptBase.json
|   |   |   |   \-- TestBase.json
|   |   |   |-- build-info
|   |   |   |   |-- 4e651b878b38dc6a.json
|   |   |   |   |-- 8997d775ab47c05e.json
|   |   |   |   \-- b046b1c023c18b4d.json
|   |   |   |-- console.sol
|   |   |   |   \-- console.json
|   |   |   |-- DeployAICID.s.sol
|   |   |   |   \-- DeployAICID.json
|   |   |   |-- DeployAll.s.sol
|   |   |   |   \-- DeployAll.json
|   |   |   |-- IMulticall3.sol
|   |   |   |   \-- IMulticall3.json
|   |   |   |-- InheritanceVault.sol
|   |   |   |   \-- InheritanceVault.json
|   |   |   |-- InheritanceVault.t.sol
|   |   |   |   |-- InheritanceVaultTest.json
|   |   |   |   \-- VmContractHelperb2ac49e06e29e9748c7e074424a250f418aaf914f826b640ed1b711c2ca30c45.json
|   |   |   |-- safeconsole.sol
|   |   |   |   \-- safeconsole.json
|   |   |   |-- Script.sol
|   |   |   |   \-- Script.json
|   |   |   |-- StdAssertions.sol
|   |   |   |   \-- StdAssertions.json
|   |   |   |-- StdChains.sol
|   |   |   |   \-- StdChains.json
|   |   |   |-- StdCheats.sol
|   |   |   |   |-- StdCheats.json
|   |   |   |   \-- StdCheatsSafe.json
|   |   |   |-- StdConstants.sol
|   |   |   |   \-- StdConstants.json
|   |   |   |-- StdError.sol
|   |   |   |   \-- stdError.json
|   |   |   |-- StdInvariant.sol
|   |   |   |   \-- StdInvariant.json
|   |   |   |-- StdJson.sol
|   |   |   |   \-- stdJson.json
|   |   |   |-- StdMath.sol
|   |   |   |   \-- stdMath.json
|   |   |   |-- StdStorage.sol
|   |   |   |   |-- stdStorage.json
|   |   |   |   \-- stdStorageSafe.json
|   |   |   |-- StdStyle.sol
|   |   |   |   \-- StdStyle.json
|   |   |   |-- StdToml.sol
|   |   |   |   \-- stdToml.json
|   |   |   |-- StdUtils.sol
|   |   |   |   \-- StdUtils.json
|   |   |   |-- Test.sol
|   |   |   |   \-- Test.json
|   |   |   |-- TimeCapsule.sol
|   |   |   |   \-- TimeCapsule.json
|   |   |   |-- TimeCapsule.t.sol
|   |   |   |   |-- TimeCapsuleTest.json
|   |   |   |   \-- VmContractHelper0321a662350064d94ec52ec0b176e5304aed7e3c74eaf513879e0612a2587a24.json
|   |   |   \-- Vm.sol
|   |   |       |-- Vm.json
|   |   |       \-- VmSafe.json
|   |   |-- script
|   |   |   |-- DeployAICID.s.sol
|   |   |   \-- DeployAll.s.sol
|   |   |-- src
|   |   |   |-- AICID.sol
|   |   |   |-- InheritanceVault.sol
|   |   |   \-- TimeCapsule.sol
|   |   |-- test
|   |   |   |-- AICID.t.sol
|   |   |   |-- InheritanceVault.t.sol
|   |   |   \-- TimeCapsule.t.sol
|   |   |-- .env.example
|   |   |-- DEPLOY.md
|   |   |-- foundry.toml
|   |   |-- node-fee2.err
|   |   |-- node-fee2.log
|   |   \-- README.md
|   |-- core
|   |   |-- examples
|   |   |   |-- aic_id_resolve.rs
|   |   |   |-- all_networks.rs
|   |   |   |-- broadcast_test.rs
|   |   |   |-- erc20_balance.rs
|   |   |   |-- erc20_metadata.rs
|   |   |   |-- get_balance.rs
|   |   |   |-- multichain_balance.rs
|   |   |   |-- register_aicid.rs
|   |   |   |-- send_aic.rs
|   |   |   |-- send_eth.rs
|   |   |   |-- send_eth_1559.rs
|   |   |   \-- test_pq.rs
|   |   |-- src
|   |   |   |-- crypto
|   |   |   |   |-- mod.rs
|   |   |   |   \-- pq_crypto.rs
|   |   |   |-- providers
|   |   |   |   |-- algorand.rs
|   |   |   |   |-- aptos.rs
|   |   |   |   |-- arweave.rs
|   |   |   |   |-- bitcoin.rs
|   |   |   |   |-- cardano.rs
|   |   |   |   |-- ckb.rs
|   |   |   |   |-- cosmos.rs
|   |   |   |   |-- evm.rs
|   |   |   |   |-- evm_abi.rs
|   |   |   |   |-- factory.rs
|   |   |   |   |-- flow.rs
|   |   |   |   |-- hedera.rs
|   |   |   |   |-- icp.rs
|   |   |   |   |-- kaspa.rs
|   |   |   |   |-- mod.rs
|   |   |   |   |-- monero.rs
|   |   |   |   |-- multiversx.rs
|   |   |   |   |-- near.rs
|   |   |   |   |-- polkadot.rs
|   |   |   |   |-- prices.rs
|   |   |   |   |-- radix.rs
|   |   |   |   |-- solana.rs
|   |   |   |   |-- stellar.rs
|   |   |   |   |-- sui.rs
|   |   |   |   |-- tezos.rs
|   |   |   |   |-- theta.rs
|   |   |   |   |-- ton.rs
|   |   |   |   |-- tron.rs
|   |   |   |   |-- vechain.rs
|   |   |   |   |-- waves.rs
|   |   |   |   \-- xrp.rs
|   |   |   |-- wallet
|   |   |   |   |-- erc20.rs
|   |   |   |   |-- eth.rs
|   |   |   |   |-- eth_tx.rs
|   |   |   |   |-- eth_tx_1559.rs
|   |   |   |   |-- mnemonic.rs
|   |   |   |   \-- mod.rs
|   |   |   |-- chain.rs
|   |   |   |-- error.rs
|   |   |   |-- lib.rs
|   |   |   |-- settings.rs
|   |   |   \-- traits.rs
|   |   |-- tests
|   |   |   |-- eth_address.rs
|   |   |   |-- eth_tx.rs
|   |   |   |-- eth_tx_1559.rs
|   |   |   |-- factory.rs
|   |   |   |-- mnemonic.rs
|   |   |   \-- settings.rs
|   |   \-- Cargo.toml
|   |-- node
|   |   |-- src
|   |   |   |-- block.rs
|   |   |   |-- chain.rs
|   |   |   |-- evm.rs
|   |   |   |-- genesis_contracts.rs
|   |   |   |-- main.rs
|   |   |   |-- mempool.rs
|   |   |   |-- p2p.rs
|   |   |   |-- rpc.rs
|   |   |   |-- serde_helpers.rs
|   |   |   |-- state.rs
|   |   |   \-- transaction.rs
|   |   \-- Cargo.toml
|   |-- sdk
|   \-- wallet
|-- tools
|   \-- scripts
|       \-- update-tree.ps1
|-- .gitignore
|-- Cargo.lock
|-- Cargo.toml
|-- node.err
|-- node.log
|-- node1.err
|-- node1.log
|-- node2.err
|-- node2.log
|-- nodeA.err
|-- nodeA.log
|-- nodeB.err
|-- nodeB.log
|-- node-check.err
|-- node-check.log
|-- node-debug.err
|-- node-debug.log
|-- node-evm.err
|-- node-evm.log
|-- node-evm2.err
|-- node-evm2.log
|-- node-evm-final.err
|-- node-evm-final.log
|-- node-fee.err
|-- node-fee.log
|-- node-fee2.err
|-- node-fee2.log
|-- node-final.err
|-- node-final.log
|-- node-final2.err
|-- node-final2.log
|-- node-fix.err
|-- node-fix.log
|-- node-genesis.err
|-- node-genesis.log
|-- node-persist1.err
|-- node-persist1.log
|-- node-persist2.err
|-- node-persist2.log
|-- node-pq.err
|-- node-pq.log
|-- node-reg.err
|-- node-reg.log
|-- node-test.err
|-- node-test.log
|-- node-trace.err
|-- node-trace.log
|-- node-v03.err
|-- node-v03.log
|-- node-win.err
|-- node-win.log
|-- node-write.err
|-- node-write.log
|-- PRINCIPLES.md
|-- README.md
\-- ROADMAP.md
```

