//! Diff operations
//!
//! This module handles all diff-related Git operations.

use git2::{DiffOptions, Status, StatusOptions};
use anyhow::{Context, Result};

use super::repository::GitRepository;
use super::types::{DiffResult, DiffHunk, DiffLine};

impl GitRepository {
    /// Get the diff for a specific file
    pub fn get_file_diff(&self, file_path: &str, staged: bool) -> Result<DiffResult> {
        // Check if this is a new file (untracked)
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);
        let statuses = self.repo.statuses(Some(&mut status_opts))?;

        let is_new_file = statuses.iter().any(|entry| {
            entry.path().map(|p| p == file_path).unwrap_or(false)
                && entry.status().contains(Status::WT_NEW)
        });

        // For new files, read the entire file content and display as additions
        if is_new_file && !staged {
            return self.get_new_file_diff(file_path);
        }

        // Original logic for non-new files
        self.get_existing_file_diff(file_path, staged)
    }

    /// Get diff for a new (untracked) file
    fn get_new_file_diff(&self, file_path: &str) -> Result<DiffResult> {
        use std::fs;

        // Safely get repository working directory
        let workdir = self.repo.workdir()
            .ok_or_else(|| anyhow::anyhow!("Repository has no working directory"))?;

        let file_full_path = workdir.join(file_path);

        // Validate that the file path is within the repository (prevent path traversal)
        let canonical_file = file_full_path.canonicalize()
            .context(format!("Failed to resolve file path: {}", file_path))?;
        let canonical_workdir = workdir.canonicalize()
            .context("Failed to resolve repository working directory")?;

        if !canonical_file.starts_with(&canonical_workdir) {
            anyhow::bail!("File path is outside repository: {}", file_path);
        }

        let content = fs::read_to_string(&file_full_path)
            .context(format!("Failed to read file: {}", file_path))?;

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len() as u32;

        let mut diff_lines = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            diff_lines.push(DiffLine {
                origin: '+',
                content: line.to_string() + "\n",
                old_lineno: None,
                new_lineno: Some((i + 1) as u32),
            });
        }

        let hunk = DiffHunk {
            old_start: 0,
            old_lines: 0,
            new_start: 1,
            new_lines: total_lines,
            header: format!("@@ -0,0 +1,{} @@ New file", total_lines),
            lines: diff_lines,
        };

        Ok(DiffResult {
            old_path: "/dev/null".to_string(),
            new_path: file_path.to_string(),
            status: "Added".to_string(),
            hunks: vec![hunk],
        })
    }

    /// Get diff for an existing file
    fn get_existing_file_diff(&self, file_path: &str, staged: bool) -> Result<DiffResult> {
        use std::cell::RefCell;
        use std::rc::Rc;

        let mut diff_opts = DiffOptions::new();
        diff_opts.pathspec(file_path);

        let diff = if staged {
            let head = self.repo.head()?.peel_to_tree()?;
            self.repo.diff_tree_to_index(Some(&head), None, Some(&mut diff_opts))?
        } else {
            self.repo.diff_index_to_workdir(None, Some(&mut diff_opts))?
        };

        let mut result = DiffResult {
            old_path: file_path.to_string(),
            new_path: file_path.to_string(),
            status: "Modified".to_string(),
            hunks: Vec::new(),
        };

        let hunks_ref = Rc::new(RefCell::new(Vec::new()));
        let hunks_clone = Rc::clone(&hunks_ref);

        diff.foreach(
            &mut |delta, _progress| {
                result.old_path = delta.old_file().path()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "/dev/null".to_string());
                result.new_path = delta.new_file().path()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| file_path.to_string());
                result.status = format!("{:?}", delta.status());
                true
            },
            None,
            Some(&mut |_delta, hunk| {
                hunks_clone.borrow_mut().push(DiffHunk {
                    old_start: hunk.old_start(),
                    old_lines: hunk.old_lines(),
                    new_start: hunk.new_start(),
                    new_lines: hunk.new_lines(),
                    header: String::from_utf8_lossy(hunk.header()).to_string(),
                    lines: Vec::new(),
                });
                true
            }),
            Some(&mut |_delta, _hunk, line| {
                let mut hunks = hunks_clone.borrow_mut();
                if let Some(last_hunk) = hunks.last_mut() {
                    last_hunk.lines.push(DiffLine {
                        origin: line.origin(),
                        content: String::from_utf8_lossy(line.content()).to_string(),
                        old_lineno: line.old_lineno(),
                        new_lineno: line.new_lineno(),
                    });
                }
                true
            }),
        )?;

        drop(hunks_clone);
        result.hunks = Rc::try_unwrap(hunks_ref)
            .map_err(|_| git2::Error::from_str("Failed to unwrap hunks reference"))?
            .into_inner();
        Ok(result)
    }
}
