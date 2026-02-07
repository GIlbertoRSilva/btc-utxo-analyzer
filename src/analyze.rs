use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Prevout {
    pub value: Option<u64>,
    pub scriptpubkey_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Vin {
    pub prevout: Option<Prevout>,
}

#[derive(Debug, Deserialize)]
pub struct Vout {
    pub value: u64,
    pub scriptpubkey_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Tx {
    pub txid: String,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub fee: Option<u64>,
    pub vsize: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Debug)]
pub struct TxSummary {
    pub txid: String,
    pub n_inputs: usize,
    pub n_outputs: usize,
    pub total_in_sat: u64,
    pub total_out_sat: u64,
    pub fee_sat: u64,
    pub vsize: u64,
}

impl TxSummary {
    pub fn fee_rate_sat_vb(&self) -> f64 {
        if self.vsize == 0 { 0.0 } else { self.fee_sat as f64 / self.vsize as f64 }
    }
}

pub fn summarize_tx(tx: Tx) -> TxSummary {
    let total_in: u64 = tx
        .vin
        .iter()
        .filter_map(|v| v.prevout.as_ref())
        .filter_map(|p| p.value)
        .sum();

    let total_out: u64 = tx.vout.iter().map(|o| o.value).sum();

    let fee = tx.fee.unwrap_or_else(|| {
        if total_in >= total_out { total_in - total_out } else { 0 }
    });

    let vsize = tx.vsize.or(tx.size).unwrap_or(0);

    TxSummary {
        txid: tx.txid,
        n_inputs: tx.vin.len(),
        n_outputs: tx.vout.len(),
        total_in_sat: total_in,
        total_out_sat: total_out,
        fee_sat: fee,
        vsize,
    }
}