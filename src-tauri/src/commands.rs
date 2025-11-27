use crate::git_ops::{GitRepository, FileChange, CommitInfo, BranchInfo, RemoteInfo, StashInfo, TagInfo, DiffResult, BlameLine, ConflictInfo};
use serde::Serialize;
use tauri::{Window, Theme};

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

#[tauri::command]
pub fn open_repository(path: String) -> ApiResponse<String> {
    match GitRepository::open(&path) {
        Ok(_) => ApiResponse::success("Repository opened successfully".to_string()),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

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

// Remote operations
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

// Merge operations
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

// Stash operations
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

// Tag operations
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

// Diff operations
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

// Clone operation
#[tauri::command]
pub fn clone_repository(url: String, path: String) -> ApiResponse<String> {
    match GitRepository::clone(&url, &path) {
        Ok(_) => ApiResponse::success("Repository cloned successfully".to_string()),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

// Init operation
#[tauri::command]
pub fn init_repository(path: String, default_branch: Option<String>) -> ApiResponse<String> {
    match GitRepository::init(&path) {
        Ok(_repo) => {
            // If a default branch name is specified, create and checkout that branch
            if let Some(branch_name) = default_branch {
                if branch_name != "master" && branch_name != "main" {
                    // Create an initial commit first (required for branch creation)
                    // We'll create an empty commit or let the user make the first commit
                    // For now, we'll just note that the branch will be created on first commit
                }
            }
            ApiResponse::success("Repository initialized successfully".to_string())
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

// Detect project type based on files in directory
#[tauri::command]
pub fn detect_project_type(path: String) -> ApiResponse<String> {
    use std::fs;
    use std::path::Path;

    let dir_path = Path::new(&path);

    if !dir_path.exists() || !dir_path.is_dir() {
        return ApiResponse::error("Path does not exist or is not a directory".to_string());
    }

    // Read directory contents
    let entries = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(e) => return ApiResponse::error(format!("Failed to read directory: {}", e)),
    };

    let mut files = Vec::new();
    for entry in entries.flatten() {
        if let Ok(file_name) = entry.file_name().into_string() {
            files.push(file_name);
        }
    }

    // Detect project type based on specific files
    let project_type = detect_type_from_files(&files);

    ApiResponse::success(project_type)
}

fn detect_type_from_files(files: &[String]) -> String {
    // Node.js / JavaScript
    if files.contains(&"package.json".to_string())
        || files.contains(&"yarn.lock".to_string())
        || files.contains(&"pnpm-lock.yaml".to_string())
        || files.contains(&"node_modules".to_string()) {
        return "node".to_string();
    }

    // Python
    if files.contains(&"requirements.txt".to_string())
        || files.contains(&"setup.py".to_string())
        || files.contains(&"pyproject.toml".to_string())
        || files.contains(&"Pipfile".to_string())
        || files.contains(&"poetry.lock".to_string())
        || files.iter().any(|f| f.ends_with(".py")) {
        return "python".to_string();
    }

    // Java
    if files.contains(&"pom.xml".to_string())
        || files.contains(&"build.gradle".to_string())
        || files.contains(&"build.gradle.kts".to_string())
        || files.contains(&"gradlew".to_string())
        || files.iter().any(|f| f.ends_with(".java")) {
        return "java".to_string();
    }

    // Go
    if files.contains(&"go.mod".to_string())
        || files.contains(&"go.sum".to_string())
        || files.iter().any(|f| f.ends_with(".go")) {
        return "go".to_string();
    }

    // Rust
    if files.contains(&"Cargo.toml".to_string())
        || files.contains(&"Cargo.lock".to_string()) {
        return "rust".to_string();
    }

    // C++
    if files.contains(&"CMakeLists.txt".to_string())
        || files.contains(&"Makefile".to_string())
        || files.iter().any(|f| f.ends_with(".cpp") || f.ends_with(".hpp") || f.ends_with(".cc") || f.ends_with(".h")) {
        return "cpp".to_string();
    }

    // C#
    if files.iter().any(|f| f.ends_with(".csproj") || f.ends_with(".sln"))
        || files.iter().any(|f| f.ends_with(".cs")) {
        return "csharp".to_string();
    }

    // Ruby
    if files.contains(&"Gemfile".to_string())
        || files.contains(&"Rakefile".to_string())
        || files.iter().any(|f| f.ends_with(".rb")) {
        return "ruby".to_string();
    }

    // PHP
    if files.contains(&"composer.json".to_string())
        || files.contains(&"composer.lock".to_string())
        || files.iter().any(|f| f.ends_with(".php")) {
        return "php".to_string();
    }

    // Swift
    if files.contains(&"Package.swift".to_string())
        || files.iter().any(|f| f.ends_with(".swift"))
        || files.iter().any(|f| f.ends_with(".xcodeproj") || f.ends_with(".xcworkspace")) {
        return "swift".to_string();
    }

    // Kotlin
    if files.contains(&"build.gradle.kts".to_string())
        || files.iter().any(|f| f.ends_with(".kt") || f.ends_with(".kts")) {
        return "kotlin".to_string();
    }

    "none".to_string()
}

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

// Window theme operations
#[tauri::command]
pub fn set_window_theme(window: Window, theme: String) -> ApiResponse<String> {
    let tauri_theme = match theme.as_str() {
        "dark" => Some(Theme::Dark),
        "light" => Some(Theme::Light),
        _ => None, // Auto/system theme
    };

    match window.set_theme(tauri_theme) {
        Ok(_) => ApiResponse::success(format!("Window theme set to {}", theme)),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

// AI API call
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct AIRequest {
    pub endpoint: String,
    pub api_key: String,
    pub model: String,
    pub messages: Vec<AIMessage>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AIResponseData {
    pub choices: Vec<AIChoice>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AIChoice {
    pub message: AIMessageResponse,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AIMessageResponse {
    pub content: String,
}

#[tauri::command]
pub async fn call_ai_api(
    endpoint: String,
    api_key: String,
    model: String,
    messages: Vec<AIMessage>,
    temperature: f32,
    max_tokens: u32
) -> ApiResponse<String> {
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "model": model,
        "messages": messages,
        "temperature": temperature,
        "max_tokens": max_tokens
    });

    match client
        .post(&endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<AIResponseData>().await {
                    Ok(data) => {
                        if let Some(choice) = data.choices.first() {
                            ApiResponse::success(choice.message.content.clone())
                        } else {
                            ApiResponse::error("AI returned no choices".to_string())
                        }
                    }
                    Err(e) => ApiResponse::error(format!("Failed to parse AI response: {}", e)),
                }
            } else {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                ApiResponse::error(format!("HTTP {}: {}", status, error_text))
            }
        }
        Err(e) => ApiResponse::error(format!("Failed to call AI API: {}", e)),
    }
}

// Clipboard operations
#[tauri::command]
pub fn copy_to_clipboard(text: String) -> ApiResponse<String> {
    use copypasta::ClipboardProvider;

    let mut clipboard = match copypasta::ClipboardContext::new() {
        Ok(clipboard) => clipboard,
        Err(e) => return ApiResponse::error(format!("Failed to access clipboard: {}", e)),
    };

    match clipboard.set_contents(text.clone()) {
        Ok(_) => ApiResponse::success(format!("Copied to clipboard: {}", text)),
        Err(e) => ApiResponse::error(format!("Failed to copy to clipboard: {}", e)),
    }
}
