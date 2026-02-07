use crate::analyze::{summarize_tx, Tx};
use crate::esplora::EsploraClient;
use crate::report::write_report_files;
use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::json;

#[derive(Parser)]
#[command(name = "btc-utxo-analyzer")]
#[command(about = "Bitcoin UTXO Analyzer (Day 3)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print summarized tx metrics
    Tx { txid: String },

    /// Fetch and display raw address info JSON
    Address { addr: String },

    /// Generate report files (reports/<txid>.json and reports/<txid>.md)
    Report { txid: String },
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let client = EsploraClient::new();

        match self.command {
            Commands::Tx { txid } => {
                let tx_json = client.fetch_tx(&txid).await?;
                let tx: Tx = serde_json::from_value(tx_json)?;
                let s = summarize_tx(tx);

                let out = json!({
                    "txid": s.txid,
                    "n_inputs": s.n_inputs,
                    "n_outputs": s.n_outputs,
                    "total_in_sat": s.total_in_sat,
                    "total_out_sat": s.total_out_sat,
                    "fee_sat": s.fee_sat,
                    "vsize": s.vsize,
                    "fee_rate_sat_vb": s.fee_rate_sat_vb()
                });

                println!("{}", serde_json::to_string_pretty(&out)?);
            }
            Commands::Address { addr } => {
                let info = client.fetch_address(&addr).await?;
                println!("{}", serde_json::to_string_pretty(&info)?);
            }
            Commands::Report { txid } => {
                let tx_json = client.fetch_tx(&txid).await?;
                let tx: Tx = serde_json::from_value(tx_json)?;
                let s = summarize_tx(tx);

                let (json_path, md_path) = write_report_files(&s)?;
                println!(
                    "{}",
                    serde_json::to_string_pretty(&json!({
                        "report_json": json_path.to_string_lossy(),
                        "report_md": md_path.to_string_lossy()
                    }))?
                );
            }
        }

        Ok(())
    }
}
