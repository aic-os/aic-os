//! Деплой Solidity-контрактов в genesis-блок AIC Chain.
//! Читает RUNTIME bytecode из артефактов Foundry (packages/contracts/out/*.sol/*.json).

use std::path::Path;

fn load_runtime_bytecode(name: &str) -> Option<Vec<u8>> {
    let candidates = [
        format!("packages/contracts/out/{name}.sol/{name}.json"),
        format!("../contracts/out/{name}.sol/{name}.json"),
        format!("contracts/out/{name}.sol/{name}.json"),
    ];
    for path in &candidates {
        let p = Path::new(path);
        if !p.exists() { continue; }
        let text = std::fs::read_to_string(p).ok()?;
        let json: serde_json::Value = serde_json::from_str(&text).ok()?;

        // ВАЖНО: deployedBytecode.object -- это RUNTIME code.
        // bytecode.object -- это creation code (init + runtime), НЕ использовать!
        let hex_str = json
            .get("deployedBytecode")?
            .get("object")?
            .as_str()?;
        let clean = hex_str.trim_start_matches("0x");
        if clean.is_empty() { return None; }
        return hex::decode(clean).ok();
    }
    None
}

/// Задеплоить контракты в state. Возвращает список (имя, адрес, размер runtime bytecode).
pub fn deploy_genesis_contracts(state: &mut crate::state::State) -> Vec<(String, [u8; 20], usize)> {
    let mut deployed = Vec::new();

    // AICID: 0x0000000000000000000000000000000000a1c1d0
    let aicid_addr: [u8; 20] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xA1, 0xC1, 0xD0,
    ];
    if let Some(code) = load_runtime_bytecode("AICID") {
        let size = code.len();
        state.set_code(aicid_addr, code);
        deployed.push(("AICID".to_string(), aicid_addr, size));
    }

    // InheritanceVault: 0x000000000000000000000000000000000000dead
    let vault_addr: [u8; 20] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xDE, 0xAD,
    ];
    if let Some(code) = load_runtime_bytecode("InheritanceVault") {
        let size = code.len();
        state.set_code(vault_addr, code);
        deployed.push(("InheritanceVault".to_string(), vault_addr, size));
    }

    // TimeCapsule: 0x0000000000000000000000000000000000c4b5e0
    let capsule_addr: [u8; 20] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xC4, 0xB5, 0xE0,
    ];
    if let Some(code) = load_runtime_bytecode("TimeCapsule") {
        let size = code.len();
        state.set_code(capsule_addr, code);
        deployed.push(("TimeCapsule".to_string(), capsule_addr, size));
    }

    // QuantumBounty: 0x000000000000000000000000000000000000B0B0
    let bounty_addr: [u8; 20] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xB0, 0xB0, 0x00,
    ];
    if let Some(code) = load_runtime_bytecode("QuantumBounty") {
        let size = code.len();
        state.set_code(bounty_addr, code);
        deployed.push(("QuantumBounty".to_string(), bounty_addr, size));
    }

    deployed
}

pub fn aicid_address() -> [u8; 20] {
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xA1, 0xC1, 0xD0]
}

