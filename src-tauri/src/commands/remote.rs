//! Remote commands
//!
//! Commands for remote repository operations.

use tauri::Window;
use crate::git_ops::{GitRepository, RemoteInfo};
use super::response::ApiResponse;

/// Fetch from a remote (异步执行，不阻塞主线程)
#[tauri::command]
pub async fn fetch_remote(window: Window, repo_path: String, remote_name: String) -> ApiResponse<String> {
    let handle = tokio::task::spawn(async move {
        match GitRepository::open(&repo_path) {
            Ok(repo) => match repo.fetch_with_progress(&remote_name, window) {
                Ok(_) => ApiResponse::success("Fetch completed".to_string()),
                Err(e) => ApiResponse::error(e.to_string()),
            },
            Err(e) => ApiResponse::error(e.to_string()),
        }
    });

    match handle.await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Task execution failed: {}", e)),
    }
}

/// Pull from a remote (异步执行，不阻塞主线程)
#[tauri::command]
pub async fn pull_remote(window: Window, repo_path: String, remote_name: String, branch_name: String) -> ApiResponse<String> {
    let handle = tokio::task::spawn(async move {
        match GitRepository::open(&repo_path) {
            Ok(repo) => match repo.pull_with_progress(&remote_name, &branch_name, window) {
                Ok(_) => ApiResponse::success("Pull completed".to_string()),
                Err(e) => ApiResponse::error(e.to_string()),
            },
            Err(e) => ApiResponse::error(e.to_string()),
        }
    });

    match handle.await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Task execution failed: {}", e)),
    }
}

/// Push to a remote (异步执行，不阻塞主线程)
#[tauri::command]
pub async fn push_remote(window: Window, repo_path: String, remote_name: String, branch_name: String) -> ApiResponse<String> {
    // 使用 tokio::task::spawn 而不是 spawn_blocking
    // 这样可以保持 Tauri 的 async 上下文，Window 可以正常发送事件
    let handle = tokio::task::spawn(async move {
        // 在 tokio runtime 中运行，但 Git 操作本身仍会阻塞这个 task
        // 不过因为是在独立的 task 中，不会阻塞主线程
        match GitRepository::open(&repo_path) {
            Ok(repo) => match repo.push_with_progress(&remote_name, &branch_name, window) {
                Ok(_) => ApiResponse::success("Push completed".to_string()),
                Err(e) => ApiResponse::error(e.to_string()),
            },
            Err(e) => ApiResponse::error(e.to_string()),
        }
    });

    match handle.await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Task execution failed: {}", e)),
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
