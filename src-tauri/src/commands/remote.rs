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
pub async fn push_remote(_window: Window, repo_path: String, remote_name: String, branch_name: String) -> ApiResponse<String> {
    eprintln!("🚀 push_remote called: repo={}, remote={}, branch={}", repo_path, remote_name, branch_name);

    // 暂时不使用进度报告，先确保基本功能能工作
    let result = tokio::task::spawn_blocking(move || {
        eprintln!("📦 Opening repository in blocking thread: {}", repo_path);
        match GitRepository::open(&repo_path) {
            Ok(repo) => {
                eprintln!("✅ Repository opened, starting push (without progress)...");
                // 使用不带进度的 push 方法
                match repo.push(&remote_name, &branch_name) {
                    Ok(_) => {
                        eprintln!("✅ Push completed successfully");
                        ApiResponse::success("Push completed".to_string())
                    },
                    Err(e) => {
                        eprintln!("❌ Push failed: {}", e);
                        ApiResponse::error(e.to_string())
                    },
                }
            },
            Err(e) => {
                eprintln!("❌ Failed to open repository: {}", e);
                ApiResponse::error(e.to_string())
            },
        }
    }).await;

    eprintln!("⏳ Push task completed");
    match result {
        Ok(response) => {
            eprintln!("🎉 Push task finished: success={}", response.success);
            response
        },
        Err(e) => {
            eprintln!("💥 Task execution failed: {}", e);
            ApiResponse::error(format!("Task execution failed: {}", e))
        },
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
