<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick } from 'vue';
import { repoStore } from '../../stores/repoStore';
import { toastStore } from '../../stores/toastStore';
import { GitApi } from '../../services/gitApi';
import PublishModal from '../modals/PublishModal.vue';
import ThemeToggle from '../common/ThemeToggle.vue';
import ProgressBar from '../common/ProgressBar.vue';
import ConfirmModal from '../common/ConfirmModal.vue';
import ReleaseManagerModal from '../modals/ReleaseManagerModal.vue';
import VersionDisplay from '../common/VersionDisplay.vue';
import { settingsStore } from '../../stores/settingsStore';

const emit = defineEmits<{
  (e: 'open-global-settings'): void;
  (e: 'check-update'): void;
}>();

const currentBranch = computed(() => repoStore.currentBranch);
const syncStatus = computed(() => repoStore.syncStatus);
const isPushing = ref(false);
const isPulling = ref(false);
const isFetching = ref(false);
const showBranchMenu = ref(false);
const showPublishModal = ref(false);
const showReleaseManager = ref(false);
const hasRemote = ref(false);
const isGitHubRepo = ref(false);

// Compute sync status display
const syncStatusDisplay = computed(() => {
  if (!syncStatus.value) return null;
  const { ahead, behind } = syncStatus.value;

  if (ahead === 0 && behind === 0) {
    return { icon: '✓', text: '已同步', class: 'synced' };
  } else if (ahead > 0 && behind === 0) {
    return { icon: '↑', text: `${ahead}`, class: 'ahead' };
  } else if (ahead === 0 && behind > 0) {
    return { icon: '↓', text: `${behind}`, class: 'behind' };
  } else {
    return { icon: '↑↓', text: `${ahead} ${behind}`, class: 'diverged' };
  }
});

// 进度条状态
const showProgress = ref(false);
const progressOperation = ref<'push' | 'pull' | 'fetch' | ''>('');
const progressMessage = ref('');

// 确认对话框状态
const confirmModal = ref({
  show: false,
  title: '',
  message: '',
  type: 'info' as 'info' | 'warning' | 'error',
  confirmText: '确定',
  onConfirm: () => {},
  onCancel: () => {}
});

function showConfirm(options: {
  title: string;
  message: string;
  type?: 'info' | 'warning' | 'error';
  confirmText?: string;
  onConfirm: () => void;
  onCancel?: () => void;
}) {
  confirmModal.value = {
    show: true,
    title: options.title,
    message: options.message,
    type: options.type || 'info',
    confirmText: options.confirmText || '确定',
    onConfirm: () => {
      options.onConfirm();
      confirmModal.value.show = false;
    },
    onCancel: () => {
      if (options.onCancel) options.onCancel();
      confirmModal.value.show = false;
    }
  };
}

// 检查是否有远程仓库
async function checkRemote() {
  if (!repoStore.activeRepo) {
    hasRemote.value = false;
    isGitHubRepo.value = false;
    return;
  }

  try {
    const response = await GitApi.getRemotes(repoStore.activeRepo.path);
    hasRemote.value = !!(response.success && response.data && response.data.length > 0);

    // 检查是否是 GitHub 仓库
    if (hasRemote.value && response.data) {
      // 查找 origin 远程仓库的 URL
      const origin = response.data.find((r: any) => r.name === 'origin');
      if (origin && origin.url) {
        const url = origin.url.toLowerCase();
        isGitHubRepo.value = url.includes('github.com');
      } else {
        isGitHubRepo.value = false;
      }
    } else {
      isGitHubRepo.value = false;
    }
  } catch (error) {
    hasRemote.value = false;
    isGitHubRepo.value = false;
  }
}

// 监听活跃仓库变化
watch(() => repoStore.activeRepo, () => {
  checkRemote();
}, { immediate: true });

onMounted(() => {
  checkRemote();
});

