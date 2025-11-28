//! Tag commands
//!
//! Commands for tag operations.

use crate::git_ops::{GitRepository, TagInfo};
use super::response::ApiResponse;

/// Create a new tag
#[tauri::command]
pub fn create_tag(repo_path: String, tag_name: String, message: Option<String>) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.create_tag(&tag_name, message.as_deref()) {
            Ok(_) => ApiResponse::success("Tag created".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get all tags
#[tauri::command]
pub fn get_tags(repo_path: String) -> ApiResponse<Vec<TagInfo>> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.get_tags() {
            Ok(tags) => ApiResponse::success(tags),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Delete a tag
#[tauri::command]
pub fn delete_tag(repo_path: String, tag_name: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.delete_tag(&tag_name) {
            Ok(_) => ApiResponse::success("Tag deleted".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}
