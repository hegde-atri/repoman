use std::{
    io::{Error, ErrorKind},
    path::Path,
    process::{Command, Output},
};

use which::which;

/// Returns output to give command
///
/// It takes in the command/binary to execute, optional flags and the path
/// to execute it in.
///
/// It will only execute if the provided `cmd` is found on host and `cwd` exists
/// as a directory.
///
/// # Arguments
///
/// * `cmd` - The binary to execute.
/// * `args` - Optional to include arguments.
/// * `cwd` - The directory to execute in.
///
/// # Examples
/// ```
/// exec("git", Some("status"), Path::new("/home/user/repo"))
/// ```
pub fn exec(cmd: &str, args: Option<&str>, cwd: &Path) -> Result<Output, Error> {
    // Check if binary for command exists
    match which(cmd) {
        Ok(_) => (),
        Err(_) => {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Could not find specified command: {}", cmd),
            ))
        }
    }
    // Check if path exists
    if !cwd.exists() {
        return Err(Error::new(
            ErrorKind::Other,
            "Specified directory is invalid!",
        ));
    }
    // Now execute the command
    if cfg!(target_os = "windows") {
        return Command::new("cmd")
            .current_dir(&cwd.as_os_str())
            .args(["/C", cmd])
            .arg(args.unwrap_or(""))
            .output();
    } else {
        return Command::new("sh")
            .current_dir(&cwd.as_os_str())
            .arg("-c")
            .arg(cmd)
            .arg(args.unwrap_or(""))
            .output();
    };
}
