# 🗺️ ROADMAP AIC-OS

Актуально на: 2026-11-09 (Сессия #29)

## 🟢 Фаза 0 — Фундамент (ЗАВЕРШЕНО)

## 🟢 Фаза 1 — Кошелёк (ЗАВЕРШЕНО ~90%)
- [x] BIP-39, BIP-32, secp256k1, EIP-55
- [x] Legacy ETH tx (EIP-155), EIP-1559
- [x] ERC-20 (balanceOf, decimals, symbol, name, transfer)
- [x] **150 сетей** с балансами и курсами USD/RUB
- [x] AIC-ID resolve/reverseResolve в Rust
- [x] 70 тестов Rust

## 🟢 Фаза 1.5 — Смарт-контракты (ЗАВЕРШЕНО)
- [x] AICID.sol (register, resolve, reverse, transfer, renew, 365d)
- [x] InheritanceVault.sol (dead man's switch)
- [x] TimeCapsule.sol (lock ETH until timestamp)
- [x] 53 теста Foundry
- [x] Деплой-скрипты, DEPLOY.md runbook
- [x] Dry-run mainnet (симуляция без broadcast)

## 🟡 Фаза 2 — AIC Chain (В РАБОТЕ, ~60%)
### ✅ Работает:
- [x] `packages/node` на Rust (axum + tokio)
- [x] Block, Transaction, State, Mempool, Chain
- [x] Block producer (3 сек)
- [x] Genesis 71 млрд AIC (40/30/20/10 + dev)
- [x] RPC: chainId, blockNumber, getBalance, getAccount, getBlockByNumber, sendRawTransaction
- [x] Подпись EIP-155 (secp256k1), verify
- [x] **Первая транзакция:** Alice→Bob 1000 AIC, блок #21, gas 21000
- [x] Persistence (chain.json save/load)
- [x] 71 тестов Rust

### 🟡 EVM (в процессе):
- [x] revm 43 в dependencies
- [x] AICID, InheritanceVault, TimeCapsule деплой в genesis
- [x] `aic_call` принимает данные, code находится
- [ ] **ПРОБЛЕМА:** `evm.transact(tx)` в revm 43 возвращает CREATE-output вместо CALL-output
- [ ] Ищем правильный API-вызов (transact_commit, Evm::builder, ...)

### 🔵 Не начато:
- [ ] P2P (несколько нод, gossip блоков)
- [ ] Мастер-ноды, DPoS консенсус
- [ ] RandomX PoW (анти-51%)
- [ ] PQ-подписи (Schnorr + Dilithium)
- [ ] Мост aCOIN с DKG

## 🔵 Фаза 3 — AIC-ID в AIC Chain
- [ ] Register @name за AIC
- [ ] Интеграция с Rust-ядром

## 🔵 Фаза 4 — UI + Мессенджер + VPN
- [ ] Flutter/RN приложение
- [ ] QR-код, DEX-агрегатор
- [ ] Matrix-мессенджер, WireGuard VPN

## 🔵 Фаза 5 — Публичный деплой
- [ ] AICID на Base/Sepolia (если нужно)
- [ ] Mainnet AIC Chain (когда аудит)

## 📊 Метрики
- Сетей: **150**
- Rust-тестов: **71**
- Solidity-тестов: **53**
- Блоков AIC Chain: **~30** (за тест)
- Транзакций: **1** (успешная)
- Контрактов в genesis: **3**
- **Токеномика:** 71 млрд AIC, старт $10, цель 2030 $120-150

## 🎯 Приоритет
1. **Добить EVM в AIC Chain** (сейчас в работе)
2. P2P + DPoS
3. PQ-подписи
4. UI
