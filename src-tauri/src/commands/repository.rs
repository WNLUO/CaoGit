//! Repository commands
//!
//! Commands for opening, initializing, cloning, and detecting repository types.

use crate::git_ops::GitRepository;
use super::response::ApiResponse;

/// Open an existing repository
#[tauri::command]
pub fn open_repository(path: String) -> ApiResponse<String> {
    match GitRepository::open(&path) {
        Ok(_) => ApiResponse::success("Repository opened successfully".to_string()),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Clone a repository from URL (异步执行，不阻塞主线程)
#[tauri::command]
pub async fn clone_repository(url: String, path: String) -> ApiResponse<String> {
    // 在独立线程中执行，避免阻塞主线程
    let result = tokio::task::spawn_blocking(move || {
        match GitRepository::clone(&url, &path) {
            Ok(_) => ApiResponse::success("Repository cloned successfully".to_string()),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }).await;

    match result {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Task execution failed: {}", e)),
    }
}

/// Initialize a new repository
#[tauri::command]
pub fn init_repository(path: String, default_branch: Option<String>) -> ApiResponse<String> {
    match GitRepository::init(&path) {
        Ok(_repo) => {
            if let Some(branch_name) = default_branch {
                if branch_name != "master" && branch_name != "main" {
                    // Branch will be created on first commit
                }
            }
            ApiResponse::success("Repository initialized successfully".to_string())
        },
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Detect project type based on files in directory
#[tauri::command]
pub fn detect_project_type(path: String) -> ApiResponse<String> {
    use std::fs;
    use std::path::Path;

    let dir_path = Path::new(&path);

    if !dir_path.exists() || !dir_path.is_dir() {
        return ApiResponse::error("Path does not exist or is not a directory".to_string());
    }

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
