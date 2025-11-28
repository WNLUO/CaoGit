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

    // 第一步：更新 tauri.conf.json 和 package.json 中的版本号
    update_tauri_config_version(&repo_path, &config.version)
        .map_err(|e| format!("更新 tauri.conf.json 版本号失败: {}", e))?;

    update_package_json_version(&repo_path, &config.version)
        .map_err(|e| format!("更新 package.json 版本号失败: {}", e))?;

    // 第二步：提交版本号更改
    repo.stage_file("src-tauri/tauri.conf.json")
        .map_err(|e| format!("暂存 tauri.conf.json 失败: {}", e))?;

    repo.stage_file("package.json")
        .map_err(|e| format!("暂存 package.json 失败: {}", e))?;

    let commit_message = format!("chore: bump version to {}", config.version);
    repo.commit(&commit_message)
        .map_err(|e| format!("提交版本号更改失败: {}", e))?;

    // 第三步：创建标签
    if config.create_tag {
        repo.create_tag(&config.version, Some(&config.message))
            .map_err(|e| format!("创建标签失败: {}", e))?;
    }

    // 第四步：推送提交和标签
    if config.push_tag {
        // 获取当前分支名称
        let current_branch = repo.get_current_branch()
            .map_err(|e| format!("获取当前分支失败: {}", e))?;

        // 先推送提交
        repo.push("origin", &current_branch)
            .map_err(|e| format!("推送提交失败: {}", e))?;

        // 再推送标签到远程
        repo.push_tag("origin", &config.version)
            .map_err(|e| format!("推送标签失败: {}", e))?;
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

/// 更新 tauri.conf.json 中的版本号
fn update_tauri_config_version(repo_path: &str, version: &str) -> Result<(), String> {
    use std::fs;
    use std::path::Path;

    // 移除版本号前缀 'v'（如果有）
    let version_without_v = version.trim_start_matches('v');

    // 构建 tauri.conf.json 路径
    let config_path = Path::new(repo_path)
        .join("src-tauri")
        .join("tauri.conf.json");

    // 读取文件内容
    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("读取 tauri.conf.json 失败: {}", e))?;

    // 解析 JSON
    let mut config: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析 tauri.conf.json 失败: {}", e))?;

    // 更新版本号
    if let Some(obj) = config.as_object_mut() {
        obj.insert("version".to_string(), serde_json::Value::String(version_without_v.to_string()));
    } else {
        return Err("tauri.conf.json 格式无效".to_string());
    }

    // 写回文件（保持格式化）
    let updated_content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("序列化 JSON 失败: {}", e))?;

    fs::write(&config_path, updated_content)
        .map_err(|e| format!("写入 tauri.conf.json 失败: {}", e))?;

    Ok(())
}

