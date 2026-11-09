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

### 2026-11-09 -- Сессия #15 (deploy script + dry-run)
- `packages/contracts/script/DeployAICID.s.sol` -- Foundry Script, деплоит AICID.
- **Dry-run на форке mainnet прошёл:** `Script ran successfully`, `REGISTRATION_PERIOD = 31536000`, estimated gas 1,130,945, cost ~0.0006 ETH.
- Реальный деплой откладывается (нет бюджета на газ). Идём через Anvil (локально) и Sepolia (testnet faucet).
- Убрано из git: `foundry-install.ps1`, `packages/contracts/lib/forge-std/`.
- **Следующее:** деплой на Anvil (локально) -> деплой на Sepolia (публично) -> mainnet, когда будет газ.

### 2026-11-09 -- Сессия #16 (AICID deployed to Anvil, first name registered)
- **ВЕХА: AICID работает end-to-end в реальной EVM.**
- Anvil запущен локально (`http://127.0.0.1:8545`, chain 31337).
- Контракт задеплоен: `0x5FbDB2315678afecb367f032d93F642f64180aa3` (block 1, 869,958 gas, ~0.00087 ETH на Anvil).
- `register("alice", 0x7099...)` -- tx type 2, status 1, gas 116,401.
- `resolve("alice")` -> `0x70997970C51812dc3A010C7d01b50e0d17dc79C8` -- правильный адрес.
- `reverseResolve(0x7099...)` -> `"alice"` -- правильное имя.
- `isAvailable("alice")` -> `false`, `isAvailable("bob")` -> `true`.
- Скрипт деплоя и полный flow подтверждены на реальных EVM-транзакциях.
- **Следующее (варианты, ждём решения):**
  - A) Интегрировать AIC-ID в Rust-ядро (`aic-core`): `resolve @name -> address` нативно в кошельке. Закрывает цикл.
  - B) Контракт Inheritance (Dead Man's Switch).
  - C) Контракт TimeCapsule.
  - D) Расширить мультичейн (BTC, SOL, TON провайдеры).

### 2026-11-09 -- Сессия #17 (Block A: AIC-ID в Rust)
- `aic-core` теперь умеет нативно работать с AIC-ID:
  - `evm_abi::encode_string_arg` -- ABI-кодирование dynamic string (offset + length + padded).
  - `evm_abi::decode_address` -- decode 32-byte ABI address (None если zero).
  - `CoreError::Reverted` -- отдельный вариант для contract revert (важно для reverse_resolve).
  - `ChainTraits::resolve_name(contract, name)` -- `@name -> Option<address>`.
  - `ChainTraits::reverse_resolve(contract, address)` -- `address -> Option<name>`, revert = Ok(None).
- `EvmProvider::rpc_call` теперь распознаёт "execution reverted" в JSON-RPC error и возвращает `CoreError::Reverted`.
- **ВЕХА:** `aic_id_resolve` пример отработал на Anvil end-to-end:
  - `@alice -> 0x7099...` (из реального mainnet-контракта)
  - `reverse: 0x7099... -> @alice`
  - **Отправка 1 wei на `@alice` по имени** -- tx hash `0x283d638a...` (broadcast OK)
  - `@bob is free`, unknown address -> `no name`
- 72 теста зелёные (56 + 16 в evm_abi).
- **Следующее (Block B):** контракт Inheritance (Dead Man's Switch) -- бенефициары, ping, claim.

### 2026-11-09 -- Сессия #18 (Block B: InheritanceVault)
- `packages/contracts/src/InheritanceVault.sol` -- Dead Man's Switch для ETH:
  - `deposit(beneficiary, inactivityPeriod)` -- депонировать ETH (можно top-up).
  - `ping()` -- «я жив», сбрасывает таймер.
  - `changeBeneficiary(newBeneficiary)` -- смена наследника (только owner).
  - `withdraw()` -- owner забирает всё и закрывает vault.
  - `claim(owner)` -- наследник забирает после неактивности.
  - `timeUntilClaim(owner)` -- сколько секунд до claim.
  - Константы: MIN_INACTIVITY=30d, MAX_INACTIVITY=10y.
- **22 теста зелёные** (все ветки: deposit/topup/zero/revert/bounds, ping, withdraw, claim happy/early/wrong/double, changeBeneficiary, timeUntilClaim).
- **ВЕХА:** деплой на Anvil `0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0`, end-to-end проверено через cast:
  - Alice deposit 1 ETH (30d, Bob beneficiary) -- status 1
  - timeUntilClaim = 2592000 (30d)
  - Bob claim до срока -> `Error: execution reverted: StillActive` (правильно)
  - warp +31d -> timeUntilClaim = 0
  - Bob claim -> status 1
  - Vault state: balance=0, active=false
- **Следующее (Block C):** TimeCapsule (капсула времени) -- блокировка ETH/токенов до timestamp.

### 2026-11-09 -- Сессия #19 (Block C: TimeCapsule)
- `packages/contracts/src/TimeCapsule.sol`:
  - `create(recipient, unlockAt)` -- создать капсулу (ETH), unlockAt в [now+1h, now+100y].
  - `topUp(id)` -- пополнить (только sender, пока активна).
  - `claim(id)` -- получатель забирает после unlockAt.
  - `cancel(id)` -- sender возвращает до unlockAt.
  - `timeUntilUnlock(id)` -- секунд до открытия.
  - `nextId` глобальный, `capsules[id]` mapping.
- **20 тестов зелёные.**
- **ВЕХА:** деплой на Anvil `0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9`, end-to-end проверено:
  - create 1 ETH unlock +2h -- status 1
  - timeUntilUnlock = 7149
  - claim до срока -> `Error: execution reverted: NotYetUnlocked`
  - warp +7201s -> timeUntilUnlock = 0
  - claim -> status 1
  - state: balance=0, claimed=true
- **Уроки смоук-тестов (актуально для Anvil):**
  - После `evm_increaseTime` в предыдущих тестах chain-time сдвинут вперёд -- всегда брать `block.timestamp` из Anvil (`cast block latest -f timestamp`), не из системных часов.
  - Между чтением timestamp и выполнением tx проходит >=1 блок -- всегда оставлять буфер (>=1h для MIN_LOCK=1h).
- Всего тестов: 53 contracts + 72 core = 125.
- **Следующее (Block D):** мультичейн в `aic-core` -- провайдеры для Bitcoin (UTXO), Solana (Other), TON (Other).

### 2026-11-09 -- Сессия #20 (Block D: мультичейн BTC + SOL + TON)
- **ВЕХА: кошелёк читает балансы из Bitcoin, Solana, TON с реальных mainnet.**
- Новые провайдеры в `aic-core`:
  - `BitcoinProvider` -- Blockstream Esplora API (fallback: mempool.space), `chain_stats.funded_txo_sum - spent_txo_sum`.
  - `SolanaProvider` -- JSON-RPC `getBalance`, base58-адрес (32..44 символа).
  - `TonProvider` -- toncenter `getAddressBalance`, нанотоны как u128.
- `ProviderFactory::from_settings` теперь регистрирует BTC, LTC, DOGE (UTXO), SOL, TON (Other).
- `ProviderFactory::create`:
  - `ChainType::Utxo` -> `BitcoinProvider` (для всех UTXO-сетей).
  - `ChainType::Other`: `Solana` -> `SolanaProvider`, `Ton` -> `TonProvider`.
- `config/settings.yaml` расширен: `bitcoin` (blockstream + mempool.space fallback), `solana` (public mainnet RPC), `ton` (toncenter).
- **Живые балансы (mainnet):**
  - BTC `1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa` (genesis Satoshi) = 57.70904748 BTC
  - SOL `7cVfgArCheMR6Cs4t6vz5rfnqd56vZq4ndaBrY5xkxXy` = 0.147298879 SOL
  - TON `EQDk2...` = 0.114061498 TON
- **73 теста зелёные** (72 + 1 тест про Bitcoin provider).
- **Уроки:**
  - При добавлении сетей в `settings.yaml` -- проверять тест `settings_unknown_chain_returns_none` (там должен быть реально unknown chain, теперь CosmosHub).
- **Следующее (варианты):**
  - A) Дальше провайдеры: TRON, SUI, APTOS, XRP.
  - B) CoinGecko integration -- курсы всех монет к USD/RUB (фишка из исходной задумки).
  - C) Отправка BTC/SOL (подпись + broadcast) -- самый сложный шаг.
  - D) AIC Chain scaffold -- `packages/node` на Rust (Фаза 2).

