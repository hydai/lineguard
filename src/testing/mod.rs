//! Testing utilities for LineGuard
//!
//! This module provides mock implementations and test builders
//! to support comprehensive unit testing across the codebase.

pub mod builders;
pub mod mocks;

// Re-export commonly used testing utilities
pub use builders::*;
pub use mocks::*;
