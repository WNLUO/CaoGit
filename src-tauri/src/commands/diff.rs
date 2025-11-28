//! Diff commands
//!
//! Commands for diff and blame operations.

use crate::git_ops::{GitRepository, DiffResult, BlameLine};
use super::response::ApiResponse;

/// Get file diff
#[tauri::command]
pub fn get_file_diff(repo_path: String, file_path: String, staged: bool) -> ApiResponse<DiffResult> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.get_file_diff(&file_path, staged) {
            Ok(diff) => ApiResponse::success(diff),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get file blame
#[tauri::command]
pub fn get_file_blame(repo_path: String, file_path: String) -> ApiResponse<Vec<BlameLine>> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.blame_file(&file_path) {
            Ok(blame) => ApiResponse::success(blame),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}
