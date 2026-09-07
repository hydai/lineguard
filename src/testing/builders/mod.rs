//! Test builders for creating test data
//!
//! This module provides builder patterns for constructing test data
//! in a fluent and readable manner.

pub mod file_builder;

// Re-export commonly used builders
pub use file_builder::{LineEnding, TestFileBuilder};
