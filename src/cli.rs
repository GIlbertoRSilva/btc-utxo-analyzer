use crate::esplora::EsploraClient;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "btc-utxo-analyzer")]
#[command(about = "Bitcoin UTXO Analyzer (Day 1 MVP)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Fetch and display raw transaction JSON
    Tx { txid: String },

    /// Fetch and display raw address info JSON
    Address { addr: String },
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let client = EsploraClient::new();

        match self.command {
            Commands::Tx { txid } => {
                let tx = client.fetch_tx(&txid).await?;
                println!("{}", serde_json::to_string_pretty(&tx)?);
            }
            Commands::Address { addr } => {
                let info = client.fetch_address(&addr).await?;
                println!("{}", serde_json::to_string_pretty(&info)?);
            }
        }

        Ok(())
    }
}