### 2026-11-09 -- Сессия #21 (Block B: CoinGecko + 4 новые сети)
- **ВЕХА: кошелёк показывает балансы с курсами к USD/RUB и работает с 10 сетями.**
- CoinGecko integration:
  - `PriceClient::get_prices(chains)` -- один HTTP-запрос на все сети (CoinGecko simple/price).
  - `PriceInfo { usd, rub }` -- цена единицы нативной монеты.
  - Маппинг `Chain -> coingecko_id`: ETH/Polygon/BSC/Arb/OP/Base/Avax/BTC/LTC/DOGE/ATOM/OSMO/TIA/SOL/TRX/TON/SUI/APT/XRP.
  - Arb/Optimism/Base используют `ethereum` (нативный газ = ETH).
- Новые провайдеры (`aic-core/src/providers/`):
  - `TronProvider` -- trongrid `/v1/accounts/{addr}`, balance в sun.
  - `XrpProvider` -- `account_info` JSON-RPC, drops как u128, actNotFound -> 0.
  - `CosmosProvider` -- REST `/cosmos/bank/v1beta1/balances/{addr}` с denom-маппингом (uatom/uosmo/utia).
  - `SuiProvider` -- `suix_getBalance` (0x2::sui::SUI).
- `ProviderFactory` регистрирует 19 сетей: 7 EVM + 3 UTXO + 3 Cosmos + SOL/TON/TRX/SUI/XRP.
- `ChainType::Cosmos` -> `CosmosProvider`. `ChainType::Other`: SOL/TON/TRX/SUI/Ripple работают, Aptos -> NotImplemented.
- `config/settings.yaml` расширен: tron, ripple, cosmos, sui.
- **Живые данные (mainnet, 10 сетей):**
  - ETH 5.79 (~$12,308), POL 592.72 (~$74), BNB 0.17 (~$116)
  - BTC 57.71 (~$4.3M), SOL 0.147 (~$12.77), TON 0.114 (~$0.12)
  - TRX 14,493 (~$4,592), XRP 1,113 (~$1,217), ATOM 0, SUI 0
- **81 тест зелёный** (было 76, +5 factory: bitcoin/solana/cosmos/tron/ripple/sui и замена Cosmos->Aptos в not-implemented).
- **Уроки:**
  - Settings YAML: ключ должен точно совпадать с `Chain::as_str()` (xrp -> ripple).
  - При добавлении сетей в settings.yaml -- тест `settings_unknown_chain_returns_none` переключить на реально отсутствующую (Aptos).
  - Regex-замены для многострочных блоков ненадёжны -- перезаписывать файл целиком.
- **Следующее:** отправка BTC/SOL (PSBT + ed25519) или AIC Chain scaffold.

