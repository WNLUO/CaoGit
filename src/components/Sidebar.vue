<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { repoStore } from '../stores/repoStore';
import { settingsStore } from '../stores/settingsStore';
import { toastStore } from '../stores/toastStore';
import { GitApi } from '../services/gitApi';
import type { Repository } from '../types';
import AddRepoModal from './AddRepoModal.vue';
import NetworkStatus from './NetworkStatus.vue';
import ContextMenu from './ContextMenu.vue';
import { invoke } from '@tauri-apps/api/core';

const activeRepoId = ref(1);
const showAddRepoModal = ref(false);
const contextMenuVisible = ref(false);
const contextMenuX = ref(0);
const contextMenuY = ref(0);
const contextMenuRepo = ref<Repository | null>(null);

// Load from global settings
const proxyEnabled = computed(() => settingsStore.settings.proxy.enabled);
const testUrl = computed(() => settingsStore.settings.networkTest.testUrl);

const emit = defineEmits<{
  (e: 'open-global-settings'): void;
  (e: 'open-repo-settings', repo: Repository): void;
  (e: 'select-repo', repo: Repository): void;
}>();

// 在组件挂载时，为所有没有 remoteUrl 的仓库加载远程 URL
onMounted(async () => {
  for (const repo of repoStore.repositories) {
    if (!repo.remoteUrl) {
      try {
        const response = await GitApi.getRemotes(repo.path);
        if (response.success && response.data && response.data.length > 0) {
          const origin = response.data.find(r => r.name === 'origin');
          const remoteUrl = origin ? origin.url : response.data[0].url;
          repoStore.updateRepository(repo.id, { remoteUrl });
        }
      } catch (error) {
        console.log(`No remote found for repository: ${repo.name}`);
      }
    }
  }
});

async function selectRepo(repo: Repository) {
  activeRepoId.value = repo.id;
  emit('select-repo', repo);
  await repoStore.loadRepoData(repo);
}

function addRepo() {
  showAddRepoModal.value = true;
}

async function handleRepoAdded(path: string) {
  // Extract repository name from path
  const repoName = path.split('/').filter(Boolean).pop() || 'Unknown';

  // Generate new ID
  const newId = repoStore.repositories.length > 0
    ? Math.max(...repoStore.repositories.map(r => r.id)) + 1
    : 1;

  // Try to get remote URL
  let remoteUrl = '';
  try {
    const response = await GitApi.getRemotes(path);
    if (response.success && response.data && response.data.length > 0) {
      const origin = response.data.find(r => r.name === 'origin');
      remoteUrl = origin ? origin.url : response.data[0].url;
    }
  } catch (error) {
    console.log('No remote found for this repository');
  }

  // Create new repository object
  const newRepo: Repository = {
    id: newId,
    name: repoName,
    path: path,
    status: 'online',
    protocol: 'https',
    authType: 'none',
    remoteUrl: remoteUrl || undefined
  };

  // Add to store
  repoStore.addRepository(newRepo);

  // Select the newly added repo
  selectRepo(newRepo);

  // Close modal
  showAddRepoModal.value = false;
}

function getStatusColor(status: Repository['status']) {
  switch (status) {
    case 'online': return '#10b981'; // green
    case 'offline': return '#9ca3af'; // gray
    case 'syncing': return '#3b82f6'; // blue
    case 'error': return '#ef4444'; // red
    default: return '#9ca3af';
  }
}

