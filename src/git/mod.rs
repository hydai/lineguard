use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Get list of files changed between two commits
pub fn get_changed_files(from: &str, to: Option<&str>, repo_path: &Path) -> Result<Vec<PathBuf>> {
    // Check if we're in a git repository
    if !is_git_repository(repo_path)? {
        return Err(anyhow!("not a git repository"));
    }

    // Resolve git references to actual commit hashes
    let from_hash = resolve_commit_hash(from, repo_path)?;
    let to_hash = resolve_commit_hash(to.unwrap_or("HEAD"), repo_path)?;

    // Get list of changed files using resolved commit hashes
    let output = Command::new("git")
        .args(["diff", "--name-only", &from_hash, &to_hash])
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

/// Resolve any git reference (branch, tag, HEAD, commit hash) to a short commit hash
pub fn resolve_commit_hash(reference: &str, repo_path: &Path) -> Result<String> {
    let output = Command::new("git")
        .args(["rev-list", "-n", "1", "--abbrev-commit", reference])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "Invalid git reference: {}: {}",
            reference,
            stderr.trim()
        ));
    }

    let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if hash.is_empty() {
        return Err(anyhow!(
            "Could not resolve git reference '{}' to a commit hash",
            reference
        ));
    }
    Ok(hash)
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

        // Disable GPG signing for tests
        Command::new("git")
            .args(["config", "commit.gpgsign", "false"])
            .current_dir(dir.path())
            .output()?;

        Ok(())
    }

    /// Helper function to create a test commit with specified file and content
    fn create_test_commit(
        temp_dir: &TempDir,
        filename: &str,
        content: &str,
        message: &str,
    ) -> String {
        // Write file
        std::fs::write(temp_dir.path().join(filename), content).unwrap();

        // git add
        let add_output = Command::new("git")
            .args(["add", filename])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();
        assert!(
            add_output.status.success(),
            "git add failed: {}",
            String::from_utf8_lossy(&add_output.stderr)
        );

        // git commit
        let commit_output = Command::new("git")
            .args(["commit", "-m", message])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();
        assert!(
            commit_output.status.success(),
            "git commit failed: {}",
            String::from_utf8_lossy(&commit_output.stderr)
        );

        // Return commit hash
        let output = Command::new("git")
            .args(["rev-list", "-n", "1", "--abbrev-commit", "HEAD"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    /// Helper function to create a basic initial commit
    fn create_initial_commit(temp_dir: &TempDir) -> String {
        create_test_commit(temp_dir, "test.txt", "test content", "Initial commit")
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
    fn test_resolve_commit_hash_invalid_reference() {
        let temp_dir = TempDir::new().unwrap();
        init_test_repo(&temp_dir).unwrap();

        // Invalid reference should fail
        let result = resolve_commit_hash("invalid-ref", temp_dir.path());
        assert!(result.is_err());

        // Check that the error message is appropriate
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid git reference"));
    }

    #[test]
    fn test_resolve_commit_hash_with_commit() {
        let temp_dir = TempDir::new().unwrap();
        init_test_repo(&temp_dir).unwrap();

        // Create initial commit using helper function
        let expected_hash = create_initial_commit(&temp_dir);

        // Test resolving HEAD
        let resolved = resolve_commit_hash("HEAD", temp_dir.path()).unwrap();
        assert_eq!(resolved, expected_hash);

        // Test resolving full commit hash - get full hash first
        let full_output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to get full commit hash");
        let full_hash = String::from_utf8_lossy(&full_output.stdout)
            .trim()
            .to_string();

        let resolved_full = resolve_commit_hash(&full_hash, temp_dir.path()).unwrap();
        assert_eq!(resolved_full, expected_hash);

        // Test resolving short commit hash
        let resolved_short = resolve_commit_hash(&expected_hash, temp_dir.path()).unwrap();
        assert_eq!(resolved_short, expected_hash);
    }

    #[test]
    fn test_resolve_commit_hash_with_branch() {
        let temp_dir = TempDir::new().unwrap();
        init_test_repo(&temp_dir).unwrap();

        // Create initial commit and get its hash
        let expected_hash = create_initial_commit(&temp_dir);

        // Create a new branch, which will point to the initial commit
        Command::new("git")
            .args(["checkout", "-b", "feature-branch"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        // Test resolving branch name
        let resolved = resolve_commit_hash("feature-branch", temp_dir.path()).unwrap();
        assert_eq!(resolved, expected_hash);
    }

    #[test]
    fn test_resolve_commit_hash_with_tag() {
        let temp_dir = TempDir::new().unwrap();
        init_test_repo(&temp_dir).unwrap();

        // Create initial commit and get its hash
        let expected_hash = create_initial_commit(&temp_dir);

        // Create a tag pointing to the initial commit
        let tag_output = Command::new("git")
            .args(["tag", "v1.0.0"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();
        assert!(
            tag_output.status.success(),
            "git tag failed: {}",
            String::from_utf8_lossy(&tag_output.stderr)
        );

        // Test resolving tag name
        let resolved = resolve_commit_hash("v1.0.0", temp_dir.path()).unwrap();
        assert_eq!(resolved, expected_hash);
    }

    #[test]
    fn test_resolve_commit_hash_with_relative_reference() {
        let temp_dir = TempDir::new().unwrap();
        init_test_repo(&temp_dir).unwrap();

        // Create first commit using helper function
        let expected_hash =
            create_test_commit(&temp_dir, "test1.txt", "test content 1", "First commit");

        // Create second commit using helper function
        create_test_commit(&temp_dir, "test2.txt", "test content 2", "Second commit");

        // Test resolving HEAD~1
        let resolved = resolve_commit_hash("HEAD~1", temp_dir.path()).unwrap();
        assert_eq!(resolved, expected_hash);
    }

    #[test]
    fn test_get_changed_files_with_branch_names() {
        let temp_dir = TempDir::new().unwrap();
        init_test_repo(&temp_dir).unwrap();

        // Create initial commit on default branch using helper function
        create_test_commit(&temp_dir, "file1.txt", "content 1", "Initial commit");

        // Get the current branch name using rev-parse
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to get current branch name");
        let default_branch = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // If the branch name is empty or "HEAD", use "master" as fallback
        let default_branch = if default_branch.is_empty() || default_branch == "HEAD" {
            "master".to_string()
        } else {
            default_branch
        };

        // Create and switch to feature branch
        Command::new("git")
            .args(["checkout", "-b", "feature"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        // Add a file in feature branch using helper function
        create_test_commit(&temp_dir, "feature.txt", "feature content", "Add feature");

        // Test get_changed_files with branch names
        let changed_files =
            get_changed_files(&default_branch, Some("feature"), temp_dir.path()).unwrap();

        // Should find the feature.txt file
        assert_eq!(changed_files.len(), 1);
        assert!(changed_files[0].file_name().unwrap() == "feature.txt");
    }

    #[test]
    fn test_get_changed_files_with_tag() {
        let temp_dir = TempDir::new().unwrap();
        init_test_repo(&temp_dir).unwrap();

        // Create initial commit using helper function
        create_test_commit(&temp_dir, "file1.txt", "content 1", "Initial commit");

        // Create a tag
        Command::new("git")
            .args(["tag", "v1.0.0"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        // Add another file using helper function
        create_test_commit(&temp_dir, "file2.txt", "content 2", "Second commit");

        // Test get_changed_files with tag
        let changed_files = get_changed_files("v1.0.0", Some("HEAD"), temp_dir.path()).unwrap();

        // Should find the file2.txt file
        assert_eq!(changed_files.len(), 1);
        assert!(changed_files[0].file_name().unwrap() == "file2.txt");
    }

    #[test]
    fn test_get_changed_files_with_relative_reference() {
        let temp_dir = TempDir::new().unwrap();
        init_test_repo(&temp_dir).unwrap();

        // Create first commit using helper function
        create_test_commit(&temp_dir, "file1.txt", "content 1", "First commit");

        // Create second commit using helper function
        create_test_commit(&temp_dir, "file2.txt", "content 2", "Second commit");

        // Test get_changed_files with HEAD~1
        let changed_files = get_changed_files("HEAD~1", Some("HEAD"), temp_dir.path()).unwrap();

        // Should find the file2.txt file
        assert_eq!(changed_files.len(), 1);
        assert!(changed_files[0].file_name().unwrap() == "file2.txt");
    }
}
