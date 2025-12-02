<script setup lang="ts">
import { ref, onMounted } from 'vue';

import Sidebar from "./components/layout/Sidebar.vue";
import TopBar from "./components/layout/TopBar.vue";
import RepoMain from "./components/RepoMain.vue";
import SettingsModal from "./components/modals/SettingsModal.vue";
import DebugErrorDialog from "./components/modals/DebugErrorDialog.vue";
import UpdateDialog from "./components/modals/UpdateDialog.vue";
import AddRepoModal from "./components/modals/AddRepoModal.vue";
import Resizer from "./components/layout/Resizer.vue";
import Toast from "./components/common/Toast.vue";
import { settingsStore } from "./stores/settingsStore";
import { toastStore } from "./stores/toastStore";
import type { Repository } from "./types";

const isSettingsOpen = ref(false);
const settingsMode = ref<'global' | 'repo'>('global');
const selectedRepo = ref<Repository | undefined>(undefined);
const activeRepo = ref<Repository | undefined>(undefined);
const isAddRepoOpen = ref(false);
const sidebarWidth = ref(240);
const updateDialogRef = ref<InstanceType<typeof UpdateDialog> | null>(null);

function openGlobalSettings() {
  settingsMode.value = 'global';
  selectedRepo.value = undefined;
  isSettingsOpen.value = true;
}

function openRepoSettings(repo: Repository) {
  settingsMode.value = 'repo';
  selectedRepo.value = repo;
  isSettingsOpen.value = true;
}

function selectRepo(repo: Repository) {
  activeRepo.value = repo;
}

function showAddRepoModal() {
  isAddRepoOpen.value = true;
}

function handleRepoAdded(_path: string) {
  // Repo was added successfully
  isAddRepoOpen.value = false;
  // TODO: Refresh repo list or select the new repo
}

onMounted(() => {
  // 从设置中恢复 Sidebar 宽度
  sidebarWidth.value = settingsStore.settings.layout.sidebarWidth ?? 240;
});

function handleSidebarResize(delta: number) {
  // 最小宽度设置为网络状态框的宽度 (240px)
  const newWidth = Math.max(240, Math.min(400, sidebarWidth.value + delta));
  sidebarWidth.value = newWidth;
  settingsStore.updateLayoutSettings({ sidebarWidth: newWidth });
}

function handleCheckUpdate() {
  // 手动检查更新时，触发 UpdateDialog 显示
  if (updateDialogRef.value) {
    updateDialogRef.value.checkForUpdates();
  }
}
</script>

<template>
  <div class="app-layout">
    <Sidebar
      :style="{ width: sidebarWidth + 'px' }"
      @open-global-settings="openGlobalSettings"
      @open-repo-settings="openRepoSettings"
      @select-repo="selectRepo"
    />
    <Resizer direction="horizontal" @resize="handleSidebarResize" />
    <main class="main-content">
      <TopBar @open-global-settings="openGlobalSettings" @check-update="handleCheckUpdate" />
      <div class="content-area">
        <RepoMain v-if="activeRepo" :repo="activeRepo" />
        <div v-else class="welcome-container">
          <div class="welcome-content">
            <div class="welcome-icon">
              <svg width="120" height="120" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
              </svg>
            </div>

            <h1>欢迎使用 Git管理器</h1>
            <p class="welcome-subtitle">现代化的 Git 仓库管理工具</p>

            <div class="quick-actions">
              <button class="action-btn primary" @click="showAddRepoModal">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="12" y1="5" x2="12" y2="19"></line>
                  <line x1="5" y1="12" x2="19" y2="12"></line>
                </svg>
                添加仓库
              </button>
              <button class="action-btn secondary" @click="openGlobalSettings">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="3"></circle>
                  <path d="M12 1v6m0 6v6m9-9h-6m-6 0H3m15.364 6.364l-4.243-4.243m-6.364 0l-4.243 4.243m12.728 0l-4.242-4.243m-6.364 0L3.636 15.364"></path>
                </svg>
                全局设置
              </button>
            </div>
          </div>
        </div>
      </div>
    </main>

    <SettingsModal
      :is-open="isSettingsOpen"
      :mode="settingsMode"
      :repo="selectedRepo"
      @close="isSettingsOpen = false"
    />

    <AddRepoModal
      :is-open="isAddRepoOpen"
      @close="isAddRepoOpen = false"
      @repo-added="handleRepoAdded"
    />

    <DebugErrorDialog />

    <UpdateDialog ref="updateDialogRef" />

    <!-- Toast Notifications -->
    <Toast
      :messages="toastStore.messages"
      @remove="(id) => toastStore.remove(id)"
    />
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  min-width: 0;
}

.app-layout > :first-child {
  flex-shrink: 0;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary);
  min-width: 0;
  overflow: hidden;
}

.content-area {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  min-width: 0;
  min-height: 0;
}

.welcome-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  overflow-y: auto;
  padding: var(--spacing-xl);
}

.welcome-content {
  max-width: 900px;
  width: 100%;
}

.welcome-icon {
  text-align: center;
  margin-bottom: var(--spacing-lg);
  color: var(--accent-color);
  opacity: 0.8;
}

h1 {
  font-size: 2.5rem;
  margin-bottom: var(--spacing-sm);
  color: var(--text-primary);
  text-align: center;
  font-weight: 700;
}

.welcome-subtitle {
  text-align: center;
  font-size: var(--font-size-lg);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xl);
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-xl);
}

.feature-card {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  text-align: center;
  transition: all var(--transition-normal);
}

.feature-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
  border-color: var(--accent-color);
}

.feature-icon {
  font-size: 2.5rem;
  margin-bottom: var(--spacing-sm);
}

.feature-card h3 {
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.feature-card p {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.5;
}

.quick-actions {
  display: flex;
  gap: var(--spacing-md);
  justify-content: center;
  margin-bottom: var(--spacing-xl);
}

.action-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border-radius: var(--radius-md);
  font-size: var(--font-size-base);
  font-weight: 600;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.action-btn svg {
  flex-shrink: 0;
}

.action-btn.primary {
  background-color: var(--accent-color);
  color: white;
}

.action-btn.primary:hover {
  background-color: var(--accent-hover);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.action-btn.secondary {
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.action-btn.secondary:hover {
  background-color: var(--bg-tertiary);
  border-color: var(--accent-color);
}

.tips {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  border-left: 4px solid var(--accent-color);
}

.tip-header {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: var(--spacing-md);
}

.tips ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.tips li {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  padding: var(--spacing-xs) 0;
  line-height: 1.6;
  position: relative;
  padding-left: var(--spacing-lg);
}

.tips li::before {
  content: "→";
  position: absolute;
  left: 0;
  color: var(--accent-color);
  font-weight: bold;
}

.tips li strong {
  color: var(--text-primary);
  font-weight: 600;
}
</style>