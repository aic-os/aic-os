# 🗺️ ROADMAP AIC-OS

Актуально на: 2026-11-10 (AIC-OS v0.2)

## 🟢 Фаза 0 — Фундамент (ЗАВЕРШЕНО)
- [x] Репозиторий aic-os/aic-os (GitHub)
- [x] Rust workspace (packages/core, packages/node)
- [x] .ai/MEMORY.md + автогенерация PROJECT_TREE
- [x] CI (GitHub Actions: fmt + clippy + test)

## 🟢 Фаза 1 — Кошелёк (ЗАВЕРШЕНО)
Мультичейн-кошелёк для чтения 150 сетей и отправки ETH/BTC/SOL. Не отправляет наши контракты в чужие сети.
- [x] BIP-39, BIP-32, secp256k1, EIP-55
- [x] Legacy ETH tx (EIP-155) + EIP-1559
- [x] ERC-20 (balanceOf/decimals/symbol/name/transfer)
- [x] 150 сетей: live балансы + курсы USD/RUB (CoinGecko)
- [x] AIC-ID resolve через RPC AIC Chain
- [x] 74 Rust-теста

## 🟡 Фаза 2 — AIC Chain (В РАБОТЕ ~85%)
**Наша собственная децентрализованная криптовалюта. Всё живёт здесь.**

### ✅ Работает
- [x] Rust-нода (axum + tokio + revm 43 + libp2p 0.56)
- [x] Chain ID 2026, block producer (3 сек)
- [x] Genesis: 71 млрд AIC (40/30/20/10 + dev 1M)
- [x] Подпись EIP-155 (secp256k1) + verify
- [x] Первая транзакция AIC Alice→Bob (блок #21)
- [x] Persistence: chain.json (блоки, state, storage контрактов)
- [x] **EVM работает** (revm 43): read + write
- [x] **3 контракта в genesis AIC Chain** (Solidity 0.8.24, деплой через revm при genesis):
  - AICID (`0x...a1c1d0`) -- register @name за 10 AIC → казна
  - InheritanceVault (`0x...dead`) -- dead man's switch
  - TimeCapsule (`0x...c4b5e0`) -- блокировка AIC до timestamp
- [x] **Монетизация:** 10 AIC/регистрация @name → казна (проверено: +10.0012 AIC на тест)
- [x] **Монетизация на уровне ноды:** tx.value → TREASURY (обходит баг revm с `call{value}`)
- [x] **PQ-подписи:** Schnorr (BIP-340) + ML-DSA-65 (Dilithium3) для value ≥ 1000 AIC (проверено: 5000 AIC без PQ отклоняется)
- [x] **P2P (libp2p):** Kademlia + Gossipsub + Request-Response
- [x] **Sync истории:** новая нода догоняет цепочку (B догнал A до #8)
- [x] **Mempool gossip:** broadcast tx через gossipsub
- [x] 125 тестов (74 Rust + 51 Solidity)

### 🔵 Не начато (для production-ready)
- [x] **DPoS консенсус (round-robin) -- РАБОТАЕТ** (A=B=9, без форков)
- [x] Валидаторы (CLI: --validators, --my-address)
- [ ] Награда за блок валидаторам
- [ ] Mempool dedup (re-broadcast чужих tx с флагом "уже видел")
- [ ] Мост aCOIN (DKG 7/10 + карантин)
- [ ] DEX-агрегатор внутри AIC Chain
- [ ] Социальное восстановление AIC-ID (3–5 хранителей, мультисиг)
- [ ] RandomX PoW (доп. защита от 51%)

## 🟡 Фаза 3 — UI (Flutter) -- В РАБОТЕ ~30%
- [x] Flutter-проект (packages/mobile/)
- [x] Главный экран: баланс AIC + block height + адрес (работает!)
- [x] RPC-клиент (AicRpc) -- читает из AIC Chain
- [x] CORS в ноде
- [x] WASM собран (7 функций для подписи)
- [x] Dart-обёртка WasmBridge (готова, не подключена)
- [ ] Красивый UI (кнопки, курсы, градиенты, авто-poll)
- [x] **Экран Отправить -- РАБОТАЕТ** (100 AIC, tx 0x885ca7..., Bob=115 AIC после)
- [x] Экран Получить (QR-код) -- работает
- [x] **AIC-ID регистрация через UI** -- работает (@alice за 10 AIC, казна +10)
- [x] Каталог 23 монет + добавление/удаление (портфель в UI)
- [x] **История транзакций** -- работает (aic_getAccountTransactions)
- [ ] **APK (вариант С: web3dart)** -- следующая веха

## 🔵 Фаза 4## 🔵 Фаза 4## 🔵 Фаза 4 — Публичный запуск
- [ ] Аудит консенсуса и PQ-подписей (внешний)
- [ ] 10+ мастер-нод (foundation + партнёры)
- [ ] Airdrop 100 AIC первым 10 000 пользователей
- [ ] Faucet 0.1 AIC/сутки
- [ ] Листинг aCOIN на внешних DEX (через наш мост)

## 🟢 Фаза 4.5 — Quantum Puzzle MVP (готово)
- [x] **QuantumBounty.sol** -- контракт в AIC Chain (13/13 тестов)
- [x] **Режим A (классический ECDSA):** solveClassic, 1M AIC
- [x] **Режим B (PQ, multisig):** votePQSolved + claimPQ, 2M AIC
- [x] fund(), setClassicChallenge, addValidator, setPQThreshold
- [x] Replay protection, access control
- [x] **Funding скрипт работает:** fund 300k + setPQReward 200k + setClassicChallenge 100k
- [x] UI Quantum Bounty -- показывает Total Prize Pool, Mode A/B, target address
- [ ] Deploy в mainnet (когда AIC Chain стабилен)

## 📊 Метрики## 📊 Метрики
- **Сетей в кошельке:** 150 (EVM 80, UTXO 10, Cosmos 36, Other 24)
- **Rust-тестов:** 74
- **Solidity-тестов:** 51
- **Всего тестов:** 125
- **Chain ID AIC:** 2026
- **Эмиссия AIC:** 71 000 000 000 (71 млрд)
- **Стартовая цена:** $10
- **Цель 2030:** $120–150
- **Монетизация:** 10 AIC/регистрация + газ → казна
- **PQ-защита:** Schnorr + ML-DSA-65 для value ≥ 1000 AIC
- **P2P:** 2+ ноды, sync истории, gossipsub

## 🎯 Ближайшие 3 приоритета
1. **DPoS консенсус** (финальный кусок production-ready сети)
2. **Мост aCOIN** (DKG 7/10)
3. **UI (Flutter/RN)**

## 🏛️ Технические решения
- **Ядро:** Rust (tokio, k256, bip39, bip32)
- **Нода:** axum + revm 43 + libp2p 0.56 + ml-dsa + serde_json
- **Контракты:** Solidity 0.8.24, компилятор Foundry, **деплой только в AIC Chain genesis** (никакого деплоя в чужие сети)
- **Монетизация:** на уровне ноды (tx.value → TREASURY)
- **PQ:** Schnorr + ML-DSA-65 для value ≥ 1000 AIC
- **P2P:** libp2p (Kademlia + Gossipsub + Request-Response)
- **Мессенджер:** Matrix (sandbox)
- **VPN:** WireGuard (sandbox)














