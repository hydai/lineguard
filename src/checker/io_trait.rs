//! I/O traits for dependency injection
//!
//! This module defines traits that abstract file system operations,
//! allowing for better testability through dependency injection.

use std::io;
use std::path::Path;

/// Metadata about a file
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub len: u64,
    pub is_file: bool,
    pub is_dir: bool,
}

/// Trait for reading files and getting metadata
pub trait FileReader: Send + Sync {
    /// Read the entire contents of a file into a string
    fn read_to_string(&self, path: &Path) -> io::Result<String>;

    /// Open a file for reading
    fn open(&self, path: &Path) -> io::Result<Box<dyn io::Read>>;

    /// Get metadata about a file
    fn metadata(&self, path: &Path) -> io::Result<FileMetadata>;
}
