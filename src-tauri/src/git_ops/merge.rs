//! Merge operations
//!
//! This module handles all merge-related Git operations including cherry-pick and conflict resolution.

use anyhow::{Context, Result};

use super::repository::GitRepository;
use super::types::ConflictInfo;

impl GitRepository {
    /// Merge a branch into the current branch
    pub fn merge(&self, branch_name: &str) -> Result<String> {
        let (their_commit, _) = self.repo.revparse_ext(branch_name)?;
        let annotated_commit = self.repo.find_annotated_commit(their_commit.id())?;

        let (analysis, _) = self.repo.merge_analysis(&[&annotated_commit])?;

        if analysis.is_up_to_date() {
            return Ok("Already up-to-date".to_string());
        } else if analysis.is_fast_forward() {
            let refname = format!("refs/heads/{}", self.get_current_branch()?);
            let mut reference = self.repo.find_reference(&refname)?;
            reference.set_target(annotated_commit.id(), "Fast-forward merge")?;
            self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            return Ok("Fast-forward merge completed".to_string());
        } else {
            self.repo.merge(&[&annotated_commit], None, None)?;

            let mut index = self.repo.index()?;
            if index.has_conflicts() {
                return Ok("Merge has conflicts - needs resolution".to_string());
            }

            let signature = self.repo.signature()?;
            let tree_id = index.write_tree()?;
            let tree = self.repo.find_tree(tree_id)?;
            let head_commit = self.repo.head()?.peel_to_commit()?;
            let their_commit = self.repo.find_commit(annotated_commit.id())?;

            let message = format!("Merge branch '{}'", branch_name);
            self.repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &message,
                &tree,
                &[&head_commit, &their_commit],
            )?;

            return Ok("Merge completed successfully".to_string());
        }
    }

    /// Cherry-pick a single commit
    pub fn cherry_pick(&self, commit_hash: &str) -> Result<String> {
        let oid = git2::Oid::from_str(commit_hash)
            .context(format!("Invalid commit hash: {}", commit_hash))?;
        let commit = self.repo.find_commit(oid)
            .context(format!("Failed to find commit: {}", commit_hash))?;

        let mut opts = git2::CherrypickOptions::new();
        self.repo.cherrypick(&commit, Some(&mut opts))
            .context("Cherry-pick failed")?;

        let mut index = self.repo.index()?;
        if index.has_conflicts() {
            return Ok("Cherry-pick has conflicts - needs resolution".to_string());
        }

        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;
        let signature = self.repo.signature()?;
        let head = self.repo.head()?.peel_to_commit()?;

        let message = commit.message().unwrap_or("Cherry-picked commit");
        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&head],
        )?;

        Ok(format!("Cherry-pick successful: {}", &commit_hash[..7]))
    }

    /// Cherry-pick multiple commits in batch
    pub fn cherry_pick_batch(&self, commit_hashes: Vec<String>) -> Result<Vec<String>> {
        let mut results = Vec::new();

        for hash in commit_hashes {
            match self.cherry_pick(&hash) {
                Ok(msg) => results.push(format!("{}: {}", &hash[..7], msg)),
                Err(e) => {
                    results.push(format!("{}: Failed - {}", &hash[..7], e));
                    break;
                }
            }
        }

        Ok(results)
    }

    /// Get all merge conflicts
    pub fn get_conflicts(&self) -> Result<Vec<ConflictInfo>> {
        use std::fs;
        use std::io::Read;

        let index = self.repo.index()?;
        if !index.has_conflicts() {
            return Ok(Vec::new());
        }

        let mut conflicts = Vec::new();
        let conflicts_iter = index.conflicts()?;

        for conflict in conflicts_iter {
            let conflict = conflict?;

            let path = if let Some(ours) = &conflict.our {
                String::from_utf8_lossy(&ours.path).to_string()
            } else if let Some(theirs) = &conflict.their {
                String::from_utf8_lossy(&theirs.path).to_string()
            } else {
                continue;
            };

            // Get working directory safely
            let workdir = self.repo.workdir()
                .unwrap_or_else(|| self.repo.path());
            let full_path = workdir.join(&path);

            let mut file_content = String::new();
            if let Ok(mut file) = fs::File::open(&full_path) {
                let _ = file.read_to_string(&mut file_content);
            }

            let ours = if let Some(our_entry) = &conflict.our {
                let blob = self.repo.find_blob(our_entry.id)?;
                String::from_utf8_lossy(blob.content()).to_string()
            } else {
                String::new()
            };

            let theirs = if let Some(their_entry) = &conflict.their {
                let blob = self.repo.find_blob(their_entry.id)?;
                String::from_utf8_lossy(blob.content()).to_string()
            } else {
                String::new()
            };

            let base = if let Some(ancestor_entry) = &conflict.ancestor {
                let blob = self.repo.find_blob(ancestor_entry.id)?;
                Some(String::from_utf8_lossy(blob.content()).to_string())
            } else {
                None
            };

            conflicts.push(ConflictInfo {
                path,
                ours,
                theirs,
                base,
            });
        }

        Ok(conflicts)
    }

    /// Resolve a merge conflict
    pub fn resolve_conflict(&self, path: &str, resolution: &str) -> Result<()> {
        use std::fs;
        use std::io::Write;

        // Get working directory safely
        let workdir = self.repo.workdir()
            .ok_or_else(|| anyhow::anyhow!("Repository has no working directory"))?;

        let full_path = workdir.join(path);

        // Validate path is within repository bounds
        let canonical_workdir = workdir.canonicalize()
            .context("Failed to resolve repository working directory")?;

        // For conflict resolution, we're creating/overwriting the file, so use parent dir check
        if let Some(parent) = full_path.parent() {
            if let Ok(canonical_parent) = parent.canonicalize() {
                if !canonical_parent.starts_with(&canonical_workdir) {
                    anyhow::bail!("File path is outside repository: {}", path);
                }
            }
        }

        let mut file = fs::File::create(&full_path)
            .context(format!("Failed to open file for writing: {}", path))?;
        file.write_all(resolution.as_bytes())
            .context("Failed to write resolution")?;

        self.stage_file(path)?;

        Ok(())
    }

    /// Abort the current merge
    pub fn abort_merge(&self) -> Result<()> {
        let head = self.repo.head()?.peel_to_commit()?;
        self.repo.reset(head.as_object(), git2::ResetType::Hard, None)?;

        let repo_path = self.repo.path();
        let merge_head = repo_path.join("MERGE_HEAD");
        let merge_msg = repo_path.join("MERGE_MSG");

        if merge_head.exists() {
            std::fs::remove_file(merge_head)?;
        }
        if merge_msg.exists() {
            std::fs::remove_file(merge_msg)?;
        }

        Ok(())
    }
}
