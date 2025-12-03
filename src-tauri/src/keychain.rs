//! macOS Keychain integration for secure credential storage
//!
//! This module provides secure storage for sensitive data like API keys,
//! tokens, and passwords using the macOS Keychain.

use anyhow::{Context, Result};

#[cfg(target_os = "macos")]
use security_framework::passwords::{get_generic_password, set_generic_password, delete_generic_password};

/// Service name for Keychain items
const SERVICE_NAME: &str = "com.caogit.app";

/// Save a password to the macOS Keychain
///
/// # Arguments
/// * `account` - The account/key name (e.g., "ai_api_key", "github_token")
/// * `password` - The password/secret to store
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err` if the operation fails
#[cfg(target_os = "macos")]
pub fn save_password(account: &str, password: &str) -> Result<()> {
    set_generic_password(SERVICE_NAME, account, password.as_bytes())
        .context(format!("Failed to save password for account: {}", account))
}

/// Retrieve a password from the macOS Keychain
///
/// # Arguments
/// * `account` - The account/key name
///
/// # Returns
/// * `Ok(String)` containing the password if found
/// * `Err` if the password is not found or operation fails
#[cfg(target_os = "macos")]
pub fn get_password(account: &str) -> Result<String> {
    let password_bytes = get_generic_password(SERVICE_NAME, account)
        .context(format!("Failed to get password for account: {}", account))?;

    String::from_utf8(password_bytes.to_vec())
        .context("Failed to convert password bytes to string")
}

/// Delete a password from the macOS Keychain
///
/// # Arguments
/// * `account` - The account/key name
///
/// # Returns
/// * `Ok(())` if successful or if the item doesn't exist
/// * `Err` if the operation fails
#[cfg(target_os = "macos")]
pub fn delete_password(account: &str) -> Result<()> {
    delete_generic_password(SERVICE_NAME, account)
        .or_else(|e| {
            // If the item doesn't exist, that's fine
            if e.to_string().contains("not found") {
                Ok(())
            } else {
                Err(e)
            }
        })
        .context(format!("Failed to delete password for account: {}", account))
}

/// Check if a password exists in the Keychain
///
/// # Arguments
/// * `account` - The account/key name
///
/// # Returns
/// * `true` if the password exists
/// * `false` if it doesn't exist or on error
#[cfg(target_os = "macos")]
pub fn password_exists(account: &str) -> bool {
    get_generic_password(SERVICE_NAME, account).is_ok()
}

// Stub implementations for non-macOS platforms
#[cfg(not(target_os = "macos"))]
pub fn save_password(_account: &str, _password: &str) -> Result<()> {
    anyhow::bail!("Keychain is only supported on macOS")
}

#[cfg(not(target_os = "macos"))]
pub fn get_password(_account: &str) -> Result<String> {
    anyhow::bail!("Keychain is only supported on macOS")
}

#[cfg(not(target_os = "macos"))]
pub fn delete_password(_account: &str) -> Result<()> {
    anyhow::bail!("Keychain is only supported on macOS")
}

#[cfg(not(target_os = "macos"))]
pub fn password_exists(_account: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn test_keychain_operations() {
        let test_account = "test_account_caogit";
        let test_password = "test_password_123";

        // Clean up any existing test data
        let _ = delete_password(test_account);

        // Test save
        assert!(save_password(test_account, test_password).is_ok());

        // Test exists
        assert!(password_exists(test_account));

        // Test get
        let retrieved = get_password(test_account);
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap(), test_password);

        // Test delete
        assert!(delete_password(test_account).is_ok());

        // Test not exists after delete
        assert!(!password_exists(test_account));
    }
}
