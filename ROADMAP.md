# 🗺️ ROADMAP AIC-OS

Актуально на: 2026-11-09 (Сессия #22)

## 🟢 Фаза 0 — Фундамент (ЗАВЕРШЕНО)
- [x] Репозиторий на GitHub (aic-os/aic-os)
- [x] Rust workspace (`packages/core`)
- [x] Foundry для контрактов (`packages/contracts`)
- [x] `.ai/MEMORY.md` + автогенерация `PROJECT_TREE.md`
- [x] GitHub Actions CI (fmt + clippy + test на Linux/Win/Mac)

## 🟢 Фаза 1 — Ядро кошелька (ЗАВЕРШЕНО ~85%)
### Rust-core (`aic-core`)
- [x] BIP-39 (12/24 слов, seed с passphrase, официальные BIP-39 spec vectors)
- [x] BIP-32 derivation (m/44'/60'/0'/0/i)
- [x] secp256k1 + EIP-55 checksum (Hardhat vectors проходят)
- [x] Legacy ETH transaction (EIP-155, RLP-encode, sign, broadcast)
- [x] EIP-1559 Type 2 (baseFee/maxPriority/maxFee, recovery test)
- [x] ERC-20: balanceOf, decimals, symbol, name, transfer calldata
- [x] AIC-ID: resolve(@name) / reverseResolve(addr) в Rust
- [x] Мультичейн-абстракция (Chain / ChainType / ChainTraits / ProviderFactory)
- [x] **107 сетей** — live balances + курсы USD/RUB через CoinGecko
  - EVM: 66, UTXO: 10, Cosmos: 25, Other: 6
- [x] RPC fallback (по списку endpoint'ов)
- [x] 70 тестов

### Смарт-контракты (`packages/contracts`)
- [x] **AICID.sol** — регистрация `@name`, resolve, reverse, transfer, renew (365 дней), валидация `[a-z0-9_-]{1,32}`
- [x] **InheritanceVault.sol** — dead man's switch (deposit / ping / claim / withdraw / changeBeneficiary)
- [x] **TimeCapsule.sol** — блокировка ETH до timestamp (create / topUp / claim / cancel)
- [x] 53 теста (Foundry)
- [x] Скрипт деплоя `DeployAICID.s.sol`
- [x] End-to-end проверка на Anvil (все три контракта работают)
- [x] Публичный deployer-кошелёк + keystore (план)

## 🟡 Фаза 1.5 — Добить до 150+ сетей (В РАБОТЕ)
- [ ] +16 сетей Other: NEAR, ADA, ALGO, XLM, DOT, ICP, KAS, XMR, XTZ, EGLD, HBAR, FLOW, VET, THETA, XDC, BAND
- [ ] Отправка BTC (PSBT + UTXO selection + SIGHASH)
- [ ] Отправка SOL (ed25519 + recent blockhash)
- [ ] Отправка TON (BOC + seqno)
- [ ] Отправка TRX, SUI, XRP, APT

## 🟡 Фаза 1.6 — Публичный деплой (В РАБОТЕ)
- [ ] Деплой AICID + InheritanceVault + TimeCapsule на Sepolia
- [ ] Верификация на Etherscan
- [ ] Деплой на Ethereum mainnet (~0.002 ETH на газ, ждём бюджет)
- [ ] Деплой на Polygon, BSC, Base, Arbitrum (мультичейн-контракты)

## 🔵 Фаза 2 — AIC Chain (ЗАПЛАНИРОВАНО)
- [ ] `packages/node` — нода на Rust
- [ ] Genesis-блок + генезис-аллокация
- [ ] Гибридный консенсус: DPoS мастер-ноды + RandomX PoW
- [ ] PQ-подписи (Schnorr + ML-DSA-65 / Dilithium)
- [ ] Mempool, storage, RPC
- [ ] Мост aCOIN с DKG (7/10 мультисиг) + карантин
- [ ] Лотерея «Proof of Luck» (1 AIC случайно при проверке блоков)

## 🔵 Фаза 3 — AIC-ID в AIC Chain (ЗАПЛАНИРОВАНО)
- [ ] AICID контракт деплой в AIC Chain (тот же код, новая сеть)
- [ ] Регистрация `@name` с оплатой в AIC
- [ ] Интеграция резолва в кошелёк (Rust)
- [ ] Социальное восстановление (3–5 хранителей, мультисиг)

## 🔵 Фаза 4 — UI + Мессенджер + VPN (ЗАПЛАНИРОВАНО)
- [ ] Flutter или React Native приложение
- [ ] Интеграция `aic-core` через FFI
- [ ] QR-код приём/отправка
- [ ] DEX-агрегатор (1inch / Jupiter / 0x)
- [ ] Matrix-мессенджер (sandboxed)
- [ ] WireGuard VPN (sandboxed, не имеет доступа к ключам)

## 🔵 Фаза 5 — CoinGecko + fiat (ЧАСТИЧНО)
- [x] Курсы USD/RUB для всех 100+ сетей
- [ ] Fiat-онрамп (MoonPay / Ramp) — покупка AIC за карту
- [ ] Отображение портфеля в USD/RUB

## 📊 Метрики текущего состояния
- Сетей: **107**
- Rust-тестов: **70**
- Solidity-тестов: **53**
- Контрактов задеплоено (Anvil): **3**
- Контрактов задеплоено (публично): **0**
- Строк кода (Rust+Solidity): ~4500

## 🎯 Ближайшие 3 приоритета
1. **Добить до 150 сетей** + отправка BTC/SOL (полный цикл non-EVM)
2. **Публичный деплой** контрактов на Sepolia (публичная веха)
3. **AIC Chain scaffold** — genesis, block struct, mempool (Фаза 2)

## 🏛️ Технические решения (зафиксированы)
- Ядро: Rust (tokio async, reqwest, k256, bip39, bip32)
- Контракты: Solidity 0.8.24 + Foundry 1.9.0
- Гибридный консенсус AIC Chain: DPoS + RandomX PoW (Фаза 2)
- Квантовая защита: Schnorr (все tx) + ML-DSA-65 (критичные операции)
- AIC-ID: ENS-подобный контракт, `@name -> address`
- Наследство: Dead Man's Switch (InheritanceVault), 30d–10y
- TimeCapsule: блокировка ETH до timestamp, 1h–100y
- Курсы: CoinGecko simple/price (USD+RUB одним запросом)
- Мессенджер: Matrix (не Telegram API) — sandbox
- VPN: WireGuard — sandbox, без доступа к ключам
- Майнинг в кошельке: Proof of Luck (не настоящий PoW)
- DEX: агрегатор (1inch/Jupiter/0x), не свой AMM
