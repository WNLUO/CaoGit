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
  background-color: var(--bg-primary);
}

.diff-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
}

.file-info {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
}

.no-selection {
  color: var(--text-tertiary);
}

.diff-controls {
  display: flex;
  gap: var(--spacing-xs);
}

.diff-controls button {
  padding: 4px 12px;
  font-size: var(--font-size-xs);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-color);
  background-color: var(--bg-primary);
  color: var(--text-secondary);
}

.diff-controls button.active {
  background-color: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.diff-content {
  flex: 1;
  overflow: auto;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.5;
}

.hunk {
  margin-bottom: var(--spacing-md);
}

.hunk-header {
  background-color: #f0f6ff;
  color: #1e40af;
  padding: 4px 8px;
  font-weight: 600;
  border-top: 1px solid #dbeafe;
  border-bottom: 1px solid #dbeafe;
}

/* Inline Mode */
.diff-inline {
  padding: var(--spacing-sm);
}

.diff-line {
  display: flex;
  align-items: stretch;
  border-bottom: 1px solid #f3f4f6;
}

.line-number {
  display: inline-block;
  width: 50px;
  text-align: right;
  padding: 0 8px;
  color: #9ca3af;
  background-color: #f9fafb;
  border-right: 1px solid #e5e7eb;
  user-select: none;
  flex-shrink: 0;
}

.old-number {
  border-right: none;
}

.new-number {
  margin-right: 8px;
}

.line-content {
  flex: 1;
  padding: 0 8px;
  white-space: pre-wrap;
  word-break: break-all;
}

.line-prefix {
  display: inline-block;
  width: 20px;
  font-weight: bold;
}

.line-add {
  background-color: #dcfce7;
  color: #166534;
}

.line-add .line-number {
  background-color: #bbf7d0;
}

.line-remove {
  background-color: #fee2e2;
  color: #991b1b;
}

.line-remove .line-number {
  background-color: #fecaca;
}

.line-context {
  background-color: white;
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
  background-color: #f0f6ff;
  color: #1e40af;
  padding: 4px 8px;
  font-weight: 600;
  border-top: 1px solid #dbeafe;
  border-bottom: 1px solid #dbeafe;
}

.hunk-header.left {
  border-right: 1px solid #dbeafe;
}

.hunk-lines {
  display: flex;
  flex-direction: column;
}

.diff-row {
  display: flex;
  border-bottom: 1px solid #f3f4f6;
}

.diff-cell {
  flex: 1;
  display: flex;
  align-items: stretch;
}

.diff-cell.left {
  border-right: 2px solid #e5e7eb;
}

.diff-cell .line-number {
  width: 50px;
  text-align: right;
  padding: 0 8px;
  color: #9ca3af;
  background-color: #f9fafb;
  border-right: 1px solid #e5e7eb;
  flex-shrink: 0;
}

.diff-cell .line-content {
  flex: 1;
  padding: 0 8px;
  white-space: pre-wrap;
  word-break: break-all;
}

.diff-cell.empty {
  background-color: #f9fafb;
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
