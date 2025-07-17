//! Testing utilities for LineLint
//!
//! This module provides mock implementations, test builders, and fixtures
//! to support comprehensive unit testing across the codebase.

pub mod builders;
pub mod fixtures;
pub mod mocks;

// Re-export commonly used testing utilities
pub use builders::*;
pub use mocks::*;
