use std::path::PathBuf;

use args::RepoArgs;
use clap::Parser;
use ha_utils::cmd::get_pwd;
use utils::{dirs, actions::status::git_status_oneline};

mod args;
mod utils;

#[tokio::main]
async fn main() {
    let args = RepoArgs::parse();

    println!("{:?}\n", args);


    let dir = if let Some(dir_string) = args.dir {
        PathBuf::from(dir_string)
    } else {
        get_pwd(None)
    };

    match args.action {
        Some(args::Action::Test) => {
            println!("Test");
        }
        None => default_action(dir).await
    }


}

async fn default_action(dir: PathBuf) {
    let repos = dirs::find_git_repos(dir).await.await;
    for repo in repos {
        git_status_oneline(Some(&repo));
    }
}
