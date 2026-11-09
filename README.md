# AIC-OS 🚀

> **Независимая криптовалюта AIC + мультичейн-кошелёк (150 сетей) + смарт-контракты + DeFi.**

## 🎯 Главный принцип
**AIC Chain — независимая сеть. Берём чужой опыт как пример, делаем лучше, ЗАРАБАТЫВАЕМ МЫ.**
Все комиссии — в AIC. Не платим Ethereum/Base/Polygon за действия пользователей.

## 💰 Токеномика
- **Эмиссия:** 71 000 000 000 AIC (71 млрд)
- **Стартовая цена:** $10
- **Цель к 2030:** $120–150 (кап $8.5–10.6T)
- **Распределение:** 40% казна, 30% airdrop, 20% стейкинг, 10% foundation
- Подробно: [docs/TOKENOMICS.md](./docs/TOKENOMICS.md)

## ✅ Что уже работает

### Кошелёк (`packages/core`, Rust)
- BIP-39 сид-фраза (12/24 слова), BIP-32 derivation
- secp256k1 + EIP-55 (совместимость с MetaMask/Trust/Ledger)
- Legacy ETH tx (EIP-155) + EIP-1559
- ERC-20: balanceOf, decimals, symbol, name, transfer calldata
- **150 сетей** с балансами и курсами USD/RUB (CoinGecko)
- AIC-ID: resolve(@name) / reverseResolve(addr)
- 70 тестов

### Смарт-контракты (`packages/contracts`, Solidity + Foundry)
- **AICID.sol** — регистрация @name, resolve, reverse, transfer, renew (365 дней)
- **InheritanceVault.sol** — dead man's switch (наследство в блокчейне)
- **TimeCapsule.sol** — блокировка ETH до timestamp
- 53 теста Foundry
- Деплой-скрипты + DEPLOY.md runbook

### AIC Chain (`packages/node`, Rust)
- Своя нода, порт 8545, chain_id 2026
- Block producer каждые 3 секунды
- Genesis: 71 млрд AIC + 3 контракта
- Подпись EIP-155 (secp256k1), verify
- **Работающая транзакция:** Alice→Bob 1000 AIC, блок #21
- Persistence (chain.json save/load)
- RPC: chainId, blockNumber, getBalance, getAccount, getBlockByNumber, sendRawTransaction
- revm 43 интеграция (в процессе — CREATE вместо CALL, ищем фикс)

## 🚧 В работе
- EVM в AIC Chain (aic_call выполняет CREATE вместо CALL)
- P2P, DPoS консенсус
- PQ-подписи (Schnorr + Dilithium)
- Мост aCOIN

## 📚 Документация
- [ROADMAP.md](./ROADMAP.md) — полная дорожная карта
- [PRINCIPLES.md](./PRINCIPLES.md) — принципы проекта
- [docs/TOKENOMICS.md](./docs/TOKENOMICS.md) — экономика
- [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) — архитектура
- [packages/contracts/DEPLOY.md](./packages/contracts/DEPLOY.md) — деплой
- [.ai/MEMORY.md](./.ai/MEMORY.md) — память проекта

## 🛠️ Стек
- Rust (tokio, revm, k256, reqwest)
- Solidity 0.8.24 + Foundry 1.9.0
- Axum (RPC), serde_json

## 📊 Метрики
- Сетей: **150** (EVM 80, UTXO 10, Cosmos 36, Other 24)
- Rust-тестов: **71**
- Solidity-тестов: **53**
- Контрактов в AIC Chain genesis: **3**
- Chain ID: **2026**