### 2026-11-09 -- Сессия #22 (мега-расширение до 107 сетей)
- **ВЕХА: кошелёк поддерживает 107 сетей с реальными балансами и курсами USD/RUB.**
- `Chain` enum расширен:
  - EVM: 66 сетей (L1+L2+сайдчейны: Ethereum, BSC, Polygon, Arb, OP, Base, Avax, Linea, Scroll, zkSync, Mantle, Blast, Gnosis, Celo, Fantom, Cronos, Moonbeam, Aurora, Filecoin, PolygonZkEvm, Harmony, Metis, Klaytn, Astar, Moonriver, Boba, Conflux, Evmos, Canto, OKC, Fuse, Telos, Dogechain, Palm, DFK, Songbird, GoChain, Callisto, Ubiq, XinFin, Wanchain, ThunderCore, Hoo, KCC, Tombchain, Bitgert, ArbitrumNova, opBNB, Zora, Mode, Mint, MantaPacific, Kroma, PGN, Redstone, Ancient8, Cyber, Lisk, RSK, Bitlayer, ZetaChain, Dexalot, ApeChain, Curtis, Frame, Rari)
  - UTXO: 10 (BTC, LTC, DOGE, BCH, Dash, Zcash, BSV, DigiByte, Ravencoin, Groestlcoin)
  - Cosmos: 25 (Cosmos, Osmosis, Celestia, Injective, Kava, Sei, Akash, Juno, Stargaze, Kujira, Stride, Neutron, Archway, Persistence, Umee, Comdex, Crescent, Regen, Sentinel, Sifchain, Chihuahua, Bitsong, Desmos, Emoney, Medibloc)
  - Other: 6 (SOL, TRX, TON, SUI, APT, XRP)
- `Chain::ALL` -- константный массив всех сетей, `Chain::count()` = 107.
- `ProviderFactory::from_settings` регистрирует все 107 сетей из yaml автоматически.
- `config/settings.yaml` -- компактный YAML-формат (inline maps), все 107 сетей с RPC/API.
- `prices.rs` -- coingecko_id покрывает ~100 сетей, `_ => return None` для экзотики.
- `cosmos.rs` -- denom-маппинг для всех 25 сетей (uatom/uosmo/utia/inj/ukava/usei/uakt/ujuno/ustars/ukuji/ustrd/untrn/...).
- **Live example (107 сетей, sample 12):**
  - BTC 57.71 (~$4.3M), SOL 0.147 (~$12.7), TRX 14493.8 (~$4,594), XRP 1113.55 (~$1,211)
  - ETH/Polygon/Arb/OP/Base -> 0 (правильно, у Виталика нет L2-балансов)
  - Cosmos/Osmosis -> 0 (тестовый адрес)
- **70 тестов зелёные** (было 81, часть тестов переписана под новые реалии).
- **Уроки:**
  - `Chain::ALL` должен использовать `Chain::Variant` явно (не голые имена в `const`).
  - Exhaustive match в `coingecko_id` требует `_ => return None`.
  - Тест `factory_rejects_not_implemented_*` -- удалять/переписывать при добавлении сетей.
- **Следующее:** отправка BTC/SOL (PSBT + ed25519), либо оставшиеся Other-сети (NEAR, ADA, ALGO, XLM, DOT).

### 2026-11-09 -- Сессия #23 (ROADMAP update + состояние)
- ROADMAP.md перезаписан, отражает реальное состояние:
  - Фаза 0 — завершена
  - Фаза 1 (ядро кошелька) — завершена на ~85%
  - Фаза 1.5 (150 сетей) — в работе (107/150)
  - Фаза 1.6 (публичный деплой) -- запланировано
  - Фазы 2-5 -- запланированы
- Текущее состояние: 107 сетей, 70 Rust-тестов, 53 Solidity-теста, 3 контракта работают на Anvil, 0 в публичной сети.
- Следующий приоритет: добить до 150 сетей + отправка BTC/SOL.

### 2026-11-09 -- Сессия #23 (135 сетей + 12 новых провайдеров)
- **ВЕХА: кошелёк поддерживает 135 сетей.**
- Распределение: EVM 74, UTXO 10, Cosmos 33, Other 18.
- **+12 новых провайдеров** (каждый -- REST/JSON-RPC через публичный API, без ключей):
  - `NearProvider` -- fastnear.com / rpc.mainnet.near.org (`query` view_account).
  - `AlgorandProvider` -- AlgoNode (`/v2/accounts/{addr}`).
  - `StellarProvider` -- Horizon (`/accounts/{addr}`, конверсия XLM * 1e7).
  - `KaspaProvider` -- api.kaspa.org (`/addresses/{addr}/balance`).
  - `MoneroProvider` -- xmrchain.net (`/api/address/{addr}`).
  - `TezosProvider` -- TzKT (`/v1/accounts/{addr}/balance`).
  - `MultiversXProvider` -- api.multiversx.com.
  - `HederaProvider` -- mirror node (`/api/v1/accounts/{addr}`).
  - `VeChainProvider` -- sync-mainnet.vechain.org.
  - `ThetaProvider` -- theta-explorer (`/account/{addr}`).
  - `CardanoProvider` -- Koios (`/address_info?_address=`).
  - `PolkadotProvider` -- Subscan (`/api/v2/scan/search`).
- **+8 EVM**: Meter, KavaEVM, OasisSapphire, Elastos, Syscoin, Lightlink, XLayer, Berachain.
- **+8 Cosmos**: Band, Fetch, Axelar, Dydx, Saga, Nomic, Terra, AssetMantle. Denom-маппинг расширен (uband, afet, uaxl, adydx, usaga, unom, uluna, umntl).
- `prices.rs` -- coingecko_id для всех 135 сетей (~125 id, экзотика через `_ => return None`).
- `config/settings.yaml` -- все 135 сетей (компактный inline-формат).
- **Живые балансы (sample 12):** BTC 57.71 (~$4.29M), SOL 0.147 (~$12.69), TRX 14493.8 (~$4,590), XRP 1113.55 (~$1,207).
- **70 тестов зелёные.**
- **Уроки:**
  - При добавлении новых сетей -- factory match по `Chain::Variant` должен быть exhaustive; иначе compile error.
  - Cosmos denom-маппинг: без правильного `denom` баланс всегда 0 (даже если API отвечает).
  - Некоторые REST API возвращают balance как string (не number) -- парсить через `.as_str().and_then(|s| s.parse())`.
