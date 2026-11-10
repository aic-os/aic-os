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

        // Constructor не вызывается при genesis -> инициализируем storage вручную.
        // TimeCapsule.sol: uint256 public nextId = 1; -> slot 0
        let mut slot_next_id = [0u8; 32];
        slot_next_id[31] = 1;
        state.set_storage(capsule_addr, [0u8; 32], slot_next_id);
    }

    // QuantumBounty: 0x000000000000000000000000000000000000B0B0
    let bounty_addr: [u8; 20] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xB0, 0xB0, 0x00,
    ];
    if let Some(code) = load_runtime_bytecode("QuantumBounty") {
        let size = code.len();
        state.set_code(bounty_addr, code);
        deployed.push(("QuantumBounty".to_string(), bounty_addr, size));

        // Owner = Alice (dev), treasury = 0x...0F
        let alice: [u8; 20] = [
            0xf3, 0x9F, 0xd6, 0xe5, 0x1a, 0xad, 0x88, 0xF6,
            0xF4, 0xce, 0x6a, 0xB8, 0x82, 0x72, 0x79, 0xcf,
            0xff, 0xb9, 0x22, 0x66,
        ];
        let treasury: [u8; 20] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0F,
        ];

        // Slot 0 = owner
        let mut slot0 = [0u8; 32];
        slot0[12..].copy_from_slice(&alice);
        state.set_storage(bounty_addr, [0u8; 32], slot0);

        // Slot 1 = treasury
        let mut slot1 = [0u8; 32];
        slot1[12..].copy_from_slice(&treasury);
        let mut key1 = [0u8; 32];
        key1[31] = 1;
        state.set_storage(bounty_addr, key1, slot1);
    }

    // AcoinBridge: 0x0000000000000000000000000000000000AA11CE
    let bridge_addr: [u8; 20] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xAA, 0x11, 0xCE,
    ];
    if let Some(code) = load_runtime_bytecode("AcoinBridge") {
        let size = code.len();
        state.set_code(bridge_addr, code);
        deployed.push(("AcoinBridge".to_string(), bridge_addr, size));

        let alice_bridge: [u8; 20] = [
            0xf3, 0x9F, 0xd6, 0xe5, 0x1a, 0xad, 0x88, 0xF6,
            0xF4, 0xce, 0x6a, 0xB8, 0x82, 0x72, 0x79, 0xcf,
            0xff, 0xb9, 0x22, 0x66,
        ];
        // Slot 0: owner (20 bytes) + threshold=1 (1 byte at offset 20)
        let mut slot0 = [0u8; 32];
        slot0[12..].copy_from_slice(&alice_bridge);
        slot0[11] = 1;
        state.set_storage(bridge_addr, [0u8; 32], slot0);
    }

    deployed
}

pub fn aicid_address() -> [u8; 20] {
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xA1, 0xC1, 0xD0]
}




