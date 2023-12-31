use std::{future::Future, path::PathBuf, pin::Pin};
use tokio::fs;

pub async fn find_git_repos(dir: PathBuf) -> Pin<Box<dyn Future<Output = Vec<PathBuf>>>> {
    Box::pin(async {
        let mut repos = Vec::new();

        if dir.join(".git").is_dir() {
            repos.push(dir.clone());
        }

        if let Ok(mut entries) = fs::read_dir(dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_dir() {
                    if path.join(".git").is_dir() {
                        repos.push(path);
                    } else {
                        let new_repos = find_git_repos(path).await;
                        repos.extend(new_repos.await);
                    }
                }
            }
        }
        repos
    })
}
