//! Mock implementations for testing
//!
//! This module contains mock implementations of various traits and interfaces
//! used throughout the LineLint codebase, enabling isolated unit testing.

pub mod filesystem;
pub mod output;

// Re-export commonly used mocks
pub use filesystem::{MockFileSystem, MockMetadata};
pub use output::MockOutput;

// Re-export reporter traits from the reporter module
pub use crate::reporter::{Color, ColoredOutput, Output};
