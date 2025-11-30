use git2::Repository;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use anyhow::{Context, Result};

/// Cache entry for repository instances
struct CacheEntry {
    repo: Repository,
    last_accessed: Instant,
}

/// Repository cache manager
/// Keeps repository instances in memory to avoid repeated opening
pub struct RepoCache {
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    ttl: Duration,
}

impl RepoCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            ttl: Duration::from_secs(300), // 5 minutes TTL
        }
    }

    /// Get or open a repository
    pub fn get_or_open(&self, path: &str) -> Result<Repository> {
        let mut cache = self.cache.lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire cache lock: {}", e))?;

        // Check if cached and not expired
        if let Some(entry) = cache.get_mut(path) {
            if entry.last_accessed.elapsed() < self.ttl {
                entry.last_accessed = Instant::now();
                // Clone the repository handle (lightweight operation)
                return Repository::open(path)
                    .context(format!("Failed to reopen cached repository at {}", path));
            } else {
                // Expired, remove it
                cache.remove(path);
            }
        }

        // Open fresh repository
        let repo = Repository::open(path)
            .context(format!("Failed to open repository at {}", path))?;

        // Cache it
        cache.insert(path.to_string(), CacheEntry {
            repo: Repository::open(path)?,
            last_accessed: Instant::now(),
        });

        Ok(repo)
    }

    /// Invalidate cache for a specific path
    pub fn invalidate(&self, path: &str) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.remove(path);
        }
    }

    /// Clear all cache
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    /// Clean up expired entries
    pub fn cleanup(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            let now = Instant::now();
            cache.retain(|_, entry| now.duration_since(entry.last_accessed) < self.ttl);
        }
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.lock()
            .map(|cache| cache.len())
            .unwrap_or(0)
    }
}

impl Default for RepoCache {
    fn default() -> Self {
        Self::new()
    }
}

// Global cache instance
lazy_static::lazy_static! {
    pub static ref REPO_CACHE: RepoCache = RepoCache::new();
}
