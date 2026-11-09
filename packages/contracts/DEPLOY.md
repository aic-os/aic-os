# Как контракты попадают в AIC Chain

## Принцип №1

Контракты НЕ деплоятся в Ethereum, Base, Sepolia или любую другую чужую сеть.
Они деплоятся ТОЛЬКО в genesis AIC Chain.

## Как это работает

1. Контракты написаны на Solidity (src/AICID.sol, src/InheritanceVault.sol, src/TimeCapsule.sol).
2. Foundry компилирует их: forge build -> out/*.sol/*.json.
3. Нода (packages/node/src/genesis_contracts.rs) читает deployedBytecode.object (runtime code, НЕ bytecode.object!) из артефактов.
4. При создании genesis (первый запуск ноды) deploy_genesis_contracts() кладёт runtime code в State по фиксированным адресам:
   - AICID: 0x0000000000000000000000000000000000a1c1d0
   - InheritanceVault: 0x000000000000000000000000000000000000dead
   - TimeCapsule: 0x0000000000000000000000000000000000c4b5e0
5. Всё. Контракты живут в AIC Chain, вызываются через aic_call (read) и aic_sendRawTransaction (write).

## Команды

Скомпилировать контракты (при изменениях в .sol):
  cd packages/contracts
  forge build

Запустить тесты контрактов:
  forge test

Запустить ноду (контракты уже в genesis):
  cd C:\Users\КОЛЮНЯ\aic-os
  .\target\debug\aic-node.exe

## Никаких faucet, mainnet, Sepolia, Etherscan

Это не нужно. Все контракты -- у нас.

## Проверка

resolve('alice') должен вернуть адрес или 0x0:
  cast call 0x0000000000000000000000000000000000a1c1d0 "resolve(string)(address)" "alice" --rpc-url http://127.0.0.1:8545

## Что НЕ надо делать

- forge create -- мы не создаём контракты, они уже в genesis.
- forge script script/DeployAll.s.sol --broadcast -- скрипт устарел.
- cast wallet import -- нет deployer'а, нет чужих сетей.
- faucet для Sepolia/mainnet -- не нужно.
