use ansi_term::ANSIGenericString;
use ansi_term::Colour;
use ha_utils::cmd::exec;
use ha_utils::cmd::get_pwd;

use std::process::Command;
use std::{
    io::{self, Write},
    path::Path,
};

use crate::utils::actions::branch::get_branch;
use crate::utils::common::is_repo;

pub fn git_status_oneline(p: Option<&Path>) {
    let path = get_pwd(p);

    if is_repo(path.as_path()) {
        // print the status with path red and bold.
        print!(
            "{} ",
            Colour::Cyan.paint(get_branch(&path.as_path()))
        );
        for symbol in get_status_symbols(&path.as_path()) {
            print!("{}", symbol.to_string());
        }
        println!(
            " - {}",
            Colour::dimmed(Colour::White).paint(path.to_str().unwrap()) //ets
        );
        get_status_symbols(&path.as_path());
    } else {
        println!("{}", Colour::Red.bold().paint("Not a git directory"));
    }
}

fn get_status_symbols(path: &Path) -> Vec<String> {
    let mut cmd = Command::new("git");
    cmd.arg("status");
    cmd.arg("-s");
    cmd.current_dir(path);
    let output = cmd.output().expect("failed to execute process");
    let mut symbols = Vec::new();
    if output.status.success() {
        let output = String::from_utf8(output.stdout).unwrap();
        let lines = output.lines();
        for line in lines {
            // trim the line
            let line = line.trim();
            // get the first character
            let status = &line[0..1];
            // symbols.push(status.to_string());
            // get its symbol
             let symbol = get_symbol(status);
            // append to final
             symbols.push(symbol.to_string());
        }
    }
    return symbols;
}

fn get_symbol(status: &str) -> ANSIGenericString<str> {
    let status: ANSIGenericString<str> = match status {
        "M" => Colour::Red.bold().paint("!"),
        "A" => Colour::Red.bold().paint("A"),
        "D" => Colour::Red.bold().paint("D"),
        "R" => Colour::Red.bold().paint("R"),
        "C" => Colour::Red.bold().paint("C"),
        "U" => Colour::Red.bold().paint("U"),
        "?" => Colour::Red.bold().paint("?"),
        "!" => Colour::Red.bold().paint("!"),
        _ => Colour::White.bold().paint(""),
    };

    status
}

/// Prints git status either `pwd` or provided path
///
/// # Arguments
///
/// * `p` - An `Option<&Path>` to run the command `git status` in
pub fn git_status(p: Option<&Path>) {
    // decide the path
    let path = get_pwd(p);

    if is_repo(path.as_path()) {
        // print the status with path red and bold.
        println!("{}", Colour::Blue.bold().paint(path.to_str().unwrap()));

        match exec("git status", path.as_path()) {
            Ok(o) => io::stdout().write_all(&o.stdout).unwrap(),
            Err(err) => println!("Error: {}", err),
        };
    } else {
        println!("{}", Colour::Red.bold().paint("Not a git directory"));
    }
}
