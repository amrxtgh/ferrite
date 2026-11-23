use std::{
    fs,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn data_path() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "ferrite-cli-{}-{}.wal",
        std::process::id(),
        NEXT_ID.fetch_add(1, Ordering::Relaxed),
    ))
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ferrite"))
        .args(args)
        .output()
        .expect("failed to run ferrite")
}

#[test]
fn help_and_version_are_available() {
    let help = run(&["--help"]);
    assert!(help.status.success());
    let help = String::from_utf8(help.stdout).unwrap();
    assert!(help.contains("persistent key-value store"));
    assert!(help.contains("put"));
    assert!(help.contains("get"));
    assert!(help.contains("del"));

    let version = run(&["--version"]);
    assert!(version.status.success());
    assert!(
        String::from_utf8(version.stdout)
            .unwrap()
            .starts_with("ferrite ")
    );
}

#[test]
fn commands_use_the_selected_data_file() {
    let path = data_path();
    let path_arg = path.to_str().unwrap();

    let put = run(&["--data", path_arg, "put", "language", "rust"]);
    assert!(put.status.success());
    assert_eq!(String::from_utf8(put.stdout).unwrap().trim(), "OK");

    let get = run(&["get", "language", "--data", path_arg]);
    assert!(get.status.success());
    assert_eq!(String::from_utf8(get.stdout).unwrap().trim(), "rust");

    let delete = run(&["--data", path_arg, "del", "language"]);
    assert!(delete.status.success());
    assert_eq!(String::from_utf8(delete.stdout).unwrap().trim(), "OK");

    let missing = run(&["--data", path_arg, "get", "language"]);
    assert!(missing.status.success());
    assert_eq!(String::from_utf8(missing.stdout).unwrap().trim(), "(nil)");

    fs::remove_file(path).unwrap();
}

#[test]
fn invalid_arguments_return_a_cli_error() {
    let output = run(&["put", "only-a-key"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8(output.stderr).unwrap().contains("Usage:"));
}
