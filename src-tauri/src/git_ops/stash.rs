//! Stash operations
//!
//! This module handles all stash-related Git operations.

use anyhow::Result;

use super::repository::GitRepository;
use super::types::StashInfo;

impl GitRepository {
    /// Save changes to stash
    pub fn stash_save(&mut self, message: Option<&str>) -> Result<()> {
        let signature = self.repo.signature()?;
        let msg = message.unwrap_or("WIP");
        self.repo.stash_save(&signature, msg, None)?;
        Ok(())
    }

    /// List all stash entries
    pub fn stash_list(&mut self) -> Result<Vec<StashInfo>> {
        let mut stashes = Vec::new();
        self.repo.stash_foreach(|index, message, _oid| {
            stashes.push(StashInfo {
                index,
                message: message.to_string(),
            });
            true
        })?;
        Ok(stashes)
    }

    /// Pop a stash entry (apply and remove)
    pub fn stash_pop(&mut self, index: usize) -> Result<()> {
        self.repo.stash_pop(index, None)?;
        Ok(())
    }

    /// Drop a stash entry (remove without applying)
    pub fn stash_drop(&mut self, index: usize) -> Result<()> {
        self.repo.stash_drop(index)?;
        Ok(())
    }
}
