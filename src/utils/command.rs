use std::process::Command;

macro_rules! unwrap_to_bool {
    ($e: expr) => {
        match $e {
            Ok(x) => return true,
            Err(x) => {
                println!("Failed to execute command: {}", x);
                return false;
            }
        }
    };
}

pub fn exec(cmd: &str) {
    // TODO
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", cmd]).output()
    } else {
        Command::new("sh").arg("-c").arg(cmd).output()
    };
}
