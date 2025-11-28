//! Git operations module
//!
//! This module provides a high-level API for Git operations using git2-rs.
//! It's organized into submodules for better maintainability.

mod types;
mod repository;
mod branch;
mod remote;
mod diff;
mod stash;
mod tag;
mod merge;
mod blame;

// Re-export all public types and structs
pub use types::*;
pub use repository::GitRepository;
