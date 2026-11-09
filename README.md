# AIC-OS 🚀

> **Независимая криптовалюта AIC + мультичейн-кошелёк (150 сетей) + EVM смарт-контракты + DeFi.**

## 🎯 Принцип №1
**AIC Chain -- независимая сеть. Берём чужой опыт как пример, делаем лучше, ЗАРАБАТЫВАЕМ МЫ.**
Все комиссии идут в казну AIC-OS в AIC. Не платим Ethereum/Base/Polygon за действия пользователей.

## 💰 Токеномика
- **Эмиссия:** 71 000 000 000 AIC (71 млрд)
- **Стартовая цена:** $10
- **Цель к 2030:** $120–150 (кап $8.5–10.6T)
- **Распределение:** 40% казна, 30% airdrop, 20% стейкинг, 10% foundation
- **Монетизация (проверено):** 10 AIC за регистрацию @name, 5 AIC продление, газ
- Подробно: [docs/TOKENOMICS.md](./docs/TOKENOMICS.md)

## ✅ Что работает

### Кошелёк (aic-core, Rust)
- BIP-39, BIP-32, secp256k1, EIP-55
- Legacy ETH tx (EIP-155) + EIP-1559
- ERC-20 (balanceOf/decimals/symbol/name/transfer)
- **150 сетей** с балансами и курсами USD/RUB
- AIC-ID: resolve(@name) в Rust
- 70 тестов

### Смарт-контракты (Solidity + Foundry)
- **AICID.sol** -- регистрация @name (10 AIC), resolve, reverse, transfer, renew (5 AIC)
- **InheritanceVault.sol** -- dead man's switch
- **TimeCapsule.sol** -- блокировка ETH до timestamp
- 51 тест
- Деплой + DEPLOY.md

### AIC Chain (своя нода, Rust)
- Chain ID 2026, порт 8545
- Block producer каждые 3 сек
- Genesis 71 млрд AIC
- **EVM работает** (revm 43): read + write
- **AICID.register() -- 10 AIC в казну AIC-OS** ✅
- **resolve("alice") -> 0xf39Fd6e5...** ✅
- Persistence: chain.json (блоки, state, storage контрактов)
- Первая транзакция AIC Alice→Bob (блок #21)
- RPC: chainId, blockNumber, getBalance, getAccount, getBlockByNumber, aic_call, aic_sendRawTransaction

## 🚧 В работе
- ✅ **PQ-подписи (Schnorr + ML-DSA-65 / Dilithium)** -- квантовая устойчивость РАБОТАЕТ:
  - Все транзакции: Schnorr (BIP-340).
  - Транзакции >= 1000 AIC: дополнительно ML-DSA-65 (FIPS 204, Dilithium3).
  - Проверено: 5000 AIC без PQ отклоняется, с PQ -- принимается.
  - AIC Chain -- первая криптовалюта с гибридной квантовой защитой.
- ✅ **P2P (несколько нод)** -- работает: 2+ ноды синхронизируются через libp2p (Kademlia + Gossipsub)
- Монетизация InheritanceVault/TimeCapsule (1 AIC каждая)
- Мост aCOIN (DKG 7/10)
- DPoS консенсус + мастер-ноды
- UI (Flutter/RN)

## 📚 Документация
- [ROADMAP.md](./ROADMAP.md) -- полная карта
- [PRINCIPLES.md](./PRINCIPLES.md) -- принципы проекта
- [docs/TOKENOMICS.md](./docs/TOKENOMICS.md) -- экономика
- [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) -- архитектура
- [packages/contracts/DEPLOY.md](./packages/contracts/DEPLOY.md) -- деплой
- [.ai/MEMORY.md](./.ai/MEMORY.md) -- память проекта

## 🛠️ Стек
- Rust (tokio, revm 43, k256, bip39, bip32, axum)
- Solidity 0.8.24 + Foundry 1.9.0
- 121 тест (70 Rust + 51 Solidity)

## 📊 Метрики
- Сетей: **150** (EVM 80, UTXO 10, Cosmos 36, Other 24)
- Chain ID: **2026**
- Эмиссия: **71 млрд AIC**
- Стартовая цена: **$10**
- Первая регистрация @alice: **+10.0012 AIC в казну** ✅