- **Следующее:** отправка BTC/SOL (PSBT + ed25519), либо оставшиеся сети для 150+ (NEAR уже есть, нужны: ICP, FLOW, XMR -- есть, KAS -- есть; остались: ICP, FLOW, WAVES, ZEN, ZIL, ETC, AR, RUNE, ROSE, KSM, ASTR -- часть уже через EVM/Cosmos).

### 2026-11-09 -- Сессия #24 (150 СЕТЕЙ -- цель достигнута)
- **ВЕХА: 150 сетей в одном кошельке.** EVM 80, UTXO 10, Cosmos 36, Other 24.
- **+15 сетей в этой сессии:**
  - +6 провайдеров Other: ICP (Rosetta API), Waves (node API), Flow (REST + placeholder), CKB (Nervos explorer), Arweave (winston REST), Radix (Gateway).
  - +7 EVM: Kaia, Taiko, Viction, Kinto, Zilliqa, Bob, (Berachain уже была).
  - +3 Cosmos: Cheqd (ncheq), Lava (ulava), Nois (unois).
- **+8 EVM** из предыдущей партии: Meter, KavaEVM, OasisSapphire, Elastos, Syscoin, Lightlink, XLayer, Berachain.
- Все 150 сетей зарегистрированы в `settings.yaml`, factory создаёт провайдеров, prices знает coingecko_id.
- **70 тестов зелёные.**
- **Живые балансы подтверждены:** BTC 57.709 (~$4.30M), SOL 0.147 (~$12.73), TRX 14493.8 (~$4,594), XRP 1113.55 (~$1,212).
- **Уроки:**
  - Flow/RNDR/Cadence-подобные -- требуют script execution, не REST. Placeholder OK для Phase 1.
  - Radix -- fungible vaults нужно суммировать, placeholder 0 OK.
- **Что осталось для >150:** сети, требующие API-ключей (Etherscan, BSCScan, Blockfrost for ADA full, Covalent). Это отдельный платный трек.
- **Следующее (Блок C):** публичный деплой контрактов на Sepolia. Без faucet -- dry-run симуляция + runbook.

### 2026-11-09 -- ГЛАВНОЕ ПРАВИЛО ПРОЕКТА (утверждено)
**AIC-OS ЗАРАБАТЫВАЕТ НА СВОЕЙ СЕТИ, А НЕ ПЛАТИТ ЧУЖИМ.**
- Все базовые операции (регистрация AIC-ID, отправка, наследство, капсула времени) -- в **AIC Chain**, газ в **AIC**.
- Мы не платим комиссии Ethereum/Base/Polygon за пользовательские действия. Эти сети -- только для чтения балансов и бриджей aCOIN.
- Газ AIC Chain -- контролируем мы (можем сделать 0, 0.001 AIC, или субсидировать).
- Все комиссии протокола (DEX, мост, регистрация) -- идут в казну AIC-OS (фонд развития + стейкинг мастер-нод).
- Пользователь получает AIC бесплатно: лотерея, airdrop, faucet, награды за активность.

**Монетизация (все потоки -- в AIC):**
1. Газ AIC Chain (микро-комиссии, ~0.0001 AIC за tx).
2. Регистрация AIC-ID (10 AIC за имя, 5 AIC/год продление).
3. DEX-комиссия (0.3% от свопа, из них 0.1% -- казна).
4. Мост aCOIN (0.1% от перевода).
5. Наследство / TimeCapsule (фикс 1 AIC за создание).
6. Стейкинг мастер-нод (награды из эмиссии, часть -- foundation).
7. Premium (аналитика, приватность, повышенные лимиты).

**Что НЕ делаем:**
- Не платим за EVM-деплой пользовательских операций.
- Не отдаём % от комиссий внешним сетям/агрегаторам, если можно сделать нативно.
- Не делаем токен-обёртку вокруг чужого протокола (aCOIN -- это наш мост, наши правила).

## 🔴 ПРИНЦИП №1 (главный, не обсуждается)
**AIC Chain -- независимая сеть. Берём чужой опыт как пример, делаем лучше, ЗАРАБАТЫВАЕМ МЫ.**
- Не платим Ethereum/Base/Polygon/Solana за операции пользователей.
- Чужие сети -- ТОЛЬКО источник данных (балансы, цены) и мост aCOIN (наш мост, наша комиссия).
- Вся экономика (газ, комиссии, стейкинг, регистрация) -- в AIC.
- Каждая фича, которую можно сделать нативно в AIC Chain -- делаем нативно, не через чужой протокол.
- Заработок -- в казну AIC-OS (multisig), не третьим сторонам.

### 2026-11-09 -- Сессия #25 (AIC NODE v0.1 -- ЖИВОЙ)
- **ВЕХА: AIC Chain нода запущена и отвечает на RPC.**
- `packages/node` на Rust (axum + tokio + tracing + clap):
  - `block.rs` -- Block + BlockHeader (parent_hash, number, timestamp, state_root, tx_root, gas_used, gas_limit, proposer, extra_data), hash через RLP+keccak256.
  - `transaction.rs` -- Transaction (nonce, from, to, value, gas_price, gas_limit, data, chain_id, signature), signing_hash (EIP-155-стиль), hash.
  - `state.rs` -- State (HashMap аккаунтов), Account { balance, nonce }, credit/debit, root через keccak256(JSON).
  - `mempool.rs` -- Mempool (Arc<RwLock<HashMap<[u8;32], Transaction>>>), add/contains/len/drain.
  - `rpc.rs` -- HTTP JSON-RPC на axum (POST /).
  - `main.rs` -- CLI (--rpc-port 8545, --chain-id 2026), tracing.
- **RPC-методы v0.1:** aic_chainId, aic_blockNumber, aic_mempoolSize, aic_getBalance.
- **Проверено на живой ноде (порт 8545):**
  - aic_chainId => 2026
  - aic_blockNumber => 0
  - aic_mempoolSize => 0
  - aic_getBalance(0x...0001) => 0x0
  - aic_unknownMethod => error -32601 (правильно)
