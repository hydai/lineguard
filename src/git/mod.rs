use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Get list of files changed between two commits
pub fn get_changed_files(from: &str, to: Option<&str>, repo_path: &Path) -> Result<Vec<PathBuf>> {
    // Check if we're in a git repository
    if !is_git_repository(repo_path)? {
        return Err(anyhow!("not a git repository"));
    }

    // Validate commits exist
    validate_commit(from, repo_path)?;
    if let Some(to_commit) = to {
        validate_commit(to_commit, repo_path)?;
    }

    let to_commit = to.unwrap_or("HEAD");

    // Get list of changed files
    let output = Command::new("git")
        .args(["diff", "--name-only", from, to_commit])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to get changed files: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| repo_path.join(line))
        .filter(|path| path.exists() && path.is_file())
        .collect();

    Ok(files)
}

/// Check if the current directory is a git repository
pub fn is_git_repository(path: &Path) -> Result<bool> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(path)
        .output()?;

    Ok(output.status.success())
}

/// Validate that a commit exists
fn validate_commit(commit: &str, repo_path: &Path) -> Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--verify", commit])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Invalid commit: {}", commit));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn init_test_repo(dir: &TempDir) -> Result<()> {
        Command::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .output()?;

        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(dir.path())
            .output()?;

        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(dir.path())
            .output()?;

        Ok(())
    }

    #[test]
    fn test_is_git_repository() {
        let temp_dir = TempDir::new().unwrap();

        // Should return false for non-git directory
        assert!(!is_git_repository(temp_dir.path()).unwrap());

        // Initialize git repo
        init_test_repo(&temp_dir).unwrap();

        // Should return true for git directory
        assert!(is_git_repository(temp_dir.path()).unwrap());
    }

    #[test]
    fn test_validate_commit_invalid() {
        let temp_dir = TempDir::new().unwrap();
        init_test_repo(&temp_dir).unwrap();

        // Invalid commit should fail
        assert!(validate_commit("invalid-hash", temp_dir.path()).is_err());
    }
}
