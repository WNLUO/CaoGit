import { reactive } from 'vue';
import type { Repository, ProxySettings, FileChange, Commit } from '../types';

export const store = reactive({
    repositories: [
        {
            id: 1,
            name: 'git-manager',
            path: '/Users/wnluo/Desktop/code/Git',
            status: 'online',
            protocol: 'https',
            authType: 'token',
            token: 'ghp_xxxxxxxxxxxx',
            proxy: { enabled: false, host: '', port: '', type: 'http' }
        },
        {
            id: 2,
            name: 'wkshop-vue',
            path: '/Users/wnluo/Desktop/code/vue/wkshop-vue',
            status: 'syncing',
            protocol: 'ssh',
            authType: 'none',
            proxy: { enabled: true, host: '127.0.0.1', port: '7890', type: 'socks5' }
        },
        {
            id: 3,
            name: 'cx_vue_front',
            path: '/Users/wnluo/Desktop/code/vue/cx_vue_front',
            status: 'error',
            protocol: 'https',
            authType: 'password',
            proxy: { enabled: false, host: '', port: '', type: 'http' }
        },
        {
            id: 4,
            name: 'zhixiaocha',
            path: '/Users/wnluo/Desktop/code/zhixiaocha',
            status: 'offline',
            protocol: 'http',
            authType: 'none',
            proxy: { enabled: false, host: '', port: '', type: 'http' }
        }
    ] as Repository[],

    globalProxy: {
        enabled: false,
        host: '127.0.0.1',
        port: '7890',
        type: 'http'
    } as ProxySettings,

    // Mock data for the active repository view
    activeRepoChanges: [
        { path: 'src/components/Sidebar.vue', status: 'modified', staged: true },
        { path: 'src/types.ts', status: 'modified', staged: false },
        { path: 'src/stores/mockData.ts', status: 'modified', staged: false },
        { path: 'src/assets/new_icon.png', status: 'untracked', staged: false },
    ] as FileChange[],

    activeRepoHistory: [
        { hash: 'a1b2c3d', message: 'feat: implement sidebar', author: 'wnluo', date: '2023-10-27 14:00', parents: ['d4e5f6g'] },
        { hash: 'd4e5f6g', message: 'init: project setup', author: 'wnluo', date: '2023-10-27 10:00', parents: [] },
    ] as Commit[],

    selectedFile: null as FileChange | null,

    updateRepository(id: number, updates: Partial<Repository>) {
        const repo = this.repositories.find(r => r.id === id);
        if (repo) {
            // Handle nested proxy updates carefully if needed, but Object.assign is shallow.
            // For deep updates, we might need more logic, but for now:
            if (updates.proxy && repo.proxy) {
                Object.assign(repo.proxy, updates.proxy);
                delete updates.proxy; // Prevent overwriting the proxy object reference if we just updated props
            }
            Object.assign(repo, updates);
        }
    },

    updateGlobalProxy(updates: Partial<ProxySettings>) {
        Object.assign(this.globalProxy, updates);
    }
});