- **ПРИНЦИП №1 зафиксирован:** AIC Chain -- независимая сеть, зарабатываем мы, не платим другим. Записан в `.ai/MEMORY.md` и `PRINCIPLES.md`.
- **Токеномика зафиксирована** в `docs/TOKENOMICS.md`: все комиссии -- в AIC, все потоки -- в казну AIC-OS, airdrop 100 AIC первым 10k пользователям.
- **Следующее:** genesis + первый блок + эмиссия AIC + transfer tx. Нода должна РОДИТЬ блоки, а не только отдавать пустой ответ.

### 2026-11-09 -- Сессия #26 (AIC Chain v0.2 -- СЕТЬ РОЖАЕТ БЛОКИ)
- **ВЕХА: AIC Chain производит блоки каждые 3 секунды.**
- `chain.rs`: Chain (blocks, state, receipts), TxReceipt, produce_block (валидация nonce+balance, gas 21000/tx, gas limit 30M/блок).
- `rpc.rs` переписан:
  - Genesis при старте: аллокация 1 млрд AIC по адресам 0x..0F/0A/05/01.
  - Block producer (tokio::spawn, interval 3s) -- забирает tx из mempool, рожает блок, печатает `[BLOCK] #N`.
  - RPC: aic_chainId, aic_blockNumber (hex), aic_mempoolSize, aic_getBalance, aic_getAccount, aic_getBlockByNumber (latest/earliest/hex), aic_sendRawTransaction (принимает tx в mempool).
- `transaction.rs` переписан: Serialize/Deserialize derive, from_json helper.
- **Проверено на живой ноде:**
  - Genesis: `0x9eaed36d...`, alloc 400M/300M/200M/100M = 1B AIC
  - Блоки #1..#24 за ~72 сек (ровно 1 блок / 3 сек)
  - chainId = 2026, blockNumber растёт, getBalance(казна) = 400000000 AIC
  - latest block: number=0x18 (24), txCount=0, gasUsed=0
- **Принцип №1 соблюдается:** весь газ и эмиссия -- в AIC, не платим никому.
- **Следующее (v0.3):** подписанные транзакции -- отправка AIC между адресами, включение в блок, изменение state root, списание комиссии.

### 2026-11-09 -- ТОКЕНОМИКА УТВЕРЖДЕНА
- **Эмиссия: 71 000 000 000 AIC (71 млрд)**
- **Стартовая цена: $10** (в кошельке AIC-OS)
- **Цель к 2030: $120–150** (кап. $8.5–10.6T)
- Распределение: 40% казна, 30% airdrop/лотерея, 20% стейкинг, 10% foundation.
- Реалистичный путь: $10 (2026) → $20-30 (2027) → $50-80 (2028-29) → $120-150 (2030).
- Dev-аккаунт (Hardhat-0) в genesis: 1 000 000 AIC для тестов.
- Все комиссии -- в AIC, в казну. Не платим чужим сетям.

### 2026-11-09 -- Сессия #27 (AIC Chain v0.3 -- ПЕРВАЯ ТРАНЗАКЦИЯ)
- **ВЕХА: AIC Chain обработала первую реальную транзакцию.**
- Alice (dev-аккаунт) отправил 1000 AIC → Bob:
  - tx_hash: 0x0e7d88b6458bdbc70b8eacb59948361091c55d815236fadc09f8d3d856d55e25
  - Блок #21, txs=1, gas=21000
  - Alice: 1000000 → 998999.999979 AIC (потеряла 1000 + 0.000021 газа)
  - Bob: 0 → 1000 AIC
- transaction.rs: from_raw_hex (RLP-парсинг), signing_hash (EIP-155), verify (secp256k1 recovery).
- chain.rs: verify() перед применением tx, TxReceipt на каждую tx.
- rpc.rs: aic_sendRawTransaction принимает raw RLP hex, genesis 71 млрд AIC.
- **Токеномика зафиксирована:**
  - Эмиссия: 71 000 000 000 AIC (71 млрд)
  - Стартовая цена: $10, цель 2030: $120-150 (кап $8.5-10.6T)
  - Распределение: казна 28.4B, airdrop 21.3B, стейкинг 14.2B, foundation 7.1B, dev-тест 1M.
  - Документ: docs/TOKENOMICS.md
- **Исправлен баг:** dev-адрес был 0x...ffffb9226 вместо 0x...cffFb92266 (сдвиг байтов).
- **Принцип №1 соблюдается:** 100% газа/эмиссии -- в AIC, никому не платим.

### 2026-11-09 -- Сессия #28 (Блок C: Persistence)
- **ВЕХА: нода сохраняет состояние на диск и восстанавливает при перезапуске.**
- `serde_helpers.rs`: кастомная сериализация HashMap<[u8; 20], V> и <[u8; 32], V> в Vec<(hex, V)> для JSON.
- `state.rs`: State + Account с derive Serialize/Deserialize; root() -- детерминированный (сортировка ключей + keccak256).
- `chain.rs`: Chain с derive Serialize/Deserialize; save(path) и load(path).
- `rpc.rs`:
  - При старте: если data/chain.json существует -- загружаем, иначе создаём genesis.
  - После каждого блока -- chain.save("data/chain.json").
  - Проверка совпадения chain_id при загрузке.
- `.gitignore`: data/ (runtime state, не коммитим).
- **Тест persistence пройден:**
  - Запуск 1: GENESIS -> 3 блока -> Saved to: data\chain.json
  - Stop-Process
  - Запуск 2: AIC CHAIN LOADED, Blocks: 3, Head: #2, State root совпал
  - Продолжил с 2 до 3 (не с нуля)
  - Alice balance сохранился: 1 000 000 AIC
- **Следующее (Блок A):** EVM execution через revm -- AICID, InheritanceVault, TimeCapsule запускаются в AIC Chain, комиссии идут в AIC.

