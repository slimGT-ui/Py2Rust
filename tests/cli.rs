use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_dir(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "py2rust-cli-{label}-{}-{nonce}",
        std::process::id()
    ))
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_py2rust"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn help_and_version_work() {
    let help = run(&["--help"]);
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("USAGE:"));

    let version = run(&["--version"]);
    assert!(version.status.success());
    assert!(String::from_utf8_lossy(&version.stdout).contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn missing_argument_is_a_usage_error() {
    let output = run(&[]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("missing input file"));
}

#[test]
fn valid_input_is_loaded_without_claiming_translation() {
    let dir = temp_dir("valid");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("example.py");
    fs::write(&path, "print('hello')\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_py2rust"))
        .arg(&path)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Loaded Python source:"));
    assert!(stdout.contains("Translation is not implemented yet."));
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn missing_file_returns_an_error() {
    let path = temp_dir("missing").join("missing.py");
    let output = Command::new(env!("CARGO_BIN_EXE_py2rust"))
        .arg(path)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("does not exist"));
}

#[test]
fn directory_and_wrong_extension_are_rejected() {
    let dir = temp_dir("directory.py");
    fs::create_dir_all(&dir).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_py2rust"))
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("is not a file"));
    fs::remove_dir_all(&dir).unwrap();

    let dir = temp_dir("extension");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("input.txt");
    fs::write(&path, "text").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_py2rust"))
        .arg(&path)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("expected a Python source file"));
    fs::remove_dir_all(dir).unwrap();
}
