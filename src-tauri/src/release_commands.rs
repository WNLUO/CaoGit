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

    // 第一步：更新所有版本文件
    // 注意：这会修改文件，所以状态检查必须在更新之前完成（但我们允许自动生成的改动）
    update_tauri_config_version(&repo_path, &config.version)
        .map_err(|e| format!("更新 tauri.conf.json 版本号失败: {}", e))?;

    update_package_json_version(&repo_path, &config.version)
        .map_err(|e| format!("更新 package.json 版本号失败: {}", e))?;

    update_cargo_toml_version(&repo_path, &config.version)
        .map_err(|e| format!("更新 Cargo.toml 版本号失败: {}", e))?;

    // 第二步：提交版本号更改
    repo.stage_file("src-tauri/tauri.conf.json")
        .map_err(|e| format!("暂存 tauri.conf.json 失败: {}", e))?;

    repo.stage_file("package.json")
        .map_err(|e| format!("暂存 package.json 失败: {}", e))?;

    repo.stage_file("src-tauri/Cargo.toml")
        .map_err(|e| format!("暂存 Cargo.toml 失败: {}", e))?;

    // 检查是否还有其他未暂存的改动（除了我们刚刚修改的配置文件）
    let status_after_stage = repo.get_status().map_err(|e| e.to_string())?;
    let has_other_changes = status_after_stage.iter().any(|item| {
        // 如果有除了已暂存的 tauri.conf.json、package.json 和 Cargo.toml 之外的其他改动
        !item.path.ends_with("tauri.conf.json")
            && !item.path.ends_with("package.json")
            && !item.path.ends_with("Cargo.toml")
    });

    if has_other_changes {
        // 回滚暂存的文件
        let _ = repo.unstage_file("src-tauri/tauri.conf.json");
        let _ = repo.unstage_file("package.json");
        let _ = repo.unstage_file("src-tauri/Cargo.toml");
        return Err("仓库有其他未提交的更改，请先提交或暂存这些更改后再发布。".to_string());
    }

    let commit_message = format!("chore: bump version to {}", config.version);
    repo.commit(&commit_message)
        .map_err(|e| format!("提交版本号更改失败: {}", e))?;

    // 第三步：创建标签
    if config.create_tag {
        println!("正在创建标签: {}", config.version);
        repo.create_tag(&config.version, Some(&config.message))
            .map_err(|e| {
                eprintln!("创建标签失败: {}", e);
                format!("创建标签失败: {}", e)
            })?;
        println!("标签创建成功");
    }

    // 第四步：推送提交和标签
    if config.push_tag {
        // 获取当前分支名称
        println!("正在获取当前分支名称...");
        let current_branch = repo.get_current_branch()
            .map_err(|e| {
                eprintln!("获取当前分支失败: {}", e);
                format!("获取当前分支失败: {}", e)
            })?;
        println!("当前分支: {}", current_branch);

        // 先推送提交
        println!("正在推送提交到 origin/{}...", current_branch);
        repo.push("origin", &current_branch)
            .map_err(|e| {
                eprintln!("推送提交失败: {}", e);
                format!("推送提交失败: {}。请确保已配置 Git 认证（SSH 密钥或凭据管理器）", e)
            })?;
        println!("提交推送成功");

        // 再推送标签到远程
        println!("正在推送标签 {} 到 origin...", config.version);
        repo.push_tag("origin", &config.version)
            .map_err(|e| {
                eprintln!("推送标签失败: {}", e);
                format!("推送标签失败: {}。请确保已配置 Git 认证（SSH 密钥或凭据管理器）", e)
            })?;
        println!("标签推送成功");
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

/// 更新 Cargo.toml 中的版本号
fn update_cargo_toml_version(repo_path: &str, version: &str) -> Result<(), String> {
    use std::fs;
    use std::path::Path;

    // 移除版本号前缀 'v'（如果有）
    let version_without_v = version.trim_start_matches('v');

    // 构建 Cargo.toml 路径
    let cargo_path = Path::new(repo_path)
        .join("src-tauri")
        .join("Cargo.toml");

    // 读取文件内容
    let content = fs::read_to_string(&cargo_path)
        .map_err(|e| format!("读取 Cargo.toml 失败: {}", e))?;

    // 使用正则表达式替换 [package] 块中的 version 字段
    // 只替换第一次出现的 version = "..." (在 [package] 块中)
    let version_regex = regex::Regex::new(r#"(?m)^version\s*=\s*"[^"]*""#)
        .map_err(|e| format!("创建正则表达式失败: {}", e))?;

    let updated_content = version_regex.replace(
        &content,
        format!(r#"version = "{}""#, version_without_v)
    );

    // 写回文件
    fs::write(&cargo_path, updated_content.as_ref())
        .map_err(|e| format!("写入 Cargo.toml 失败: {}", e))?;

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

                // 如果有更新，验证当前平台是否有可用的下载资源
                if has_update {
                    let platform = detect_current_platform();
                    // 检查是否存在可下载的资源
                    match verify_platform_asset_exists(latest_version, &platform).await {
                        Ok(true) => {
                            // 资源存在，返回更新信息
                            Ok(UpdateCheckResult {
                                success: true,
                                has_update: true,
                                current_version: current_version.to_string(),
                                latest_version: latest_version.to_string(),
                                download_url: latest.html_url.clone(),
                                released_at: latest.published_at.clone().unwrap_or_else(|| "unknown".to_string()),
                                channel: "built-in-proxy".to_string(),
                                error: None,
                            })
                        }
                        Ok(false) => {
                            // 有新版本但没有当前平台的资源，不提示更新
                            Ok(UpdateCheckResult {
                                success: true,
                                has_update: false,
                                current_version: current_version.to_string(),
                                latest_version: current_version.to_string(),
                                download_url: String::new(),
                                released_at: String::new(),
                                channel: "built-in-proxy".to_string(),
                                error: Some(format!("Latest version {} found, but no asset available for platform: {}", latest_version, platform)),
                            })
                        }
                        Err(e) => {
                            // 验证失败，为了安全起见，不提示更新
                            Ok(UpdateCheckResult {
                                success: true,
                                has_update: false,
                                current_version: current_version.to_string(),
                                latest_version: current_version.to_string(),
                                download_url: String::new(),
                                released_at: String::new(),
                                channel: "built-in-proxy".to_string(),
                                error: Some(format!("Failed to verify assets: {}", e)),
                            })
                        }
                    }
                } else {
                    // 没有更新
                    Ok(UpdateCheckResult {
                        success: true,
                        has_update: false,
                        current_version: current_version.to_string(),
                        latest_version: latest_version.to_string(),
                        download_url: latest.html_url.clone(),
                        released_at: latest.published_at.clone().unwrap_or_else(|| "unknown".to_string()),
                        channel: "built-in-proxy".to_string(),
                        error: None,
                    })
                }
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

                            // 如果有更新，验证当前平台是否有可用的下载资源
                            if has_update {
                                let platform = detect_current_platform();
                                // 检查是否存在可下载的资源
                                match verify_platform_asset_exists(latest_version, &platform).await {
                                    Ok(true) => {
                                        // 资源存在，返回更新信息
                                        Ok(UpdateCheckResult {
                                            success: true,
                                            has_update: true,
                                            current_version: current_version.to_string(),
                                            latest_version: latest_version.to_string(),
                                            download_url: html_url.to_string(),
                                            released_at: published_at.to_string(),
                                            channel: "direct-network".to_string(),
                                            error: None,
                                        })
                                    }
                                    Ok(false) => {
                                        // 有新版本但没有当前平台的资源，不提示更新
                                        Ok(UpdateCheckResult {
                                            success: true,
                                            has_update: false,
                                            current_version: current_version.to_string(),
                                            latest_version: current_version.to_string(),
                                            download_url: String::new(),
                                            released_at: String::new(),
                                            channel: "direct-network".to_string(),
                                            error: Some(format!("Latest version {} found, but no asset available for platform: {}", latest_version, platform)),
                                        })
                                    }
                                    Err(e) => {
                                        // 验证失败，为了安全起见，不提示更新
                                        Ok(UpdateCheckResult {
                                            success: true,
                                            has_update: false,
                                            current_version: current_version.to_string(),
                                            latest_version: current_version.to_string(),
                                            download_url: String::new(),
                                            released_at: String::new(),
                                            channel: "direct-network".to_string(),
                                            error: Some(format!("Failed to verify assets: {}", e)),
                                        })
                                    }
                                }
                            } else {
                                // 没有更新
                                Ok(UpdateCheckResult {
                                    success: true,
                                    has_update: false,
                                    current_version: current_version.to_string(),
                                    latest_version: latest_version.to_string(),
                                    download_url: html_url.to_string(),
                                    released_at: published_at.to_string(),
                                    channel: "direct-network".to_string(),
                                    error: None,
                                })
                            }
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

/// 检测当前运行的平台
fn detect_current_platform() -> String {
    #[cfg(target_os = "windows")]
    return "windows".to_string();

    #[cfg(target_os = "macos")]
    return "macos".to_string();

    #[cfg(target_os = "linux")]
    return "linux".to_string();

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return "unknown".to_string();
}

/// 验证指定版本的 Release 是否有当前平台的可下载资源
async fn verify_platform_asset_exists(version: &str, platform: &str) -> Result<bool, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let url = format!("https://api.github.com/repos/wnluo/caogit/releases/tags/v{}", version.trim_start_matches('v'));

    let response = client
        .get(&url)
        .header("User-Agent", "CaoGit")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch release info: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Release not found: HTTP {}", response.status()));
    }

    let data: serde_json::Value = response.json().await
        .map_err(|e| format!("Failed to parse release data: {}", e))?;

    let assets = data["assets"].as_array()
        .ok_or_else(|| "No assets found in release".to_string())?;

    // 根据平台匹配文件后缀
    let patterns = match platform {
        "windows" => vec![".msi", ".exe"],
        "macos" => vec![".dmg"],
        "linux" => vec![".AppImage", ".deb"],
        _ => return Ok(false), // 未知平台
    };

    // 检查是否存在匹配的资源
    for pattern in patterns {
        for asset in assets {
            if let Some(name) = asset["name"].as_str() {
                if name.ends_with(pattern) && !name.contains("blockmap") {
                    return Ok(true); // 找到匹配的资源
                }
            }
        }
    }

    Ok(false) // 没有找到匹配的资源
}

/// 从 Release 获取实际的下载文件
async fn get_release_asset_url(version: &str, platform: &str) -> Result<(String, String), String> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/wnluo/caogit/releases/tags/v{}", version.trim_start_matches('v'));

    let response = client
        .get(&url)
        .header("User-Agent", "CaoGit")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch release info: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Release not found: HTTP {}", response.status()));
    }

    let data: serde_json::Value = response.json().await
        .map_err(|e| format!("Failed to parse release data: {}", e))?;

    let assets = data["assets"].as_array()
        .ok_or_else(|| "No assets found in release".to_string())?;

    // 根据平台匹配文件
    let pattern = match platform {
        "windows" => vec![".msi", ".exe"], // 优先 MSI, 其次 EXE
        "macos" => vec![".dmg"],
        "linux" => vec![".AppImage", ".deb"], // 优先 AppImage, 其次 DEB
        _ => return Err(format!("Unsupported platform: {}", platform)),
    };

    for pat in pattern {
        for asset in assets {
            if let Some(name) = asset["name"].as_str() {
                if name.ends_with(pat) && !name.contains("blockmap") {
                    let download_url = asset["browser_download_url"].as_str()
                        .ok_or_else(|| "Missing download URL".to_string())?;
                    return Ok((name.to_string(), download_url.to_string()));
                }
            }
        }
    }

    Err(format!("No compatible file found for platform: {}", platform))
}

