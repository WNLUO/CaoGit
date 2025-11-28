mod git_ops;
mod commands;
mod github_api;
mod release_commands;

use commands::*;
use release_commands::*;
use tauri_plugin_updater::UpdaterExt;
use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();

            // 延迟 2 秒后检查更新，避免阻塞启动
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(2));

                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    if let Ok(updater) = handle.updater() {
                        match updater.check().await {
                            Ok(Some(_)) => {
                                // 有新版本，通知前端
                                let _ = handle.emit("update-available", ());
                            }
                            Ok(None) => {
                                // 已是最新版本
                            }
                            Err(_) => {
                                // 检查更新失败，静默处理
                            }
                        }
                    }
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
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
            check_for_updates,
            get_release_info,
            publish_new_release,
            rerun_failed_build,
            increment_version,
            generate_release_notes,
            install_update,
            restart_app,
            exit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
