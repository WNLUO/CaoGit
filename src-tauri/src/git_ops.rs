use git2::{
    BranchType, DiffOptions, Repository, Status, StatusOptions,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub status: String,
    pub staged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub date: String,
    pub parents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub is_head: bool,
    pub is_remote: bool,
    pub upstream: Option<String>,
    pub last_commit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteInfo {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagInfo {
    pub name: String,
    pub target: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StashInfo {
    pub index: usize,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    pub old_path: String,
    pub new_path: String,
    pub status: String,
    pub hunks: Vec<DiffHunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub header: String,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub origin: char,
    pub content: String,
    pub old_lineno: Option<u32>,
    pub new_lineno: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlameLine {
    pub line_number: u32,
    pub commit_hash: String,
    pub author: String,
    pub author_email: String,
    pub date: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub path: String,
    pub ours: String,
    pub theirs: String,
    pub base: Option<String>,
}

pub struct GitRepository {
    repo: Repository,
}

impl GitRepository {
    pub fn open(path: &str) -> Result<Self> {
        let repo = Repository::open(path)
            .context(format!("Failed to open repository at {}", path))?;
        Ok(Self { repo })
    }

    pub fn init(path: &str) -> Result<Self> {
        let repo = Repository::init(path)
            .context(format!("Failed to initialize repository at {}", path))?;
        Ok(Self { repo })
    }

    pub fn clone(url: &str, path: &str) -> Result<Self> {
        let repo = Repository::clone(url, path)
            .context(format!("Failed to clone repository from {}", url))?;
        Ok(Self { repo })
    }

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

    pub fn stage_file(&self, path: &str) -> Result<()> {
        let mut index = self.repo.index()?;
        index.add_path(Path::new(path))?;
        index.write()?;
        Ok(())
    }

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

    pub fn get_commits(&self, max_count: usize) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;

        // Try to push HEAD, if it fails (no commits yet), return empty list
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

    pub fn get_branches(&self) -> Result<Vec<BranchInfo>> {
        let mut branches = Vec::new();

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

    pub fn create_branch(&self, name: &str) -> Result<()> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        self.repo.branch(name, &commit, false)?;
        Ok(())
    }

    pub fn checkout_branch(&self, name: &str) -> Result<()> {
        let obj = self.repo.revparse_single(&format!("refs/heads/{}", name))?;
        self.repo.checkout_tree(&obj, None)?;
        self.repo.set_head(&format!("refs/heads/{}", name))?;
        Ok(())
    }

    pub fn delete_branch(&self, name: &str) -> Result<()> {
        let mut branch = self.repo.find_branch(name, BranchType::Local)?;
        branch.delete()?;
        Ok(())
    }

    pub fn get_current_branch(&self) -> Result<String> {
        match self.repo.head() {
            Ok(head) => Ok(head.shorthand().unwrap_or("HEAD").to_string()),
            Err(_) => {
                // For new repos with no commits, return "main" as default
                Ok("main".to_string())
            }
        }
    }

    pub fn fetch(&self, remote_name: &str) -> Result<()> {
        let mut remote = self.repo.find_remote(remote_name)?;
        remote.fetch(&[] as &[&str], None, None)?;
        Ok(())
    }

    pub fn pull(&self, remote_name: &str, branch_name: &str) -> Result<()> {
        // Fetch first
        self.fetch(remote_name)?;

        // Get the remote branch
        let fetch_head = self.repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = self.repo.reference_to_annotated_commit(&fetch_head)?;

        // Perform merge analysis
        let (analysis, _) = self.repo.merge_analysis(&[&fetch_commit])?;

        if analysis.is_up_to_date() {
            return Ok(());
        } else if analysis.is_fast_forward() {
            // Fast-forward merge
            let refname = format!("refs/heads/{}", branch_name);
            let mut reference = self.repo.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            self.repo.set_head(&refname)?;
            self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else {
            // Need to merge
            self.repo.merge(&[&fetch_commit], None, None)?;
        }

        Ok(())
    }

    pub fn push(&self, remote_name: &str, branch_name: &str) -> Result<()> {
        let mut remote = self.repo.find_remote(remote_name)?;
        let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
        remote.push(&[&refspec], None)?;
        Ok(())
    }

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

    pub fn add_remote(&self, name: &str, url: &str) -> Result<()> {
        self.repo.remote(name, url)?;
        Ok(())
    }

    pub fn remove_remote(&self, name: &str) -> Result<()> {
        self.repo.remote_delete(name)?;
        Ok(())
    }

    // Merge operations
    pub fn merge(&self, branch_name: &str) -> Result<String> {
        let (their_commit, _) = self.repo.revparse_ext(branch_name)?;
        let annotated_commit = self.repo.find_annotated_commit(their_commit.id())?;

        let (analysis, _) = self.repo.merge_analysis(&[&annotated_commit])?;

        if analysis.is_up_to_date() {
            return Ok("Already up-to-date".to_string());
        } else if analysis.is_fast_forward() {
            // Fast-forward merge
            let refname = format!("refs/heads/{}", self.get_current_branch()?);
            let mut reference = self.repo.find_reference(&refname)?;
            reference.set_target(annotated_commit.id(), "Fast-forward merge")?;
            self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            return Ok("Fast-forward merge completed".to_string());
        } else {
            // Normal merge
            self.repo.merge(&[&annotated_commit], None, None)?;

            // Check for conflicts
            let mut index = self.repo.index()?;
            if index.has_conflicts() {
                return Ok("Merge has conflicts - needs resolution".to_string());
            }

            // Auto-commit if no conflicts
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

    // Stash operations
    pub fn stash_save(&mut self, message: Option<&str>) -> Result<()> {
        let signature = self.repo.signature()?;
        let msg = message.unwrap_or("WIP");
        self.repo.stash_save(&signature, msg, None)?;
        Ok(())
    }

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

    pub fn stash_pop(&mut self, index: usize) -> Result<()> {
        self.repo.stash_pop(index, None)?;
        Ok(())
    }

    pub fn stash_drop(&mut self, index: usize) -> Result<()> {
        self.repo.stash_drop(index)?;
        Ok(())
    }

    // Tag operations
    pub fn create_tag(&self, tag_name: &str, message: Option<&str>) -> Result<()> {
        let head = self.repo.head()?;
        let target = head.peel_to_commit()?;

        if let Some(msg) = message {
            let signature = self.repo.signature()?;
            self.repo.tag(tag_name, target.as_object(), &signature, msg, false)?;
        } else {
            self.repo.tag_lightweight(tag_name, target.as_object(), false)?;
        }

        Ok(())
    }

    pub fn get_tags(&self) -> Result<Vec<TagInfo>> {
        let tag_names = self.repo.tag_names(None)?;
        let mut tags = Vec::new();

        for name in tag_names.iter() {
            if let Some(name) = name {
                if let Ok(obj) = self.repo.revparse_single(name) {
                    let tag_info = TagInfo {
                        name: name.to_string(),
                        target: obj.id().to_string(),
                        message: None, // Could be extended to fetch tag message
                    };
                    tags.push(tag_info);
                }
            }
        }

        Ok(tags)
    }

    pub fn delete_tag(&self, tag_name: &str) -> Result<()> {
        self.repo.tag_delete(tag_name)?;
        Ok(())
    }

    // Get detailed file diff
    pub fn get_file_diff(&self, file_path: &str, staged: bool) -> Result<DiffResult> {
        let mut diff_opts = DiffOptions::new();
        diff_opts.pathspec(file_path);

        let diff = if staged {
            // Diff between HEAD and index (staged changes)
            let head = self.repo.head()?.peel_to_tree()?;
            self.repo.diff_tree_to_index(Some(&head), None, Some(&mut diff_opts))?
        } else {
            // Diff between index and working directory (unstaged changes)
            self.repo.diff_index_to_workdir(None, Some(&mut diff_opts))?
        };

        let mut result = DiffResult {
            old_path: file_path.to_string(),
            new_path: file_path.to_string(),
            status: "Modified".to_string(),
            hunks: Vec::new(),
        };

        // Use print to get diff as text, then parse it
        // For now, create a simple single-hunk result
        use std::cell::RefCell;
        use std::rc::Rc;

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

        // Drop the clone to ensure only one reference remains
        drop(hunks_clone);
        result.hunks = Rc::try_unwrap(hunks_ref)
            .map_err(|_| git2::Error::from_str("Failed to unwrap hunks reference"))?
            .into_inner();
        Ok(result)
    }

    pub fn blame_file(&self, file_path: &str) -> Result<Vec<BlameLine>> {
        use std::fs;
        use std::io::{BufRead, BufReader};

        // Get blame for the file
        let blame = self.repo.blame_file(Path::new(file_path), None)
            .context(format!("Failed to get blame for file: {}", file_path))?;

        // Read file content
        let repo_path = self.repo.path().parent().unwrap_or(self.repo.path());
        let full_path = repo_path.join(file_path);
        let file = fs::File::open(&full_path)
            .context(format!("Failed to open file: {:?}", full_path))?;

        let reader = BufReader::new(file);
        let mut results = Vec::new();

        for (line_number, line_result) in reader.lines().enumerate() {
            let line_content = line_result?;
            let line_num = (line_number + 1) as u32;

            // Get blame hunk for this line
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

    // Cherry-pick operations
    pub fn cherry_pick(&self, commit_hash: &str) -> Result<String> {
        let oid = git2::Oid::from_str(commit_hash)
            .context(format!("Invalid commit hash: {}", commit_hash))?;
        let commit = self.repo.find_commit(oid)
            .context(format!("Failed to find commit: {}", commit_hash))?;

        let mut opts = git2::CherrypickOptions::new();
        self.repo.cherrypick(&commit, Some(&mut opts))
            .context("Cherry-pick failed")?;

        // Check for conflicts
        let mut index = self.repo.index()?;
        if index.has_conflicts() {
            return Ok("Cherry-pick has conflicts - needs resolution".to_string());
        }

        // Create commit if no conflicts
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

    pub fn cherry_pick_batch(&self, commit_hashes: Vec<String>) -> Result<Vec<String>> {
        let mut results = Vec::new();

        for hash in commit_hashes {
            match self.cherry_pick(&hash) {
                Ok(msg) => results.push(format!("{}: {}", &hash[..7], msg)),
                Err(e) => {
                    results.push(format!("{}: Failed - {}", &hash[..7], e));
                    // Stop on first error to prevent cascading failures
                    break;
                }
            }
        }

        Ok(results)
    }

    // Conflict resolution operations
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

            let repo_path = self.repo.path().parent().unwrap_or(self.repo.path());
            let full_path = repo_path.join(&path);

            // Read current conflicted file content
            let mut file_content = String::new();
            if let Ok(mut file) = fs::File::open(&full_path) {
                let _ = file.read_to_string(&mut file_content);
            }

            // Try to get ours, theirs, and base versions
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

    pub fn resolve_conflict(&self, path: &str, resolution: &str) -> Result<()> {
        use std::fs;
        use std::io::Write;

        // Write the resolution to the file
        let repo_path = self.repo.path().parent().unwrap_or(self.repo.path());
        let full_path = repo_path.join(path);

        let mut file = fs::File::create(&full_path)
            .context(format!("Failed to open file for writing: {}", path))?;
        file.write_all(resolution.as_bytes())
            .context("Failed to write resolution")?;

        // Stage the resolved file
        self.stage_file(path)?;

        Ok(())
    }

    pub fn abort_merge(&self) -> Result<()> {
        let head = self.repo.head()?.peel_to_commit()?;
        self.repo.reset(head.as_object(), git2::ResetType::Hard, None)?;

        // Clean up merge state files
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
