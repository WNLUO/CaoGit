mod git_ops;
mod commands;
mod github_api;
mod release_commands;
mod keychain;

#[cfg(feature = "appstore")]
mod appstore_update;

use commands::*;
use release_commands::*;

#[cfg(feature = "appstore")]
use appstore_update::*;

use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            // App Store 版本禁用自动更新检查
            // App Store 通过其自身机制处理应用更新，不允许应用内更新检查
            #[cfg(not(feature = "appstore"))]
            {
                let handle = app.handle().clone();

                // 延迟 2 秒后检查更新，避免阻塞启动
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(2));

                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async {
                                // 使用自定义的更新检查函数
                                match check_for_updates(None).await {
                                    Ok(result) if result.has_update => {
                                        // 有新版本，通知前端
                                        let _ = handle.emit("update-available", ());
                                    }
                                    _ => {
                                        // 已是最新版本或检查失败，静默处理
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Failed to create tokio runtime for update check: {}", e);
                        }
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Git 基础操作
            open_repository,
            get_repository_status,
            stage_file,
            unstage_file,
            discard_file,
            commit_changes,
            get_commits,
            get_branches,
            create_branch,
            checkout_branch,
            delete_branch,
            get_current_branch,
            get_sync_status,
            fetch_remote,
            pull_remote,
            push_remote,
            get_remotes,
            add_remote,
            remove_remote,
            merge_branch,
            stash_save,
            stash_list,
            stash_pop,
            stash_drop,
            create_tag,
            get_tags,
            delete_tag,
            get_file_diff,
            clone_repository,
            init_repository,
            detect_project_type,
            get_file_blame,
            cherry_pick,
            cherry_pick_batch,
            get_conflicts,
            resolve_conflict,
            abort_merge,
            set_window_theme,
            call_ai_api,
            copy_to_clipboard,
            get_app_version,
            // 条件编译：自动更新检查（仅 DMG 版本）
            #[cfg(not(feature = "appstore"))]
            check_for_updates,
            get_release_info,
            publish_new_release,
            rerun_failed_build,
            increment_version,
            generate_release_notes,
            get_platform_download_url,
            restart_app,
            exit_app,
            // Keychain 安全存储
            keychain_save,
            keychain_get,
            keychain_delete,
            keychain_exists,
            keychain_migrate,
            // 条件编译：自动更新功能（仅 DMG 版本）
            #[cfg(feature = "auto-update")]
            install_update,
            // 条件编译：App Store 更新功能
            #[cfg(feature = "appstore")]
            check_update_appstore,
            #[cfg(feature = "appstore")]
            open_app_store,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
