<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { repoStore } from '../../stores/repoStore';
import { GitApi } from '../../services/gitApi';
import DiffStats from '../common/DiffStats.vue';
import type { DiffData, DiffStats as DiffStatsType } from '../../types';

type DiffMode = 'inline' | 'side-by-side';

const diffMode = ref<DiffMode>('side-by-side');
const diffData = ref<DiffData | null>(null);
const isLoading = ref(false);

const selectedFile = computed(() => repoStore.selectedFile);

// 计算变更统计
const diffStats = computed<DiffStatsType>(() => {
  if (!diffData.value || !diffData.value.hunks) {
    return { additions: 0, deletions: 0, total: 0 };
  }

  let additions = 0;
  let deletions = 0;

  diffData.value.hunks.forEach(hunk => {
    hunk.lines.forEach(line => {
      if (line.origin === '+') additions++;
      if (line.origin === '-') deletions++;
    });
  });

  return { additions, deletions, total: additions + deletions };
});

watch(selectedFile, async (file) => {
  if (!file || !repoStore.activeRepo) {
    diffData.value = null;
    return;
  }

  isLoading.value = true;

  try {
    const response = await GitApi.getFileDiff(
      repoStore.activeRepo.path,
      file.path,
      file.staged
    );

    if (response.success && response.data) {
      diffData.value = response.data;
    }
  } catch (error) {
    console.error('Failed to load diff:', error);
  } finally {
    isLoading.value = false;
  }
});

function getLineClass(origin: string) {
  switch (origin) {
    case '+': return 'line-add';
    case '-': return 'line-remove';
    default: return 'line-context';
  }
}

function getLinePrefix(origin: string) {
  switch (origin) {
    case '+': return '+';
    case '-': return '-';
    default: return ' ';
  }
}

// 获取文件状态徽章颜色
function getStatusBadgeColor(status: string) {
  switch (status) {
    case 'added': return '#10b981';
    case 'modified': return '#eab308';
    case 'deleted': return '#ef4444';
    case 'renamed': return '#3b82f6';
    default: return 'var(--text-tertiary)';
  }
}

function getStatusText(status: string) {
  switch (status) {
    case 'added': return '新增';
    case 'modified': return '修改';
    case 'deleted': return '删除';
    case 'renamed': return '重命名';
    case 'untracked': return '未跟踪';
    default: return status;
  }
}
</script>

<template>
  <div class="enhanced-diff-view">
    <div class="diff-header">
      <div class="file-info">
        <div v-if="selectedFile" class="file-details">
          <span class="file-name">{{ selectedFile.path }}</span>
          <span
            class="file-status-badge"
            :style="{ backgroundColor: getStatusBadgeColor(selectedFile.status) }"
          >
            {{ getStatusText(selectedFile.status) }}
          </span>
          <DiffStats v-if="diffStats.total > 0" :stats="diffStats" compact />
        </div>
        <span v-else class="no-selection">选择文件以查看差异</span>
      </div>

      <div class="diff-controls">
        <button
          :class="{ active: diffMode === 'inline' }"
          @click="diffMode = 'inline'"
        >
          内联模式
        </button>
        <button
          :class="{ active: diffMode === 'side-by-side' }"
          @click="diffMode = 'side-by-side'"
        >
          并排模式
        </button>
      </div>
    </div>

    <div class="diff-content" v-if="diffData && !isLoading && diffData.hunks && diffData.hunks.length > 0">
      <!-- Inline Mode -->
      <div v-if="diffMode === 'inline'" class="diff-inline">
        <div
          v-for="(hunk, hunkIndex) in diffData.hunks"
          :key="hunkIndex"
          class="hunk"
        >
          <div class="hunk-header">{{ hunk.header }}</div>
          <div v-for="(line, lineIndex) in hunk.lines" :key="lineIndex"
               :class="['diff-line', getLineClass(line.origin)]">
            <span class="line-number old-number">{{ line.old_lineno || '' }}</span>
            <span class="line-number new-number">{{ line.new_lineno || '' }}</span>
            <span class="line-content">
              <span class="line-prefix">{{ getLinePrefix(line.origin) }}</span>{{ line.content }}
            </span>
          </div>
        </div>
      </div>

      <!-- Side-by-side Mode -->
      <div v-else class="diff-side-by-side">
        <div
          v-for="(hunk, hunkIndex) in diffData.hunks"
          :key="hunkIndex"
          class="hunk"
        >
          <div class="hunk-header-row">
            <div class="hunk-header left">{{ hunk.header }}</div>
            <div class="hunk-header right">{{ hunk.header }}</div>
          </div>
          <div class="hunk-lines">
            <div v-for="(line, lineIndex) in hunk.lines" :key="lineIndex" class="diff-row">
              <!-- Left side (old) -->
              <div v-if="line.origin === '-' || line.origin === ' '"
                   :class="['diff-cell left', line.origin === '-' ? 'line-remove' : 'line-context']">
                <span class="line-number">{{ line.old_lineno || '' }}</span>
                <span class="line-content">{{ line.content }}</span>
              </div>
              <div v-else class="diff-cell left empty"></div>

              <!-- Right side (new) -->
              <div v-if="line.origin === '+' || line.origin === ' '"
                   :class="['diff-cell right', line.origin === '+' ? 'line-add' : 'line-context']">
                <span class="line-number">{{ line.new_lineno || '' }}</span>
                <span class="line-content">{{ line.content }}</span>
              </div>
              <div v-else class="diff-cell right empty"></div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-else-if="isLoading" class="loading">加载中...</div>
    <div v-else-if="!selectedFile" class="no-diff">
      <div class="no-diff-icon">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
          <polyline points="14 2 14 8 20 8"></polyline>
          <line x1="16" y1="13" x2="8" y2="13"></line>
          <line x1="16" y1="17" x2="8" y2="17"></line>
          <polyline points="10 9 9 9 8 9"></polyline>
        </svg>
      </div>
      <div class="no-diff-message">选择文件以查看差异</div>
      <div class="no-diff-hint">在左侧文件列表中点击文件</div>
    </div>
    <div v-else class="no-diff">
      <div class="no-diff-icon">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
      </div>
      <div class="no-diff-message">无差异数据</div>
      <div class="no-diff-hint">该文件可能没有变更</div>
    </div>
  </div>
