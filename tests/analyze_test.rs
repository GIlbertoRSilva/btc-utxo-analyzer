use btc_utxo_analyzer::analyze::{summarize_tx, Tx};

#[test]
fn summarize_tx_computes_metrics() {
    let tx_json = serde_json::json!({
        "txid": "testtx",
        "vin": [
            { "prevout": { "value": 20688 } }
        ],
        "vout": [
            { "value": 20668 },
            { "value": 0 }
        ],
        "fee": 20,
        "vsize": 221
    });

    let tx: Tx = serde_json::from_value(tx_json).unwrap();
    let s = summarize_tx(tx);

    assert_eq!(s.n_inputs, 1);
    assert_eq!(s.n_outputs, 2);
    assert_eq!(s.total_in_sat, 20688);
    assert_eq!(s.total_out_sat, 20668);
    assert_eq!(s.fee_sat, 20);
    assert_eq!(s.vsize, 221);
}
