//! Utility commands
//!
//! General utility commands for clipboard, theme, and version.

use tauri::{Window, Theme};
use serde::Serialize;
use super::response::ApiResponse;

/// Set window theme
#[tauri::command]
pub fn set_window_theme(window: Window, theme: String) -> ApiResponse<String> {
    let tauri_theme = match theme.as_str() {
        "dark" => Some(Theme::Dark),
        "light" => Some(Theme::Light),
        _ => None,
    };

    match window.set_theme(tauri_theme) {
        Ok(_) => ApiResponse::success(format!("Window theme set to {}", theme)),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Copy text to clipboard
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

/// Version information
#[derive(Debug, Serialize)]
pub struct VersionInfo {
    pub version: String,
}

/// Get application version
#[tauri::command]
pub fn get_app_version() -> VersionInfo {
    VersionInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}
