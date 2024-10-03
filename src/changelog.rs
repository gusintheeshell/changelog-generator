use chrono::Utc;

pub struct Commit {
  pub hash: String,
  pub message: String,
  pub author: String,
}

pub fn generate_changelog(commits: Vec<Commit>) -> String {
  let mut changelog = String::new();
  
  for commit in commits {
    if commit.message.starts_with("feat:") {
      changelog.push_str(&format!("## New Feature\n- {}\n", commit.message));
    } else if commit.message.starts_with("fix:") {
      changelog.push_str(&format!("## Bug Fixes\n- {}\n", commit.message));
    }
  }
  
  changelog
}

pub fn generate_markdown_changelog(commits: Vec<Commit>) -> String {
  let mut changelog = String::new();

  let current_date = Utc::now();
  changelog.push_str(&format!("# Changelog - {}\n\n", current_date.format("%Y-%m-%d")));

  let mut features = Vec::new();
  let mut fixes = Vec::new();
  let mut others = Vec::new();

  for commit in commits {
    if commit.message.starts_with("feat:") {
      features.push(commit);
    } else if commit.message.starts_with("fix:") {
      fixes.push(commit);
    } else {
      others.push(commit);
    }
  }

  if !features.is_empty() {
    changelog.push_str("## Features\n\n");
    for feat in features {
      changelog.push_str(&format!("- {} by {} ({})\n", feat.message, feat.author, feat.hash));
    }
    changelog.push_str("\n");
  }

  if !fixes.is_empty() {
    changelog.push_str("## Bug Fixes\n\n");
    for fix in fixes {
      changelog.push_str(&format!("- {} by {} ({})\n", fix.message, fix.author, fix.hash));
    }
    changelog.push_str("\n");
  }

  if !others.is_empty() {
    changelog.push_str("## Other Changes\n\n");
    for other in others {
      changelog.push_str(&format!("- {} by {} ({})\n", other.message, other.author, other.hash));
    }
    changelog.push_str("\n");
  }

  changelog
}
    