/// 更新 package.json 中的版本号
fn update_package_json_version(repo_path: &str, version: &str) -> Result<(), String> {
    use std::fs;
    use std::path::Path;

    // 移除版本号前缀 'v'（如果有）
    let version_without_v = version.trim_start_matches('v');

    // 构建 package.json 路径
    let package_path = Path::new(repo_path).join("package.json");

    // 读取文件内容
    let content = fs::read_to_string(&package_path)
        .map_err(|e| format!("读取 package.json 失败: {}", e))?;

    // 解析 JSON
    let mut package: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析 package.json 失败: {}", e))?;

    // 更新版本号
    if let Some(obj) = package.as_object_mut() {
        obj.insert("version".to_string(), serde_json::Value::String(version_without_v.to_string()));
    } else {
        return Err("package.json 格式无效".to_string());
    }

    // 写回文件（保持格式化，使用 2 空格缩进）
    let updated_content = serde_json::to_string_pretty(&package)
        .map_err(|e| format!("序列化 JSON 失败: {}", e))?;

    fs::write(&package_path, updated_content)
        .map_err(|e| format!("写入 package.json 失败: {}", e))?;

    Ok(())
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

/// 生成发布说明
#[tauri::command]
pub fn generate_release_notes(
    repo_path: String,
    from_version: String,
    to_version: String,
) -> Result<String, String> {
    let repo = GitRepository::open(&repo_path).map_err(|e| e.to_string())?;

    // 如果 to_version 为空或者是新版本，使用 HEAD
    let to_ref = if to_version.is_empty() || to_version.starts_with("v0.") {
        "HEAD"
    } else {
        &to_version
    };

    // 获取两个版本之间的提交记录
    let commits = repo
        .get_commits_between(&from_version, to_ref)
        .map_err(|e| format!("获取提交记录失败: {}", e))?;

    if commits.is_empty() {
        return Ok("没有新的提交记录".to_string());
    }

    // 按照 Conventional Commits 规范分类提交
    let mut features = Vec::new();
    let mut fixes = Vec::new();
    let mut docs = Vec::new();
    let mut refactors = Vec::new();
    let mut perfs = Vec::new();
    let mut tests = Vec::new();
    let mut chores = Vec::new();
    let mut others = Vec::new();

    for commit in commits {
        let message = commit.message.trim();
        let first_line = message.lines().next().unwrap_or("");

        // 移除所有反引号（包括 markdown 代码块标记）和多余的空格
        let cleaned_line = first_line
            .chars()
            .filter(|c| *c != '`')
            .collect::<String>()
            .trim()
            .to_string();

        // 解析提交类型
        if let Some(colon_pos) = cleaned_line.find(':') {
            let prefix = &cleaned_line[..colon_pos].trim();
            let description = cleaned_line[colon_pos + 1..].trim();

            // 处理 scope，例如 "feat(ui): xxx" -> "feat"
            let commit_type = if let Some(paren_pos) = prefix.find('(') {
                &prefix[..paren_pos]
            } else {
                prefix
            };

            match commit_type {
                "feat" | "feature" => features.push(description.to_string()),
                "fix" => fixes.push(description.to_string()),
                "docs" => docs.push(description.to_string()),
                "refactor" => refactors.push(description.to_string()),
                "perf" => perfs.push(description.to_string()),
                "test" => tests.push(description.to_string()),
                "chore" | "build" | "ci" => chores.push(description.to_string()),
                _ => others.push(cleaned_line.clone()),
            }
        } else {
            others.push(cleaned_line);
        }
    }

    // 生成 Markdown 格式的发布说明
    let mut notes = String::new();

    if !features.is_empty() {
        notes.push_str("## ✨ 新功能\n\n");
        for feat in features {
            notes.push_str(&format!("- {}\n", feat));
        }
        notes.push('\n');
    }

    if !fixes.is_empty() {
        notes.push_str("## 🐛 Bug 修复\n\n");
        for fix in fixes {
            notes.push_str(&format!("- {}\n", fix));
        }
        notes.push('\n');
    }

    if !perfs.is_empty() {
        notes.push_str("## ⚡ 性能优化\n\n");
        for perf in perfs {
            notes.push_str(&format!("- {}\n", perf));
        }
        notes.push('\n');
    }

    if !refactors.is_empty() {
        notes.push_str("## ♻️ 代码重构\n\n");
        for refactor in refactors {
            notes.push_str(&format!("- {}\n", refactor));
        }
        notes.push('\n');
    }

    if !docs.is_empty() {
        notes.push_str("## 📝 文档更新\n\n");
        for doc in docs {
            notes.push_str(&format!("- {}\n", doc));
        }
        notes.push('\n');
    }

    if !tests.is_empty() {
        notes.push_str("## ✅ 测试\n\n");
        for test in tests {
            notes.push_str(&format!("- {}\n", test));
        }
        notes.push('\n');
    }

    if !chores.is_empty() {
        notes.push_str("## 🔧 其他改动\n\n");
        for chore in chores {
            notes.push_str(&format!("- {}\n", chore));
        }
        notes.push('\n');
    }

    if !others.is_empty() {
        notes.push_str("## 📋 其他提交\n\n");
        for other in others {
            notes.push_str(&format!("- {}\n", other));
        }
        notes.push('\n');
    }

    Ok(notes.trim().to_string())
}

/// 检查更新（双通道）
#[tauri::command]
pub async fn check_for_updates(github_token: Option<String>) -> Result<UpdateCheckResult, String> {
    let current_version = env!("CARGO_PKG_VERSION");

    // 尝试通道 1：使用软件内置的网络代理
    if let Ok(result) = check_updates_channel1(current_version, github_token.as_deref()).await {
        return Ok(result);
    }

    // 如果通道 1 失败，尝试通道 2：本地网络直连
    check_updates_channel2(current_version).await
}

/// 通道 1：使用 Tauri API（支持网络代理）
async fn check_updates_channel1(
    current_version: &str,
    github_token: Option<&str>,
) -> Result<UpdateCheckResult, String> {
    let client = crate::github_api::GitHubClient::new(github_token.map(String::from));

    match client
        .list_releases("wnluo", "caogit")
        .await
    {
        Ok(releases) => {
            if let Some(latest) = releases.first() {
                let latest_version = latest.tag_name.trim_start_matches('v');

                let has_update = compare_versions(latest_version, current_version);

                Ok(UpdateCheckResult {
                    success: true,
                    has_update,
                    current_version: current_version.to_string(),
                    latest_version: latest_version.to_string(),
                    download_url: latest.html_url.clone(),
                    released_at: latest.published_at.clone().unwrap_or_else(|| "unknown".to_string()),
                    channel: "built-in-proxy".to_string(),
                    error: None,
                })
            } else {
                Err("No releases found".to_string())
            }
        }
        Err(e) => Err(format!("Channel 1 failed: {}", e)),
    }
}

/// 通道 2：本地网络直连
async fn check_updates_channel2(current_version: &str) -> Result<UpdateCheckResult, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let url = "https://api.github.com/repos/wnluo/caogit/releases/latest";

    match client.get(url).header("User-Agent", "GitManager").send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => {
                        if let (Some(tag), Some(html_url), Some(published_at)) = (
                            data.get("tag_name").and_then(|v| v.as_str()),
                            data.get("html_url").and_then(|v| v.as_str()),
                            data.get("published_at").and_then(|v| v.as_str()),
                        ) {
                            let latest_version = tag.trim_start_matches('v');
                            let has_update = compare_versions(latest_version, current_version);

                            Ok(UpdateCheckResult {
                                success: true,
                                has_update,
                                current_version: current_version.to_string(),
                                latest_version: latest_version.to_string(),
                                download_url: html_url.to_string(),
                                released_at: published_at.to_string(),
                                channel: "direct-network".to_string(),
                                error: None,
                            })
                        } else {
                            Err("Missing required fields in response".to_string())
                        }
                    }
                    Err(e) => Err(format!("Failed to parse response: {}", e)),
                }
            } else {
                Err(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Channel 2 failed: {}", e)),
    }
}

