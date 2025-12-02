//! Remote operations
//!
//! This module handles all remote-related Git operations (fetch, pull, push).

use git2::{Cred, RemoteCallbacks, PushOptions, FetchOptions};
use std::sync::{Arc, Mutex};
use anyhow::Result;
use tauri::Emitter;

use super::repository::GitRepository;
use super::types::{RemoteInfo, GitProgress};

impl GitRepository {
    /// Fetch from a remote (without progress)
    #[allow(dead_code)]
    pub fn fetch(&self, remote_name: &str) -> Result<()> {
        let mut remote = self.repo.find_remote(remote_name)?;
        remote.fetch(&[] as &[&str], None, None)?;
        Ok(())
    }

    /// Fetch from a remote with progress reporting and timeout
    pub fn fetch_with_progress(&self, remote_name: &str, window: tauri::Window) -> Result<()> {
        let mut remote = self.repo.find_remote(remote_name)?;

        let mut callbacks = RemoteCallbacks::new();
        let window_clone = window.clone();
        let last_update = Arc::new(Mutex::new(std::time::Instant::now()));
        let start_time = Arc::new(std::time::Instant::now());
        let timeout_secs = 300; // 5 分钟超时

        callbacks.transfer_progress(move |stats| {
            // 检查超时
            if start_time.elapsed().as_secs() > timeout_secs {
                eprintln!("⏱️  Fetch operation timed out after {} seconds", timeout_secs);
                return false; // 中止操作
            }

            let mut last = match last_update.lock() {
                Ok(guard) => guard,
                Err(_) => return true, // Continue on lock error
            };
            let now = std::time::Instant::now();

            if now.duration_since(*last).as_millis() >= 200 {
                let received_bytes = stats.received_bytes();
                let total_objects = stats.total_objects();
                let received_objects = stats.received_objects();

                let duration = now.duration_since(*last).as_secs_f64();
                let speed = if duration > 0.0 {
                    (received_bytes as f64 / duration) as u64
                } else {
                    0
                };

                let progress = GitProgress {
                    operation_type: "download".to_string(),
                    total_objects,
                    received_objects,
                    total_bytes: received_bytes as u64,
                    received_bytes: received_bytes as u64,
                    speed_bytes_per_sec: speed,
                };

                let _ = window_clone.emit("git-progress", progress);
                *last = now;
            }

            true
        });

        // 添加认证回调，支持 SSH 和 HTTPS
        callbacks.credentials(|_url, username_from_url, allowed_types| {
            // 优先尝试 SSH agent
            if allowed_types.is_ssh_key() {
                if let Some(username) = username_from_url {
                    return Cred::ssh_key_from_agent(username);
                }
                return Cred::ssh_key_from_agent("git");
            }
            // 回退到默认凭据（用于 HTTPS）
            Cred::default()
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        remote.fetch(&[] as &[&str], Some(&mut fetch_options), None)?;
        Ok(())
    }

    /// Pull from a remote (without progress)
    #[allow(dead_code)]
    pub fn pull(&self, remote_name: &str, branch_name: &str) -> Result<()> {
        self.fetch(remote_name)?;

        let fetch_head = self.repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = self.repo.reference_to_annotated_commit(&fetch_head)?;

        let (analysis, _) = self.repo.merge_analysis(&[&fetch_commit])?;

        if analysis.is_up_to_date() {
            return Ok(());
        } else if analysis.is_fast_forward() {
            let refname = format!("refs/heads/{}", branch_name);
            let mut reference = self.repo.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            self.repo.set_head(&refname)?;
            self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else {
            self.repo.merge(&[&fetch_commit], None, None)?;
        }

        Ok(())
    }

    /// Pull from a remote with progress reporting
    pub fn pull_with_progress(&self, remote_name: &str, branch_name: &str, window: tauri::Window) -> Result<()> {
        self.fetch_with_progress(remote_name, window)?;

        let fetch_head = self.repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = self.repo.reference_to_annotated_commit(&fetch_head)?;

        let (analysis, _) = self.repo.merge_analysis(&[&fetch_commit])?;

        if analysis.is_up_to_date() {
            return Ok(());
        } else if analysis.is_fast_forward() {
            let refname = format!("refs/heads/{}", branch_name);
            let mut reference = self.repo.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            self.repo.set_head(&refname)?;
            self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else {
            self.repo.merge(&[&fetch_commit], None, None)?;
        }

        Ok(())
    }

    /// Push to a remote (without progress)
    #[allow(dead_code)]
    pub fn push(&self, remote_name: &str, branch_name: &str) -> Result<()> {
        let mut remote = self.repo.find_remote(remote_name)?;
        let remote_url = remote.url().unwrap_or("unknown");
        eprintln!("📡 Push to remote: {} ({})", remote_name, remote_url);

        let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);

        let mut callbacks = RemoteCallbacks::new();

        // 添加详细的认证日志和多重回退机制
        callbacks.credentials(move |url, username_from_url, allowed_types| {
            eprintln!("🔐 Credentials requested:");
            eprintln!("   URL: {}", url);
            eprintln!("   Username from URL: {:?}", username_from_url);
            eprintln!("   Allowed types: {:?}", allowed_types);

            // 1. 尝试 SSH key from agent（最常用）
            if allowed_types.is_ssh_key() {
                eprintln!("   Trying SSH key from agent...");
                let username = username_from_url.unwrap_or("git");
                match Cred::ssh_key_from_agent(username) {
                    Ok(cred) => {
                        eprintln!("   ✅ SSH agent credential obtained");
                        return Ok(cred);
                    }
                    Err(e) => {
                        eprintln!("   ❌ SSH agent failed: {}", e);
                    }
                }
            }

            // 2. 尝试从默认位置读取 SSH 密钥
            if allowed_types.is_ssh_key() {
                eprintln!("   Trying SSH key from ~/.ssh/id_rsa...");
                let username = username_from_url.unwrap_or("git");
                match std::env::var("HOME") {
                    Ok(home) => {
                        let private_key = format!("{}/.ssh/id_rsa", home);
                        let public_key = format!("{}/.ssh/id_rsa.pub", home);
                        match Cred::ssh_key(username, Some(std::path::Path::new(&public_key)), std::path::Path::new(&private_key), None) {
                            Ok(cred) => {
                                eprintln!("   ✅ SSH key from ~/.ssh obtained");
                                return Ok(cred);
                            }
                            Err(e) => {
                                eprintln!("   ❌ SSH key from file failed: {}", e);
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("   ❌ HOME environment variable not set");
                    }
                }
            }

            // 3. 尝试默认凭据（用于 HTTPS）
            if allowed_types.is_user_pass_plaintext() {
                eprintln!("   Trying default credentials (HTTPS)...");
                match Cred::default() {
                    Ok(cred) => {
                        eprintln!("   ✅ Default credential obtained");
                        return Ok(cred);
                    }
                    Err(e) => {
                        eprintln!("   ❌ Default credential failed: {}", e);
                    }
                }
            }

            eprintln!("   ❌ All authentication methods failed");
            Err(git2::Error::from_str("No valid authentication method available"))
        });

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);

        eprintln!("🚀 Starting push operation...");
        remote.push(&[&refspec], Some(&mut push_options))?;
        eprintln!("✅ Push completed successfully");

        // Set upstream tracking after successful push
        let mut branch = self.repo.find_branch(branch_name, git2::BranchType::Local)?;
        branch.set_upstream(Some(&format!("{}/{}", remote_name, branch_name)))?;

        Ok(())
    }

    /// Push a tag to a remote
    pub fn push_tag(&self, remote_name: &str, tag_name: &str) -> Result<()> {
        let mut remote = self.repo.find_remote(remote_name)?;
        let refspec = format!("refs/tags/{}:refs/tags/{}", tag_name, tag_name);

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            if let Some(username) = username_from_url {
                Cred::ssh_key_from_agent(username)
            } else {
                Cred::ssh_key_from_agent("git")
            }
        });

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);

        remote.push(&[&refspec], Some(&mut push_options))?;
        Ok(())
    }

    /// Push to a remote with progress reporting and timeout
    pub fn push_with_progress(&self, remote_name: &str, branch_name: &str, window: tauri::Window) -> Result<()> {
        let mut remote = self.repo.find_remote(remote_name)?;

        let mut callbacks = RemoteCallbacks::new();
        let start_time = Arc::new(std::time::Instant::now());
        let timeout_secs = 300; // 5 分钟超时

        callbacks.credentials(|_url, username_from_url, allowed_types| {
            // 优先尝试 SSH agent
            if allowed_types.is_ssh_key() {
                if let Some(username) = username_from_url {
                    return Cred::ssh_key_from_agent(username);
                }
                return Cred::ssh_key_from_agent("git");
            }
            // 回退到默认凭据（用于 HTTPS）
            Cred::default()
        });

        let window_clone = window.clone();
        let last_update = Arc::new(Mutex::new(std::time::Instant::now()));
        let last_bytes = Arc::new(Mutex::new(0usize));

        callbacks.push_transfer_progress(move |current, total, bytes| {
            // 检查超时
            if start_time.elapsed().as_secs() > timeout_secs {
                eprintln!("⏱️  Push operation timed out after {} seconds", timeout_secs);
                // push_transfer_progress 没有返回值，无法中止
                // 但可以停止发送进度更新
                return;
            }

            let mut last = match last_update.lock() {
                Ok(guard) => guard,
                Err(_) => return, // Continue on lock error
            };
            let mut prev_bytes = match last_bytes.lock() {
                Ok(guard) => guard,
                Err(_) => return, // Continue on lock error
            };
            let now = std::time::Instant::now();

            if now.duration_since(*last).as_millis() >= 200 {
                let duration = now.duration_since(*last).as_secs_f64();
                let bytes_diff = bytes.saturating_sub(*prev_bytes);
                let speed = if duration > 0.0 {
                    (bytes_diff as f64 / duration) as u64
                } else {
                    0
                };

                let progress = GitProgress {
                    operation_type: "upload".to_string(),
                    total_objects: total,
                    received_objects: current,
                    total_bytes: bytes as u64,
                    received_bytes: bytes as u64,
                    speed_bytes_per_sec: speed,
                };

                let _ = window_clone.emit("git-progress", progress);
                *last = now;
                *prev_bytes = bytes;
            }
        });

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);

        let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
        remote.push(&[&refspec], Some(&mut push_options))?;

        // Set upstream tracking after successful push
        let mut branch = self.repo.find_branch(branch_name, git2::BranchType::Local)?;
        branch.set_upstream(Some(&format!("{}/{}", remote_name, branch_name)))?;

        Ok(())
    }

    /// Get all remotes
    pub fn get_remotes(&self) -> Result<Vec<RemoteInfo>> {
        let remotes = self.repo.remotes()?;
        let mut remote_infos = Vec::new();

        for name in remotes.iter() {
            if let Some(name) = name {
                if let Ok(remote) = self.repo.find_remote(name) {
                    if let Some(url) = remote.url() {
                        remote_infos.push(RemoteInfo {
                            name: name.to_string(),
                            url: url.to_string(),
                        });
                    }
                }
            }
        }

        Ok(remote_infos)
    }

    /// Get the URL of a specific remote
    pub fn get_remote_url(&self, name: &str) -> Result<String> {
        let remote = self.repo.find_remote(name)?;
        remote
            .url()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Remote '{}' has no URL", name))
    }

    /// Add a new remote
    pub fn add_remote(&self, name: &str, url: &str) -> Result<()> {
        self.repo.remote(name, url)?;
        Ok(())
    }

    /// Remove a remote
    pub fn remove_remote(&self, name: &str) -> Result<()> {
        self.repo.remote_delete(name)?;
        Ok(())
    }
}
