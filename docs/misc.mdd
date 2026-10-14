`ferrite-storage, ferrite-raft, ferrite-server, ferrite-client, ferrite-sim, ferrite-bench.`
ci with `cargo test cargo clippy and fmt`

building an lsm tree
- append only log with crc checksums and crash recovery by replay
- memtable `BTreeMap` or skiplist flushed to disk when full
- sstable with sorted block, an index block and a footer
- bloomfilter per sstable 
- Compaction (size tiered first, leveled later) and tombstones for deletes
- range scans via a merging iterator

kill the process at random points and verify recovery.
proptest to compare engine against plain hashmap model

