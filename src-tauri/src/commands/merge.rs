//! Merge commands
//!
//! Commands for merge, cherry-pick, and conflict resolution operations.

use crate::git_ops::{GitRepository, ConflictInfo};
use super::response::ApiResponse;

/// Merge a branch
#[tauri::command]
pub fn merge_branch(repo_path: String, branch_name: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.merge(&branch_name) {
            Ok(msg) => ApiResponse::success(msg),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Cherry-pick a single commit
#[tauri::command]
pub fn cherry_pick(repo_path: String, commit_hash: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.cherry_pick(&commit_hash) {
            Ok(msg) => ApiResponse::success(msg),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Cherry-pick multiple commits
#[tauri::command]
pub fn cherry_pick_batch(repo_path: String, commit_hashes: Vec<String>) -> ApiResponse<Vec<String>> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.cherry_pick_batch(commit_hashes) {
            Ok(results) => ApiResponse::success(results),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get merge conflicts
#[tauri::command]
pub fn get_conflicts(repo_path: String) -> ApiResponse<Vec<ConflictInfo>> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.get_conflicts() {
            Ok(conflicts) => ApiResponse::success(conflicts),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Resolve a merge conflict
#[tauri::command]
pub fn resolve_conflict(repo_path: String, file_path: String, resolution: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.resolve_conflict(&file_path, &resolution) {
            Ok(_) => ApiResponse::success("Conflict resolved successfully".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Abort the current merge
#[tauri::command]
pub fn abort_merge(repo_path: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.abort_merge() {
            Ok(_) => ApiResponse::success("Merge aborted successfully".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}