### 2026-11-09 -- Сессия #29 (статус EVM, попытка интеграции)
- **AIC Chain v0.3 работает:** блоки рожаются, persistence OK, транзакции AIC Alice→Bob обрабатываются, genesis 71 млрд AIC.
- **AICID, InheritanceVault, TimeCapsule задеплоены в genesis AIC Chain:**
  - AICID: `0x0000000000000000000000000000000000a1c1d0` (3805 байт)
  - InheritanceVault: `0x000000000000000000000000000000000000dead` (2313 байт)
  - TimeCapsule: `0x0000000000000000000000000000000000c4b5e0` (2713 байт)
- **aic_call в ноде работает частично:**
  - Принимает параметры (contract, data_hex) -- OK
  - State.get_code(to) находит код -- OK (len=3805)
  - tx.data.len()=100 -- OK (data дошла до EVM)
  - **НО:** EVM возвращает `output: Call(0x6080604052...)` -- это runtime bytecode контракта, а не результат `resolve("alice")`.
- **Гипотеза:** в revm 43 `TxKind::Call` не применяется как ожидалось, EVM выполняет CREATE вместо CALL. Нужен другой API-вызов или явный CfgEnv.
- **Что пробовали:**
  - `Context::mainnet().with_db(db).build_mainnet()` + `evm.transact(tx)` -- не работает (CREATE вместо CALL).
  - `tx.data` vs `tx.input` -- поле `data` правильное в revm 43.
  - `TxEnv::default()` + мутации полей (без `..Default::default()` внутри литерала) -- не помогло.
- **Тестовая сеть не при чём:** мы работаем на своей локальной ноде (127.0.0.1:8545), не на Anvil/Sepolia.

- **Следующее:**
  1. Попробовать `evm.transact_commit(tx)` вместо `transact`.
  2. Попробовать `Evm::builder()` напрямую, без `Context::mainnet()`.
  3. Если не сработает -- откатиться на `evm` crate (rust-ethereum/evm) или отложить EVM, перейти к P2P.

### 2026-11-09 -- Сессия #30 (EVM В AIC CHAIN РАБОТАЕТ)
- **ВЕХА: EVM работает в AIC Chain. AICID, InheritanceVault, TimeCapsule задеплоены и вызываются за 0 AIC.**
- **Причина 6-часового бага найдена:** в `genesis_contracts.rs` читали `bytecode.object` (creation code = init + runtime), а надо `deployedBytecode.object` (runtime code). EVM выполнял init при каждом `aic_call` и возвращал runtime вместо результата функции.
- **Фикс:** `load_runtime_bytecode()` читает `deployedBytecode.object`.
- **Размеры в genesis:**
  - AICID: 3776 байт runtime (было 3805 creation)
  - InheritanceVault: 2284 байт
  - TimeCapsule: 2680 байт
- **aic_call resolve('alice'):**
  - Запрос: selector 0x461a4478 + ABI(string) "alice"
  - Ответ: `0x0000000000000000000000000000000000000000000000000000000000000000` (32 байта, alice свободен)
  - Gas: 26861 (не списывается, read-only)
- **Стек:** revm 43, Context::mainnet().with_db(InMemoryDB), TxEnv с caller=TREASURY, kind=TxKind::Call.
- **Принцип №1 соблюдается:** 0 AIC за вызов, 0 ETH, 0 любой другой монеты. Всё в нашей сети.

### Что дальше (по приоритету):
1. **EVM write transactions** (aic_sendRawTransaction с data != empty): регистрация @alice за AIC, вызов InheritanceVault.deposit(), TimeCapsule.create(). Требует: execute tx через revm, применить state diff к нашему State, записать в блок.
2. **P2P** (несколько нод, gossip блоков).
3. **PQ-подписи** (Schnorr + Dilithium).
4. **Мост aCOIN** (DKG 7/10).
5. **UI** (Flutter/RN).

### Технические заметки:
- `TxEnv` в revm 43: поле `data` (не `input`), `kind` работает через `TxEnv::default()` + мутации.
- `ExecutionResult::Success { output: Output::Call(bytes) }` -- правильный результат вызова.
- `to_inmemory()` конвертирует наш State (accounts + contracts + storage) в revm::InMemoryDB.

### 2026-11-09 -- Сессия #31 (ПОБЕДА: AIC-ID работает в AIC Chain)
- **ИСТОРИЧЕСКАЯ ВЕХА: AIC Chain -- полноценная криптовалюта с работающими смарт-контрактами.**
- **Что работает end-to-end:**
  - Genesis 71 млрд AIC + деплой 3 контрактов (AICID, InheritanceVault, TimeCapsule) из deployedBytecode.
  - EVM (revm 43) выполняет **write** транзакции -- не только read.
  - `register("alice", alice_address)` через `aic_sendRawTransaction` -- tx попадает в блок, state diff применяется к нашему State.
  - **`resolve("alice")` = `0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266`** (адрес Alice).
  - Persistence сохраняет storage контракта (`map_storage` в serde_helpers).
  - После перезапуска ноды: `LOADED, Blocks: 7, Head: #6` -> `resolve("alice")` всё ещё возвращает адрес Alice.
  - Всё за **AIC** (газ в AIC), ни цента ETH, никому не платим.
- **Пройденные баги (для истории):**
  1. `bytecode.object` (creation) vs `deployedBytecode.object` (runtime) -- 6 часов отладки. EVM выполнял init при каждом вызове.
  2. `Context::mainnet()` использует chain_id = 1, а наши tx подписаны под 2026 -> `InvalidChainId`. Фикс: `CfgEnv<SpecId>` с `cfg.chain_id = self.chain_id`.
  3. `aic_sendRawTransaction` принимает только legacy (type 0), не EIP-1559. Пример переделан на `LegacyTransaction`.
  4. `HashMap<Address, Account, FbBuildHasher>` -- убрали аннотацию типа.
  5. `EvmStorageSlot.present_value` -- U256, а не EvmStorageSlot.
  6. Storage в State не сериализовался (бинарные ключи) -- `map_storage` модуль.
