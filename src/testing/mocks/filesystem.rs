//! Mock implementation of filesystem operations
//!
//! This module provides a mock filesystem for testing purposes,
//! allowing tests to simulate file operations without touching the real filesystem.

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

// Import the FileReader trait from checker module
use crate::checker::{FileMetadata, FileReader};

/// Re-export MockMetadata as alias for FileMetadata for backward compatibility
pub type MockMetadata = FileMetadata;

/// Helper functions for creating metadata
pub fn file_metadata(len: u64) -> FileMetadata {
    FileMetadata {
        len,
        is_file: true,
        is_dir: false,
    }
}

pub fn dir_metadata() -> FileMetadata {
    FileMetadata {
        len: 0,
        is_file: false,
        is_dir: true,
    }
}

/// Mock filesystem implementation
pub struct MockFileSystem {
    files: HashMap<PathBuf, String>,
    metadata: HashMap<PathBuf, FileMetadata>,
    errors: HashMap<PathBuf, io::Error>,
}

impl MockFileSystem {
    /// Create a new mock filesystem
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            metadata: HashMap::new(),
            errors: HashMap::new(),
        }
    }

    /// Add a file to the mock filesystem
    pub fn add_file<P: Into<PathBuf>>(&mut self, path: P, content: impl Into<String>) {
        let path = path.into();
        let content = content.into();
        let len = content.len() as u64;

        self.files.insert(path.clone(), content);
        self.metadata.insert(path, file_metadata(len));
    }

    /// Add a directory to the mock filesystem
    pub fn add_dir<P: Into<PathBuf>>(&mut self, path: P) {
        let path = path.into();
        self.metadata.insert(path, dir_metadata());
    }

    /// Add an error for a specific path
    pub fn add_error<P: Into<PathBuf>>(&mut self, path: P, error: io::Error) {
        self.errors.insert(path.into(), error);
    }

    /// Set specific metadata for a path
    pub fn set_metadata<P: Into<PathBuf>>(&mut self, path: P, metadata: FileMetadata) {
        self.metadata.insert(path.into(), metadata);
    }
}

impl Default for MockFileSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for MockFileSystem {
    fn clone(&self) -> Self {
        Self {
            files: self.files.clone(),
            metadata: self.metadata.clone(),
            // Re-create errors since io::Error doesn't implement Clone
            errors: self
                .errors
                .iter()
                .map(|(k, e)| (k.clone(), io::Error::new(e.kind(), e.to_string())))
                .collect(),
        }
    }
}

impl FileReader for MockFileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        // Check for configured errors first
        if let Some(error) = self.errors.get(path) {
            return Err(io::Error::new(error.kind(), error.to_string()));
        }

        // Return the file content if it exists
        self.files.get(path).cloned().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, format!("File not found: {path:?}"))
        })
    }

    fn open(&self, path: &Path) -> io::Result<Box<dyn io::Read>> {
        // Check for configured errors first
        if let Some(error) = self.errors.get(path) {
            return Err(io::Error::new(error.kind(), error.to_string()));
        }

        // Return a cursor over the file content
        let content = self.files.get(path).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, format!("File not found: {path:?}"))
        })?;

        Ok(Box::new(io::Cursor::new(content.clone())))
    }

    fn metadata(&self, path: &Path) -> io::Result<FileMetadata> {
        // Check for configured errors first
        if let Some(error) = self.errors.get(path) {
            return Err(io::Error::new(error.kind(), error.to_string()));
        }

        // Return metadata if it exists
        self.metadata.get(path).cloned().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, format!("File not found: {path:?}"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_mock_filesystem_read_file() {
        let mut fs = MockFileSystem::new();
        fs.add_file("test.txt", "content");
        assert_eq!(fs.read_to_string(Path::new("test.txt")).unwrap(), "content");
    }

    #[test]
    fn test_mock_filesystem_file_not_found() {
        let fs = MockFileSystem::new();
        let result = fs.read_to_string(Path::new("nonexistent.txt"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_mock_filesystem_with_error() {
        let mut fs = MockFileSystem::new();
        fs.add_file("test.txt", "content");
        fs.add_error(
            "test.txt",
            io::Error::new(io::ErrorKind::PermissionDenied, "Access denied"),
        );

        let result = fs.read_to_string(Path::new("test.txt"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::PermissionDenied);
    }

    #[test]
    fn test_mock_filesystem_metadata() {
        let mut fs = MockFileSystem::new();
        fs.add_file("test.txt", "hello world");

        let metadata = fs.metadata(Path::new("test.txt")).unwrap();
        assert_eq!(metadata.len, 11);
        assert!(metadata.is_file);
        assert!(!metadata.is_dir);
    }

    #[test]
    fn test_mock_filesystem_directory() {
        let mut fs = MockFileSystem::new();
        fs.add_dir("test_dir");

        let metadata = fs.metadata(Path::new("test_dir")).unwrap();
        assert_eq!(metadata.len, 0);
        assert!(!metadata.is_file);
        assert!(metadata.is_dir);
    }

    #[test]
    fn test_mock_filesystem_open_file() {
        let mut fs = MockFileSystem::new();
        fs.add_file("test.txt", "hello world");

        let mut reader = fs.open(Path::new("test.txt")).unwrap();
        let mut content = String::new();
        reader.read_to_string(&mut content).unwrap();
        assert_eq!(content, "hello world");
    }

    #[test]
    fn test_mock_filesystem_custom_metadata() {
        let mut fs = MockFileSystem::new();
        fs.add_file("test.txt", "content");

        // Override with custom metadata
        let custom_metadata = file_metadata(1024);
        fs.set_metadata("test.txt", custom_metadata);

        let metadata = fs.metadata(Path::new("test.txt")).unwrap();
        assert_eq!(metadata.len, 1024);
    }
}
