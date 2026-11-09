# 🗺️ ROADMAP AIC-OS

Актуально на: 2026-11-10 (Сессия #32)

## 🟢 Фаза 0 — Фундамент (ЗАВЕРШЕНО)
- [x] Репозиторий aic-os/aic-os (GitHub)
- [x] Rust workspace (packages/core, packages/node)
- [x] Foundry (packages/contracts)
- [x] .ai/MEMORY.md + автогенерация PROJECT_TREE
- [x] CI (GitHub Actions: fmt + clippy + test)

## 🟢 Фаза 1 — Кошелёк (ЗАВЕРШЕНО 95%)
### Rust-core (aic-core)
- [x] BIP-39 (12/24, seed с passphrase, spec vectors)
- [x] BIP-32 derivation (m/44'/60'/0'/0/i)
- [x] secp256k1 + EIP-55
- [x] Legacy ETH tx (EIP-155) + EIP-1559
- [x] ERC-20 (balanceOf, decimals, symbol, name, transfer)
- [x] **150 сетей** -- live балансы + курсы USD/RUB (CoinGecko)
- [x] AIC-ID: resolve/reverseResolve в Rust
- [x] 70 тестов

## 🟢 Фаза 1.5 — Смарт-контракты (ЗАВЕРШЕНО)
- [x] AICID.sol (register/resolve/reverse/transfer/renew, 365d)
- [x] InheritanceVault.sol (dead man's switch)
- [x] TimeCapsule.sol (lock ETH until timestamp)
- [x] 51 тест Foundry
- [x] Деплой-скрипты + DEPLOY.md
- [x] Dry-run mainnet

## 🟢 Фаза 2 — AIC Chain (В РАБОТЕ, ~75%)
### ✅ Работает
- [x] Rust-нода (axum + tokio + revm 43)
- [x] Block producer (3 сек)
- [x] Genesis 71 млрд AIC (40/30/20/10 + dev 1M)
- [x] Подпись EIP-155 (secp256k1) + verify
- [x] Первая транзакция AIC Alice→Bob (блок #21)
- [x] Persistence: chain.json save/load + storage контрактов
- [x] **EVM read (aic_call)**
- [x] **EVM write (aic_sendRawTransaction)**
- [x] Деплой 3 контрактов в genesis (runtime из deployedBytecode)
- [x] **AICID.register() -- 10 AIC в казну (проверено +10.0012 AIC)**
- [x] Архитектура: комиссии забирает нода (tx.value -> TREASURY)
- [x] 121 тест (70 Rust + 51 Solidity)

### 🟡 В работе
- [ ] InheritanceVault.deposit() = 1 AIC в казну
- [ ] TimeCapsule.create() = 1 AIC в казну
- [ ] Продление @name = 5 AIC в казну

### 🔵 Не начато
- [x] **P2P (несколько нод, gossip блоков) -- РАБОТАЕТ** (2 ноды синхронизируются, блоки #2..#4 приняты)
- [x] **PQ-подписи (Schnorr + ML-DSA-65 / Dilithium) -- РАБОТАЕТ** (проверено: 5000 AIC без PQ отклоняется, с PQ -- принимается)
- [ ] Мастер-ноды, DPoS консенсус (сейчас любой может майнить)
- [ ] Sync истории блоков при подключении (Request-Response готов, не активирован)
- [ ] Mempool gossip (broadcast чужих tx)
- [ ] RandomX PoW (анти-51%)
- [ ] Мост aCOIN (DKG 7/10, карантин)
- [ ] DEX-агрегатор (1inch/Jupiter/0x) в кошельке

## 🟡 Фаза 3 — AIC-ID в AIC Chain (В РАБОТЕ)
- [x] Register @name за AIC (10 AIC в казну)
- [x] Resolve/reverse нативно
- [ ] Социальное восстановление (3–5 хранителей, мультисиг)
- [ ] Интеграция AIC-ID resolve в aic-core (Rust) через RPC ноды

## 🔵 Фаза 4 — UI + Мессенджер + VPN
- [ ] Flutter или React Native приложение
- [ ] QR-код приём/отправка
- [ ] Matrix-мессенджер (sandbox)
- [ ] WireGuard VPN (sandbox)
- [ ] Proof of Luck (лотерея при проверке блоков)

## 🔵 Фаза 5 — Публичный запуск
- [ ] Аудит консенсуса и PQ-подписей
- [ ] 10+ мастер-нод
- [ ] Airdrop 100 AIC первым 10 000 пользователей
- [ ] Листинг на DEX (Uniswap/Sushi через aCOIN-мост)

## 📊 Метрики
- **Сетей в кошельке:** 150 (EVM 80, UTXO 10, Cosmos 36, Other 24)
- **Rust-тестов:** 74
- **Solidity-тестов:** 51
- **Всего тестов:** 125
- **Chain ID AIC:** 2026
- **Эмиссия AIC:** 71 000 000 000 (71 млрд)
- **Стартовая цена:** $10
- **Цель 2030:** $120–150
- **Комиссии в казну:** 10 AIC/регистрация + газ
- **Контрактов в genesis:** 3 (AICID, InheritanceVault, TimeCapsule)
- **PQ-защита:** Schnorr + ML-DSA-65 для value >= 1000 AIC
- **P2P:** 2+ ноды синхронизируются (libp2p, Kademlia, Gossipsub)
## 🎯 Ближайшие 3 приоритета
1. **PQ-подписи (Schnorr + Dilithium)** -- квантовая устойчивость
2. **P2P (несколько нод)** -- децентрализация
3. **Монетизация** InheritanceVault/TimeCapsule (1 AIC в казну каждая)

## 🏛️ Зафиксированные технические решения
- Ядро: Rust (tokio, reqwest, k256, bip39, bip32)
- Нода: axum + revm 43 + serde_json
- Контракты: Solidity 0.8.24 + Foundry 1.9.0
- **Монетизация на уровне ноды** (tx.value -> TREASURY), не через call в контракте
- Комиссии в AIC, ни цента ETH (Принцип №1)
- PQ: Schnorr (все tx) + Dilithium (критичные операции)
- Мессенджер: Matrix, VPN: WireGuard (sandboxed)
- DEX: агрегатор (1inch/Jupiter/0x)