async function handlePull() {
  if (!repoStore.activeRepo) {
    toastStore.warning('请先打开一个仓库');
    return;
  }

  // 检查是否有未提交的更改
  if (repoStore.fileChanges.length > 0) {
    showConfirm({
      title: '存在未提交的更改',
      message: '您有未提交的更改，拉取操作可能会导致冲突或覆盖您的修改。建议先提交或暂存更改。\n\n是否仍要继续拉取？',
      type: 'warning',
      confirmText: '继续拉取',
      onConfirm: executePull
    });
    return;
  }

  executePull();
}

async function executePull() {
  // 立即显示进度条
  isPulling.value = true;
  progressOperation.value = 'pull';
  progressMessage.value = `正在从 origin/${currentBranch.value} 拉取代码...`;
  showProgress.value = true;

  // 确保进度条先渲染，然后再执行耗时操作
  await nextTick();
  await new Promise(resolve => setTimeout(resolve, 50));

  try {
    if (!repoStore.activeRepo) return;

    const response = await GitApi.pull(
      repoStore.activeRepo.path,
      'origin',
      currentBranch.value
    );

    if (response.success) {
      progressMessage.value = 'Pull 成功!';
      // 延迟一下让用户看到成功消息
      await new Promise(resolve => setTimeout(resolve, 800));
      await repoStore.loadRepoData(repoStore.activeRepo);
    } else {
      // 只在失败时显示 alert
      toastStore.error('Pull 失败: ' + response.error);
    }
  } catch (error: any) {
    toastStore.error('Pull 失败: ' + error.message);
  } finally {
    isPulling.value = false;
    showProgress.value = false;
  }
}

async function handlePush() {
  if (!repoStore.activeRepo) {
    toastStore.warning('请先打开一个仓库');
    return;
  }

  // 检查是否落后于远程
  if (repoStore.syncStatus && repoStore.syncStatus.behind > 0) {
    showConfirm({
      title: '落后于远程分支',
      message: `当前分支落后于远程分支 ${repoStore.syncStatus.behind} 个提交。推送操作可能会失败。\n\n建议先拉取更新。是否仍要尝试推送？`,
      type: 'warning',
      confirmText: '尝试推送',
      onConfirm: executePush
    });
    return;
  }

  // 检查是否有提交可推送
  if (repoStore.syncStatus && repoStore.syncStatus.ahead === 0) {
    toastStore.info('没有需要推送的提交');
    return;
  }

  executePush();
}

async function executePush() {
  // 立即显示进度条
  isPushing.value = true;
  progressOperation.value = 'push';
  progressMessage.value = `正在推送到 origin/${currentBranch.value}...`;
  showProgress.value = true;

  // 确保进度条先渲染，然后再执行耗时操作
  await nextTick();
  await new Promise(resolve => setTimeout(resolve, 50));

  try {
    if (!repoStore.activeRepo) return; // Safety check

    // 获取当前仓库的认证配置
    const authConfig = repoStore.activeRepo ? {
      authType: repoStore.activeRepo.authType || 'none',
      token: repoStore.activeRepo.token,
      username: repoStore.activeRepo.username,
      password: repoStore.activeRepo.password
    } : undefined;

    const response = await GitApi.push(
      repoStore.activeRepo.path,
      'origin',
      currentBranch.value,
      authConfig
    );

    if (response.success) {
      progressMessage.value = 'Push 成功!';
      // 延迟一下让用户看到成功消息
      await new Promise(resolve => setTimeout(resolve, 800));
      // Refresh sync status after push
      await repoStore.refreshSyncStatus();
    } else {
      // 只在失败时显示 toast
      toastStore.error('Push 失败: ' + response.error);
    }
  } catch (error: any) {
    toastStore.error('Push 失败: ' + error.message);
  } finally {
    isPushing.value = false;
    showProgress.value = false;
  }
}

