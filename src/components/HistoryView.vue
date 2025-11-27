<script setup lang="ts">
import { ref, computed } from 'vue';
import { repoStore } from '../stores/repoStore';
import VirtualScroller from './VirtualScroller.vue';
import CommitFilter from './CommitFilter.vue';
import type { Commit } from '../types';
import type { FilterOptions } from './CommitFilter.vue';

const isLoadingMore = ref(false);
const filterOptions = ref<FilterOptions>({
  searchText: '',
  author: '',
  dateFrom: '',
  dateTo: '',
  branch: ''
});

const branchNames = computed(() => {
  return repoStore.branches.map(b => b.name);
});

const filteredCommits = computed(() => {
  let commits = repoStore.commits;

  // Search text filter (message or hash)
  if (filterOptions.value.searchText) {
    const searchLower = filterOptions.value.searchText.toLowerCase();
    commits = commits.filter(c =>
      c.message.toLowerCase().includes(searchLower) ||
      c.hash.toLowerCase().includes(searchLower)
    );
  }

  // Author filter
  if (filterOptions.value.author) {
    const authorLower = filterOptions.value.author.toLowerCase();
    commits = commits.filter(c =>
      c.author.toLowerCase().includes(authorLower) ||
      c.email.toLowerCase().includes(authorLower)
    );
  }

  // Date range filter
  if (filterOptions.value.dateFrom) {
    const fromDate = new Date(filterOptions.value.dateFrom);
    commits = commits.filter(c => new Date(c.date) >= fromDate);
  }
  if (filterOptions.value.dateTo) {
    const toDate = new Date(filterOptions.value.dateTo);
    toDate.setHours(23, 59, 59, 999); // End of day
    commits = commits.filter(c => new Date(c.date) <= toDate);
  }

  return commits;
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

function handleFilter(options: FilterOptions) {
  filterOptions.value = options;
}
</script>

<template>
  <div class="history-view">
    <CommitFilter
      :branches="branchNames"
      @filter="handleFilter"
    />

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
          <div class="commit-row">
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
  </div>
</template>

<style scoped>
.history-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary);
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
