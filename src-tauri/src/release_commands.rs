use crate::git_ops::GitRepository;
use crate::github_api::{GitHubClient, GitHubRelease, WorkflowRun};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseInfo {
    pub current_version: String,
    pub releases: Vec<GitHubRelease>,
    pub workflow_runs: Vec<WorkflowRun>,
    pub repo_info: RepoInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoInfo {
    pub owner: String,
    pub repo: String,
    pub remote_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishConfig {
    pub version: String,
    pub message: String,
    pub create_tag: bool,
    pub push_tag: bool,
}

/// 获取发布信息（Releases 和 Workflow Runs）
#[tauri::command]
pub async fn get_release_info(
    repo_path: String,
    github_token: Option<String>,
) -> Result<ReleaseInfo, String> {
    // 打开仓库
    let repo = GitRepository::open(&repo_path).map_err(|e| e.to_string())?;

    // 获取远程 URL
    let remote_url = repo
        .get_remote_url("origin")
        .map_err(|e| format!("Failed to get remote URL: {}", e))?;

    // 解析仓库信息
    let (owner, repo_name) = GitHubClient::parse_repo_url(&remote_url)
        .map_err(|e| format!("Not a GitHub repository: {}", e))?;

    // 获取当前版本（从 tauri.conf.json）
    let current_version = get_current_version(&repo_path).unwrap_or_else(|_| "0.1.0".to_string());

    // 创建 GitHub 客户端
    let client = GitHubClient::new(github_token);

    // 获取 Releases
    let releases = client
        .list_releases(&owner, &repo_name)
        .await
        .map_err(|e| format!("Failed to fetch releases: {}", e))?;

    // 获取 Workflow Runs
    let workflow_runs = client
        .list_workflow_runs(&owner, &repo_name)
        .await
        .map_err(|e| format!("Failed to fetch workflow runs: {}", e))?;

    Ok(ReleaseInfo {
        current_version,
        releases,
        workflow_runs,
        repo_info: RepoInfo {
            owner,
            repo: repo_name,
            remote_url,
        },
    })
}

/// 发布新版本
#[tauri::command]
pub async fn publish_new_release(
    repo_path: String,
    config: PublishConfig,
    github_token: Option<String>,
) -> Result<String, String> {
    let repo = GitRepository::open(&repo_path).map_err(|e| e.to_string())?;

    // 检查是否有未提交的改动
    let status = repo.get_status().map_err(|e| e.to_string())?;
    if !status.is_clean() {
        return Err("Repository has uncommitted changes. Please commit or stash them first.".to_string());
    }

    // 创建标签
    if config.create_tag {
        repo.create_tag(&config.version, &config.message)
            .map_err(|e| format!("Failed to create tag: {}", e))?;
    }

    // 推送标签
    if config.push_tag {
        // 获取当前分支
        let current_branch = repo
            .get_current_branch()
            .map_err(|e| e.to_string())?
            .unwrap_or_else(|| "master".to_string());

        // 推送标签到远程
        repo.push("origin", &format!("refs/tags/{}", config.version))
            .map_err(|e| format!("Failed to push tag: {}", e))?;
    }

    // 获取远程 URL 并返回 Actions 链接
    let remote_url = repo.get_remote_url("origin").map_err(|e| e.to_string())?;
    let (owner, repo_name) = GitHubClient::parse_repo_url(&remote_url)
        .map_err(|e| format!("Failed to parse repo URL: {}", e))?;

    Ok(format!(
        "https://github.com/{}/{}/actions",
        owner, repo_name
    ))
}

/// 重新运行失败的构建
#[tauri::command]
pub async fn rerun_failed_build(
    repo_path: String,
    run_id: u64,
    github_token: String,
) -> Result<(), String> {
    let repo = GitRepository::open(&repo_path).map_err(|e| e.to_string())?;
    let remote_url = repo.get_remote_url("origin").map_err(|e| e.to_string())?;

    let (owner, repo_name) = GitHubClient::parse_repo_url(&remote_url)
        .map_err(|e| format!("Failed to parse repo URL: {}", e))?;

    let client = GitHubClient::new(Some(github_token));

    client
        .rerun_workflow(&owner, &repo_name, run_id)
        .await
        .map_err(|e| format!("Failed to rerun workflow: {}", e))?;

    Ok(())
}

/// 自动递增版本号
#[tauri::command]
pub fn increment_version(version: String, part: String) -> Result<String, String> {
    let version = version.trim_start_matches('v');
    let parts: Vec<&str> = version.split('.').collect();

    if parts.len() != 3 {
        return Err("Invalid version format. Expected: X.Y.Z".to_string());
    }

    let mut major: u32 = parts[0].parse().map_err(|_| "Invalid major version")?;
    let mut minor: u32 = parts[1].parse().map_err(|_| "Invalid minor version")?;
    let mut patch: u32 = parts[2].parse().map_err(|_| "Invalid patch version")?;

    match part.as_str() {
        "major" => {
            major += 1;
            minor = 0;
            patch = 0;
        }
        "minor" => {
            minor += 1;
            patch = 0;
        }
        "patch" => {
            patch += 1;
        }
        _ => return Err("Invalid part. Must be: major, minor, or patch".to_string()),
    }

    Ok(format!("v{}.{}.{}", major, minor, patch))
}

/// 读取当前版本号（从 tauri.conf.json）
fn get_current_version(repo_path: &str) -> Result<String> {
    let config_path = std::path::Path::new(repo_path)
        .join("src-tauri")
        .join("tauri.conf.json");

    let content = std::fs::read_to_string(config_path).context("Failed to read tauri.conf.json")?;

    let config: serde_json::Value =
        serde_json::from_str(&content).context("Failed to parse tauri.conf.json")?;

    let version = config
        .get("version")
        .and_then(|v| v.as_str())
        .context("Version not found in tauri.conf.json")?;

    Ok(format!("v{}", version))
}