async function handleFetch() {
  if (!repoStore.activeRepo) {
    toastStore.warning('请先打开一个仓库');
    return;
  }

  // 立即显示进度条
  isFetching.value = true;
  progressOperation.value = 'fetch';
  progressMessage.value = '正在从 origin 获取更新...';
  showProgress.value = true;

  // 确保进度条先渲染，然后再执行耗时操作
  await nextTick();
  await new Promise(resolve => setTimeout(resolve, 50));

  try {
    // 获取当前仓库的认证配置
    const authConfig = repoStore.activeRepo ? {
      authType: repoStore.activeRepo.authType || 'none',
      token: repoStore.activeRepo.token,
      username: repoStore.activeRepo.username,
      password: repoStore.activeRepo.password
    } : undefined;

    const response = await GitApi.fetch(repoStore.activeRepo.path, 'origin', authConfig);

    if (response.success) {
      progressMessage.value = 'Fetch 成功!';
      // 延迟一下让用户看到成功消息
      await new Promise(resolve => setTimeout(resolve, 800));
      // Refresh sync status after fetch
      await repoStore.refreshSyncStatus();
    } else {
      // 只在失败时显示 toast
      toastStore.error('Fetch 失败: ' + response.error);
    }
  } catch (error: any) {
    toastStore.error('Fetch 失败: ' + error.message);
  } finally {
    isFetching.value = false;
    showProgress.value = false;
  }
}

function openPublishModal() {
  if (!repoStore.activeRepo) {
    toastStore.warning('请先打开一个仓库');
    return;
  }

  showPublishModal.value = true;
}

function openReleaseManager() {
  if (!repoStore.activeRepo) {
    toastStore.warning('请先打开一个仓库');
    return;
  }

  showReleaseManager.value = true;
}

function handleReleaseSuccess() {
  toastStore.success('发布操作成功!');
  // 刷新仓库数据
  if (repoStore.activeRepo) {
    repoStore.loadRepoData(repoStore.activeRepo);
  }
}

function handleRemoteAdded() {
  // 远程仓库添加成功后的回调
  // 可以在这里刷新仓库状态
  if (repoStore.activeRepo) {
    repoStore.loadRepoData(repoStore.activeRepo);
    checkRemote(); // 重新检查远程仓库状态
  }
}

function toggleBranchMenu() {
  showBranchMenu.value = !showBranchMenu.value;
}

async function switchToBranch(branchName: string) {
  if (!repoStore.activeRepo) return;

  try {
    await repoStore.checkoutBranch(branchName);
    showBranchMenu.value = false;
    toastStore.success('切换分支成功!');
  } catch (error: any) {
    toastStore.error('切换分支失败: ' + error.message);
  }
}

async function createNewBranch() {
  const branchName = prompt('输入新分支名称:');
  if (!branchName || !repoStore.activeRepo) return;

  try {
    await repoStore.createBranch(branchName);
    toastStore.success('创建分支成功!');
  } catch (error: any) {
    toastStore.error('创建分支失败: ' + error.message);
  }
}
</script>

