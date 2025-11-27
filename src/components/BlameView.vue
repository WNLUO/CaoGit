<script setup lang="ts">
import { ref } from 'vue';
import { GitApi } from '../services/gitApi';
import type { BlameLine } from '../types';

interface Props {
  repoPath: string;
  filePath: string;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  close: [];
  jumpToCommit: [hash: string];
}>();

const blameData = ref<BlameLine[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);
const hoveredLine = ref<number | null>(null);

// Load blame data
async function loadBlame() {
  if (!props.filePath) {
    error.value = 'No file selected';
    return;
  }

  isLoading.value = true;
  error.value = null;

  try {
    const result = await GitApi.getFileBlame(props.repoPath, props.filePath);
    if (result.success && result.data) {
      blameData.value = result.data;
    } else {
      error.value = result.error || 'Failed to load blame data';
    }
  } catch (err) {
    error.value = `Error: ${err}`;
  } finally {
    isLoading.value = false;
  }
}

// Format date to relative time
function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSecs = Math.floor(diffMs / 1000);
  const diffMins = Math.floor(diffSecs / 60);
  const diffHours = Math.floor(diffMins / 60);
  const diffDays = Math.floor(diffHours / 24);

  if (diffDays > 30) {
    return date.toLocaleDateString('zh-CN', { year: 'numeric', month: '2-digit', day: '2-digit' });
  } else if (diffDays > 0) {
    return `${diffDays} 天前`;
  } else if (diffHours > 0) {
    return `${diffHours} 小时前`;
  } else if (diffMins > 0) {
    return `${diffMins} 分钟前`;
  } else {
    return '刚刚';
  }
}

// Get unique color for author
function getAuthorColor(author: string): string {
  let hash = 0;
  for (let i = 0; i < author.length; i++) {
    hash = author.charCodeAt(i) + ((hash << 5) - hash);
  }
  const hue = Math.abs(hash % 360);
  return `hsl(${hue}, 65%, 45%)`;
}

function handleLineClick(line: BlameLine) {
  emit('jumpToCommit', line.commit_hash);
}

function copyCommitHash(hash: string) {
  navigator.clipboard.writeText(hash);
}

// Load on mount
loadBlame();
</script>

<template>
  <div class="blame-view">
    <div class="blame-header">
      <div class="file-info">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M4 1.5a.5.5 0 0 1 .5-.5h5.586a.5.5 0 0 1 .353.146l2.5 2.5a.5.5 0 0 1 .147.354V14.5a.5.5 0 0 1-.5.5h-9a.5.5 0 0 1-.5-.5V1.5zm6 0v2a.5.5 0 0 0 .5.5h2L10 1.5z"/>
        </svg>
        <span class="file-path">{{ filePath }}</span>
      </div>
      <button class="btn-close" @click="emit('close')" title="关闭">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8 2.146 2.854z"/>
        </svg>
      </button>
    </div>

    <div v-if="isLoading" class="loading-state">
      <div class="spinner"></div>
      <p>加载 Blame 数据中...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <svg width="48" height="48" viewBox="0 0 16 16" fill="currentColor">
        <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/>
        <path d="M7.002 11a1 1 0 1 1 2 0 1 1 0 0 1-2 0zM7.1 4.995a.905.905 0 1 1 1.8 0l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 4.995z"/>
      </svg>
      <p>{{ error }}</p>
      <button class="btn-retry" @click="loadBlame">重试</button>
    </div>

    <div v-else-if="blameData.length === 0" class="empty-state">
      <p>无法获取文件的 Blame 信息</p>
    </div>

    <div v-else class="blame-content">
      <div class="blame-table-header">
        <div class="col-commit">提交</div>
        <div class="col-author">作者</div>
        <div class="col-date">日期</div>
        <div class="col-line">行号</div>
        <div class="col-code">代码</div>
      </div>

      <div class="blame-lines">
        <div
          v-for="line in blameData"
          :key="line.line_number"
          class="blame-line"
          :class="{ hovered: hoveredLine === line.line_number }"
          @mouseenter="hoveredLine = line.line_number"
          @mouseleave="hoveredLine = null"
          @click="handleLineClick(line)"
        >
          <div class="col-commit">
            <span
              class="commit-hash"
              :title="`提交: ${line.commit_hash}\n点击跳转到此提交`"
              @click.stop="copyCommitHash(line.commit_hash)"
            >
              {{ line.commit_hash.substring(0, 7) }}
            </span>
          </div>
          <div class="col-author">
            <span
              class="author-badge"
              :style="{ backgroundColor: getAuthorColor(line.author) }"
              :title="`${line.author} <${line.author_email}>`"
            >
              {{ line.author }}
            </span>
          </div>
          <div class="col-date" :title="new Date(line.date).toLocaleString('zh-CN')">
            {{ formatDate(line.date) }}
          </div>
          <div class="col-line">{{ line.line_number }}</div>
          <div class="col-code">
            <code>{{ line.content }}</code>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.blame-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.blame-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md);
  border-bottom: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
}

.file-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  color: var(--text-primary);
  font-weight: 500;
}

.file-path {
  font-family: monospace;
  font-size: var(--font-size-sm);
}

.btn-close {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s;
}

.btn-close:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  gap: var(--spacing-md);
  color: var(--text-secondary);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--border-color);
  border-top-color: var(--accent-color);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-state svg {
  color: var(--error-color);
}

.btn-retry {
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--accent-color);
  color: white;
  border: none;
  border-radius: var(--border-radius);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: opacity 0.2s;
}

.btn-retry:hover {
  opacity: 0.9;
}

.blame-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.blame-table-header {
  display: flex;
  padding: var(--spacing-sm);
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  font-size: var(--font-size-xs);
  font-weight: 600;
  color: var(--text-secondary);
}

.blame-lines {
  flex: 1;
  overflow-y: auto;
  font-size: var(--font-size-sm);
}

.blame-line {
  display: flex;
  border-bottom: 1px solid var(--border-color);
  cursor: pointer;
  transition: background-color 0.15s;
}

.blame-line:hover,
.blame-line.hovered {
  background-color: var(--bg-secondary);
}

.col-commit {
  width: 80px;
  flex-shrink: 0;
  padding: var(--spacing-xs) var(--spacing-sm);
}

.col-author {
  width: 150px;
  flex-shrink: 0;
  padding: var(--spacing-xs) var(--spacing-sm);
}

.col-date {
  width: 120px;
  flex-shrink: 0;
  padding: var(--spacing-xs) var(--spacing-sm);
  color: var(--text-secondary);
  font-size: var(--font-size-xs);
}

.col-line {
  width: 60px;
  flex-shrink: 0;
  padding: var(--spacing-xs) var(--spacing-sm);
  text-align: right;
  color: var(--text-secondary);
  font-family: monospace;
}

.col-code {
  flex: 1;
  padding: var(--spacing-xs) var(--spacing-sm);
  overflow-x: auto;
}

.col-code code {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: var(--font-size-xs);
  white-space: pre;
  color: var(--text-primary);
}

.commit-hash {
  font-family: monospace;
  font-size: var(--font-size-xs);
  color: var(--accent-color);
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 3px;
  transition: background-color 0.2s;
}

.commit-hash:hover {
  background-color: var(--bg-hover);
}

.author-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: var(--font-size-xs);
  color: white;
  font-weight: 500;
  max-width: 130px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* Scrollbar styling */
.blame-lines::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.blame-lines::-webkit-scrollbar-track {
  background: var(--bg-secondary);
}

.blame-lines::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 4px;
}

.blame-lines::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}
</style>
