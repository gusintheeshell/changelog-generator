use git2::{Repository, Error};
use crate::changelog::Commit;

pub fn get_commits(repo_path: &str) -> Result<Vec<Commit>, Error> {
  let repo = Repository::open(repo_path)?;
  let mut revwalk = repo.revwalk()?;
  revwalk.push_head()?;
  
  let mut commits = Vec::new();

  for id in revwalk {
    let commit = repo.find_commit(id?)?;
    let message = commit.message().unwrap_or("No commit message").to_string();
    let author = commit.author().name().unwrap_or("Unknown").to_string();
    let hash = commit.id().to_string();
    let commit_obj = Commit { message, author, hash };
    commits.push(commit_obj);
  }
  
  Ok(commits)
}