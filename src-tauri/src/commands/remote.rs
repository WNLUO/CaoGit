//! Remote commands
//!
//! Commands for remote repository operations.

use tauri::Window;
use crate::git_ops::{GitRepository, RemoteInfo};
use super::response::ApiResponse;

/// Fetch from a remote
#[tauri::command]
pub fn fetch_remote(window: Window, repo_path: String, remote_name: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.fetch_with_progress(&remote_name, window) {
            Ok(_) => ApiResponse::success("Fetch completed".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Pull from a remote
#[tauri::command]
pub fn pull_remote(window: Window, repo_path: String, remote_name: String, branch_name: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.pull_with_progress(&remote_name, &branch_name, window) {
            Ok(_) => ApiResponse::success("Pull completed".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Push to a remote
#[tauri::command]
pub fn push_remote(window: Window, repo_path: String, remote_name: String, branch_name: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.push_with_progress(&remote_name, &branch_name, window) {
            Ok(_) => ApiResponse::success("Push completed".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get all remotes
#[tauri::command]
pub fn get_remotes(repo_path: String) -> ApiResponse<Vec<RemoteInfo>> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.get_remotes() {
            Ok(remotes) => ApiResponse::success(remotes),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Add a new remote
#[tauri::command]
pub fn add_remote(repo_path: String, name: String, url: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.add_remote(&name, &url) {
            Ok(_) => ApiResponse::success("Remote added".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Remove a remote
#[tauri::command]
pub fn remove_remote(repo_path: String, name: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.remove_remote(&name) {
            Ok(_) => ApiResponse::success("Remote removed".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}