</template>

<style scoped>
.enhanced-diff-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
  background-color: var(--bg-primary);
  overflow: hidden;
}

.diff-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
  background-color: var(--bg-primary);
  flex-shrink: 0;
  min-width: 0;
  overflow: hidden;
}

.file-info {
  display: flex;
  align-items: center;
  min-width: 0;
  flex: 1;
}

.file-details {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
  flex: 1;
}

.file-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-status-badge {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  color: white;
  text-transform: uppercase;
  flex-shrink: 0;
}

.no-selection {
  color: var(--text-tertiary);
  font-weight: 400;
  font-size: 14px;
}

.diff-controls {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.diff-controls button {
  padding: 6px 12px;
  font-size: 13px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-primary);
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  font-weight: 500;
}

.diff-controls button:hover {
  background-color: var(--bg-hover);
  border-color: var(--text-tertiary);
}

.diff-controls button.active {
  background-color: var(--accent-color);
  color: #ffffff;
  border-color: var(--accent-color);
}

.diff-content {
  flex: 1;
  overflow: auto;
  font-family: ui-monospace, 'SF Mono', 'Monaco', 'Cascadia Code', 'Segoe UI Mono', 'Roboto Mono', 'Oxygen Mono', 'Ubuntu Monospace', 'Source Code Pro', 'Fira Mono', 'Droid Sans Mono', 'Courier New', monospace;
  font-size: 12px;
  line-height: 20px;
  background-color: var(--bg-primary);
  min-height: 0;
}

.diff-content::-webkit-scrollbar {
  width: 12px;
  height: 12px;
}

.diff-content::-webkit-scrollbar-track {
  background: var(--bg-primary);
}

.diff-content::-webkit-scrollbar-thumb {
  background-color: var(--border-color);
  border-radius: 6px;
  border: 3px solid var(--bg-primary);
}

.diff-content::-webkit-scrollbar-thumb:hover {
  background-color: var(--text-tertiary);
}

.hunk {
  margin: 16px;
  border: 1px solid var(--diff-line-border);
  border-radius: 8px;
  overflow: hidden;
}

.hunk-header {
  color: var(--diff-hunk-text);
  padding: 8px 12px;
  font-weight: 600;
  font-size: 12px;
  line-height: 18px;
  border-bottom: 1px solid var(--diff-line-border);
}

.hunk-header-row {
  display: flex;
  border-bottom: 1px solid var(--diff-line-border);
}