function formatRemoteUrl(url?: string): string {
  if (!url) return '无远程仓库';

  // Extract hostname and path from URL
  try {
    // Handle SSH format: git@github.com:user/repo.git
    if (url.startsWith('git@')) {
      const match = url.match(/git@([^:]+):(.+)/);
      if (match) {
        const host = match[1];
        const path = match[2].replace('.git', '');
        return `${host}/${path}`;
      }
    }

    // Handle HTTP/HTTPS format
    if (url.startsWith('http://') || url.startsWith('https://')) {
      const urlObj = new URL(url);
      const path = urlObj.pathname.replace('.git', '').replace(/^\//, '');
      return `${urlObj.hostname}/${path}`;
    }

    return url;
  } catch (error) {
    return url;
  }
}

function handleContextMenu(event: MouseEvent, repo: Repository) {
  event.preventDefault();

  contextMenuRepo.value = repo;
  contextMenuX.value = event.clientX;
  contextMenuY.value = event.clientY;
  contextMenuVisible.value = true;
}

function removeRepository() {
  if (!contextMenuRepo.value) return;

  if (confirm(`确定要从列表中移除仓库 "${contextMenuRepo.value.name}" 吗？\n\n注意：这只会从列表中移除，不会删除实际文件。`)) {
    repoStore.removeRepository(contextMenuRepo.value.id);

    // 如果删除的是当前选中的仓库，清空选择
    if (activeRepoId.value === contextMenuRepo.value.id) {
      activeRepoId.value = 0;
      repoStore.activeRepo = null;
    }
  }
}

async function copyRepositoryLink() {
  if (!contextMenuRepo.value) return;

  // 优先复制远程仓库 URL，如果没有则复制本地路径
  const repo = contextMenuRepo.value;

  try {
    // 尝试从 GitApi 获取远程 URL
    const response = await GitApi.getRemotes(repo.path);
    let linkToCopy = repo.path; // 默认复制本地路径

    if (response.success && response.data && response.data.length > 0) {
      // 优先使用 origin 远程，其次使用第一个远程
      const origin = response.data.find((r: any) => r.name === 'origin');
      linkToCopy = origin?.url || response.data[0]?.url || repo.path;
    }

    // 使用可靠的复制方法
    await copyToClipboardReliable(linkToCopy);
  } catch (error) {
    // 如果获取远程失败，就复制本地路径
    await copyToClipboardReliable(repo.path);
  }
}

async function copyToClipboardReliable(text: string) {
  try {
    // 使用 Tauri 的 invoke 命令调用 Rust 后端的剪贴板功能
    await invoke('copy_to_clipboard', { text });
    toastStore.success(`已复制`);
  } catch (error) {
    console.error('复制失败:', error);
    toastStore.error('复制失败，请重试');
  }
}

const contextMenuItems = computed(() => [
  {
    label: '复制项目链接',
    action: copyRepositoryLink
  },
  {
    label: '从列表中移除',
    action: removeRepository,
    danger: true
  }
]);
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <h2>Git管理器</h2>
    </div>
    
    <div class="repo-list">
      <div class="list-header">
        <span>仓库列表</span>
        <button class="add-btn" title="添加仓库" @click="addRepo">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
        </button>
      </div>
      
      <ul>
        <li
          v-for="repo in repoStore.repositories"
          :key="repo.id"
          :class="{ active: activeRepoId === repo.id }"
          @click="selectRepo(repo)"
          @contextmenu="handleContextMenu($event, repo)"
        >
          <div class="repo-info">
            <span class="repo-name">{{ repo.name }}</span>
            <span class="repo-remote">{{ formatRemoteUrl(repo.remoteUrl) }}</span>
          </div>
          
          <div class="repo-actions">
            <div 
              class="status-dot" 
              :style="{ backgroundColor: getStatusColor(repo.status) }"
              :title="repo.status"
            ></div>
            <button 
              class="settings-icon-btn" 
              @click.stop="emit('open-repo-settings', repo)"
              title="仓库设置"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>
            </button>
          </div>
        </li>
      </ul>
    </div>

    <div class="sidebar-footer">
      <NetworkStatus :proxy-enabled="proxyEnabled" :test-url="testUrl" />
      <button class="settings-btn" @click="emit('open-global-settings')">
        全局设置
      </button>
    </div>

    <!-- Add Repo Modal -->
    <AddRepoModal
      :is-open="showAddRepoModal"
      @close="showAddRepoModal = false"
      @repo-added="handleRepoAdded"
    />

    <!-- Context Menu -->
    <ContextMenu
      v-if="contextMenuVisible"
      :items="contextMenuItems"
      :x="contextMenuX"
      :y="contextMenuY"
      @close="contextMenuVisible = false"
    />
  </aside>
</template>

<style scoped>
.sidebar {
  background-color: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  flex-shrink: 0;
  min-width: 0;
}

.sidebar-header {
  padding: var(--spacing-lg) var(--spacing-md);
  flex-shrink: 0;
  /* border-bottom: 1px solid var(--border-color); */
}

.sidebar-header h2 {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-bold);
  color: var(--text-primary);
  letter-spacing: -0.025em;
}

.repo-list {
  flex: 1;
  overflow-y: auto;
  padding: 0 var(--spacing-sm);
  min-height: 0;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-xs) var(--spacing-sm);
  margin-bottom: var(--spacing-xs);
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  font-weight: var(--font-weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.add-btn {
  color: var(--text-tertiary);
  padding: 4px;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
}

.add-btn:hover {
  color: var(--text-primary);
  background-color: var(--bg-tertiary);
}

ul {
  list-style: none;
}

li {
  padding: var(--spacing-sm);
  border-radius: var(--radius-md);
  cursor: pointer;
  margin-bottom: 2px;
  transition: all var(--transition-fast);
  display: flex;
  justify-content: space-between;
  align-items: center;
  border: 1px solid transparent;
}

li:hover {
  background-color: var(--bg-hover);
}

li.active {
  background-color: var(--bg-primary);
  border-color: var(--border-color);
  box-shadow: var(--shadow-sm);
}

li.active .repo-name {
  color: var(--text-primary);
  font-weight: var(--font-weight-semibold);
}

li.active .repo-remote {
  color: var(--text-secondary);
}

.repo-info {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.repo-name {
  display: block;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: color var(--transition-fast);
}

.repo-remote {
  display: block;
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-family: monospace;
}

.repo-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  margin-left: var(--spacing-sm);
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.settings-icon-btn {
  color: var(--text-tertiary);
  opacity: 0;
  transition: all var(--transition-fast);
  padding: 4px;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
}

li:hover .settings-icon-btn {
  opacity: 1;
}

.settings-icon-btn:hover {
  color: var(--text-primary);
  background-color: var(--bg-tertiary);
}

.sidebar-footer {
  padding: var(--spacing-md);
  border-top: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  background-color: var(--bg-secondary);
  flex-shrink: 0;
}

.settings-btn {
  width: 100%;
  padding: var(--spacing-sm);
  text-align: center;
  border-radius: var(--radius-md);
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  box-shadow: var(--shadow-sm);
}

.settings-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
  border-color: var(--text-tertiary);
}
</style>
