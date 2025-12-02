//! Data types for Git operations
//!
//! This module contains all the data structures used across Git operations.

use serde::{Deserialize, Serialize};

/// Represents a file change in the working directory or staging area
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub status: String,
    pub staged: bool,
}

/// Information about a commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub date: String,
    pub parents: Vec<String>,
}

/// Information about a branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub is_head: bool,
    pub is_remote: bool,
    pub upstream: Option<String>,
    pub last_commit: Option<String>,
}

/// Information about a remote repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteInfo {
    pub name: String,
    pub url: String,
}

/// Information about a tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagInfo {
    pub name: String,
    pub target: String,
    pub message: Option<String>,
}

/// Information about a stash entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StashInfo {
    pub index: usize,
    pub message: String,
}

/// Result of a diff operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    pub old_path: String,
    pub new_path: String,
    pub status: String,
    pub hunks: Vec<DiffHunk>,
}

/// A hunk in a diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub header: String,
    pub lines: Vec<DiffLine>,
}

/// A line in a diff hunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub origin: char,
    pub content: String,
    pub old_lineno: Option<u32>,
    pub new_lineno: Option<u32>,
}

/// Information about a blame line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlameLine {
    pub line_number: u32,
    pub commit_hash: String,
    pub author: String,
    pub author_email: String,
    pub date: String,
    pub content: String,
}

/// Information about a merge conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub path: String,
    pub ours: String,
    pub theirs: String,
    pub base: Option<String>,
}

/// Progress information for Git network operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitProgress {
    pub operation_type: String, // "upload" or "download"
    pub total_objects: usize,
    pub received_objects: usize,
    pub total_bytes: u64,
    pub received_bytes: u64,
    pub speed_bytes_per_sec: u64,
}

/// Authentication configuration for Git operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    #[serde(rename = "authType")]
    pub auth_type: String,  // "none", "token", "password"
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// Sync status between local and remote branches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub ahead: usize,   // 本地领先远程的提交数
    pub behind: usize,  // 本地落后远程的提交数
}
