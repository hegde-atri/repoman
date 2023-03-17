#![allow(unused)]

mod args;
mod utils;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let k = "hh";
    hello_world(k);
}

fn hello_world(hello: &str) {
    println!("{}", hello);
}
