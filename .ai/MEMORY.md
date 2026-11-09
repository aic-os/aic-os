# AIC-OS — ПАМЯТЬ ПРОЕКТА

> Этот файл — «долговременная память» AI-ассистента.
> В новом чате — прикрепить первым сообщением.
> Обновлять в конце каждой сессии: добавлять запись в «Журнал решений» и обновлять «Следующие задачи».

## 🎯 Видение
AIC-OS — операционная система для крипто-активов: мега-мультичейн-кошелек + собственный блокчейн + встроенная биржа + AIC-ID + наследование + мессенджер + VPN.
Цель: превзойти Binance / OKX / Trust / Gem / Bitget по UX и связке функций.

## 💰 Токеномика
- **AIC (AI Coin)** — нативная монета проекта.
- **aCOIN** — обёрнутые активы (aBTC, aETH и т.д., префикс `a`).

## ✅ Ключевые решения (принято)
- [2026-11-09] Ядро и нода — **Rust**.
- [2026-11-09] Мобильный кошелек — Flutter или React Native (TBD).
- [2026-11-09] Мультичейн-архитектура: `ChainType` enum + `ChainTraits` + `ProviderFactory` + `Settings.yaml` (идея из Gem Wallet).
- [2026-11-09] Гибридный консенсус: **DPoS мастер-ноды (финализация) + легкий PoW RandomX (анти-51%)**.
- [2026-11-09] Квантовая защита: **гибридные подписи** — Schnorr для всех транзакций, Dilithium (ML-DSA-65) для критичных (крупные суммы, наследство).
- [2026-11-09] AIC-ID `@name` → адрес через смарт-контракт (ENS-подобный).
- [2026-11-09] Наследство: Dead Man's Switch + автоконвертация в стейблкоин перед передачей.
- [2026-11-09] DEX: **агрегатор** (1inch, Jupiter, 0x), не свой AMM.
- [2026-11-09] Мессенджер: **Matrix** (open-source), НЕ Telegram API.
- [2026-11-09] VPN: **WireGuard**, изолирован от кошелька.
- [2026-11-09] Майнинг в кошельке: не настоящий PoW, а **Proof of Luck** (лотерея за проверку блоков).
- [2026-11-09] Стратегия запуска: сначала MVP-кошелек (Фаза 1), затем AIC Chain testnet (Фаза 2).

## ⛔ Запреты и осторожности
- НЕ хранить ключи рядом с мессенджером/VPN — sandbox-изоляция.
- НЕ обещать «майнинг на телефоне», который жжет батарею.
- НЕ делать мосты на все 150 сетей сразу — курируемый список + карантин.
- НЕ копировать код Bitcoin дословно — пишем свой.
- Постквантовые подписи тяжёлые — использовать «ленивую» схему, не в каждой транзакции.

## 🗂️ Стек (предварительно)
- `packages/core` — Rust: крипто, chain traits, providers
- `packages/node` — Rust: нода AIC Chain
- `packages/contracts` — Solidity: AIC-ID, Inheritance, Bridge, TimeCapsule
- `packages/wallet` — Flutter / RN (TBD)
- `packages/sdk` — TypeScript: интеграции
- `tools/scripts` — PowerShell автоматизация

## 📍 Текущая фаза
**Фаза 0: Фундамент** — структура, документация, память, окружение.

## 🎯 Следующие задачи
- [ ] Скаффолд `packages/core` (Rust): Cargo workspace, `ChainType`, `ChainTraits`
- [ ] Настроить GitHub Actions (CI: fmt, clippy, test)
- [ ] Выбрать мобильный стек: Flutter vs React Native
- [ ] Создать первый смарт-контракт AIC-ID (Solidity, Foundry)

## 📓 Журнал решений

### 2026-11-09 — Сессия #1
- Согласована архитектура из 4 фаз.
- Создан скелет репозитория: `.ai/`, `docs/`, `packages/`, `tools/`.
- Зафиксированы ключевые решения (см. выше).
- Определён workflow: скрипты `@'...'@` в PowerShell + обновление MEMORY.md в конце сессии.

### 2026-11-09 -- Сессия #1 (исправления)
- **ИНЦИДЕНТ:** box-drawing chars (└ ├ │), эмодзи и «ёлочки» ломаются в Windows PowerShell при вводе через heredoc (кодировка консоли CP866).
- **РЕШЕНИЕ:** в начале терминальной сессии выполнять:
chcp 65001
[Console]::OutputEncoding = [Console]::InputEncoding = [System.Text.Encoding]::UTF8

text
- **ПРАВИЛО:** в скриптах, которые передаются через терминал, использовать ТОЛЬКО ASCII (без эмодзи, box-chars, типографики). Для дерева: `|-- `, `\-- `, `|   `, `    `.
- Переписаны: `tools/scripts/update-tree.ps1`, `docs/ARCHITECTURE.md` -- теперь pure ASCII.
- README.md, ROADMAP.md, GLOSSARY.md содержат эмодзи в заголовках -- если отображаются как `?`, это косметика, поправим через VS Code.

### 2026-11-09 -- Сессия #3 (первый живой RPC)
- `aic-core` собран, компилируется без ошибок.
- Реализован `EvmProvider` с реальным JSON-RPC (`eth_getBalance`, `eth_blockNumber`).
- **ВЕХА:** получен живой баланс Виталика на Ethereum mainnet: `5.793820 ETH` (5793819533768043912 wei) через `https://ethereum-rpc.publicnode.com`.
- RPC-диагностика: `ethereum-rpc.publicnode.com` OK, `eth.drpc.org` OK, `cloudflare-eth.com` пустой, `eth.llamarpc.com` мертв (SSL 525).
- `rpc_call` устойчив: сначала text, потом JSON-парсинг, при не-JSON -- понятная ошибка с превью.
- Пример: `cargo run --example get_balance -p aic-core`.
- **Следующие задачи (Phase 1.2):** `estimate_fee` (eth_gasPrice + eth_estimateGas), `Settings.yaml` загрузчик с RPC-фолбэком, тесты.

