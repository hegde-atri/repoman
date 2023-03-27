use std::{
    io,
    io::Error,
    io::Write,
    process::{Command, Output},
};

pub fn exec(cmd: &str) -> Result<Output, Error> {
    if cfg!(target_os = "windows") {
        return Command::new("cmd").args(["/C", cmd]).output();
    } else {
        return Command::new("sh").arg("-c").arg(cmd).output();
    };
}
