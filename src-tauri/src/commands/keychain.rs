//! Keychain commands for secure credential storage

use super::response::ApiResponse;

/// Save a credential to the macOS Keychain
#[tauri::command]
pub fn keychain_save(account: String, password: String) -> ApiResponse<()> {
    match crate::keychain::save_password(&account, &password) {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error(format!("Failed to save to keychain: {}", e)),
    }
}

/// Retrieve a credential from the macOS Keychain
#[tauri::command]
pub fn keychain_get(account: String) -> ApiResponse<String> {
    match crate::keychain::get_password(&account) {
        Ok(password) => ApiResponse::success(password),
        Err(e) => ApiResponse::error(format!("Failed to get from keychain: {}", e)),
    }
}

/// Delete a credential from the macOS Keychain
#[tauri::command]
pub fn keychain_delete(account: String) -> ApiResponse<()> {
    match crate::keychain::delete_password(&account) {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error(format!("Failed to delete from keychain: {}", e)),
    }
}

/// Check if a credential exists in the Keychain
#[tauri::command]
pub fn keychain_exists(account: String) -> ApiResponse<bool> {
    let exists = crate::keychain::password_exists(&account);
    ApiResponse::success(exists)
}

/// Migrate a credential from localStorage to Keychain
///
/// This is a helper command for migrating existing credentials
#[tauri::command]
pub fn keychain_migrate(account: String, password: String) -> ApiResponse<()> {
    match crate::keychain::save_password(&account, &password) {
        Ok(_) => {
            // Successfully migrated
            ApiResponse::success(())
        }
        Err(e) => ApiResponse::error(format!("Failed to migrate to keychain: {}", e)),
    }
}
