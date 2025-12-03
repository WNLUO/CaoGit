/**
 * Keychain API for secure credential storage
 *
 * This module provides a TypeScript interface to the macOS Keychain
 * for secure storage of sensitive data like API keys, tokens, and passwords.
 */

import { invoke } from '@tauri-apps/api/core';

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

/**
 * Save a credential to the macOS Keychain
 *
 * @param account - The account/key name (e.g., "ai_api_key", "github_token")
 * @param password - The password/secret to store
 * @returns Promise resolving to success/error
 */
export async function saveToKeychain(account: string, password: string): Promise<ApiResponse<void>> {
  try {
    const response = await invoke<ApiResponse<void>>('keychain_save', {
      account,
      password
    });
    return response;
  } catch (error) {
    return {
      success: false,
      error: String(error)
    };
  }
}

/**
 * Retrieve a credential from the macOS Keychain
 *
 * @param account - The account/key name
 * @returns Promise resolving to the password or error
 */
export async function getFromKeychain(account: string): Promise<ApiResponse<string>> {
  try {
    const response = await invoke<ApiResponse<string>>('keychain_get', {
      account
    });
    return response;
  } catch (error) {
    return {
      success: false,
      error: String(error)
    };
  }
}

/**
 * Delete a credential from the macOS Keychain
 *
 * @param account - The account/key name
 * @returns Promise resolving to success/error
 */
export async function deleteFromKeychain(account: String): Promise<ApiResponse<void>> {
  try {
    const response = await invoke<ApiResponse<void>>('keychain_delete', {
      account
    });
    return response;
  } catch (error) {
    return {
      success: false,
      error: String(error)
    };
  }
}

/**
 * Check if a credential exists in the Keychain
 *
 * @param account - The account/key name
 * @returns Promise resolving to true if exists, false otherwise
 */
export async function keychainExists(account: string): Promise<boolean> {
  try {
    const response = await invoke<ApiResponse<boolean>>('keychain_exists', {
      account
    });
    return response.success && response.data === true;
  } catch (error) {
    return false;
  }
}

/**
 * Migrate a credential from localStorage to Keychain
 *
 * This helper function migrates existing credentials and removes them from localStorage
 *
 * @param account - The account/key name
 * @param localStorageKey - The key used in localStorage
 * @returns Promise resolving to success/error
 */
export async function migrateToKeychain(account: string, localStorageKey: string): Promise<ApiResponse<void>> {
  try {
    // Get value from localStorage
    const value = localStorage.getItem(localStorageKey);
    if (!value) {
      return { success: true }; // Nothing to migrate
    }

    // Save to Keychain
    const response = await invoke<ApiResponse<void>>('keychain_migrate', {
      account,
      password: value
    });

    if (response.success) {
      // Remove from localStorage after successful migration
      localStorage.removeItem(localStorageKey);
    }

    return response;
  } catch (error) {
    return {
      success: false,
      error: String(error)
    };
  }
}

/**
 * Keychain account names used by the application
 */
export const KeychainAccounts = {
  AI_API_KEY: 'ai_api_key',
  GITHUB_TOKEN: 'github_token',
  GITLAB_TOKEN: 'gitlab_token',
  GITEE_TOKEN: 'gitee_token',
  AI_CONSENT: 'ai_consent_status',
} as const;
