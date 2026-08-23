// SPDX-License-Identifier: GPL-2.0-or-later
//! Filenames that cannot live as fixtures in git.

use std::ffi::OsString;
use std::fs;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;
use std::process::Command;

fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/it-scratch")
        .join(format!("{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn non_utf8_names_are_octal_escaped_not_a_panic() {
    let dir = scratch("non-utf8");
    fs::write(dir.join(OsString::from_vec(vec![0xff, 0xfe, b'x'])), b"").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_utree"))
        .arg(&dir)
        .env("LC_ALL", "C")
        .env("TREE_CHARSET", "UTF-8")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\\377\\376x"), "got: {stdout}");
    assert!(stdout.contains("1 directory, 1 file"), "got: {stdout}");
}

#[test]
fn filenames_with_spaces_are_backslash_escaped() {
    let dir = scratch("spaces");
    fs::write(dir.join("with space"), b"").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_utree"))
        .arg(&dir)
        .env("LC_ALL", "C")
        .env("TREE_CHARSET", "UTF-8")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("with\\ space"), "got: {stdout}");
}
