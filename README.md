
# btc-utxo-analyzer

Rust CLI to analyze Bitcoin transactions (UTXO model) using the public Esplora (Blockstream) API.
It outputs summarized metrics, generates reports (MD/JSON), caches requests locally, and exports Graphviz DOT graphs.

## Commands
- `tx <txid>`: summarized metrics (inputs/outputs, total_in/out, fee, vsize, feerate)
- `report <txid>`: writes `reports/<txid>.md` and `reports/<txid>.json`
- `graph <txid>`: writes `graphs/<txid>.dot`
- `address <addr>`: fetches raw address JSON

## Quickstart
```bash
cargo run -- tx <TXID>
cargo run -- report <TXID>
cargo run -- graph <TXID>
