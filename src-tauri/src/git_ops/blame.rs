//! Blame operations
//!
//! This module handles git blame functionality.

use std::path::Path;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

use super::repository::GitRepository;
use super::types::BlameLine;

impl GitRepository {
    /// Get blame information for a file
    pub fn blame_file(&self, file_path: &str) -> Result<Vec<BlameLine>> {
        use std::fs;
        use std::io::{BufRead, BufReader};

        let blame = self.repo.blame_file(Path::new(file_path), None)
            .context(format!("Failed to get blame for file: {}", file_path))?;

        let repo_path = self.repo.path().parent().unwrap_or(self.repo.path());
        let full_path = repo_path.join(file_path);
        let file = fs::File::open(&full_path)
            .context(format!("Failed to open file: {:?}", full_path))?;

        let reader = BufReader::new(file);
        let mut results = Vec::new();

        for (line_number, line_result) in reader.lines().enumerate() {
            let line_content = line_result?;
            let line_num = (line_number + 1) as u32;

            if let Some(hunk) = blame.get_line(line_num as usize) {
                let commit = self.repo.find_commit(hunk.final_commit_id())?;
                let author = commit.author();

                results.push(BlameLine {
                    line_number: line_num,
                    commit_hash: hunk.final_commit_id().to_string(),
                    author: author.name().unwrap_or("Unknown").to_string(),
                    author_email: author.email().unwrap_or("").to_string(),
                    date: DateTime::<Utc>::from_timestamp(commit.time().seconds(), 0)
                        .unwrap_or_default()
                        .to_rfc3339(),
                    content: line_content,
                });
            }
        }

        Ok(results)
    }
}
