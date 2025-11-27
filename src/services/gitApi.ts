import { invoke } from '@tauri-apps/api/core';

// Helper function to safely invoke Tauri commands
async function safeInvoke<T>(cmd: string, args: Record<string, any> = {}): Promise<T> {
    if (typeof window === 'undefined' || !('__TAURI_INTERNALS__' in window)) {
        throw new Error('Not running in Tauri context. Please run the app using `npm run tauri dev` or the built executable.');
    }
    return invoke<T>(cmd, args);
}

export interface ApiResponse<T> {
    success: boolean;
    data?: T;
    error?: string;
}

export interface FileChange {
    path: string;
    status: 'modified' | 'added' | 'deleted' | 'renamed' | 'untracked';
    staged: boolean;
}

export interface CommitInfo {
    hash: string;
    message: string;
    author: string;
    email: string;
    date: string;
    parents: string[];
}

export interface BranchInfo {
    name: string;
    is_head: boolean;
    is_remote: boolean;
    upstream?: string;
    last_commit?: string;
}

export class GitApi {
    static async openRepository(path: string): Promise<ApiResponse<string>> {
        return await safeInvoke('open_repository', { path });
    }

    static async getRepositoryStatus(path: string): Promise<ApiResponse<FileChange[]>> {
        return await safeInvoke('get_repository_status', { path });
    }

    static async stageFile(repoPath: string, filePath: string): Promise<ApiResponse<string>> {
        return await safeInvoke('stage_file', { repoPath, filePath });
    }

    static async unstageFile(repoPath: string, filePath: string): Promise<ApiResponse<string>> {
        return await safeInvoke('unstage_file', { repoPath, filePath });
    }

    static async commitChanges(repoPath: string, message: string): Promise<ApiResponse<string>> {
        return await safeInvoke('commit_changes', { repoPath, message });
    }

    static async getCommits(repoPath: string, maxCount: number = 100): Promise<ApiResponse<CommitInfo[]>> {
        return await safeInvoke('get_commits', { repoPath, maxCount });
    }

    static async getBranches(repoPath: string): Promise<ApiResponse<BranchInfo[]>> {
        return await safeInvoke('get_branches', { repoPath });
    }

    static async createBranch(repoPath: string, branchName: string): Promise<ApiResponse<string>> {
        return await safeInvoke('create_branch', { repoPath, branchName });
    }

    static async checkoutBranch(repoPath: string, branchName: string): Promise<ApiResponse<string>> {
        return await safeInvoke('checkout_branch', { repoPath, branchName });
    }

    static async deleteBranch(repoPath: string, branchName: string): Promise<ApiResponse<string>> {
        return await safeInvoke('delete_branch', { repoPath, branchName });
    }

    static async getCurrentBranch(repoPath: string): Promise<ApiResponse<string>> {
        return await safeInvoke('get_current_branch', { repoPath });
    }

    // Remote operations
    static async fetch(repoPath: string, remoteName: string = 'origin'): Promise<ApiResponse<string>> {
        return await safeInvoke('fetch_remote', { repoPath, remoteName });
    }

    static async pull(repoPath: string, remoteName: string = 'origin', branchName: string): Promise<ApiResponse<string>> {
        return await safeInvoke('pull_remote', { repoPath, remoteName, branchName });
    }

    static async push(repoPath: string, remoteName: string = 'origin', branchName: string): Promise<ApiResponse<string>> {
        return await safeInvoke('push_remote', { repoPath, remoteName, branchName });
    }

    static async getRemotes(repoPath: string): Promise<ApiResponse<any[]>> {
        return await safeInvoke('get_remotes', { repoPath });
    }

    static async addRemote(repoPath: string, name: string, url: string): Promise<ApiResponse<string>> {
        return await safeInvoke('add_remote', { repoPath, name, url });
    }

    static async removeRemote(repoPath: string, name: string): Promise<ApiResponse<string>> {
        return await safeInvoke('remove_remote', { repoPath, name });
    }

    // Merge operations
    static async mergeBranch(repoPath: string, branchName: string): Promise<ApiResponse<string>> {
        return await safeInvoke('merge_branch', { repoPath, branchName });
    }

    // Stash operations
    static async stashSave(repoPath: string, message?: string): Promise<ApiResponse<string>> {
        return await safeInvoke('stash_save', { repoPath, message });
    }

    static async stashList(repoPath: string): Promise<ApiResponse<any[]>> {
        return await safeInvoke('stash_list', { repoPath });
    }

    static async stashPop(repoPath: string, index: number): Promise<ApiResponse<string>> {
        return await safeInvoke('stash_pop', { repoPath, index });
    }

    static async stashDrop(repoPath: string, index: number): Promise<ApiResponse<string>> {
        return await safeInvoke('stash_drop', { repoPath, index });
    }

    // Tag operations
    static async createTag(repoPath: string, tagName: string, message?: string): Promise<ApiResponse<string>> {
        return await safeInvoke('create_tag', { repoPath, tagName, message });
    }

    static async getTags(repoPath: string): Promise<ApiResponse<any[]>> {
        return await safeInvoke('get_tags', { repoPath });
    }

    static async deleteTag(repoPath: string, tagName: string): Promise<ApiResponse<string>> {
        return await safeInvoke('delete_tag', { repoPath, tagName });
    }

    // Diff operations
    static async getFileDiff(repoPath: string, filePath: string, staged: boolean): Promise<ApiResponse<any>> {
        return await safeInvoke('get_file_diff', { repoPath, filePath, staged });
    }

    // Clone operation
    static async cloneRepository(url: string, path: string): Promise<ApiResponse<string>> {
        return await safeInvoke('clone_repository', { url, path });
    }

    // Init operation
    static async initRepository(path: string, defaultBranch?: string): Promise<ApiResponse<string>> {
        return await safeInvoke('init_repository', { path, defaultBranch });
    }

    // Detect project type
    static async detectProjectType(path: string): Promise<ApiResponse<string>> {
        return await safeInvoke('detect_project_type', { path });
    }
}
