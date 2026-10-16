use ferrite_core::{Key, KvStore, Result, Value};
use ferrite_storage::Storage;
// multi-node, raft-replicated, shared KV store with MVCC transaction
// raft- consensus algorithms lets the group of machines agrees on the same
// seq of events, even when some of them crashes or the network drop messages.

// Outcome of this is 3 to 5 node that survives crashes and network
// partitions without losing ack writes with dashboards, benchmark numbers
// - async runtime - Tokio
// - RPC and wire format
// - checksums and bytes
// - Logging and tracing
// - metrics
// - cli
// - benchmarks
// - property-tests
// - deterministic network simulation
// - concurrency model checking
// - chaos
// - dashboards

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
