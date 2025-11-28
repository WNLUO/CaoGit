//! Core repository operations
//!
//! This module provides the main GitRepository struct and basic operations.

use git2::{Repository, Status, StatusOptions};
use std::path::Path;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

use super::types::{FileChange, CommitInfo};

/// Main struct for Git repository operations
pub struct GitRepository {
    pub(crate) repo: Repository,
}

impl GitRepository {
    /// Open an existing repository
    pub fn open(path: &str) -> Result<Self> {
        let repo = Repository::open(path)
            .context(format!("Failed to open repository at {}", path))?;
        Ok(Self { repo })
    }

    /// Initialize a new repository
    pub fn init(path: &str) -> Result<Self> {
        let repo = Repository::init(path)
            .context(format!("Failed to initialize repository at {}", path))?;
        Ok(Self { repo })
    }

    /// Clone a repository from a URL
    pub fn clone(url: &str, path: &str) -> Result<Self> {
        let repo = Repository::clone(url, path)
            .context(format!("Failed to clone repository from {}", url))?;
        Ok(Self { repo })
    }

    /// Get the status of all files in the repository
    pub fn get_status(&self) -> Result<Vec<FileChange>> {
        let mut status_opts = StatusOptions::new();
        status_opts
            .include_untracked(true)
            .recurse_untracked_dirs(true);

        let statuses = self.repo.statuses(Some(&mut status_opts))?;
        let mut changes = Vec::new();

        for entry in statuses.iter() {
            let path = entry.path().unwrap_or("").to_string();
            let status_flags = entry.status();

            let (status, staged) = if status_flags.contains(Status::INDEX_NEW) {
                ("added".to_string(), true)
            } else if status_flags.contains(Status::INDEX_MODIFIED) {
                ("modified".to_string(), true)
            } else if status_flags.contains(Status::INDEX_DELETED) {
                ("deleted".to_string(), true)
            } else if status_flags.contains(Status::INDEX_RENAMED) {
                ("renamed".to_string(), true)
            } else if status_flags.contains(Status::WT_MODIFIED) {
                ("modified".to_string(), false)
            } else if status_flags.contains(Status::WT_DELETED) {
                ("deleted".to_string(), false)
            } else if status_flags.contains(Status::WT_NEW) {
                ("untracked".to_string(), false)
            } else {
                continue;
            };

            changes.push(FileChange { path, status, staged });
        }

        Ok(changes)
    }

    /// Stage a file for commit
    pub fn stage_file(&self, path: &str) -> Result<()> {
        let mut index = self.repo.index()?;
        let path_obj = Path::new(path);

        let workdir = self.repo.workdir()
            .ok_or_else(|| anyhow::anyhow!("Repository has no working directory"))?;
        let full_path = workdir.join(path_obj);

        if full_path.exists() {
            index.add_path(path_obj)?;
        } else {
            index.remove_path(path_obj)?;
        }

        index.write()?;
        Ok(())
    }

    /// Unstage a file
    pub fn unstage_file(&self, path: &str) -> Result<()> {
        let head = self.repo.head()?;
        let head_commit = head.peel_to_commit()?;
        let head_tree = head_commit.tree()?;

        let mut index = self.repo.index()?;
        let entry = head_tree.get_path(Path::new(path));

        if let Ok(entry) = entry {
            index.add(&git2::IndexEntry {
                ctime: git2::IndexTime::new(0, 0),
                mtime: git2::IndexTime::new(0, 0),
                dev: 0,
                ino: 0,
                mode: entry.filemode() as u32,
                uid: 0,
                gid: 0,
                file_size: 0,
                id: entry.id(),
                flags: 0,
                flags_extended: 0,
                path: path.as_bytes().to_vec(),
            })?;
        } else {
            index.remove_path(Path::new(path))?;
        }

        index.write()?;
        Ok(())
    }

    /// Discard changes to a file (restore to HEAD state)
    pub fn discard_file(&self, path: &str) -> Result<()> {
        let head = self.repo.head()?;
        let head_commit = head.peel_to_commit()?;
        let head_tree = head_commit.tree()?;

        self.repo.checkout_tree(
            head_tree.as_object(),
            Some(
                git2::build::CheckoutBuilder::new()
                    .path(Path::new(path))
                    .force()
            ),
        )?;

        Ok(())
    }

    /// Create a new commit
    pub fn commit(&self, message: &str) -> Result<String> {
        let signature = self.repo.signature()?;
        let mut index = self.repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let parent_commit = match self.repo.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(_) => None,
        };

        let parents = match &parent_commit {
            Some(commit) => vec![commit],
            None => vec![],
        };

        let oid = self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;

        Ok(oid.to_string())
    }

    /// Get commit history
    pub fn get_commits(&self, max_count: usize) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;

        if revwalk.push_head().is_err() {
            return Ok(Vec::new());
        }

        let mut commits = Vec::new();
        for (i, oid) in revwalk.enumerate() {
            if i >= max_count {
                break;
            }

            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            commits.push(CommitInfo {
                hash: oid.to_string(),
                message: commit.message().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("").to_string(),
                email: commit.author().email().unwrap_or("").to_string(),
                date: DateTime::<Utc>::from_timestamp(commit.time().seconds(), 0)
                    .unwrap()
                    .to_rfc3339(),
                parents: commit.parents().map(|p| p.id().to_string()).collect(),
            });
        }

        Ok(commits)
    }

    /// Get commits between two refs
    pub fn get_commits_between(&self, from_ref: &str, to_ref: &str) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;

        let to_obj = self.repo.revparse_single(to_ref)
            .context(format!("无法找到引用: {}", to_ref))?;
        let to_commit = to_obj.peel_to_commit()
            .context(format!("引用 {} 不是有效的提交", to_ref))?;

        let from_obj = self.repo.revparse_single(from_ref)
            .context(format!("无法找到引用: {}", from_ref))?;
        let from_commit = from_obj.peel_to_commit()
            .context(format!("引用 {} 不是有效的提交", from_ref))?;

        revwalk.push(to_commit.id())?;
        revwalk.hide(from_commit.id())?;

        let mut commits = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            commits.push(CommitInfo {
                hash: oid.to_string(),
                message: commit.message().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("").to_string(),
                email: commit.author().email().unwrap_or("").to_string(),
                date: DateTime::<Utc>::from_timestamp(commit.time().seconds(), 0)
                    .unwrap()
                    .to_rfc3339(),
                parents: commit.parents().map(|p| p.id().to_string()).collect(),
            });
        }

        Ok(commits)
    }
}
