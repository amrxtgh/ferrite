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


fn main() {
    println!("Hello, world!");
}
