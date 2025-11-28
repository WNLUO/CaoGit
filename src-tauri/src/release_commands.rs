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
#[serde(rename_all = "camelCase")]
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
    _github_token: Option<String>,
) -> Result<String, String> {
    let repo = GitRepository::open(&repo_path).map_err(|e| e.to_string())?;

    // 检查是否有未提交的改动
    let status = repo.get_status().map_err(|e| e.to_string())?;
    if !status.is_empty() {
        return Err("仓库有未提交的更改，请先提交或暂存这些更改。".to_string());
    }

    // 创建标签
    if config.create_tag {
        repo.create_tag(&config.version, Some(&config.message))
            .map_err(|e| format!("Failed to create tag: {}", e))?;
    }

    // 推送标签
    if config.push_tag {
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
        return Err("版本号格式无效，期望格式: X.Y.Z".to_string());
    }

    let mut major: u32 = parts[0].parse().map_err(|_| "主版本号无效")?;
    let mut minor: u32 = parts[1].parse().map_err(|_| "次版本号无效")?;
    let mut patch: u32 = parts[2].parse().map_err(|_| "补丁版本号无效")?;

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
        _ => return Err("递增类型无效，必须是: major, minor 或 patch".to_string()),
    }

    Ok(format!("v{}.{}.{}", major, minor, patch))
}

/// 读取当前版本号（优先从 Git 标签，否则从 tauri.conf.json）
fn get_current_version(repo_path: &str) -> Result<String> {
    // 尝试从 Git 仓库获取最新标签
    if let Ok(version) = get_latest_git_tag(repo_path) {
        return Ok(version);
    }

    // 如果没有 Git 标签，从 tauri.conf.json 读取
    let config_path = std::path::Path::new(repo_path)
        .join("src-tauri")
        .join("tauri.conf.json");

    let content = std::fs::read_to_string(config_path).context("读取 tauri.conf.json 失败")?;

    let config: serde_json::Value =
        serde_json::from_str(&content).context("解析 tauri.conf.json 失败")?;

    let version = config
        .get("version")
        .and_then(|v| v.as_str())
        .context("在 tauri.conf.json 中未找到版本号")?;

    Ok(format!("v{}", version))
}

/// 从 Git 仓库获取最新的版本标签
fn get_latest_git_tag(repo_path: &str) -> Result<String> {
    use git2::Repository;

    let repo = Repository::open(repo_path).context("打开仓库失败")?;

    // 获取所有标签
    let tags = repo.tag_names(None)?;

    // 过滤出版本号格式的标签 (v开头的)
    let mut version_tags: Vec<String> = tags
        .iter()
        .flatten()
        .filter(|tag| tag.starts_with('v'))
        .map(|s| s.to_string())
        .collect();

    if version_tags.is_empty() {
        anyhow::bail!("未找到版本标签");
    }

    // 按版本号排序（简单的字符串排序，对于标准的 semver 格式足够用）
    version_tags.sort_by(|a, b| {
        // 移除 'v' 前缀并按版本号比较
        let parse_version = |v: &str| -> (u32, u32, u32) {
            let without_v = v.trim_start_matches('v');
            let parts: Vec<&str> = without_v.split('.').collect();
            (
                parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0),
                parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
                parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0),
            )
        };

        let ver_a = parse_version(a);
        let ver_b = parse_version(b);
        ver_a.cmp(&ver_b)
    });

    // 返回最新的标签
    version_tags.last()
        .cloned()
        .context("获取最新标签失败")
}
