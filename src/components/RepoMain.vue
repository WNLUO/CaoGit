<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import type { Repository } from '../types';
import ChangesView from './views/ChangesView.vue';
import HistoryView from './views/HistoryView.vue';
import DiffView from './views/DiffView.vue';
import Resizer from './layout/Resizer.vue';
import CommitFilter from './common/CommitFilter.vue';
import type { FilterOptions } from './common/CommitFilter.vue';
import { settingsStore } from '../stores/settingsStore';
import { repoStore } from '../stores/repoStore';

defineProps<{
  repo: Repository;
}>();

const leftPanelWidth = ref(320);
const rightPanelTopHeight = ref(60); // 百分比
let refreshInterval: number | null = null;
let isUserOperating = ref(false);
let lastOperationTime = 0;

const historyFilterOptions = ref<FilterOptions>({
  searchText: '',
  author: '',
  dateFrom: '',
  dateTo: '',
  branch: ''
});

onMounted(() => {
  // 从设置中恢复面板宽度和高度
  leftPanelWidth.value = settingsStore.settings.layout.leftPanelWidth ?? 320;
  rightPanelTopHeight.value = settingsStore.settings.layout.rightPanelTopHeight ?? 60;

  // 启动文件变更自动监控
  startAutoRefresh();
});

onUnmounted(() => {
  stopAutoRefresh();
});

// 监听活跃仓库变化，重启自动刷新
watch(() => repoStore.activeRepo, (newRepo) => {
  stopAutoRefresh();
  if (newRepo) {
    startAutoRefresh();
  }
});

function startAutoRefresh() {
  if (repoStore.activeRepo) {
    // 每5秒自动刷新一次文件状态 (增加间隔减少冲突)
    refreshInterval = window.setInterval(() => {
      if (repoStore.activeRepo && !isUserOperating.value) {
        // Only refresh if user hasn't operated recently (within last 2 seconds)
        const timeSinceLastOp = Date.now() - lastOperationTime;
        if (timeSinceLastOp > 2000) {
          repoStore.refreshStatus().catch(error => {
            console.error('Auto-refresh failed:', error);
          });
        }
      }
    }, 5000); // Increased from 3s to 5s
  }
}

// Track user operations to prevent refresh conflicts
// This function should be called when user performs operations like stage/unstage
// For now, we'll export it for future use
const markUserOperation = () => {
  lastOperationTime = Date.now();
  isUserOperating.value = true;

  // Reset after 2 seconds
  setTimeout(() => {
    isUserOperating.value = false;
  }, 2000);
};

// Export for potential use by child components
defineExpose({ markUserOperation });

function stopAutoRefresh() {
  if (refreshInterval !== null) {
    clearInterval(refreshInterval);
    refreshInterval = null;
  }
}

function handleLeftPanelResize(delta: number) {
  // 最小宽度设置为网络状态框的宽度 (240px)
  const newWidth = Math.max(240, Math.min(600, leftPanelWidth.value + delta));
  leftPanelWidth.value = newWidth;
  settingsStore.updateLayoutSettings({ leftPanelWidth: newWidth });
}

function handleTopPaneResize(delta: number) {
  // 获取右面板的高度
  const rightPanel = document.querySelector('.right-panel') as HTMLElement;
  if (!rightPanel) return;

  const rightPanelHeight = rightPanel.offsetHeight;
  const newTopHeight = Math.max(20, Math.min(80, rightPanelTopHeight.value + (delta / rightPanelHeight) * 100));
  rightPanelTopHeight.value = newTopHeight;
  settingsStore.updateLayoutSettings({ rightPanelTopHeight: newTopHeight });
}

function handleHistoryFilter(options: FilterOptions) {
  historyFilterOptions.value = options;
}
</script>

<template>
  <div class="repo-main">
    <!-- Left Panel: Changes & Commit -->
    <div class="left-panel" :style="{ width: leftPanelWidth + 'px' }">
      <ChangesView />
    </div>

    <!-- Horizontal Resizer between Left and Right -->
    <Resizer direction="horizontal" @resize="handleLeftPanelResize" />

    <!-- Right Panel: Diff & History -->
    <div class="right-panel">
      <div class="top-pane" :style="{ flex: `${rightPanelTopHeight}%` }">
        <DiffView />
      </div>

      <!-- Vertical Resizer between Top and Bottom -->
      <Resizer direction="vertical" @resize="handleTopPaneResize" />

      <div class="bottom-pane" :style="{ flex: `${100 - rightPanelTopHeight}%` }">
        <div class="pane-header-container">
          <div class="pane-header-title">提交历史</div>
          <div class="pane-header-actions">
            <CommitFilter
              :branches="[]"
              @filter="handleHistoryFilter"
            />
          </div>
        </div>
        <HistoryView :filter-options="historyFilterOptions" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.repo-main {
  display: flex;
  flex: 1;
  height: 100%;
  overflow: hidden;
  min-width: 0;
  padding: var(--spacing-md);
  gap: var(--spacing-xs);
}

.left-panel {
  width: 320px;
  min-width: 240px;
  max-width: 600px;
  height: 100%;
  flex-shrink: 0;
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.5);
  border-radius: var(--radius-lg);
  overflow: hidden;
  box-shadow: var(--shadow-sm);
}

:root[data-theme="dark"] .left-panel {
  background: rgba(30, 41, 59, 0.6);
  border-color: rgba(255, 255, 255, 0.05);
}

.right-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  min-width: 0;
  gap: var(--spacing-xs);
}

.top-pane {
  flex: 1;
  min-height: 200px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.5);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
}

:root[data-theme="dark"] .top-pane {
  background: rgba(30, 41, 59, 0.6);
  border-color: rgba(255, 255, 255, 0.05);
}

.bottom-pane {
  flex: 1;
  min-height: 150px;
  display: flex;
  flex-direction: column;
  min-height: 0;
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.5);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
}

:root[data-theme="dark"] .bottom-pane {
  background: rgba(30, 41, 59, 0.6);
  border-color: rgba(255, 255, 255, 0.05);
}

.pane-header-container {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-xs) var(--spacing-md);
  background-color: rgba(255, 255, 255, 0.3);
  border-bottom: 1px solid var(--border-color);
  gap: var(--spacing-md);
  flex-shrink: 0;
}

:root[data-theme="dark"] .pane-header-container {
  background-color: rgba(0, 0, 0, 0.2);
}

.pane-header-title {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-tertiary);
  text-transform: uppercase;
  white-space: nowrap;
  letter-spacing: 0.05em;
}

.pane-header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  flex: 1;
  justify-content: flex-end;
}
</style>
