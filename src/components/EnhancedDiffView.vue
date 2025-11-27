<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { repoStore } from '../stores/repoStore';
import { GitApi } from '../services/gitApi';

type DiffMode = 'inline' | 'side-by-side';

const diffMode = ref<DiffMode>('side-by-side');
const diffData = ref<any>(null);
const isLoading = ref(false);

const selectedFile = computed(() => repoStore.selectedFile);

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
</script>

<template>
  <div class="enhanced-diff-view">
    <div class="diff-header">
      <div class="file-info">
        <span v-if="selectedFile" class="file-name">{{ selectedFile.path }}</span>
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
        <div v-for="(hunk, hunkIndex) in diffData.hunks" :key="hunkIndex" class="hunk">
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
        <div v-for="(hunk, hunkIndex) in diffData.hunks" :key="hunkIndex" class="hunk">
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
      <div class="no-diff-message">选择文件以查看差异</div>
    </div>
    <div v-else-if="selectedFile && selectedFile.status === 'untracked'" class="no-diff">
      <div class="no-diff-message">新文件</div>
      <div class="no-diff-hint">这是一个未跟踪的新文件，暂存后才能查看完整内容</div>
    </div>
    <div v-else class="no-diff">
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
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
  flex-shrink: 0;
  min-width: 0;
  overflow: hidden;
}

.file-info {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex-shrink: 1;
}

.no-selection {
  color: var(--text-tertiary);
}

.diff-controls {
  display: flex;
  gap: 0;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  overflow: hidden;
  flex-shrink: 0;
}

.diff-controls button {
  padding: 5px 12px;
  font-size: 12px;
  border: none;
  border-right: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  transition: all var(--transition-fast);
}

.diff-controls button:last-child {
  border-right: none;
}

.diff-controls button:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.diff-controls button.active {
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-weight: 500;
}

.diff-content {
  flex: 1;
  overflow: auto;
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Fira Code', 'Fira Mono', 'Droid Sans Mono', 'Source Code Pro', 'Consolas', 'Courier New', monospace;
  font-size: 13px;
  line-height: 20px;
  background-color: var(--bg-primary);
  min-height: 0;
}

.diff-content::-webkit-scrollbar {
  width: 10px;
  height: 10px;
}

.diff-content::-webkit-scrollbar-track {
  background: var(--bg-primary);
}

.diff-content::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 5px;
}

.diff-content::-webkit-scrollbar-thumb:hover {
  background: var(--text-tertiary);
}

.hunk {
  margin-bottom: 0;
}

.hunk-header {
  background-color: var(--diff-hunk-bg);
  color: var(--diff-hunk-text);
  padding: 6px 12px;
  font-weight: 400;
  font-size: 12px;
  line-height: 20px;
  border: none;
  position: sticky;
  top: 0;
  z-index: 10;
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
}

.diff-line:hover .line-number {
  opacity: 1;
}

.line-number {
  display: inline-flex;
  align-items: center;
  justify-content: flex-end;
  width: 56px;
  padding: 0 10px;
  color: var(--diff-line-number);
  background-color: var(--diff-gutter-bg);
  user-select: none;
  flex-shrink: 0;
  font-size: 12px;
  line-height: 20px;
  border: none;
}

.old-number {
  border-right: 1px solid var(--diff-line-border);
}

.new-number {
  border-right: 1px solid var(--diff-line-border);
}

.line-content {
  flex: 1;
  padding: 0 16px;
  white-space: pre;
  overflow-x: auto;
  line-height: 20px;
  min-width: 0;
}

.line-prefix {
  display: inline-block;
  width: 16px;
  font-weight: normal;
  opacity: 0.6;
}

.line-add {
  background-color: var(--diff-add-bg);
  color: var(--diff-add-text);
}

.line-add .line-number {
  background-color: var(--diff-add-gutter);
}

.line-remove {
  background-color: var(--diff-remove-bg);
  color: var(--diff-remove-text);
}

.line-remove .line-number {
  background-color: var(--diff-remove-gutter);
}

.line-context {
  background-color: var(--bg-primary);
}

/* Side-by-side Mode */
.diff-side-by-side {
  display: flex;
  flex-direction: column;
}

.hunk-header-row {
  display: flex;
}

.hunk-header.left,
.hunk-header.right {
  flex: 1;
  background-color: var(--diff-hunk-bg);
  color: var(--diff-hunk-text);
  padding: 6px 12px;
  font-weight: 400;
  font-size: 12px;
  line-height: 20px;
  border: none;
  position: sticky;
  top: 0;
  z-index: 10;
}

.hunk-header.left {
  border-right: 1px solid var(--diff-line-border);
}

.hunk-lines {
  display: flex;
  flex-direction: column;
}

.diff-row {
  display: flex;
  min-height: 20px;
  border: none;
}

.diff-cell {
  flex: 1;
  display: flex;
  align-items: stretch;
  min-width: 0;
  overflow: hidden;
}

.diff-cell.left {
  border-right: 1px solid var(--diff-line-border);
}

.diff-cell .line-number {
  width: 56px;
  display: inline-flex;
  align-items: center;
  justify-content: flex-end;
  padding: 0 10px;
  color: var(--diff-line-number);
  background-color: var(--diff-gutter-bg);
  border-right: 1px solid var(--diff-line-border);
  flex-shrink: 0;
  font-size: 12px;
  line-height: 20px;
}

.diff-cell .line-content {
  flex: 1;
  padding: 0 16px;
  white-space: pre;
  overflow-x: auto;
  line-height: 20px;
  min-width: 0;
}

.diff-cell.empty {
  background-color: var(--diff-empty-bg);
}

.diff-cell.line-add .line-number {
  background-color: var(--diff-add-gutter);
  color: var(--diff-line-number);
}

.diff-cell.line-remove .line-number {
  background-color: var(--diff-remove-gutter);
  color: var(--diff-line-number);
}

.diff-cell.line-context .line-number {
  background-color: var(--diff-gutter-bg);
}

.loading,
.no-diff {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-tertiary);
  gap: var(--spacing-sm);
  padding: var(--spacing-xl);
  text-align: center;
}

.no-diff-icon {
  font-size: 3rem;
  margin-bottom: var(--spacing-sm);
}

.no-diff-message {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-secondary);
}

.no-diff-hint {
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
  max-width: 400px;
}
</style>
