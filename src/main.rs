use std::{path::PathBuf, process};

use clap::{Parser, Subcommand};
use ferrite_core::{Key, KvStore, Value};
use ferrite_storage::Storage;

#[derive(Debug, Parser)]
#[command(
    name = "ferrite",
    version,
    about = "A persistent key-value store",
    long_about = None
)]
struct Cli {
    /// WAL file to use [env: FERRITE_DATA=]
    #[arg(
        short,
        long,
        global = true,
        env = "FERRITE_DATA",
        value_name = "PATH",
        default_value = "ferrite.wal"
    )]
    data: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Store a value for a key
    Put {
        #[arg(value_name = "KEY")]
        key: String,
        #[arg(value_name = "VALUE")]
        value: String,
    },

    /// Retrieve the value for a key
    Get {
        #[arg(value_name = "KEY")]
        key: String,
    },

    /// Delete a key
    #[command(alias = "delete")]
    Del {
        #[arg(value_name = "KEY")]
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("error: {error}");
        process::exit(1);
    }
}

fn run(cli: Cli) -> ferrite_core::Result<()> {
    let mut store = Storage::open(cli.data)?;

    match cli.command {
        Command::Put { key, value } => {
            store.put(Key::new(key), Value::new(value.into_bytes()))?;
            println!("OK");
        }
        Command::Get { key } => match store.get(&Key::new(key))? {
            Some(value) => println!("{}", String::from_utf8_lossy(value.as_bytes())),
            None => println!("(nil)"),
        },
        Command::Del { key } => {
            store.del(Key::new(key))?;
            println!("OK");
        }
    }

    Ok(())
}
