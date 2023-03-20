use clap::{Args, Parser, SubCommand, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct RepoArgs {
    /// The directory to search for git repositories (optional)
    /// Will return summarised git status for each repo.
    pub dir: Option<String>,
    /// Choose a repo to perform the action on, if not specified will
    /// perform action on all repos
    #[clap(subcommand)]
    pub repo: Option<Action>,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Runs git status
    Status,
    /// Runs git commit
    Commit(Message),
    /// Runs git push
    Push,
}

#[derive(Debug, Args)]
pub struct Message {
    pub message: String,
    pub description: Option<String>,
}
