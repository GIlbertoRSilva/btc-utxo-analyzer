use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

const BASE_URL: &str = "https://blockstream.info/api";

pub struct EsploraClient {
    client: Client,
}

impl EsploraClient {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    pub async fn fetch_tx(&self, txid: &str) -> Result<Value> {
        let url = format!("{}/tx/{}", BASE_URL, txid);
        let res = self.client.get(&url).send().await?.error_for_status()?;
        Ok(res.json::<Value>().await?)
    }

    pub async fn fetch_address(&self, addr: &str) -> Result<Value> {
        let url = format!("{}/address/{}", BASE_URL, addr);
        let res = self.client.get(&url).send().await?.error_for_status()?;
        Ok(res.json::<Value>().await?)
    }
}
