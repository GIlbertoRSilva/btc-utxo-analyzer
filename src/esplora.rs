use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

const BASE_URL: &str = "https://blockstream.info/api";

pub struct EsploraClient {
    client: Client,
    cache_dir: PathBuf,
}

impl EsploraClient {
    pub fn new() -> Self {
        let cache_dir = PathBuf::from("cache");
        if !cache_dir.exists() {
            let _ = fs::create_dir_all(&cache_dir);
        }

        Self {
            client: Client::new(),
            cache_dir,
        }
    }

    fn cache_path(&self, prefix: &str, key: &str) -> PathBuf {
        self.cache_dir.join(format!("{}_{}.json", prefix, key))
    }

    fn read_cache(&self, path: &Path) -> Option<Value> {
        let data = fs::read_to_string(path).ok()?;
        serde_json::from_str(&data).ok()
    }

    fn write_cache(&self, path: &Path, value: &Value) -> Result<()> {
        fs::write(path, serde_json::to_string_pretty(value)?)?;
        Ok(())
    }

    async fn fetch_json_cached(&self, prefix: &str, url: String, key: &str) -> Result<Value> {
        let path = self.cache_path(prefix, key);

        if let Some(v) = self.read_cache(&path) {
            return Ok(v);
        }

        let res = self.client.get(&url).send().await?.error_for_status()?;
        let json = res.json::<Value>().await?;
        self.write_cache(&path, &json)?;
        Ok(json)
    }

    pub async fn fetch_tx(&self, txid: &str) -> Result<Value> {
        let url = format!("{}/tx/{}", BASE_URL, txid);
        self.fetch_json_cached("tx", url, txid).await
    }

    pub async fn fetch_address(&self, addr: &str) -> Result<Value> {
        let url = format!("{}/address/{}", BASE_URL, addr);
        let safe = addr.replace(":", "_");
        self.fetch_json_cached("addr", url, &safe).await
    }
}