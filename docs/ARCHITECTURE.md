# Архитектура AIC-OS

## 1. Общая картина

AIC Chain (своя сеть) -- здесь живут все контракты и все деньги:

- packages/node (Rust)
  - Block producer (3 сек)
  - EVM (revm 43) -- read + write
  - P2P (libp2p: Kademlia + Gossipsub + Sync)
  - PQ (Schnorr + ML-DSA-65, threshold 1000 AIC)
  - State (accounts + contracts + storage)
  - Persistence (chain.json)
- Genesis contracts (Solidity, вшиты в genesis):
  - AICID (0x...a1c1d0) -- register @name
  - InheritanceVault (0x...dead)
  - TimeCapsule (0x...c4b5e0)

Кошелёк (packages/core, Rust) -- только для чтения чужих сетей:

- BIP-39/BIP-32 -- один сид на 150 сетей
- secp256k1 + EIP-155/1559 -- отправка ETH (если пользователь сам захочет)
- 150 сетей -- чтение балансов + курсы USD/RUB
- AIC-ID resolve -- через RPC AIC Chain

## 2. AIC Chain -- детали

### Нода (packages/node)
- Язык: Rust
- RPC: axum (HTTP JSON-RPC на 8545)
- P2P: libp2p 0.56
- EVM: revm 43 (CfgEnv chain_id 2026)
- Persistence: chain.json (serde + кастомные map_bytes20/map_bytes32/map_storage)

### Модули
| Файл | Назначение |
|---|---|
| main.rs | Точка входа, CLI, создание chain, запуск P2P + RPC |
| chain.rs | Chain, Block, TxReceipt, produce_block, add_block_from_peer, apply_evm_tx, init_or_genesis |
| block.rs | BlockHeader, RLP-сериализация |
| transaction.rs | Transaction (Legacy EIP-155), pq_message, verify, verify_pq |
| state.rs | State (accounts + contracts + storage), to_inmemory для revm |
| mempool.rs | Mempool (Arc<RwLock<HashMap>>) |
| rpc.rs | JSON-RPC: chainId, blockNumber, getBalance, getAccount, getBlockByNumber, aic_call, aic_sendRawTransaction |
| p2p.rs | AicBehaviour: Kademlia + Gossipsub + Sync + Identify + Ping |
| genesis_contracts.rs | Загрузка runtime bytecode из Foundry artifacts + деплой в state |
| serde_helpers.rs | Сериализация HashMap с бинарными ключами |

### Правила исполнения транзакций
- Если tx.data.len() > 0 или tx.to -- контракт -> EVM (revm).
- Иначе -> простой transfer AIC.
- Комиссия за газ -> TREASURY.
- Если tx.value > 0 и tx успешна -> дополнительно tx.value -> TREASURY (монетизация).
- PQ: если tx.value >= 1000 AIC -- требуется Dilithium-подпись.

### P2P протоколы
- Kademlia DHT -- discovery пиров.
- Gossipsub (топики aic/blocks/1 и aic/txs/1) -- broadcast.
- Request-Response (/aic/sync/1) -- SyncRequest -> блоки, SyncResponse -> применяем.
- Identify + Ping -- служебные.

## 3. Кошелёк (packages/core)
Только для чтения чужих сетей и отправки ETH/BTC пользователем по его желанию.

## 4. Безопасность
- PQ-подписи (Schnorr + ML-DSA-65).
- Приватные ключи -- только в Secure Enclave (фаза UI).
- Социальное восстановление (3-5 хранителей).
- Мессенджер/VPN -- sandbox (фаза UI).

## 5. Что НЕ делаем
- НЕ деплоим AICID/InheritanceVault/TimeCapsule в Ethereum/Base/Sepolia.
- НЕ платим газ в чужих сетях за операции пользователя.
- НЕ зависим от внешних API для базовых операций AIC Chain.
