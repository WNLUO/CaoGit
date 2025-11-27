<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import type { Repository } from '../types';
import ChangesView from './ChangesView.vue';
import HistoryView from './HistoryView.vue';
import DiffView from './DiffView.vue';
import Resizer from './Resizer.vue';
import { settingsStore } from '../stores/settingsStore';
import { repoStore } from '../stores/repoStore';

defineProps<{
  repo: Repository;
}>();

const leftPanelWidth = ref(320);
const rightPanelTopHeight = ref(60); // 百分比
let refreshInterval: number | null = null;

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
    // 每3秒自动刷新一次文件状态
    refreshInterval = window.setInterval(() => {
      if (repoStore.activeRepo) {
        repoStore.refreshStatus().catch(error => {
          console.error('Auto-refresh failed:', error);
        });
      }
    }, 3000);
  }
}

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
        <div class="pane-header">提交历史 (HISTORY)</div>
        <HistoryView />
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
}

.left-panel {
  width: 320px;
  min-width: 240px;
  max-width: 600px;
  height: 100%;
  border-right: 1px solid var(--border-color);
  flex-shrink: 0;
}

.right-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  min-width: 0;
}

.top-pane {
  flex: 1;
  min-height: 200px;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}

.bottom-pane {
  flex: 1;
  min-height: 150px;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary);
  min-height: 0;
}

.pane-header {
  padding: var(--spacing-xs) var(--spacing-md);
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-tertiary);
  text-transform: uppercase;
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}
</style>
