//! Staging commands
//!
//! Commands for staging area operations (status, stage, unstage, commit).

use crate::git_ops::{GitRepository, FileChange, CommitInfo};
use super::response::ApiResponse;

/// Get repository status
#[tauri::command]
pub fn get_repository_status(path: String) -> ApiResponse<Vec<FileChange>> {
    match GitRepository::open(&path) {
        Ok(repo) => match repo.get_status() {
            Ok(changes) => ApiResponse::success(changes),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Stage a file
#[tauri::command]
pub fn stage_file(repo_path: String, file_path: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.stage_file(&file_path) {
            Ok(_) => ApiResponse::success("File staged successfully".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Unstage a file
#[tauri::command]
pub fn unstage_file(repo_path: String, file_path: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.unstage_file(&file_path) {
            Ok(_) => ApiResponse::success("File unstaged successfully".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Discard changes to a file
#[tauri::command]
pub fn discard_file(repo_path: String, file_path: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.discard_file(&file_path) {
            Ok(_) => ApiResponse::success("File changes discarded successfully".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Commit changes
#[tauri::command]
pub fn commit_changes(repo_path: String, message: String) -> ApiResponse<String> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.commit(&message) {
            Ok(oid) => ApiResponse::success(oid),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get commit history
#[tauri::command]
pub fn get_commits(repo_path: String, max_count: usize) -> ApiResponse<Vec<CommitInfo>> {
    match GitRepository::open(&repo_path) {
        Ok(repo) => match repo.get_commits(max_count) {
            Ok(commits) => ApiResponse::success(commits),
            Err(e) => ApiResponse::error(e.to_string()),
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}