### 2026-11-09 -- Сессия #4 (Settings.yaml + RPC fallback + мультичейн)
- Добавлен `serde_yaml_ng`, реализован `Settings` + `ChainConfig` (`config/settings.yaml`).
- `ProviderFactory::from_settings(&Settings)` строит фабрику из YAML.
- `ChainSettings.rpc_urls: Vec<String>` -- первый = primary, остальные = fallback.
- `EvmProvider::rpc_call` перебирает все RPC по очереди, собирает ошибки, возвращает первый успех.
- **ВЕХА:** живые балансы и комиссии с 3 mainnet-сетей одновременно:
  - Ethereum: 5.793820 ETH, fee 0.00002287 ETH
  - Polygon: 592.719867 MATIC, fee 0.00524279 MATIC
  - BSC: 0.170081 BNB, fee 0.00000105 BNB
- `config/settings.yaml` покрывает: ethereum, bsc, polygon, arbitrum, optimism, base, avalanche.
- **Следующее:** (1) таймауты в reqwest; (2) тесты для Settings и fallback; (3) `broadcast` через `eth_sendRawTransaction`; (4) подпись транзакций (secp256k1 + RLP) -- отдельный большой блок.

### 2026-11-09 -- Сессия #5 (timeouts + tests)
- `reqwest::Client` получил `timeout(5s)` и `connect_timeout(5s)` -- зависший RPC больше не вешает приложение.
- 11 тестов зелёные:
  - `tests/settings.rs` (6): парсинг YAML, алиасы сетей, chain_id, unknown-chain.
  - `tests/factory.rs` (5): создание EVM-провайдера с 1/несколькими RPC, отказ на unknown и на not-implemented.
- Для тестов, читающих `config/settings.yaml`, используем `env!("CARGO_MANIFEST_DIR")` + `../..` -- работает и локально, и в CI на любой ОС.
- **Следующее:** `broadcast` (eth_sendRawTransaction) -- принимает подписанный raw-hex, возвращает tx hash. Плюс тест на невалидный hex.

### 2026-11-09 -- Сессия #6 (broadcast)
- `EvmProvider::broadcast` реализован через `eth_sendRawTransaction`.
- Валидация raw-hex: префикс `0x`, только hex-символы, чётная длина (иначе ошибка до похода в RPC).
- Ошибки от RPC прокидываются с превью от каждого endpoint'а (фолбэк по всем).
- **14 тестов зелёные** (3 unit в evm.rs + 5 factory + 6 settings).
- Проверка на реальной сети: `broadcast("0xdeadbeef")` -- оба RPC корректно отвергли, газ не потрачен.
- **Следующее:** Wallet-модуль. Block A: BIP-39 (генерация сид-фразы, валидация, восстановление). Block B: secp256k1 + Ethereum-адрес. Block C: подпись ETH-транзакции (RLP + EIP-155).

### 2026-11-09 -- Сессия #7 (Block A: BIP-39)
- `wallet::Mnemonic` + `WordCount` (12 / 24).
- Методы: `generate`, `from_phrase`, `validate`, `phrase`, `word_count`, `to_seed`.
- Нормализация: case-insensitive, коллапс пробелов (как в MetaMask/Ledger).
- 12 новых тестов, включая ОФИЦИАЛЬНЫЙ BIP-39 test vector: `to_seed("TREZOR")` == hex из spec.
- Всего тестов в проекте: **26** (3 evm + 5 factory + 6 settings + 12 mnemonic).
- **Следующее:** Block B -- secp256k1 + Ethereum адрес (BIP-32 derivation m/44'/60'/0'/0/i, EIP-55 checksum).

### 2026-11-09 -- Сессия #8 (Block B: secp256k1 + ETH-адрес)
- `wallet::EthAccount` + `derive_account(mnemonic, index)`.
- Путь: `m/44'/60'/0'/0/{index}` (BIP-44, стандарт MetaMask).
- Приватный ключ zeroized на Drop.
- EIP-55 checksum реализован вручную (Keccak256 + mixed-case по нибблам хеша).
- **8 новых тестов, все зелёные, включая:**
  - Hardhat test mnemonic -- адрес и privkey совпадают с известным вектором (`0xf39Fd...`).
  - 3 официальных EIP-55 spec vector.
- Всего в проекте: **34 теста**.
- **Следующее:** Block C -- подпись ETH-транзакции (RLP-encode + EIP-155 chain_id) → broadcast на mainnet.

### 2026-11-09 -- Сессия #9 (Block C: подпись ETH-транзакции)
- `wallet::LegacyTransaction` (Type 0) + `SignedTransaction`.
- `signing_hash()` -- keccak256(rlp([nonce, gasPrice, gasLimit, to, value, data, chain_id, 0, 0])).
- `sign(&account)` -- secp256k1 + EIP-155 (`v = rec_id + chain_id * 2 + 35`), RLP-encode 9 полей.
- **Официальный EIP-155 test vector проходит:** signing hash = `0xdaf5a779...`.
- Всего тестов в проекте: **36**.
- Rust-ядро теперь умеет полный цикл: seed -> address -> build tx -> sign -> broadcast.
- **Следующее (Session #10):** пример `send_eth.rs`, который собирает tx, подписывает, отправляет в mainnet (запуск с 0 балансом даст корректную RPC-ошибку -- докажем цикл). Потом -- `get_nonce` helper и `Settings.yaml` расширим (SOL, BTC, TON).
