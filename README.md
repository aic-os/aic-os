# AIC-OS 🚀

> **Независимая криптовалюта AIC + мультичейн-кошелёк (150 сетей) + DeFi.**

## 🎯 Принцип №1
**AIC Chain -- независимая сеть. Берём чужой опыт как пример, делаем лучше, ЗАРАБАТЫВАЕМ МЫ.**
Все комиссии идут в казну AIC-OS в AIC. Не платим Ethereum/Base/Polygon за действия пользователей.

## 💰 Токеномика
- **Эмиссия:** 71 000 000 000 AIC (71 млрд)
- **Стартовая цена:** $10
- **Цель к 2030:** $120–150 (кап $8.5–10.6T)
- **Распределение:** 40% казна, 30% airdrop, 20% стейкинг, 10% foundation
- **Монетизация:** 10 AIC за регистрацию @name, 5 AIC продление, газ → казна
- Подробно: [docs/TOKENOMICS.md](./docs/TOKENOMICS.md)

---

## ✅ AIC Chain (своя сеть, Rust)

**Это главное. Наша собственная децентрализованная криптовалюта.**

- **Chain ID:** 2026
- **Нода:** Rust + axum (RPC) + libp2p (P2P) + revm (EVM)
- **Эмиссия в genesis:** 71 млрд AIC
- **Block producer:** каждые 3 секунды
- **EVM-совместимость:** наша нода исполняет smart-контракты через revm. **Внутри AIC Chain. За AIC.**
- **Контракты в genesis AIC Chain** (написаны на Solidity, но исполняются НАМИ в НАШЕЙ сети):
  - AICID (`0x...a1c1d0`) -- регистрация @name за 10 AIC → казна
  - InheritanceVault (`0x...dead`) -- dead man's switch
  - TimeCapsule (`0x...c4b5e0`) -- блокировка AIC до timestamp
- **PQ-защита:** Schnorr (BIP-340) + ML-DSA-65 (Dilithium3, FIPS 204) для value ≥ 1000 AIC
- **P2P:** libp2p 0.56, Kademlia + Gossipsub + Request-Response
- **DPoS консенсус:** round-robin валидаторов, без форков. Persistence в chain.json. Timeout-skip (9s). Reward 5 AIC/блок.
  - Sync истории: новая нода догоняет цепочку
  - Real-time broadcast блоков и транзакций
- **Persistence:** chain.json (блоки, state, storage контрактов)
- **Первая реальная транзакция:** Alice→Bob 1000 AIC, блок #21
- **Первая регистрация @name:** `@alice → 0xf39Fd6e5...`, +10 AIC в казну
- **Тесты:** 74 Rust-теста (ядро) + 51 Solidity-тест (контракты)

---

## ✅ Мультичейн-кошелёк (Rust, aic-core)

**Чтение балансов 150 сетей + отправка ETH/BTC/SOL.**

- BIP-39, BIP-32, secp256k1, EIP-55
- Legacy ETH tx (EIP-155) + EIP-1559
- ERC-20: balanceOf/decimals/symbol/name/transfer
- **150 сетей** -- live балансы + курсы USD/RUB (CoinGecko)
- AIC-ID resolve(@name) в Rust (через RPC AIC Chain)
- 74 теста

**Важно:** кошелёк используется для чтения чужих сетей и отправки ETH/BTC (если пользователь сам того хочет). Он **не отправляет наши контракты в чужие сети.** Все контракты -- только в AIC Chain.

---

## 🧩 В разработке: Quantum Puzzle

**Публичная головоломка с наградой за взлом защиты:**
- **Режим A:** 1 000 000 AIC за взлом классического ECDSA (как Bitcoin).
- **Режим B:** 2 000 000 AIC за взлом нашей PQ-защиты (Schnorr + ML-DSA-65). Через 10 лет без взлома -- приз в казну.
- Публичный счётчик "День N: 2M AIC в безопасности".
- Смарт-контракт живёт в AIC Chain (никаких чужих сетей).

## ✅ AIC Wallet (Flutter, UI)

- Flutter 3.47.7, web + windows + android
- Главный экран: баланс AIC, курсы USD/RUB, block height, адрес
- RPC-клиент к AIC Chain (chainId, blockNumber, getBalance)
- Тёмная тема, градиенты
- Запуск: `cd packages/mobile && flutter run -d web-server --web-port=8080`
- Нода должна быть запущена: `.\target\debug\aic-node.exe`

## 🛠️ Стек

- **Ядро:** Rust (tokio, k256, bip39, bip32)
- **Нода:** axum + revm 43 + libp2p 0.56 + ml-dsa + serde_json
- **Контракты в AIC Chain:** Solidity 0.8.24 (компилятор Foundry, но деплой -- только в AIC Chain genesis)
- **Тесты:** 125 (74 Rust + 51 Solidity)

---

## 📚 Документация

- [ROADMAP.md](./ROADMAP.md) -- полная карта
- [PRINCIPLES.md](./PRINCIPLES.md) -- принципы проекта
- [docs/TOKENOMICS.md](./docs/TOKENOMICS.md) -- экономика
- [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) -- архитектура
- [.ai/MEMORY.md](./.ai/MEMORY.md) -- память проекта

---

## 📊 Метрики

- **Сетей в кошельке:** 150 (EVM 80, UTXO 10, Cosmos 36, Other 24)
- **Chain ID AIC:** 2026
- **Эмиссия:** 71 млрд AIC
- **Стартовая цена:** $10
- **Монетизация:** 10 AIC/регистрация → казна (проверено)
- **PQ:** Schnorr + ML-DSA-65 для value ≥ 1000 AIC (проверено)
- **P2P:** 2+ ноды синхронизируются (проверено: B догнал A до #8)
- **Тестов:** 125