<template>
  <header class="top-bar">
    <!-- Draggable area for window -->
    <div class="drag-region" data-tauri-drag-region></div>

    <!-- 有仓库时显示分支信息 -->
    <div v-if="repoStore.activeRepo" class="branch-info">
      <span class="label">当前分支:</span>
      <div class="branch-selector" @click="toggleBranchMenu">
        <span class="branch-name">{{ currentBranch }}</span>
        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="6 9 12 15 18 9"></polyline>
        </svg>

        <div v-if="showBranchMenu" class="branch-menu" @click.stop>
          <div class="menu-header">
            <span>选择分支</span>
            <button @click="createNewBranch" class="create-branch-btn">新建</button>
          </div>
          <div class="menu-list">
            <div
              v-for="branch in repoStore.branches.filter(b => !b.is_remote)"
              :key="branch.name"
              :class="['branch-item', { active: branch.is_head }]"
              @click="switchToBranch(branch.name)"
            >
              <svg class="branch-icon" xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="6" y1="3" x2="6" y2="15"></line>
                <circle cx="18" cy="6" r="3"></circle>
                <circle cx="6" cy="18" r="3"></circle>
                <path d="M18 9a9 9 0 0 1-9 9"></path>
              </svg>
              <span>{{ branch.name }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Sync Status Display -->
      <div v-if="syncStatusDisplay" :class="['sync-status', syncStatusDisplay.class]" :title="`领先: ${syncStatus?.ahead || 0} 提交, 落后: ${syncStatus?.behind || 0} 提交`">
        <span class="sync-icon">{{ syncStatusDisplay.icon }}</span>
        <span class="sync-text">{{ syncStatusDisplay.text }}</span>
      </div>
    </div>


    <div class="actions">
      <!-- Theme Toggle -->
      <ThemeToggle />

      <!-- Version Display -->
      <VersionDisplay @check-update="emit('check-update')" />

      <!-- Git操作按钮 - 只在有仓库时显示 -->
      <template v-if="repoStore.activeRepo">
        <div class="separator"></div>

        <button
          class="action-btn"
          title="拉取最新代码"
          :disabled="isPulling"
          @click="handlePull"
        >
          <span class="icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
          </span>
          <span>{{ isPulling ? '拉取中...' : '拉取' }}</span>
        </button>
        <button
          class="action-btn"
          title="推送本地提交"
          :disabled="isPushing"
          @click="handlePush"
        >
          <span class="icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
          </span>
          <span>{{ isPushing ? '推送中...' : '推送' }}</span>
        </button>
        <button
          class="action-btn primary"
          title="获取远程更新"
          :disabled="isFetching"
          @click="handleFetch"
        >
          <span class="icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="23 4 23 10 17 10"></polyline><polyline points="1 20 1 14 7 14"></polyline><path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path></svg>
          </span>
          <span>{{ isFetching ? '获取中...' : '获取' }}</span>
        </button>
        <button
          v-if="!hasRemote"
          class="action-btn"
          title="发布/管理远程仓库"
          @click="openPublishModal"
        >
          <span class="icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line></svg>
          </span>
          <span>发布</span>
        </button>

        <div class="separator"></div>

        <button
          v-if="hasRemote && isGitHubRepo"
          class="action-btn release-btn"
          title="发布管理 - 一键构建多平台版本"
          @click="openReleaseManager"
        >
          <span class="icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><polyline points="16 12 12 8 8 12"></polyline><line x1="12" y1="16" x2="12" y2="8"></line></svg>
          </span>
          <span>发布管理</span>
        </button>
      </template>
    </div>

    <!-- Publish/Remote Modal -->
    <PublishModal
      v-if="repoStore.activeRepo"
      :is-open="showPublishModal"
      :repo-path="repoStore.activeRepo.path"
      @close="showPublishModal = false"
      @remote-added="handleRemoteAdded"
      @open-settings="emit('open-global-settings')"
    />

    <!-- Release Manager Modal -->
    <ReleaseManagerModal
      v-if="repoStore.activeRepo"
      :show="showReleaseManager"
      :repo-path="repoStore.activeRepo.path"
      :github-token="settingsStore.settings.githubToken || null"
      @close="showReleaseManager = false"
      @success="handleReleaseSuccess"
    />

    <ProgressBar
      :show="showProgress"
      :operation="progressOperation"
      :message="progressMessage"
    />

    <!-- Confirm Modal -->
    <ConfirmModal
      :show="confirmModal.show"
      :title="confirmModal.title"
      :message="confirmModal.message"
      :type="confirmModal.type"
      :confirm-text="confirmModal.confirmText"
      @confirm="confirmModal.onConfirm"
      @cancel="confirmModal.onCancel"
    />
  </header>
</template>

<style scoped>
.top-bar {
  height: 56px;
  background-color: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-start;
  align-items: center;
  padding: 0 var(--spacing-lg);
  position: relative;
  flex-shrink: 0;
  min-width: 0;
  overflow: visible;
  z-index: 100;
}

:root[data-theme="dark"] .top-bar {
  background-color: rgba(15, 23, 42, 0.8);
}

/* Draggable region covering the entire top bar */
.drag-region {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 0;
  /* This makes the entire top bar draggable */
}

/* Ensure all interactive elements are above the drag region */
.branch-info,
.actions {
  position: relative;
  z-index: 1;
  flex-shrink: 1;
  min-width: 0;
}

.branch-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  flex-shrink: 1;
  min-width: 0;
}


