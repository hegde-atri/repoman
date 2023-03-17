use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The directory to search for git repositories (optional)
    dir: Option<String>,
}
