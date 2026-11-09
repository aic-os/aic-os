# Инструкция по деплою контрактов AIC-OS

Три контракта: **AICID**, **InheritanceVault**, **TimeCapsule**.

## 0. Проверка перед деплоем

~~~bash
forge --version          # 1.9.0+
forge build              # без ошибок
forge test               # 53 теста зелёные
~~~

## 1. Создать зашифрованный кошелёк деплоера (один раз)

Никогда не коммить приватный ключ. Зашифруй его локально:

~~~bash
cast wallet import deployer --interactive
#   Введи приватный ключ: (скрыто)
#   Введи пароль:         (скрыто, запомни его)
~~~

Файл сохранится в ~/.foundry/keystores/deployer.

## 2. Пополнить кошелёк ETH

Узнать адрес:

~~~bash
cast wallet address --account deployer
~~~

Сколько газа нужно:

| Сеть | Газ (оценка) | Стоимость при 1 gwei |
|---|---|---|
| AICID | ~900k | ~0.0009 ETH |
| InheritanceVault | ~800k | ~0.0008 ETH |
| TimeCapsule | ~900k | ~0.0009 ETH |
| Итого | ~2.6M | ~0.0026 ETH |

### Sepolia (бесплатно)

Варианты faucet:

- Google Web3: https://cloud.google.com/application/web3/faucet/ethereum/sepolia (нужен 0.001 ETH на mainnet)
- Alchemy: https://www.alchemy.com/faucets/ethereum-sepolia (нужен аккаунт + баланс на mainnet)
- PoW: https://sepolia-faucet.pk910.de (proof-of-work, ~30 минут, баланс не нужен)

### Mainnet (платно)

Купи ~$5 ETH в любом обменнике, отправь на адрес деплоера.

## 3. Прогон без отправки (без газа, без broadcast)

~~~bash
forge script script/DeployAll.s.sol --rpc-url mainnet
~~~

Ожидаемый вывод:

~~~
AICID (реестр имён)          : 0x...
InheritanceVault (наследство): 0x...
TimeCapsule (капсула времени): 0x...
~~~

Если прошло -- скрипт корректен, готов к реальному деплою.

## 4. Реальный деплой -- Sepolia

~~~bash
export ETHERSCAN_API_KEY=...   # взять на https://etherscan.io/myapikey

forge script script/DeployAll.s.sol \
  --rpc-url sepolia \
  --broadcast \
  --verify \
  --account deployer
~~~

Ожидаемый вывод:

~~~
[Success] Hash: 0x...
Contract: AICID
Contract Address: 0x...
ONCHAIN EXECUTION COMPLETE & SUCCESSFUL.
~~~

Скопируй три адреса. Проверь на https://sepolia.etherscan.io.

## 5. Реальный деплой -- Mainnet

Та же команда, но --rpc-url mainnet. Только после того, как Sepolia отработала 24 часа без проблем и у тебя есть ~$5 газа.

## 6. Smoke-тест после деплоя

~~~bash
# Проверить, что имя свободно (должен вернуть 0x000...)
cast call $AICID_ADDRESS "resolve(string)(address)" "alice" --rpc-url sepolia

# Зарегистрировать имя
cast send $AICID_ADDRESS "register(string,address)" "alice" $YOUR_ADDRESS --rpc-url sepolia --account deployer

# Обратный резолв
cast call $AICID_ADDRESS "reverseResolve(address)(string)" $YOUR_ADDRESS --rpc-url sepolia
# => "alice"
~~~

## 7. Сохранить адреса

Создай файл deployments/sepolia.json:

~~~json
{
  "chainId": 11155111,
  "AICID": "0x...",
  "InheritanceVault": "0x...",
  "TimeCapsule": "0x..."
}
~~~

Закоммить -- публичная запись о деплое.

## Частые проблемы

- Error: Failed to estimate gas -- баланс деплоера = 0, пополни кошелёк.
- Invalid username or token -- неверный ETHERSCAN_API_KEY, перегенерируй.
- --verify зависает -- rate limit Etherscan, убери --verify и верифицируй позже командой forge verify-contract.