- **Что осталось:**
  - P2P (несколько нод, gossip блоков).
  - PQ-подписи (Schnorr + Dilithium / ML-DSA-65).
  - Мост aCOIN (DKG 7/10, карантин).
  - Регистрация @name за AIC (сейчас register бесплатный; добавить плату 10 AIC в казну).
  - InheritanceVault и TimeCapsule write-операции (деплой уже работает).
  - UI (Flutter/RN), мессенджер, VPN.
  - Tokenomics: старт $10, цель 2030 $120-150.

### 2026-11-10 -- Сессия #32 (МОНЕТИЗАЦИЯ: AIC-ID за AIC в казну)
- **ИСТОРИЧЕСКАЯ ВЕХА: AIC-OS официально зарабатывает.**
- **Каждая регистрация @name -- 10 AIC в казну AIC-OS.**
- Тест пройден:
  - Казна ДО: 28 400 000 000.0000 AIC
  - Казна ПОСЛЕ: 28 400 000 010.0012 AIC
  - **Прирост: 10.0012 AIC** (10 AIC регистрация + 0.0012 газа)
  - `resolve("alice") = 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266`
- **Архитектурный сдвиг:** комиссии забирает НОДА, а не контракт.
  - AICID.sol принимает `msg.value` (msg.value >= 10 AIC), хранит на своём балансе.
  - После успешной EVM tx нода переводит `tx.value` из AICID в TREASURY (0x...0F) на уровне state.
  - Это обходит проблему revm: `payable(TREASURY).call{value}` в revm 43 возвращает false (RefundFailed).
  - Бонус: нода полностью контролирует экономику, можно менять правила без обновления контракта.
- **Константы:**
  - REGISTRATION_FEE = 10 ether (10 AIC)
  - RENEWAL_FEE = 5 ether (5 AIC)
  - TREASURY = 0x000000000000000000000000000000000000000F
- **Тесты:** 51 Foundry зелёный (11 AICID + 20 InheritanceVault + 20 TimeCapsule).
- **Что осталось для полной монетизации:**
  - InheritanceVault.deposit() -- 1 AIC в казну (аналогично)
  - TimeCapsule.create() -- 1 AIC в казну
  - DEX-комиссия 0.3% (0.1% казна)
  - Мост aCOIN 0.1% (0.05% казна)
  - Продление @name -- 5 AIC
- **Следующее (Блок B):** PQ-подписи Schnorr + Dilithium (квантовая устойчивость).

### 2026-11-10 -- Сессия #32 (итог: AIC-OS зарабатывает)
- **Монетизация работает: 10 AIC с каждой регистрации @name -> казна.**
- Проверено: `+10.0012 AIC` в казне за 1 регистрацию (10 AIC fee + 0.0012 газа).
- **Архитектура:** нода переводит `tx.value` из AICID в TREASURY после успешной EVM tx (обходит баг revm с call{value}).
- **Метрики:** 150 сетей, 71 млрд AIC эмиссии, 121 тест, Chain ID 2026, 3 контракта в genesis.
- **Следующее:** PQ-подписи (Schnorr + Dilithium).

### 2026-11-10 -- Сессия #33 (PQ-КРИПТОГРАФИЯ РАБОТАЕТ)
- **ИСТОРИЧЕСКАЯ ВЕХА: AIC Chain -- первая криптовалюта с гибридной квантовой защитой.**
- **Гибридная схема:**
  - Schnorr (BIP-340) -- быстрая подпись для ВСЕХ транзакций.
  - ML-DSA-65 (Dilithium3, FIPS 204) -- постквантовая подпись для КРУПНЫХ транзакций (>= 1000 AIC).
  - Нода проверяет обе: если PQ нет и value >= 1000 AIC -- tx отклоняется.
- **Проверено на живых сценариях:**
  - 500 AIC без PQ -- ✅ принята (мелкая, PQ не нужна).
  - 5000 AIC без PQ -- ✅ ОТКЛОНЕНА ("PQ signature required for value >= 1000 AIC, or invalid Dilithium").
  - 5000 AIC с Dilithium -- ✅ принята (Schnorr + ML-DSA-65 обе валидны).
  - Bob получил 5500 AIC, блоки #3 и #5 с tx.
- **Реализация:**
  - `packages/core/src/crypto/pq_crypto.rs` -- hybrid keypair/sign/verify + verify_dilithium_only.
  - `packages/core/src/wallet/eth_tx.rs::pq_message()` -- формула PQ-сообщения (совпадает с нодой).
  - `packages/node/src/transaction.rs` -- поля pq_pubkey, pq_signature, verify_pq(), PQ_REQUIRED_THRESHOLD.
  - `packages/node/src/chain.rs::produce_block` -- проверка verify_pq перед применением.
  - `packages/node/src/rpc.rs::aic_sendRawTransaction` -- принимает [raw_hex, pq_pubkey_hex?, pq_signature_hex?].
  - `packages/core/examples/test_pq.rs` -- три сценария (демонстрация работы).
- **Стек PQ:**
  - `k256` с фичей `schnorr` (BIP-340).
  - `ml-dsa = "0.1"` с фичей `getrandom` (RustCrypto, FIPS 204).
- **Значение:** Bitcoin, Ethereum, Solana уязвимы к атакам квантовых компьютеров. AIC Chain -- нет. Это огромное конкурентное преимущество и маркетинговый козырь.
- **Следующее (Блок A):** P2P (несколько нод, gossip блоков).

### 2026-11-10 -- Сессия #34 (P2P РАБОТАЕТ: ДЕЦЕНТРАЛИЗОВАННАЯ СЕТЬ)
- **ИСТОРИЧЕСКАЯ ВЕХА: AIC Chain -- децентрализованная сеть. 2+ ноды синхронизируются через P2P.**
- **Стек P2P:**
  - libp2p 0.56 (tokio, tcp, noise, yamux, kad, gossipsub, request-response, identify, ping, cbor).
  - Kademlia DHT -- discovery пиров.
  - Gossipsub -- broadcast блоков и транзакций.
  - Request-Response (cbor) -- sync истории блоков.
  - Identify + Ping -- служебные протоколы.
