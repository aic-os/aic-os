//! Минимальная интеграция с revm для выполнения EVM-транзакций.
//! На этом шаге -- только простой перевод (CALL) без состояния нашего блокчейна.
//! Следующий шаг -- адаптер Database для связи revm с нашим State.

use revm::{
    context::{Context, TxEnv},
    context_interface::result::{ExecutionResult, Output},
    primitives::{Address, Bytes, TxKind, U256},
    ExecuteEvm, MainBuilder, MainContext,
};

/// Выполнить простой EVM-перевод (CALL) с одного адреса на другой.
/// Возвращает `(success, gas_used, output)`.
pub fn execute_transfer(
    from: Address,
    to: Address,
    value: U256,
    gas_limit: u64,
    gas_price: u128,
    nonce: u64,
) -> Result<(bool, u64, Option<Bytes>), Box<dyn std::error::Error>> {
    // Создаём EVM-контекст с mainnet-конфигом (для простоты).
    // В будущем заменим на кастомный CfgEnv с chain_id = 2026.
    let mut evm = Context::mainnet().build_mainnet();

    // Настраиваем транзакцию.
    let tx = TxEnv {
        caller: from,
        gas_limit,
        gas_price,
        kind: TxKind::Call(to),
        value,
        data: Bytes::new(),
        nonce,
        ..Default::default()
    };

    let result = evm.transact(tx);

    match result {
        Ok(res) => {
            let success = res.result.is_success();
            let gas_used = res.result.gas_used();
            let output = match res.result {
                ExecutionResult::Success { output, .. } => {
                    if let Output::Call(data) = output {
                        Some(data)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            Ok((success, gas_used, output))
        }
        Err(e) => Err(format!("EVM execution failed: {e:?}").into()),
    }
}
