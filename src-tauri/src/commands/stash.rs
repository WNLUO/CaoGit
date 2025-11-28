//! Stash commands
//!
//! Commands for stash operations.

use crate::git_ops::{GitRepository, StashInfo};
use super::response::ApiResponse;

/// Save changes to stash
#[tauri::command]
pub fn stash_save(repo_path: String, message: Option<String>) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(mut repo) => match repo.stash_save(message.as_deref()) {
            Ok(_) => ApiResponse::success("Stash saved".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// List stash entries
#[tauri::command]
pub fn stash_list(repo_path: String) -> ApiResponse<Vec<StashInfo>> {
    match GitRepository::open(&repo_path) {
        Ok(mut repo) => match repo.stash_list() {
            Ok(stashes) => ApiResponse::success(stashes),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Pop a stash entry
#[tauri::command]
pub fn stash_pop(repo_path: String, index: usize) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(mut repo) => match repo.stash_pop(index) {
            Ok(_) => ApiResponse::success("Stash popped".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Drop a stash entry
#[tauri::command]
pub fn stash_drop(repo_path: String, index: usize) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(mut repo) => match repo.stash_drop(index) {
            Ok(_) => ApiResponse::success("Stash dropped".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}
