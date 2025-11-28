<script setup lang="ts">
import { ref, computed } from 'vue';
import { repoStore } from '../../stores/repoStore';
import { toastStore } from '../../stores/toastStore';
import { GitApi } from '../../services/gitApi';
import VirtualScroller from '../common/VirtualScroller.vue';
import ContextMenu from '../common/ContextMenu.vue';
import type { FilterOptions } from '../common/CommitFilter.vue';
import type { CommitInfo } from '../../types';

interface Props {
  filterOptions?: FilterOptions;
}

const props = withDefaults(defineProps<Props>(), {
  filterOptions: () => ({
    searchText: '',
    author: '',
    dateFrom: '',
    dateTo: '',
    branch: ''
  })
});

const isLoadingMore = ref(false);

// Context menu state
const contextMenu = ref<{
  show: boolean;
  x: number;
  y: number;
  commit: CommitInfo | null;
}>({
  show: false,
  x: 0,
  y: 0,
  commit: null
});

// Multi-selection state
const selectedCommits = ref<Set<string>>(new Set());

const filteredCommits = computed(() => {
  let commits = repoStore.commits;

  // Search text filter (message or hash)
  if (props.filterOptions?.searchText) {
    const searchLower = props.filterOptions.searchText.toLowerCase();
    commits = commits.filter(c =>
      c.message.toLowerCase().includes(searchLower) ||
      c.hash.toLowerCase().includes(searchLower)
    );
  }

  // Author filter
  if (props.filterOptions?.author) {
    const authorLower = props.filterOptions.author.toLowerCase();
    commits = commits.filter(c =>
      c.author.toLowerCase().includes(authorLower) ||
      c.email.toLowerCase().includes(authorLower)
    );
  }

  // Date range filter
  if (props.filterOptions?.dateFrom) {
    const fromDate = new Date(props.filterOptions.dateFrom);
    commits = commits.filter(c => new Date(c.date) >= fromDate);
  }
  if (props.filterOptions?.dateTo) {
    const toDate = new Date(props.filterOptions.dateTo);
    toDate.setHours(23, 59, 59, 999); // End of day
    commits = commits.filter(c => new Date(c.date) <= toDate);
  }

  return commits;
});

const contextMenuItems = computed(() => {
  if (!contextMenu.value.commit) return [];

  const hasMultipleSelected = selectedCommits.value.size > 1;

  return [
    {
      label: hasMultipleSelected ? `Cherry-pick ${selectedCommits.value.size} 提交` : 'Cherry-pick 此提交',
      action: () => cherryPickCommits()
    },
    {
      label: '复制提交哈希',
      action: () => copyCommitHash(contextMenu.value.commit!.hash)
    },
    {
      label: '复制提交信息',
      action: () => copyCommitMessage(contextMenu.value.commit!.message)
    }
  ];
});

function formatDate(dateStr: string) {
  const date = new Date(dateStr);
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  });
}

async function loadMoreCommits() {
  if (isLoadingMore.value || !repoStore.activeRepo) return;

  try {
    isLoadingMore.value = true;
    const currentCount = repoStore.commits.length;
    await repoStore.refreshCommits(100, currentCount);
  } catch (error) {
    console.error('Failed to load more commits:', error);
  } finally {
    isLoadingMore.value = false;
  }
}

function handleCommitClick(commit: CommitInfo, event: MouseEvent) {
  if (event.ctrlKey || event.metaKey) {
    // Multi-select with Ctrl/Cmd
    if (selectedCommits.value.has(commit.hash)) {
      selectedCommits.value.delete(commit.hash);
    } else {
      selectedCommits.value.add(commit.hash);
    }
  } else {
    // Single select
    selectedCommits.value.clear();
    selectedCommits.value.add(commit.hash);
  }
}

function handleContextMenu(commit: CommitInfo, event: MouseEvent) {
  event.preventDefault();

  // If right-clicked commit is not in selection, make it the only selection
  if (!selectedCommits.value.has(commit.hash)) {
    selectedCommits.value.clear();
    selectedCommits.value.add(commit.hash);
  }

  contextMenu.value = {
    show: true,
    x: event.clientX,
    y: event.clientY,
    commit
  };
}

function closeContextMenu() {
  contextMenu.value.show = false;
}

async function cherryPickCommits() {
  if (!repoStore.activeRepo) return;

  const commits = Array.from(selectedCommits.value);

  try {
    if (commits.length === 1) {
      const result = await GitApi.cherryPick(repoStore.activeRepo.path, commits[0]);
      if (result.success && result.data) {
        toastStore.success(result.data);
        await Promise.all([
          repoStore.refreshStatus(),
          repoStore.refreshCommits(),
          repoStore.refreshBranches()
        ]);
      } else {
        toastStore.error(`Cherry-pick 失败: ${result.error}`);
      }
    } else {
      const result = await GitApi.cherryPickBatch(repoStore.activeRepo.path, commits);
      if (result.success && result.data) {
        toastStore.success(`批量 Cherry-pick 结果:\n${result.data.join('\n')}`);
        await Promise.all([
          repoStore.refreshStatus(),
          repoStore.refreshCommits(),
          repoStore.refreshBranches()
        ]);
      } else {
        toastStore.error(`批量 Cherry-pick 失败: ${result.error}`);
      }
    }
  } catch (error) {
    console.error('Cherry-pick failed:', error);
    toastStore.error(`Cherry-pick 错误: ${error}`);
  } finally {
    selectedCommits.value.clear();
  }
}

