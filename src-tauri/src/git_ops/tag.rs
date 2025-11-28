//! Tag operations
//!
//! This module handles all tag-related Git operations.

use anyhow::Result;

use super::repository::GitRepository;
use super::types::TagInfo;

impl GitRepository {
    /// Create a new tag
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

    /// Get all tags
    pub fn get_tags(&self) -> Result<Vec<TagInfo>> {
        let tag_names = self.repo.tag_names(None)?;
        let mut tags = Vec::new();

        for name in tag_names.iter() {
            if let Some(name) = name {
                if let Ok(obj) = self.repo.revparse_single(name) {
                    let tag_info = TagInfo {
                        name: name.to_string(),
                        target: obj.id().to_string(),
                        message: None,
                    };
                    tags.push(tag_info);
                }
            }
        }

        Ok(tags)
    }

    /// Delete a tag
    pub fn delete_tag(&self, tag_name: &str) -> Result<()> {
        self.repo.tag_delete(tag_name)?;
        Ok(())
    }
}
