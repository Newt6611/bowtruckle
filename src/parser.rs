use std::{error::Error, fmt};

use cardano_serialization_lib::{
    AssetName, FixedTransaction, Transaction as CslTransaction, Value,
};
use serde_json::{Value as JsonValue, json};

use crate::model::{AssetAmount, Transaction, TxInput, TxOutput};

#[derive(Debug)]
pub enum ParseError {
    Csl(String),
    Number(String),
    Json(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Csl(message) => write!(f, "CSL parse error: {message}"),
            Self::Number(message) => write!(f, "number parse error: {message}"),
            Self::Json(message) => write!(f, "JSON parse error: {message}"),
        }
    }
}

impl Error for ParseError {}

pub fn parse_transaction_hex(cbor_hex: &str) -> Result<Transaction, ParseError> {
    let csl_tx =
        CslTransaction::from_hex(cbor_hex).map_err(|err| ParseError::Csl(format!("{err:?}")))?;
    let fixed_tx =
        FixedTransaction::from_hex(cbor_hex).map_err(|err| ParseError::Csl(format!("{err:?}")))?;
    let body = csl_tx.body();

    let inputs = body.inputs();
    let outputs = body.outputs();

    Ok(Transaction {
        hash: fixed_tx.transaction_hash().to_hex(),
        is_valid: csl_tx.is_valid(),
        inputs: (0..inputs.len())
            .map(|index| {
                let input = inputs.get(index);
                TxInput {
                    tx_hash: input.transaction_id().to_hex(),
                    index: input.index(),
                    address: String::new(),
                    amount: Vec::new(),
                }
            })
            .collect(),
        outputs: (0..outputs.len())
            .map(|index| {
                let output = outputs.get(index);
                Ok(TxOutput {
                    address: output
                        .address()
                        .to_bech32(None)
                        .unwrap_or_else(|_| output.address().to_hex()),
                    amount: value_amounts(&output.amount())?,
                    datum: output_datum(&output),
                })
            })
            .collect::<Result<Vec<_>, ParseError>>()?,
    })
}

pub fn parse_transaction_json(cbor_hex: &str) -> Result<JsonValue, ParseError> {
    let csl_tx =
        CslTransaction::from_hex(cbor_hex).map_err(|err| ParseError::Csl(format!("{err:?}")))?;
    let fixed_tx =
        FixedTransaction::from_hex(cbor_hex).map_err(|err| ParseError::Csl(format!("{err:?}")))?;
    let tx_json: JsonValue = serde_json::from_str(
        &csl_tx
            .to_json()
            .map_err(|err| ParseError::Csl(format!("{err:?}")))?,
    )
    .map_err(|err| ParseError::Json(err.to_string()))?;

    Ok(json!({
        "transaction_hash": fixed_tx.transaction_hash().to_hex(),
        "transaction": tx_json,
    }))
}

fn value_amounts(value: &Value) -> Result<Vec<AssetAmount>, ParseError> {
    let mut amounts = vec![AssetAmount::ada(parse_u64(&value.coin().to_str())?)];

    if let Some(multiasset) = value.multiasset() {
        let policies = multiasset.keys();
        for policy_index in 0..policies.len() {
            let policy_id = policies.get(policy_index);
            let Some(assets) = multiasset.get(&policy_id) else {
                continue;
            };
            let asset_names = assets.keys();
            for asset_index in 0..asset_names.len() {
                let asset_name = asset_names.get(asset_index);
                let Some(quantity) = assets.get(&asset_name) else {
                    continue;
                };
                amounts.push(AssetAmount {
                    policy_id: Some(policy_id.to_hex()),
                    asset_name: asset_name_label(&asset_name),
                    asset_name_hex: Some(hex_string(&asset_name.name())),
                    quantity: parse_u64(&quantity.to_str())?,
                });
            }
        }
    }

    Ok(amounts)
}

fn output_datum(output: &cardano_serialization_lib::TransactionOutput) -> Option<String> {
    if let Some(data_hash) = output.data_hash() {
        return Some(format!("datum hash {}", data_hash.to_hex()));
    }

    output
        .plutus_data()
        .map(|data| format!("inline datum {}", data.to_hex()))
}

fn asset_name_label(asset_name: &AssetName) -> String {
    let bytes = asset_name.name();
    String::from_utf8(bytes.clone()).unwrap_or_else(|_| hex_string(&bytes))
}

fn parse_u64(value: &str) -> Result<u64, ParseError> {
    value
        .parse()
        .map_err(|err| ParseError::Number(format!("{value}: {err}")))
}

fn hex_string(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut value = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        value.push(HEX[(byte >> 4) as usize] as char);
        value.push(HEX[(byte & 0x0f) as usize] as char);
    }
    value
}
