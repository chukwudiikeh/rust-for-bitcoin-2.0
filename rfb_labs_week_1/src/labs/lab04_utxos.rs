//! Lab 04 — inspect UTXOs and outpoints.

use crate::model::{OutPoint, Utxo};
use crate::rpc::{parse_cli_value, RpcClient};
use crate::LabResult;

/// Return all UTXOs tracked by the selected wallet.
pub fn list_unspent<C: RpcClient>(client: &C, wallet_name: &str) -> LabResult<Vec<Utxo>> {
    let raw = client.call(Some(wallet_name), "listunspent", &[])?;
    let value = parse_cli_value(&raw)?;
    value
        .as_array()
        .ok_or_else(|| crate::LabError::Parse("expected an array of UTXOs".to_owned()))?
        .iter()
        .map(|v| {
            serde_json::from_value(v.clone()).map_err(|e| crate::LabError::Parse(e.to_string()))
        })
        .collect()
}

/// Select one spendable UTXO, preferring the one with the most confirmations.
pub fn select_spendable_utxo(utxos: &[Utxo]) -> Option<Utxo> {
    utxos
        .iter()
        .filter(|u| u.spendable)
        .max_by_key(|u| u.confirmations)
        .cloned()
}

/// Convert a UTXO into its unique `txid:vout` coordinate.
pub fn outpoint(utxo: &Utxo) -> OutPoint {
    utxo.outpoint()
}

/// Sum only the spendable UTXOs.
pub fn sum_spendable_utxos(utxos: &[Utxo]) -> f64 {
    utxos.iter().filter(|u| u.spendable).map(|u| u.amount).sum()
}
