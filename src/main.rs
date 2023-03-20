#![allow(unused)]

mod args;
mod utils;

use args::RepoArgs;
use clap::Parser;

fn main() {
    let args = RepoArgs::parse();
    println!("{:?}", args);
}
