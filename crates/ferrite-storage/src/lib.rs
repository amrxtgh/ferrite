use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

use ferrite_core::{Error, Key, KvStore, Result, Value};

const MAX_RECORD_SIZE: usize = 64 * 1024 * 1024;

pub struct Storage {
    file: File,
    index: HashMap<Key, Value>,
}

impl Storage {
    // what is path here tbh
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path)
            .map_err(storage_error)?;

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(storage_error)?;

        let mut index = HashMap::new();
        let mut pos = 0;

        while pos < bytes.len() {
            let record_start = pos;

            // Record header: payload length (u32), checksum (u32).
            if bytes.len() - pos < 8 {
                break; // Incomplete final header.
            }

            let len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
            pos += 4;

            let checksum = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap());
            pos += 4;

            if len > MAX_RECORD_SIZE {
                return Err(Error::InvalidRecord("record exceeds maximum size".into()));
            }

            if bytes.len() - pos < len {
                pos = record_start;
                break; // Incomplete final payload.
            }

            let payload = &bytes[pos..pos + len];

            if crc32fast::hash(payload) != checksum {
                return Err(Error::InvalidRecord(format!(
                    "checksum mismatch at byte {record_start}"
                )));
            }

            replay_record(payload, &mut index)?;
            pos += len;
        }

        // Discard an incomplete tail left by a crash.
        if pos != bytes.len() {
            file.set_len(pos as u64).map_err(storage_error)?;
        }

        file.seek(SeekFrom::End(0)).map_err(storage_error)?;

        Ok(Self { file, index })
    }

    fn append(&mut self, payload: &[u8]) -> Result<()> {
        let len = u32::try_from(payload.len())
            .map_err(|_| Error::InvalidRecord("record too large".into()))?;

        if payload.len() > MAX_RECORD_SIZE {
            return Err(Error::InvalidRecord("record too large".into()));
        }

        let checksum = crc32fast::hash(payload);

        self.file
            .write_all(&len.to_le_bytes())
            .map_err(storage_error)?;
        self.file
            .write_all(&checksum.to_le_bytes())
            .map_err(storage_error)?;
        self.file.write_all(payload).map_err(storage_error)?;

        // A successful durable write is not returned before this succeeds.
        self.file.sync_data().map_err(storage_error)?;

        Ok(())
    }
}

impl KvStore for Storage {
    fn put(&mut self, key: Key, value: Value) -> Result<()> {
        let mut payload = Vec::new();
        payload.push(1);
        payload.extend_from_slice(
            &u32::try_from(key.as_bytes().len())
                .map_err(|_| Error::InvalidRecord("key too large".into()))?
                .to_le_bytes(),
        );
        payload.extend_from_slice(
            &u32::try_from(value.as_bytes().len())
                .map_err(|_| Error::InvalidRecord("value too large".into()))?
                .to_le_bytes(),
        );
        payload.extend_from_slice(key.as_bytes());
        payload.extend_from_slice(value.as_bytes());

        self.append(&payload)?;
        self.index.insert(key, value);
        Ok(())
    }

    fn get(&mut self, key: &Key) -> Result<Option<Value>> {
        Ok(self.index.get(key).cloned())
    }

    fn del(&mut self, key: Key) -> Result<()> {
        let mut payload = Vec::new();
        payload.push(2); // DELETE
        payload.extend_from_slice(
            &u32::try_from(key.as_bytes().len())
                .map_err(|_| Error::InvalidRecord("key too large".into()))?
                .to_le_bytes(),
        );
        payload.extend_from_slice(&0_u32.to_le_bytes());
        payload.extend_from_slice(key.as_bytes());

        self.append(&payload)?;
        self.index.remove(&key);
        Ok(())
    }
}

fn replay_record(payload: &[u8], index: &mut HashMap<Key, Value>) -> Result<()> {
    if payload.len() < 9 {
        return Err(Error::InvalidRecord("record header too short".into()));
    }

    let op = payload[0];
    let key_len = u32::from_le_bytes(payload[1..5].try_into().unwrap()) as usize;
    let value_len = u32::from_le_bytes(payload[5..9].try_into().unwrap()) as usize;

    let expected = 9_usize
        .checked_add(key_len)
        .and_then(|n| n.checked_add(value_len))
        .ok_or_else(|| Error::InvalidRecord("length overflow".into()))?;

    if expected != payload.len() {
        return Err(Error::InvalidRecord("invalid record lengths".into()));
    }

    let key = Key::new(payload[9..9 + key_len].to_vec());
    let value_bytes = &payload[9 + key_len..];

    match op {
        1 => {
            let value = Value::new(value_bytes.to_vec());
            index.insert(key, value);
        }
        2 if value_len == 0 => {
            index.remove(&key);
        }
        _ => {
            return Err(Error::InvalidRecord("unknown operation".into()));
        }
    }

    Ok(())
}

fn storage_error(error: std::io::Error) -> Error {
    Error::Storage(error.to_string())
}
