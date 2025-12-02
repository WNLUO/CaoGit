import { invoke } from '@tauri-apps/api/core';
import { networkMetricsStore } from '../stores/networkMetrics';

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

export interface SyncStatus {
    ahead: number;
    behind: number;
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

    static async discardFile(repoPath: string, filePath: string): Promise<ApiResponse<string>> {
        return await safeInvoke('discard_file', { repoPath, filePath });
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

    static async getSyncStatus(repoPath: string, branchName: string): Promise<ApiResponse<SyncStatus>> {
        return await safeInvoke('get_sync_status', { repoPath, branchName });
    }

    // Remote operations
    static async fetch(repoPath: string, remoteName: string = 'origin', authConfig?: any): Promise<ApiResponse<string>> {
        const startTime = performance.now();
        const startMs = Date.now();

        try {
            const result = await safeInvoke<ApiResponse<string>>('fetch_remote', {
                repoPath,
                remoteName,
                authConfig: authConfig || null
            });

            // 计算网络指标
            const endTime = performance.now();
            const duration = (endTime - startTime) / 1000; // 转换为秒

            if (duration > 0) {
                // 估算下载大小（通常 fetch 下载 100KB 左右的元数据和对象）
                const estimatedSize = 100; // KB
                const downloadSpeed = estimatedSize / duration;

                // 更新网络指标
                this.updateNetworkMetrics('fetch', downloadSpeed, 0, startMs);
            }

            return result;
        } catch (error) {
            throw error;
        }
    }

    static async pull(repoPath: string, remoteName: string = 'origin', branchName: string, authConfig?: any): Promise<ApiResponse<string>> {
        const startTime = performance.now();
        const startMs = Date.now();

        try {
            const result = await safeInvoke<ApiResponse<string>>('pull_remote', {
                repoPath,
                remoteName,
                branchName,
                authConfig: authConfig || null
            });

            // 计算网络指标
            const endTime = performance.now();
            const duration = (endTime - startTime) / 1000; // 转换为秒

            if (duration > 0) {
                // 估算下载大小（pull 包括 fetch 和 merge，通常 200KB-500KB）
                const estimatedSize = 300; // KB
                const downloadSpeed = estimatedSize / duration;

                // 更新网络指标
                this.updateNetworkMetrics('pull', downloadSpeed, 0, startMs);
            }

            return result;
        } catch (error) {
            throw error;
        }
    }

    static async push(repoPath: string, remoteName: string = 'origin', branchName: string, authConfig?: any): Promise<ApiResponse<string>> {
        const startTime = performance.now();
        const startMs = Date.now();

        try {
            const result = await safeInvoke<ApiResponse<string>>('push_remote', {
                repoPath,
                remoteName,
                branchName,
                authConfig: authConfig || null
            });

            // 计算网络指标
            const endTime = performance.now();
            const duration = (endTime - startTime) / 1000; // 转换为秒

            if (duration > 0) {
                // 估算上传大小（push 上传 commit 和对象，通常 50KB-200KB）
                const estimatedSize = 100; // KB
                const uploadSpeed = estimatedSize / duration;

                // 更新网络指标
                this.updateNetworkMetrics('push', 0, uploadSpeed, startMs);
            }

            return result;
        } catch (error) {
            throw error;
        }
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

    // Cherry-pick operations
    static async cherryPick(repoPath: string, commitHash: string): Promise<ApiResponse<string>> {
        return await safeInvoke('cherry_pick', { repoPath, commitHash });
    }

    static async cherryPickBatch(repoPath: string, commitHashes: string[]): Promise<ApiResponse<string[]>> {
        return await safeInvoke('cherry_pick_batch', { repoPath, commitHashes });
    }

    // Blame operations
    static async getFileBlame(repoPath: string, filePath: string): Promise<ApiResponse<any[]>> {
        return await safeInvoke('get_file_blame', { repoPath, filePath });
    }

    // Conflict resolution operations
    static async getConflicts(repoPath: string): Promise<ApiResponse<any[]>> {
        return await safeInvoke('get_conflicts', { repoPath });
    }

    static async resolveConflict(repoPath: string, filePath: string, resolution: string): Promise<ApiResponse<string>> {
        return await safeInvoke('resolve_conflict', { repoPath, filePath, resolution });
    }

    static async abortMerge(repoPath: string): Promise<ApiResponse<string>> {
        return await safeInvoke('abort_merge', { repoPath });
    }

    // Window theme operations
    static async setWindowTheme(theme: 'light' | 'dark' | 'auto'): Promise<ApiResponse<string>> {
        return await safeInvoke('set_window_theme', { theme });
    }

    // AI API operations
    static async callAIApi(
        endpoint: string,
        apiKey: string,
        model: string,
        messages: Array<{ role: string; content: string }>,
        temperature: number,
        maxTokens: number
    ): Promise<ApiResponse<string>> {
        return await safeInvoke('call_ai_api', {
            endpoint,
            apiKey,
            model,
            messages,
            temperature,
            maxTokens
        });
    }

    // 更新网络指标
    private static updateNetworkMetrics(operation: string, downloadSpeed: number, uploadSpeed: number, startMs: number) {
        // 计算延迟（基于时间戳）
        const endMs = Date.now();
        const latency = endMs - startMs;

        // 更新网络指标 store
        if (downloadSpeed > 0) {
            networkMetricsStore.setDownloadSpeed(downloadSpeed);
        }
        if (uploadSpeed > 0) {
            networkMetricsStore.setUploadSpeed(uploadSpeed);
        }
        if (latency > 0) {
            networkMetricsStore.setLatency(latency);
        }
        networkMetricsStore.setOperation(operation);
    }
}
