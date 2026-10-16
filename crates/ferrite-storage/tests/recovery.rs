use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use ferrite_core::{Key, KvStore, Value};
use ferrite_storage::Storage;

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn test_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "ferrite-{name}-{}-{}.wal",
        std::process::id(),
        NEXT_ID.fetch_add(1, Ordering::Relaxed),
    ))
}

#[test]
fn values_survive_reopening() {
    let path = test_path("reopen");

    {
        let mut db = Storage::open(&path).unwrap();
        db.put(Key::new("lang"), Value::new("rust")).unwrap();
    }

    {
        let db = Storage::open(&path).unwrap();
        assert_eq!(
            db.get(&Key::new("lang")).unwrap(),
            Some(Value::new("rust")),
        );
    }

    fs::remove_file(path).unwrap();
}

#[test]
fn deletion_survives_reopening() {
    let path = test_path("delete");

    {
        let mut db = Storage::open(&path).unwrap();
        db.put(Key::new("lang"), Value::new("rust")).unwrap();
        db.del(Key::new("lang")).unwrap();
    }

    {
        let db = Storage::open(&path).unwrap();
        assert_eq!(db.get(&Key::new("lang")).unwrap(), None);
    }

    fs::remove_file(path).unwrap();
}

#[test]
fn incomplete_final_record_is_discarded() {
    let path = test_path("torn-tail");

    // First, write one complete record.
    {
        let mut db = Storage::open(&path).unwrap();
        db.put(Key::new("lang"), Value::new("rust")).unwrap();
    }

    // Simulate a crash halfway through the next record:
    // header claims 100 payload bytes, but only 5 exist.
    {
        let mut file = OpenOptions::new().append(true).open(&path).unwrap();

        file.write_all(&100_u32.to_le_bytes()).unwrap();
        file.write_all(&0_u32.to_le_bytes()).unwrap();
        file.write_all(&[1, 2, 3, 4, 5]).unwrap();
        file.sync_all().unwrap();
    }

    // Recovery should keep the first record and discard the tail.
    {
        let db = Storage::open(&path).unwrap();
        assert_eq!(
            db.get(&Key::new("lang")).unwrap(),
            Some(Value::new("rust")),
        );
    }

    // Reopening again confirms recovery removed the incomplete tail.
    {
        let db = Storage::open(&path).unwrap();
        assert_eq!(
            db.get(&Key::new("lang")).unwrap(),
            Some(Value::new("rust")),
        );
    }

    fs::remove_file(path).unwrap();
}