.hunk-header.left,
.hunk-header.right {
  flex: 1;
  background-color: var(--diff-hunk-bg);
  color: var(--diff-hunk-text);
  padding: 8px 12px;
  font-weight: 600;
  font-size: 12px;
  line-height: 18px;
  border: none;
}

.hunk-header.left {
  border-right: 1px solid var(--diff-line-border);
}

/* Inline Mode */
.diff-inline {
  padding: 0;
}

.diff-line {
  display: flex;
  align-items: stretch;
  min-height: 20px;
  border: none;
  position: relative;
  transition: background-color 0.1s ease;
}

.diff-line:hover {
  background-color: var(--diff-context-hover);
}

.diff-line.line-context:hover {
  background-color: var(--diff-context-hover);
}

.line-number {
  display: inline-flex;
  align-items: center;
  justify-content: flex-end;
  width: 50px;
  padding: 0 10px;
  color: var(--diff-line-number);
  background-color: transparent;
  user-select: none;
  flex-shrink: 0;
  font-size: 12px;
  line-height: 20px;
  border: none;
  font-variant-numeric: tabular-nums;
}

.old-number {
  border-right: 1px solid var(--diff-line-border);
}

.new-number {
  border-right: 1px solid var(--diff-line-border);
}

.line-content {
  flex: 1;
  padding: 0 10px;
  white-space: pre;
  overflow-x: auto;
  line-height: 20px;
  min-width: 0;
}

.line-prefix {
  display: inline-block;
  width: 14px;
  font-weight: 600;
  user-select: none;
}

.line-add {
  background-color: var(--diff-add-bg);
  color: var(--diff-add-text);
}

.line-add .line-number {
  background-color: var(--diff-add-gutter);
  color: var(--diff-line-number);
}

.line-add .line-prefix {
  color: var(--diff-add-border);
}

.line-remove {
  background-color: var(--diff-remove-bg);
  color: var(--diff-remove-text);
}

.line-remove .line-number {
  background-color: var(--diff-remove-gutter);
  color: var(--diff-line-number);
}

.line-remove .line-prefix {
  color: var(--diff-remove-border);
}

.line-context {
  background-color: var(--bg-primary);
}

.line-context .line-prefix {
  opacity: 0.4;
}

/* Side-by-side Mode */
.diff-side-by-side {
  display: flex;
  flex-direction: column;
}

.hunk-lines {
  display: flex;
  flex-direction: column;
}

.diff-row {
  display: flex;
  min-height: 20px;
  border: none;
  transition: background-color 0.1s ease;
}

.diff-row:hover .diff-cell.line-context {
  background-color: var(--diff-context-hover);
}

.diff-cell {
  flex: 1;
  display: flex;
  align-items: stretch;
  min-width: 0;
  overflow: hidden;
}

.diff-cell.left {
  border-right: 2px solid var(--diff-line-border);
}

.diff-cell .line-number {
  width: 50px;
  display: inline-flex;
  align-items: center;
  justify-content: flex-end;
  padding: 0 8px;
  color: var(--diff-line-number);
  background-color: transparent;
  border-right: 1px solid var(--diff-line-border);
  flex-shrink: 0;
  font-size: 12px;
  line-height: 20px;
  user-select: none;
  font-variant-numeric: tabular-nums;
}

.diff-cell .line-content {
  flex: 1;
  padding: 0 10px;
  white-space: pre;
  overflow-x: auto;
  line-height: 20px;
  min-width: 0;
}

.diff-cell.empty {
  background-color: var(--diff-empty-bg);
}

.diff-cell.line-add {
  background-color: var(--diff-add-bg);
  color: var(--diff-add-text);
}

.diff-cell.line-add .line-number {
  background-color: var(--diff-add-gutter);
  color: var(--diff-line-number);
}

.diff-cell.line-remove {
  background-color: var(--diff-remove-bg);
  color: var(--diff-remove-text);
}

.diff-cell.line-remove .line-number {
  background-color: var(--diff-remove-gutter);
  color: var(--diff-line-number);
}

.diff-cell.line-context {
  background-color: var(--bg-primary);
}

.diff-cell.line-context .line-number {
  background-color: transparent;
}

.loading,
.no-diff {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-tertiary);
  gap: 12px;
  padding: 40px;
  text-align: center;
}

.no-diff-icon {
  opacity: 0.4;
  margin-bottom: 8px;
}

.no-diff-message {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-secondary);
}

.no-diff-hint {
  font-size: 14px;
  color: var(--text-tertiary);
  max-width: 400px;
}
</style>
