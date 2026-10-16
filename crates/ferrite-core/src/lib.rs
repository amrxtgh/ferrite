// put(key,value) get(key) del(key)
// persist write with write-ahead log (WAL) recover the state after restarting the process

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Key(Vec<u8>);

impl Key {
    // at the storage layer all the key should be in bytes
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Key(bytes.into())
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value(Vec<u8>);

impl Value {
    // but bytes can be anything
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Value(bytes.into())
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("storage record: {0}")]
    Storage(String),
    #[error("Invalid record: {0}")]
    InvalidRecord(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait KvStore {
    fn put(&mut self, key: Key, val: Value) -> Result<()>;
    fn get(&self, key: &Key) -> Result<Option<Value>>;
    fn del(&mut self, key: Key) -> Result<()>;
}