function copyCommitHash(hash: string) {
  navigator.clipboard.writeText(hash);
  toastStore.success(`已复制提交哈希: ${hash.substring(0, 7)}`);
}

function copyCommitMessage(message: string) {
  navigator.clipboard.writeText(message);
  toastStore.success('已复制提交信息');
}

function isCommitSelected(hash: string) {
  return selectedCommits.value.has(hash);
}
</script>

<template>
  <div class="history-view">
    <div class="history-header">
      <table class="commit-table-header">
        <thead>
          <tr>
            <th class="col-graph">Graph</th>
            <th class="col-message">提交信息 (Message)</th>
            <th class="col-author">作者 (Author)</th>
            <th class="col-date">日期 (Date)</th>
            <th class="col-hash">Hash</th>
          </tr>
        </thead>
      </table>
    </div>

    <div class="history-content">
      <div v-if="filteredCommits.length === 0" class="empty-state">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <circle cx="11" cy="11" r="8"></circle>
          <path d="m21 21-4.35-4.35"></path>
        </svg>
        <p>没有找到匹配的提交</p>
        <p class="hint">尝试调整搜索条件或过滤器</p>
      </div>

      <VirtualScroller
        v-else
        :items="filteredCommits"
        :item-height="40"
        :visible-count="20"
        :buffer="5"
        @load-more="loadMoreCommits"
      >
        <template #default="{ item: commit }">
          <div
            class="commit-row"
            :class="{ selected: isCommitSelected(commit.hash) }"
            @click="handleCommitClick(commit, $event)"
            @contextmenu="handleContextMenu(commit, $event)"
          >
            <div class="col-graph">
              <span class="graph-node">●</span>
              <div class="graph-line"></div>
            </div>
            <div class="col-message" :title="commit.message">{{ commit.message }}</div>
            <div class="col-author">{{ commit.author }}</div>
            <div class="col-date">{{ formatDate(commit.date) }}</div>
            <div class="col-hash">{{ commit.hash.substring(0, 7) }}</div>
          </div>
        </template>
      </VirtualScroller>

      <div v-if="isLoadingMore" class="loading-indicator">
        加载更多提交...
      </div>
    </div>

    <ContextMenu
      v-if="contextMenu.show"
      :items="contextMenuItems"
      :x="contextMenu.x"
      :y="contextMenu.y"
      @close="closeContextMenu"
    />
  </div>
</template>

<style scoped>
.history-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary);
  overflow: hidden;
}

.history-header {
  flex-shrink: 0;
  border-bottom: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
}

.commit-table-header {
  width: 100%;
  border-collapse: collapse;
  font-size: var(--font-size-sm);
}

.commit-table-header th {
  text-align: left;
  padding: var(--spacing-sm);
  color: var(--text-secondary);
  font-weight: 600;
}

.history-content {
  flex: 1;
  overflow: hidden;
  position: relative;
  min-height: 0;
}

.commit-row {
  display: flex;
  align-items: center;
  height: 100%;
  border-bottom: 1px solid var(--border-color);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: background-color 0.15s;
}

.commit-row:hover {
  background-color: var(--bg-secondary);
}

.commit-row.selected {
  background-color: var(--accent-color);
  color: white;
}

.commit-row.selected .col-message,
.commit-row.selected .col-author,
.commit-row.selected .col-date,
.commit-row.selected .col-hash {
  color: white;
}

.col-graph {
  width: 50px;
  flex-shrink: 0;
  text-align: center;
  color: var(--accent-color);
  position: relative;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.graph-node {
  position: relative;
  z-index: 2;
  background-color: var(--bg-primary);
  border-radius: 50%;
}

.graph-line {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 50%;
  width: 2px;
  background-color: var(--border-color);
  transform: translateX(-50%);
  z-index: 1;
}

.col-message {
  flex: 1;
  font-weight: 500;
  padding: 0 var(--spacing-sm);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--text-primary);
}

.col-author, .col-date, .col-hash {
  color: var(--text-secondary);
  padding: 0 var(--spacing-sm);
  flex-shrink: 0;
}

.col-author {
  width: 150px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.col-date {
  width: 150px;
}

.col-hash {
  font-family: monospace;
  width: 100px;
}

.loading-indicator {
  position: absolute;
  bottom: 10px;
  left: 50%;
  transform: translateX(-50%);
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--bg-secondary);
  border-radius: var(--border-radius);
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary);
  gap: 12px;
}

.empty-state svg {
  opacity: 0.5;
}

.empty-state p {
  margin: 0;
  font-size: var(--font-size-base);
}

.empty-state .hint {
  font-size: var(--font-size-sm);
  opacity: 0.7;
}
</style>
