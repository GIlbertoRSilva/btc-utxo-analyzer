use crate::analyze::TxSummary;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

fn ensure_dir(dir: &Path) -> Result<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

pub fn write_report_files(summary: &TxSummary) -> Result<(PathBuf, PathBuf)> {
    let out_dir = Path::new("reports");
    ensure_dir(out_dir)?;

    let json_path = out_dir.join(format!("{}.json", summary.txid));
    let md_path = out_dir.join(format!("{}.md", summary.txid));

    let json_obj = serde_json::json!({
        "txid": summary.txid,
        "n_inputs": summary.n_inputs,
        "n_outputs": summary.n_outputs,
        "total_in_sat": summary.total_in_sat,
        "total_out_sat": summary.total_out_sat,
        "fee_sat": summary.fee_sat,
        "vsize": summary.vsize,
        "fee_rate_sat_vb": summary.fee_rate_sat_vb()
    });

    fs::write(&json_path, serde_json::to_string_pretty(&json_obj)?)?;

    let md = format!(
        "# Bitcoin Transaction Report\n\n\
         **TXID:** `{}`\n\n\
         ## Summary\n\
         - Inputs: **{}**\n\
         - Outputs: **{}**\n\
         - Total in: **{} sats**\n\
         - Total out: **{} sats**\n\
         - Fee: **{} sats**\n\
         - vsize: **{} vB**\n\
         - Fee rate: **{:.2} sat/vB**\n\n\
         ## Notes\n\
         This report was generated from public Esplora API data and summarizes UTXO flow.\n",
        summary.txid,
        summary.n_inputs,
        summary.n_outputs,
        summary.total_in_sat,
        summary.total_out_sat,
        summary.fee_sat,
        summary.vsize,
        summary.fee_rate_sat_vb()
    );

    fs::write(&md_path, md)?;

    Ok((json_path, md_path))
}
