//! Branch commands
//!
//! Commands for branch management operations.

use crate::git_ops::{GitRepository, BranchInfo};
use super::response::ApiResponse;

/// Get all branches
#[tauri::command]
pub fn get_branches(repo_path: String) -> ApiResponse<Vec<BranchInfo>> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.get_branches() {
            Ok(branches) => ApiResponse::success(branches),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Create a new branch
#[tauri::command]
pub fn create_branch(repo_path: String, branch_name: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.create_branch(&branch_name) {
            Ok(_) => ApiResponse::success("Branch created successfully".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Checkout a branch
#[tauri::command]
pub fn checkout_branch(repo_path: String, branch_name: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.checkout_branch(&branch_name) {
            Ok(_) => ApiResponse::success("Branch checked out successfully".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Delete a branch
#[tauri::command]
pub fn delete_branch(repo_path: String, branch_name: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.delete_branch(&branch_name) {
            Ok(_) => ApiResponse::success("Branch deleted successfully".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get the current branch name
#[tauri::command]
pub fn get_current_branch(repo_path: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.get_current_branch() {
            Ok(branch) => ApiResponse::success(branch),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}