/// 比较版本号（简单的语义化版本比较）
fn compare_versions(latest: &str, current: &str) -> bool {
    let parse_version = |v: &str| -> (u32, u32, u32) {
        let parts: Vec<&str> = v.split('.').collect();
        (
            parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0),
            parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
            parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0),
        )
    };

    let (latest_major, latest_minor, latest_patch) = parse_version(latest);
    let (current_major, current_minor, current_patch) = parse_version(current);

    (latest_major, latest_minor, latest_patch) > (current_major, current_minor, current_patch)
}

#[derive(Debug, serde::Serialize)]
pub struct UpdateCheckResult {
    pub success: bool,
    pub has_update: bool,
    pub current_version: String,
    pub latest_version: String,
    pub download_url: String,
    pub released_at: String,
    pub channel: String,
    pub error: Option<String>,
}

/// 安装更新
#[tauri::command]
pub async fn install_update() -> Result<(), String> {
    // This command would be called to start the update installation
    // The actual update installation is handled by tauri-plugin-updater
    Ok(())
}

/// 重启应用
#[tauri::command]
pub fn restart_app(_app: tauri::AppHandle) -> Result<(), String> {
    // 使用 process 插件的命令行方式重启
    // 或者使用 Tauri 的重启 API
    std::process::exit(0);
}

/// 退出应用
#[tauri::command]
pub fn exit_app() -> Result<(), String> {
    std::process::exit(0);
}
