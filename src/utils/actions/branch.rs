use std::{path::Path, process::Command};

pub fn get_branch(pwd: &Path) -> String {
    let mut branch = String::new();
    let mut cmd = Command::new("git");
    cmd.arg("branch");
    cmd.arg("--show-current");
    cmd.current_dir(pwd);
    let output = cmd.output().expect("failed to execute process");
    if output.status.success() {
        branch = String::from_utf8(output.stdout).unwrap();
        branch = branch.trim().to_string();
    }
    branch
}
