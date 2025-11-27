import { reactive } from 'vue';
import type { Repository, FileChange, Commit } from '../types';
import { GitApi } from '../services/gitApi';
import { cacheService } from '../services/cacheService';

// Load repositories from localStorage
function loadRepositories(): Repository[] {
    const saved = localStorage.getItem('repositories');
    if (saved) {
        try {
            return JSON.parse(saved);
        } catch (e) {
            console.error('Failed to load repositories from localStorage:', e);
        }
    }
    return [];
}

// Save repositories to localStorage
function saveRepositories(repositories: Repository[]) {
    try {
        localStorage.setItem('repositories', JSON.stringify(repositories));
    } catch (e) {
        console.error('Failed to save repositories to localStorage:', e);
    }
}

export const repoStore = reactive({
    repositories: loadRepositories(),

    activeRepo: null as Repository | null,
    currentBranch: 'main',
    branches: [] as any[],
    fileChanges: [] as FileChange[],
    commits: [] as Commit[],
    selectedFile: null as FileChange | null,
    isLoading: false,
    error: null as string | null,

    async loadRepoData(repo: Repository) {
        this.activeRepo = repo;
        this.isLoading = true;
        this.error = null;

        try {
            // Parallel loading for better performance
            const [statusResult, branchesResult] = await Promise.allSettled([
                this.refreshStatus(),
                this.refreshBranches()
            ]);

            // Handle errors from parallel operations
            if (statusResult.status === 'rejected') {
                throw new Error(statusResult.reason?.message || 'Failed to get repository status');
            }
            if (branchesResult.status === 'rejected') {
                throw new Error(branchesResult.reason?.message || 'Failed to get branches');
            }

            // Only try to load commits and current branch if there are any commits
            try {
                await Promise.all([
                    this.refreshCommits(),
                    this.refreshCurrentBranch()
                ]);
            } catch (error: any) {
                // This is expected for newly initialized repos with no commits
                if (error.message?.includes('reference') || error.message?.includes('not found')) {
                    console.log('New repository with no commits yet');
                    this.commits = [];
                    this.currentBranch = 'main'; // Default branch name
                } else {
                    throw error;
                }
            }
        } catch (error: any) {
            this.error = error.message || 'Failed to load repository data';
            console.error('Error loading repo data:', error);
        } finally {
            this.isLoading = false;
        }
    },

    async refreshStatus() {
        if (!this.activeRepo) return;

        const response = await GitApi.getRepositoryStatus(this.activeRepo.path);
        if (response.success && response.data) {
            this.fileChanges = response.data;
        } else {
            throw new Error(response.error || 'Failed to get repository status');
        }
    },

    async refreshBranches() {
        if (!this.activeRepo) return;

        const response = await GitApi.getBranches(this.activeRepo.path);
        if (response.success && response.data) {
            this.branches = response.data;
        } else {
            throw new Error(response.error || 'Failed to get branches');
        }
    },

    async refreshCommits(maxCount: number = 100, offset: number = 0) {
        if (!this.activeRepo) return;

        // Use cache for commits
        const cacheKey = `commits:${this.activeRepo.path}:${maxCount}:${offset}`;
        const cached = cacheService.get<Commit[]>(cacheKey);
        if (cached) {
            this.commits = offset === 0 ? cached : [...this.commits, ...cached];
            return;
        }

        const response = await GitApi.getCommits(this.activeRepo.path, maxCount);
        if (response.success && response.data) {
            this.commits = offset === 0 ? response.data : [...this.commits, ...response.data];
            cacheService.set(cacheKey, response.data, 30000); // Cache for 30s
        } else {
            throw new Error(response.error || 'Failed to get commits');
        }
    },

    async refreshCurrentBranch() {
        if (!this.activeRepo) return;

        const response = await GitApi.getCurrentBranch(this.activeRepo.path);
        if (response.success && response.data) {
            this.currentBranch = response.data;
        }
    },

    async stageFile(filePath: string) {
        if (!this.activeRepo) return;

        const response = await GitApi.stageFile(this.activeRepo.path, filePath);
        if (response.success) {
            await this.refreshStatus();
        } else {
            throw new Error(response.error || 'Failed to stage file');
        }
    },

    async unstageFile(filePath: string) {
        if (!this.activeRepo) return;

        const response = await GitApi.unstageFile(this.activeRepo.path, filePath);
        if (response.success) {
            await this.refreshStatus();
        } else {
            throw new Error(response.error || 'Failed to unstage file');
        }
    },

    async commit(message: string) {
        if (!this.activeRepo) return;

        const response = await GitApi.commitChanges(this.activeRepo.path, message);
        if (response.success) {
            // Parallel refresh for better performance
            await Promise.all([
                this.refreshStatus(),
                this.refreshCommits()
            ]);
            return response.data;
        } else {
            throw new Error(response.error || 'Failed to commit');
        }
    },

    async createBranch(branchName: string) {
        if (!this.activeRepo) return;

        const response = await GitApi.createBranch(this.activeRepo.path, branchName);
        if (response.success) {
            await this.refreshBranches();
        } else {
            throw new Error(response.error || 'Failed to create branch');
        }
    },

    async checkoutBranch(branchName: string) {
        if (!this.activeRepo) return;

        const response = await GitApi.checkoutBranch(this.activeRepo.path, branchName);
        if (response.success) {
            // Parallel refresh for better performance
            await Promise.all([
                this.refreshBranches(),
                this.refreshStatus(),
                this.refreshCommits(),
                this.refreshCurrentBranch()
            ]);
        } else {
            throw new Error(response.error || 'Failed to checkout branch');
        }
    },

    async deleteBranch(branchName: string) {
        if (!this.activeRepo) return;

        const response = await GitApi.deleteBranch(this.activeRepo.path, branchName);
        if (response.success) {
            await this.refreshBranches();
        } else {
            throw new Error(response.error || 'Failed to delete branch');
        }
    },

    addRepository(repo: Repository) {
        this.repositories.push(repo);
        saveRepositories(this.repositories);
    },

    removeRepository(id: number) {
        const index = this.repositories.findIndex(r => r.id === id);
        if (index !== -1) {
            const repo = this.repositories[index];
            this.repositories.splice(index, 1);
            saveRepositories(this.repositories);
            // Clear cache for this repo
            cacheService.invalidatePattern(`.*:${repo.path}:.*`);
        }
    },

    // Clear cache for active repo
    clearCache() {
        if (this.activeRepo) {
            cacheService.invalidatePattern(`.*:${this.activeRepo.path}:.*`);
        }
    }
});
