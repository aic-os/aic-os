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

### 2026-11-09 -- Сессия #10 (полный цикл ETH)
- **ВЕХА: полный цикл Ethereum работает end-to-end.**
- `send_eth.rs` от Hardhat test-мнемоники:
  - derive → 0xf39Fd6... (совпадает с Hardhat vector)
  - nonce = 11632 (эта публичная мнемоника уже использовалась в mainnet)
  - build LegacyTransaction (1 wei, 21000 gas, chain_id 1)
  - sign (EIP-155) → raw_hex `0xf86582...`
  - broadcast → RPC отверг по insufficient funds (ожидаемо)
- Добавлены методы `ChainTraits::get_nonce` (pending) и `ChainTraits::get_gas_price`.
- `estimate_fee` теперь использует `get_gas_price`.
- Тесты: **36 зелёных**.
- Rust-ядро для Ethereum -- готово. Есть всё: сид-фраза, адрес, баланс, fee, nonce, подпись, broadcast.
- **Следующее -- кандидаты:**
  - A) EIP-1559 (Type 2 tx) -- MetaMask default, современный формат.
  - B) ERC-20 (balanceOf, transfer) -- база для aCOIN и USDT/USDC.
  - C) AIC-ID контракт (Solidity + Foundry) -- уникальная фишка.

### 2026-11-09 -- Сессия #11 (Block B-1: ERC-20 balanceOf)
- `providers/evm_abi.rs` -- минимальный ABI-кодек:
  - `selector(sig)` -- keccak256(sig)[0..4]
  - `encode_address(0x...)` -- 32-байтный left-pad
  - `encode_u256(u128)` -- 32-байтный big-endian
  - `decode_u128(0x...)` -- с проверкой на overflow
- `EvmProvider::eth_call(contract, data)` -- raw eth_call.
- `ChainTraits::get_erc20_balance(contract, holder)` -- default NotImplemented, EVM -- реальный.
- **ВЕХА: живой USDC mainnet:** 52363119552 raw = 52 363.12 USDC у Binance hot wallet.
- Селекторы сверены с известными: balanceOf=0x70a08231, transfer=0xa9059cbb, decimals=0x313ce567, symbol=0x95d89b41.
- Всего тестов: **45**.
- **Следующее (Session #12, Block B-2):** ERC-20 metadata (`decimals`, `symbol`, `name`) + построение `transfer(address,uint256)` calldata. Потом Block A (EIP-1559) и Block C (AIC-ID).

### 2026-11-09 -- Сессия #12 (Block B-2: ERC-20 metadata + transfer calldata)
- `evm_abi` расширен: `decode_uint8`, `decode_string` (ABI-string с offset+length), `encode_transfer_calldata`.
- `ChainTraits`: `get_erc20_decimals`, `get_erc20_symbol`, `get_erc20_name` (default NotImplemented, EVM -- реальные).
- `wallet::erc20::build_transfer_tx` -- собирает unsigned LegacyTransaction для `transfer(address,uint256)` (value=0, data=calldata).
- **ВЕХА:** живая metadata USDC mainnet: name="USD Coin", symbol="USDC", decimals=6.
- 51 тест зелёный.
- **Следующее -- Block A (EIP-1559):** Type 2 транзакции (`maxFeePerGas`, `maxPriorityFeePerGas`), MetaMask-совместимый формат.

### 2026-11-09 -- Сессия #13 (Block A: EIP-1559 / Type 2)
- `wallet::Eip1559Transaction` + `SignedEip1559`.
- Signing hash: keccak256(0x02 || rlp([chain_id, nonce, max_priority, max_fee, gas, to, value, data, []])).
- Signed raw: 0x02 || rlp([...9 полей..., y_parity, r, s]).
- `ChainTraits`: `get_base_fee()` (eth_getBlockByNumber -> baseFeePerGas), `get_max_priority_fee()` (eth_maxPriorityFeePerGas).
- 5 тестов, включая **recovery**: подпись -> восстановление pubkey -> адрес совпал с исходным. Формат подписи корректен по EIP-1559.
- **ВЕХА:** живой raw EIP-1559 отправлен в mainnet: `0x02f86801...`, tx_hash `0x8d57ccc3...`, отвергнут по insufficient funds (ожидаемо для Hardhat-мнемоники).
- Всего тестов: **56**.
- **Следующее -- Block C (AIC-ID):** смарт-контракт ENS-подобный, Foundry, регистрация/разрешение имён.

### 2026-11-09 -- Сессия #14 (Block C: AICID.sol)
- `packages/contracts` инициализирован через Foundry 1.9.0 (Solc 0.8.24, forge-std).
- Контракт `AICID.sol`:
  - `register(name, owner)` -- регистрация, 365 дней, отказ при дубликате неистёкшего имени.
  - `renew(name)` -- продление (только owner).
  - `transferName(name, newOwner)` -- трансфер, обновляет reverse-map.
  - `resolve(name)` -- имя -> адрес (address(0) если свободно/истекло).
  - `reverseResolve(addr)` -- адрес -> имя.
  - `isAvailable(name)` / `isValidName(name)`.
  - Валидация: `[a-z0-9_-]`, длина 1..32.
  - `_now()` с `// forge-lint: disable-next-line(unsafe-typecast)` (Safe: block.timestamp < uint64 max >500M лет).
- Foundry `[lint]` -- отключён `block-timestamp` (манипуляция ~12s vs окно 365 дней).
- **14/14 тестов зелёные**, включая expiry через `vm.warp` и reverse-map consistency после transfer.
- **Следующее:** деплой-скрипт (`script/DeployAICID.s.sol`) + реальный деплой (Anvil -> mainnet).

### 2026-11-09 -- Сессия #15 (deploy script + dry-run)
- `packages/contracts/script/DeployAICID.s.sol` -- Foundry Script, деплоит AICID.
- **Dry-run на форке mainnet прошёл:** `Script ran successfully`, `REGISTRATION_PERIOD = 31536000`, estimated gas 1,130,945, cost ~0.0006 ETH.
- Реальный деплой откладывается до готовности deployer-кошелька (keystore через `cast wallet import`).
- Убрано из git: `foundry-install.ps1`, `packages/contracts/lib/forge-std/` (deps через `forge install`).
- Добавлено в `.gitignore`: `packages/contracts/lib/`.
- **Следующее:** создать deployer keystore, пополнить его ETH, реальный деплой на mainnet + верификация на Etherscan.
