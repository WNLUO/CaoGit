mod git_ops;
mod commands;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
