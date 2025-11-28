//! Tauri commands module
//!
//! This module provides all Tauri command handlers organized by functionality.

mod response;
mod repository;
mod branch;
mod remote;
mod staging;
mod diff;
mod stash;
mod tag;
mod merge;
mod utils;
mod ai;

// Re-export all commands
pub use repository::*;
pub use branch::*;
pub use remote::*;
pub use staging::*;
pub use diff::*;
pub use stash::*;
pub use tag::*;
pub use merge::*;
pub use utils::*;
pub use ai::*;
