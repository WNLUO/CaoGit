//! Branch operations
//!
//! This module handles all branch-related Git operations.

use git2::BranchType;
use anyhow::Result;

use super::repository::GitRepository;
use super::types::BranchInfo;

impl GitRepository {
    /// Get all branches (local and remote)
    pub fn get_branches(&self) -> Result<Vec<BranchInfo>> {
        let mut branches = Vec::new();

        // Local branches
        let local_branches = self.repo.branches(Some(BranchType::Local))?;
        for branch in local_branches {
            let (branch, _) = branch?;
            let name = branch.name()?.unwrap_or("").to_string();
            let is_head = branch.is_head();

            let upstream = branch.upstream()
                .ok()
                .and_then(|u| u.name().ok().flatten().map(|s| s.to_string()));

            let last_commit = branch.get().peel_to_commit()
                .ok()
                .map(|c| c.id().to_string());

            branches.push(BranchInfo {
                name,
                is_head,
                is_remote: false,
                upstream,
                last_commit,
            });
        }

        // Remote branches
        let remote_branches = self.repo.branches(Some(BranchType::Remote))?;
        for branch in remote_branches {
            let (branch, _) = branch?;
            let name = branch.name()?.unwrap_or("").to_string();

            let last_commit = branch.get().peel_to_commit()
                .ok()
                .map(|c| c.id().to_string());

            branches.push(BranchInfo {
                name,
                is_head: false,
                is_remote: true,
                upstream: None,
                last_commit,
            });
        }

        Ok(branches)
    }

    /// Create a new branch
    pub fn create_branch(&self, name: &str) -> Result<()> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        self.repo.branch(name, &commit, false)?;
        Ok(())
    }

    /// Checkout a branch
    pub fn checkout_branch(&self, name: &str) -> Result<()> {
        use git2::Status;

        // Check for uncommitted changes in working directory
        let statuses = self.repo.statuses(None)?;
        let has_changes = statuses.iter().any(|entry| {
            let status = entry.status();
            status.contains(Status::WT_MODIFIED)
                || status.contains(Status::WT_DELETED)
                || status.contains(Status::WT_NEW)
                || status.contains(Status::INDEX_MODIFIED)
                || status.contains(Status::INDEX_NEW)
                || status.contains(Status::INDEX_DELETED)
        });

        if has_changes {
            anyhow::bail!(
                "Cannot checkout branch '{}': You have uncommitted changes. \
                Please commit or stash your changes first.",
                name
            );
        }

        // Proceed with checkout
        let obj = self.repo.revparse_single(&format!("refs/heads/{}", name))?;
        self.repo.checkout_tree(&obj, None)?;
        self.repo.set_head(&format!("refs/heads/{}", name))?;
        Ok(())
    }

    /// Delete a branch
    pub fn delete_branch(&self, name: &str) -> Result<()> {
        // Check if trying to delete the current branch
        let current_branch = self.get_current_branch()?;
        if current_branch == name {
            anyhow::bail!(
                "Cannot delete the currently checked out branch '{}'. \
                Please checkout another branch first.",
                name
            );
        }

        let mut branch = self.repo.find_branch(name, BranchType::Local)?;
        branch.delete()?;
        Ok(())
    }

    /// Get the current branch name
    pub fn get_current_branch(&self) -> Result<String> {
        match self.repo.head() {
            Ok(head) => Ok(head.shorthand().unwrap_or("HEAD").to_string()),
            Err(_) => Ok("main".to_string()),
        }
    }
}
