use serde::{Deserialize, Serialize};

/// App Store 版本的更新检查结果
#[derive(Debug, Serialize, Deserialize)]
pub struct AppStoreUpdateInfo {
    pub has_update: bool,
    pub current_version: String,
    pub latest_version: String,
    pub update_message: String,
}

/// App Store 版本的更新检查
/// 只检查是否有新版本，不提供下载功能
#[tauri::command]
pub async fn check_update_appstore(
    repo_path: String,
    github_token: Option<String>,
) -> Result<AppStoreUpdateInfo, String> {
    // 使用硬编码的仓库信息检查应用更新
    // App Store 版本不需要从本地仓库获取信息，因为我们在检查应用本身的更新
    use crate::github_api::GitHubClient;

    // CaoGit 应用的 GitHub 仓库信息（硬编码）
    let owner = "wnluo";
    let repo_name = "caogit";

    // repo_path 参数在此不使用，但保留以保持接口一致性
    let _ = repo_path;

    // 获取当前版本
    let current_version = env!("CARGO_PKG_VERSION").to_string();

    // 创建 GitHub 客户端
    let client = GitHubClient::new(github_token);

    // 获取最新 Release
    let releases = client
        .list_releases(&owner, &repo_name)
        .await
        .map_err(|e| format!("Failed to fetch releases: {}", e))?;

    if let Some(latest_release) = releases.first() {
        let latest_version = latest_release.tag_name.trim_start_matches('v');
        let has_update = latest_version != current_version;

        let update_message = if has_update {
            format!(
                "发现新版本 {}！\n\n请前往 Mac App Store 更新应用以获取最新功能和改进。",
                latest_version
            )
        } else {
            "您正在使用最新版本！".to_string()
        };

        Ok(AppStoreUpdateInfo {
            has_update,
            current_version: current_version.to_string(),
            latest_version: latest_version.to_string(),
            update_message,
        })
    } else {
        Ok(AppStoreUpdateInfo {
            has_update: false,
            current_version: current_version.to_string(),
            latest_version: current_version.to_string(),
            update_message: "当前版本已是最新".to_string(),
        })
    }
}

/// 打开 Mac App Store 中的应用页面
#[tauri::command]
pub fn open_app_store() -> Result<(), String> {
    // CaoGit 在 App Store 的 URL（上架后替换为实际 URL）
    let app_store_url = "macappstore://apps.apple.com/app/id6755988654";

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .arg(app_store_url)
            .spawn()
            .map_err(|e| format!("Failed to open App Store: {}", e))?;
    }

    Ok(())
}
