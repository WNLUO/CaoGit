<script setup lang="ts">
import { ref, computed } from 'vue';
import { repoStore } from '../stores/repoStore';
import { GitApi } from '../services/gitApi';
import PublishModal from './PublishModal.vue';
import ThemeToggle from './ThemeToggle.vue';

const emit = defineEmits<{
  (e: 'open-global-settings'): void;
}>();

const currentBranch = computed(() => repoStore.currentBranch);
const isPushing = ref(false);
const isPulling = ref(false);
const isFetching = ref(false);
const showBranchMenu = ref(false);
const showPublishModal = ref(false);

async function handlePull() {
  if (!repoStore.activeRepo) {
    alert('请先打开一个仓库');
    return;
  }

  isPulling.value = true;
  try {
    const response = await GitApi.pull(
      repoStore.activeRepo.path,
      'origin',
      currentBranch.value
    );

    if (response.success) {
      alert('Pull 成功!');
      await repoStore.loadRepoData(repoStore.activeRepo);
    } else {
      alert('Pull 失败: ' + response.error);
    }
  } catch (error: any) {
    alert('Pull 失败: ' + error.message);
  } finally {
    isPulling.value = false;
  }
}

async function handlePush() {
  if (!repoStore.activeRepo) {
    alert('请先打开一个仓库');
    return;
  }

  isPushing.value = true;
  try {
    const response = await GitApi.push(
      repoStore.activeRepo.path,
      'origin',
      currentBranch.value
    );

    if (response.success) {
      alert('Push 成功!');
    } else {
      alert('Push 失败: ' + response.error);
    }
  } catch (error: any) {
    alert('Push 失败: ' + error.message);
  } finally {
    isPushing.value = false;
  }
}

async function handleFetch() {
  if (!repoStore.activeRepo) {
    alert('请先打开一个仓库');
    return;
  }

  isFetching.value = true;
  try {
    const response = await GitApi.fetch(repoStore.activeRepo.path, 'origin');

    if (response.success) {
      alert('Fetch 成功!');
    } else {
      alert('Fetch 失败: ' + response.error);
    }
  } catch (error: any) {
    alert('Fetch 失败: ' + error.message);
  } finally {
    isFetching.value = false;
  }
}

function openPublishModal() {
  if (!repoStore.activeRepo) {
    alert('请先打开一个仓库');
    return;
  }

  showPublishModal.value = true;
}

function handleRemoteAdded() {
  // 远程仓库添加成功后的回调
  // 可以在这里刷新仓库状态
  if (repoStore.activeRepo) {
    repoStore.loadRepoData(repoStore.activeRepo);
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
    alert('切换分支成功!');
  } catch (error: any) {
    alert('切换分支失败: ' + error.message);
  }
}

async function createNewBranch() {
  const branchName = prompt('输入新分支名称:');
  if (!branchName || !repoStore.activeRepo) return;

  try {
    await repoStore.createBranch(branchName);
    alert('创建分支成功!');
  } catch (error: any) {
    alert('创建分支失败: ' + error.message);
  }
}
</script>

<template>
  <header class="top-bar">
    <!-- Draggable area for window -->
    <div class="drag-region" data-tauri-drag-region></div>

    <div class="branch-info">
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
              <span class="branch-icon">🔀</span>
              <span>{{ branch.name }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="actions">
      <!-- Theme Toggle -->
      <ThemeToggle />

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
        class="action-btn"
        title="发布/管理远程仓库"
        @click="openPublishModal"
      >
        <span class="icon">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line></svg>
        </span>
        <span>发布</span>
      </button>
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
  </header>
</template>

<style scoped>
.top-bar {
  height: 64px;
  background-color: var(--bg-primary);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 var(--spacing-lg);
  position: relative;
  flex-shrink: 0;
  min-width: 0;
  overflow: hidden;
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
  padding: 4px 10px;
  border-radius: var(--radius-full);
  display: flex;
  align-items: center;
  gap: 6px;
  border: 1px solid var(--border-color);
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
  z-index: 1000;
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

.actions {
  display: flex;
  gap: var(--spacing-sm);
  align-items: center;
  flex-wrap: wrap;
  justify-content: flex-end;
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

.icon {
  display: flex;
  align-items: center;
}
</style>
