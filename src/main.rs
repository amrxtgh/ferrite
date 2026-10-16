//! Ferrite: multi-node, Raft-replicated KV store with MVCC transactions.
//!
//! Raft lets a group of machines agree on the same sequence of events,
//! even when some of them crash or the network drops messages.
//!
//! Goal: 3-5 nodes that survive crashes and network partitions without
//! losing acknowledged writes, with dashboards and benchmark numbers.
//!
//! Planned building blocks:
//! - Async runtime (Tokio)
//! - RPC and wire format
//! - Checksums and byte handling
//! - Logging and tracing
//! - Metrics
//! - CLI
//! - Benchmarks
//! - Property tests
//! - Deterministic network simulation
//! - Concurrency model checking
//! - Chaos testing
//! - Dashboards

use ferrite_core::{Key, KvStore, Result, Value};
use ferrite_storage::Storage;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut store = Storage::open("ferrite.wal")?;
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["put", k, v] => {
            store.put(Key::new(*k), Value::new(v.as_bytes().to_vec()))?;
        }
        ["get", k] => match store.get(&Key::new(*k)) {
            Ok(Some(v)) => println!("{}", String::from_utf8_lossy(v.as_bytes())),
            Ok(None) => println!("(nil)"),
            Err(e) => eprintln!("get failed: {e}"),
        },
        ["del", k] => {
            store.del(Key::new(*k))?;
            println!("OK");
        }
        _ => eprintln!("usage: ferrite put <k> <v> | get <k> | del <k>"),
    }
    Ok(())
}