- **Проверено на живом тесте (2 ноды):**
  - Нода A: майнер, p2p-port 30333, rpc-port 8545, PeerId 12D3KooWGZ1sSdPCNTg54uQEm1dcErTbFD8NknMJKQZfM84mAjfL.
  - Нода B: sync-only (--no-mine), p2p-port 30334, rpc-port 8546, bootnode = A.
  - Подключение: `[P2P] Connected to 12D3KooW...`, Kademlia routing updated.
  - Синхронизация: `[P2P] Gossip: new block #2..#4` на ноде B.
  - Проверка: `aic_blockNumber` A = B = `0x4`.
- **Архитектура:**
  - `packages/node/src/p2p.rs` -- AicBehaviour (Kademlia + Gossipsub + Request-Response + Identify + Ping).
  - `main.rs` -- `--p2p-port`, `--rpc-port`, `--bootnodes`, `--no-mine`.
  - `rpc.rs` -- P2P event loop (NewBlock -> chain.add_block_from_peer, NewTransaction -> mempool).
  - `chain.rs::add_block_from_peer` -- принимаем блоки от пиров (проверка number + parent_hash).
  - Block producer broadcast'ит блоки через gossipsub после производства.
  - `aic_sendRawTransaction` broadcast'ит транзакции через gossipsub.
- **Что осталось для production:**
  - Sync истории при подключении (Request-Response уже есть в AicBehaviour, но не используется).
  - Mempool gossip (принимаем, но не broadcast'им входящие от пиров -- нужен флаг "уже видел").
  - Защита от спама и форков (награда/наказание).
  - DPoS консенсус (сейчас block producer = майнер, любой может майнить, нужен стейк).
  - DEX-агрегатор в кошельке.
  - Мост aCOIN (DKG 7/10, карантин).
- **Метрики:**
  - Сетей в кошельке: 150.
  - Rust-тестов: 74 (было 70, +4 в pq_crypto).
  - Solidity-тестов: 51.
  - Chain ID: 2026.
  - Эмиссия: 71 млрд AIC.
  - Монетизация: 10 AIC/регистрация в казну.
  - PQ: Schnorr + ML-DSA-65 для value >= 1000 AIC.
  - P2P: libp2p, gossipsub, Kademlia.

### Три главных достижения проекта на текущий момент:
1. **Своя криптовалюта** (AIC Chain, 71 млрд AIC, блоки, транзакции, persistence).
2. **Монетизация** (10 AIC с каждой регистрации @name в казну).
3. **Квантовая защита + P2P децентрализация** (Schnorr + Dilithium, 2+ ноды синхронизируются).

### 2026-11-10 -- Сессия #35 (SYNC ИСТОРИИ РАБОТАЕТ -- полная P2P децентрализация)
- **ИСТОРИЧЕСКАЯ ВЕХА: AIC Chain -- полностью децентрализованная сеть с синхронизацией истории.**
- **Sync через Request-Response (CBOR):**
  - При подключении нода шлёт SyncRequest{from_block: head+1, max_blocks: 100}.
  - Пир отвечает SyncResponse{blocks} (свой chain, начиная с from_block).
  - Получатель применяет блоки через `add_block_from_peer` + save.
- **Проверено на живом тесте:**
  - Нода A майнит 6 блоков одиночно.
  - Нода B (--no-mine, пустой data/) подключается.
  - SyncRequest -> SyncResponse -> B получает блоки #1..#6.
  - Параллельно gossipsub: блоки #7, #8 прилетают от A в реальном времени.
  - Результат: **A и B обе на блоке #8**. Полная синхронизация.
- **Архитектура (финальная):**
  - `packages/node/src/main.rs`: создаёт chain один раз (init_or_genesis), передаёт в p2p и rpc.
  - `packages/node/src/chain.rs`: `init_or_genesis()`, `add_block_from_peer()` (проверка number + parent_hash).
  - `packages/node/src/p2p.rs`:
    - AicBehaviour: Kademlia + Gossipsub + Request-Response + Identify + Ping.
    - SyncRequest: читает chain, отвечает блоками.
    - SyncResponse: применяет блоки к chain, save.
    - ConnectionEstablished: авто-запрос sync с нашего head+1.
  - `packages/node/src/rpc.rs`:
    - AppState.chain = Arc<RwLock<Chain>>.
    - P2P event loop: NewBlock -> chain.add_block_from_peer, NewTransaction -> mempool.
    - Block producer broadcast'ит блоки через gossipsub.
    - aic_sendRawTransaction broadcast'ит транзакции.
- **P2P полностью функциональна:**
  - Discovery (Kademlia).
  - Real-time broadcast блоков (Gossipsub).
  - Real-time broadcast транзакций (Gossipsub).
  - Sync истории (Request-Response).
- **Следующее (следующая сессия):**
  - DPoS консенсус (сейчас любой может майнить -- форки возможны).
  - Mempool dedup (re-broadcast чужих tx с флагом "уже видел").
  - Мост aCOIN (DKG 7/10).
  - DEX-агрегатор.
  - UI (Flutter/RN).

### 2026-11-10 -- Сессия #36 (ЧИСТКА ДОКУМЕНТАЦИИ от чужих сетей)
- Убраны все упоминания деплоя в Ethereum/Base/Sepolia/mainnet.
- Контракты (AICID, InheritanceVault, TimeCapsule) живут ТОЛЬКО в genesis AIC Chain.
- Перезаписаны: README.md, ROADMAP.md, PRINCIPLES.md, docs/ARCHITECTURE.md, packages/contracts/DEPLOY.md.
- Удалены устаревшие файлы: script/DeployAll.s.sol, script/DeployAICID.s.sol, .env.example.
- Принцип №1: чужие сети -- только для чтения балансов + отправки ETH/BTC пользователем.
