use crate::analyze::Tx;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

fn ensure_dir(dir: &Path) -> Result<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

// Escape quotes for DOT labels
fn esc(s: &str) -> String {
    s.replace('"', "\\\"")
}

pub fn write_tx_graph_dot(tx: &Tx) -> Result<PathBuf> {
    let out_dir = Path::new("graphs");
    ensure_dir(out_dir)?;

    let dot_path = out_dir.join(format!("{}.dot", tx.txid));

    let mut dot = String::new();
    dot.push_str("digraph tx {\n");
    dot.push_str("  rankdir=LR;\n");
    dot.push_str("  node [shape=box];\n");

    dot.push_str(&format!("  tx [label=\"TX\\n{}\"];\n", esc(&tx.txid)));

    // Inputs
    for (i, vin) in tx.vin.iter().enumerate() {
        let addr = vin
            .prevout
            .as_ref()
            .and_then(|p| p.scriptpubkey_address.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("unknown");

        let val = vin.prevout.as_ref().and_then(|p| p.value).unwrap_or(0);

        dot.push_str(&format!(
            "  in{} [label=\"IN {}\\n{} sats\\n{}\"];\n",
            i,
            i,
            val,
            esc(addr)
        ));
        dot.push_str(&format!("  in{} -> tx;\n", i));
    }

    // Outputs
    for (i, vout) in tx.vout.iter().enumerate() {
        let addr = vout
            .scriptpubkey_address
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("unknown");

        dot.push_str(&format!(
            "  out{} [label=\"OUT {}\\n{} sats\\n{}\"];\n",
            i,
            i,
            vout.value,
            esc(addr)
        ));
        dot.push_str(&format!("  tx -> out{};\n", i));
    }

    dot.push_str("}\n");

    fs::write(&dot_path, dot)?;
    Ok(dot_path)
}