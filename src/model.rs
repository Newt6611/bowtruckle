#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub hash: String,
    pub is_valid: bool,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxInput {
    pub tx_hash: String,
    pub index: u32,
    pub address: String,
    pub amount: Vec<AssetAmount>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxOutput {
    pub address: String,
    pub amount: Vec<AssetAmount>,
    pub datum: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetAmount {
    pub policy_id: Option<String>,
    pub asset_name: String,
    pub asset_name_hex: Option<String>,
    pub quantity: u64,
}

impl Transaction {
    pub fn demo() -> Self {
        Self {
            hash: "4d2f8f0b5d9f2a6d2c8f6a33458ef9a1b3d5c7e9f0123456789abcdef012345".to_string(),
            is_valid: true,
            inputs: vec![
                TxInput {
                    tx_hash: "a9b34df3c2198d6d0d45e8a6b02c54fcb92f5f8b2e07a03b7f0ac9417c73f591"
                        .to_string(),
                    index: 0,
                    address: "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3w9ss0t9...".to_string(),
                    amount: vec![AssetAmount::ada(1_824_500)],
                },
                TxInput {
                    tx_hash: "ff1034ea2bd44e702abc6818f09f5411f936a7f3e5f506b20978bd80a74243ee"
                        .to_string(),
                    index: 2,
                    address: "addr1q8nj9v35l6xlqva6kjfdv9x20r2kwckst2mey33q3h3nw...".to_string(),
                    amount: vec![
                        AssetAmount::ada(12_500_000),
                        AssetAmount::native("BOOK", 42),
                    ],
                },
                TxInput {
                    tx_hash: "45b04e912f66312ac9cb624e5abfca2388cc1357d31927497c8a60108b935ee0"
                        .to_string(),
                    index: 1,
                    address: "addr1vyc9mppfrru9wgjzq6qg5u2z48yg54kp5rvxk8a7f9...".to_string(),
                    amount: vec![AssetAmount::ada(3_000_000)],
                },
            ],
            outputs: vec![
                TxOutput {
                    address: "addr1qxk0te3t32v09jxk6xxh4x8v9py3s6q2shdp9ad6p...".to_string(),
                    amount: vec![AssetAmount::ada(10_000_000)],
                    datum: None,
                },
                TxOutput {
                    address: "addr1w8r4ntgeqe4l5s07j0n80r0nr66j3j42fz8e4s4a...".to_string(),
                    amount: vec![AssetAmount::ada(6_925_000), AssetAmount::native("BOOK", 42)],
                    datum: Some("inline datum: 9f581c...".to_string()),
                },
                TxOutput {
                    address: "addr1v8d9cn7q4z4j2jy4ecjztdg60gx5f7ezlmy0tz2m...".to_string(),
                    amount: vec![AssetAmount::ada(255_000)],
                    datum: None,
                },
            ],
        }
    }
}

impl AssetAmount {
    pub fn ada(lovelace: u64) -> Self {
        Self {
            policy_id: None,
            asset_name: "ADA".to_string(),
            asset_name_hex: None,
            quantity: lovelace,
        }
    }

    pub fn native(asset_name: impl Into<String>, quantity: u64) -> Self {
        Self {
            policy_id: Some("policy1m5sg4vn8k...".to_string()),
            asset_name: asset_name.into(),
            asset_name_hex: None,
            quantity,
        }
    }
}
