use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub fn in_repository() -> bool {
    let status = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match status {
        Ok(status) if status.success() => true,
        _ => false,
    }
}

pub fn repository_root() -> PathBuf {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .expect("Unable to identify root directory of the git directory.");
    let git_cmd_stdout = String::from_utf8(output.stdout).unwrap();
    Path::new(git_cmd_stdout.trim()).to_path_buf()
}

pub fn ignored(path: &str) -> bool {
    let status = Command::new("git")
        .args(["check-ignore", "-q", path])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match status {
        Ok(status) if status.success() => true,
        _ => false,
    }
}