.label {
  color: var(--text-tertiary);
}

.branch-selector {
  position: relative;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 4px;
}

.branch-name {
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  background-color: var(--bg-secondary);
  padding: 4px 12px;
  border-radius: var(--radius-full);
  display: flex;
  align-items: center;
  gap: 8px;
  border: 1px solid var(--border-color);
  transition: all var(--transition-fast);
}

.branch-name:hover {
  border-color: var(--accent-color);
  background-color: var(--bg-hover);
  box-shadow: 0 0 0 2px var(--accent-subtle);
}

.branch-name::before {
  content: '';
  display: block;
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background-color: var(--accent-color);
}

.branch-menu {
  position: absolute;
  top: 100%;
  left: 0;
  margin-top: 8px;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  min-width: 250px;
  z-index: 10000;
}

.menu-header {
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-weight: 600;
  color: var(--text-primary);
}

.create-branch-btn {
  padding: 4px 8px;
  font-size: var(--font-size-xs);
  border-radius: var(--radius-sm);
  background-color: var(--accent-color);
  color: white;
}

.menu-list {
  max-height: 300px;
  overflow-y: auto;
}

.branch-item {
  padding: var(--spacing-sm) var(--spacing-md);
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  cursor: pointer;
  transition: background-color var(--transition-fast);
}

.branch-item:hover {
  background-color: var(--bg-hover);
}

.branch-item.active {
  background-color: rgba(59, 130, 246, 0.1);
  color: var(--accent-color);
  font-weight: 600;
}

.branch-icon {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  color: var(--text-secondary);
}

.branch-item.active .branch-icon {
  color: var(--accent-color);
}

/* Sync Status Styles */
.sync-status {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: var(--radius-full);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  margin-left: var(--spacing-sm);
  border: 1px solid var(--border-color);
}

.sync-status.synced {
  background-color: rgba(34, 197, 94, 0.1);
  color: rgb(22, 163, 74);
  border-color: rgba(34, 197, 94, 0.3);
}

.sync-status.ahead {
  background-color: rgba(59, 130, 246, 0.1);
  color: rgb(37, 99, 235);
  border-color: rgba(59, 130, 246, 0.3);
}

.sync-status.behind {
  background-color: rgba(251, 146, 60, 0.1);
  color: rgb(234, 88, 12);
  border-color: rgba(251, 146, 60, 0.3);
}

.sync-status.diverged {
  background-color: rgba(244, 63, 94, 0.1);
  color: rgb(225, 29, 72);
  border-color: rgba(244, 63, 94, 0.3);
}

.sync-icon {
  font-size: 14px;
  line-height: 1;
}

.sync-text {
  font-size: var(--font-size-xs);
}

.actions {
  display: flex;
  gap: var(--spacing-sm);
  align-items: center;
  flex-wrap: wrap;
  margin-left: auto;
}

.separator {
  width: 1px;
  height: 24px;
  background-color: var(--border-color);
  margin: 0 4px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
  background-color: var(--bg-primary);
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  transition: all var(--transition-fast);
  box-shadow: var(--shadow-sm);
}

.action-btn:hover:not(:disabled) {
  background-color: var(--bg-hover);
  border-color: var(--text-tertiary);
  color: var(--text-primary);
  transform: translateY(-1px);
}

.action-btn:active {
  transform: translateY(0);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.action-btn.primary {
  background-color: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
  box-shadow: 0 1px 2px 0 rgba(37, 99, 235, 0.3);
}

.action-btn.primary:hover:not(:disabled) {
  background-color: var(--accent-hover);
  border-color: var(--accent-hover);
}

.action-btn.release-btn {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-color: #667eea;
  box-shadow: 0 2px 4px 0 rgba(102, 126, 234, 0.4);
}

.action-btn.release-btn:hover:not(:disabled) {
  background: linear-gradient(135deg, #5568d3 0%, #653a8b 100%);
  border-color: #5568d3;
  transform: translateY(-1px);
}

.icon {
  display: flex;
  align-items: center;
}
</style>