/// 安装更新（平台特定实现）
/// 仅在独立分发版本（DMG）中启用
#[cfg(feature = "auto-update")]
#[tauri::command]
pub async fn install_update(app: tauri::AppHandle, _download_url: String, platform: String, version: String) -> Result<UpdateInstallResult, String> {
    // 获取下载目录
    let download_dir = get_download_directory()?;

    // 动态获取实际的文件名和下载链接
    let (filename, actual_download_url) = get_release_asset_url(&version, &platform).await?;

    match platform.as_str() {
        "windows" => {
            let file_path = download_dir.join(&filename);

            // 下载文件
            download_update_file(&app, &actual_download_url, &file_path).await?;

            // 运行 MSI 安装程序（/passive 静默安装，/norestart 不自动重启）
            let file_path_str = file_path.to_str()
                .ok_or_else(|| "Invalid file path encoding".to_string())?;

            // 根据文件类型选择安装方式
            if filename.ends_with(".msi") {
                std::process::Command::new("msiexec")
                    .args(&["/i", file_path_str, "/passive", "/norestart"])
                    .spawn()
                    .map_err(|e| format!("启动安装程序失败: {}", e))?;
            } else if filename.ends_with(".exe") {
                std::process::Command::new(file_path_str)
                    .spawn()
                    .map_err(|e| format!("启动安装程序失败: {}", e))?;
            }

            Ok(UpdateInstallResult {
                status: "installing".to_string(),
                file_path: file_path.to_string_lossy().to_string(),
                message: "安装程序已启动，请按照提示完成安装".to_string(),
            })
        }
        "macos" => {
            let file_path = download_dir.join(&filename);

            // 下载 DMG 文件
            download_update_file(&app, &actual_download_url, &file_path).await?;

            // 自动挂载 DMG
            let output = std::process::Command::new("hdiutil")
                .args(&["attach", file_path.to_str().unwrap()])
                .output()
                .map_err(|e| format!("挂载 DMG 失败: {}", e))?;

            if !output.status.success() {
                return Err("无法挂载 DMG 文件".to_string());
            }

            // 解析挂载点路径
            let mount_output = String::from_utf8_lossy(&output.stdout);
            let mount_point = mount_output
                .lines()
                .find(|line| line.contains("/Volumes"))
                .and_then(|line| line.split_whitespace().last())
                .unwrap_or("/Volumes/CaoGit");

            // 自动打开 DMG 窗口（显示拖拽界面）
            let _ = std::process::Command::new("open")
                .arg(mount_point)
                .spawn();

            // 延迟 2 秒后自动退出当前应用
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(2));
                std::process::exit(0);
            });

            Ok(UpdateInstallResult {
                status: "ready_to_install".to_string(),
                file_path: file_path.to_string_lossy().to_string(),
                message: "DMG 已自动打开，请将 CaoGit 拖到 Applications 文件夹。\n\n应用将在 2 秒后关闭...".to_string(),
            })
        }
        "linux" => {
            let file_path = download_dir.join(&filename);

            // 下载文件
            download_update_file(&app, &actual_download_url, &file_path).await?;

            // 添加执行权限
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&file_path)
                    .map_err(|e| format!("获取文件权限失败: {}", e))?
                    .permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&file_path, perms)
                    .map_err(|e| format!("设置执行权限失败: {}", e))?;
            }

            // 打开下载目录
            let _ = opener::open(&download_dir);

            let msg = if filename.ends_with(".AppImage") {
                format!("下载完成！AppImage 已添加执行权限，可直接运行。\n\n文件位置: {}", file_path.display())
            } else {
                format!("下载完成！请使用包管理器安装 DEB 文件。\n\n文件位置: {}", file_path.display())
            };

            Ok(UpdateInstallResult {
                status: "downloaded".to_string(),
                file_path: file_path.to_string_lossy().to_string(),
                message: msg,
            })
        }
        _ => {
            Err(format!("不支持的平台: {}", platform))
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UpdateInstallResult {
    pub status: String,
    pub file_path: String,
    pub message: String,
}

/// 获取下载目录（用户的 Downloads 文件夹）
fn get_download_directory() -> Result<std::path::PathBuf, String> {
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = home::home_dir() {
            let downloads = home.join("Downloads");
            if downloads.exists() {
                return Ok(downloads);
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Some(user_profile) = std::env::var_os("USERPROFILE") {
            let downloads = std::path::PathBuf::from(user_profile).join("Downloads");
            if downloads.exists() {
                return Ok(downloads);
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(home) = home::home_dir() {
            let downloads = home.join("Downloads");
            if downloads.exists() {
                return Ok(downloads);
            }
            // 备选：使用 home 目录
            return Ok(home);
        }
    }

    // 备选：使用临时目录
    Ok(std::env::temp_dir())
}

/// 下载更新文件（支持进度回调）
async fn download_update_file(app: &tauri::AppHandle, url: &str, path: &std::path::Path) -> Result<(), String> {
    use tauri::Emitter;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(600)) // 10分钟超时
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    let response = client.get(url)
        .header("User-Agent", "CaoGit-Updater")
        .send()
        .await
        .map_err(|e| format!("下载失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("下载失败，HTTP 状态码: {}", response.status()));
    }

    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;

    // 创建文件
    let mut file = std::fs::File::create(path)
        .map_err(|e| format!("创建文件失败: {}", e))?;

    // 流式下载并报告进度
    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;
    use std::io::Write;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("读取数据失败: {}", e))?;
        file.write_all(&chunk).map_err(|e| format!("写入文件失败: {}", e))?;

        downloaded += chunk.len() as u64;

        // 发送进度事件
        if total_size > 0 {
            let progress = (downloaded as f64 / total_size as f64 * 100.0) as u32;
            let _ = app.emit("update-download-progress", DownloadProgress {
                downloaded,
                total: total_size,
                progress,
            });
        }
    }

    Ok(())
}

#[derive(Clone, serde::Serialize)]
struct DownloadProgress {
    downloaded: u64,
    total: u64,
    progress: u32,
}


/// 获取平台特定的下载 URL
#[tauri::command]
pub fn get_platform_download_url(base_url: String, version: String) -> Result<PlatformDownloadInfo, String> {
    let version_without_v = version.trim_start_matches('v');
    let download_base = base_url.replace("/tag/", "/download/");

    #[cfg(target_os = "windows")]
    {
        Ok(PlatformDownloadInfo {
            url: format!("{}/CaoGit_{}_x64-setup.msi", download_base, version_without_v),
            filename: format!("CaoGit_{}_x64-setup.msi", version_without_v),
            platform: "windows".to_string(),
        })
    }

    #[cfg(target_os = "macos")]
    {
        // 检测 CPU 架构
        let arch = if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else {
            "x64"
        };

        Ok(PlatformDownloadInfo {
            url: format!("{}/CaoGit_{}_{}.dmg", download_base, version_without_v, arch),
            filename: format!("CaoGit_{}_{}.dmg", version_without_v, arch),
            platform: "macos".to_string(),
        })
    }

    #[cfg(target_os = "linux")]
    {
        Ok(PlatformDownloadInfo {
            url: format!("{}/caogit_{}_amd64.AppImage", download_base, version_without_v),
            filename: format!("caogit_{}_amd64.AppImage", version_without_v),
            platform: "linux".to_string(),
        })
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Ok(PlatformDownloadInfo {
            url: base_url,
            filename: "unknown".to_string(),
            platform: "unknown".to_string(),
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct PlatformDownloadInfo {
    pub url: String,
    pub filename: String,
    pub platform: String,
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
