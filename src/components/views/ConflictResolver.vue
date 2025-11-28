<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { GitApi } from '../../services/gitApi';
import type { ConflictInfo } from '../../types';

interface Props {
  repoPath: string;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  close: [];
  resolved: [];
}>();

const conflicts = ref<ConflictInfo[]>([]);
const currentConflictIndex = ref(0);
const isLoading = ref(false);
const isResolving = ref(false);
const error = ref<string | null>(null);
const resolution = ref('');

const currentConflict = computed(() => conflicts.value[currentConflictIndex.value] || null);
const hasConflicts = computed(() => conflicts.value.length > 0);
const isLastConflict = computed(() => currentConflictIndex.value === conflicts.value.length - 1);
const isFirstConflict = computed(() => currentConflictIndex.value === 0);

// Load conflicts
async function loadConflicts() {
  isLoading.value = true;
  error.value = null;

  try {
    const result = await GitApi.getConflicts(props.repoPath);
    if (result.success && result.data) {
      conflicts.value = result.data;
      if (conflicts.value.length > 0) {
        // Initialize resolution with the conflicted content
        resolution.value = currentConflict.value?.ours || '';
      }
    } else {
      error.value = result.error || 'Failed to load conflicts';
    }
  } catch (err) {
    error.value = `Error: ${err}`;
  } finally {
    isLoading.value = false;
  }
}

// Accept ours (current branch)
function acceptOurs() {
  if (currentConflict.value) {
    resolution.value = currentConflict.value.ours;
  }
}

// Accept theirs (merging branch)
function acceptTheirs() {
  if (currentConflict.value) {
    resolution.value = currentConflict.value.theirs;
  }
}

// Accept both (ours + theirs)
function acceptBoth() {
  if (currentConflict.value) {
    resolution.value = currentConflict.value.ours + '\n' + currentConflict.value.theirs;
  }
}

// Resolve current conflict
async function resolveCurrentConflict() {
  if (!currentConflict.value || !resolution.value) {
    error.value = 'Please provide a resolution';
    return;
  }

  isResolving.value = true;
  error.value = null;

  try {
    const result = await GitApi.resolveConflict(
      props.repoPath,
      currentConflict.value.path,
      resolution.value
    );

    if (result.success) {
      // Move to next conflict or finish
      if (isLastConflict.value) {
        // All conflicts resolved
        emit('resolved');
        emit('close');
      } else {
        // Move to next conflict
        currentConflictIndex.value++;
        resolution.value = currentConflict.value?.ours || '';
      }
    } else {
      error.value = result.error || 'Failed to resolve conflict';
    }
  } catch (err) {
    error.value = `Error: ${err}`;
  } finally {
    isResolving.value = false;
  }
}

// Navigate conflicts
function prevConflict() {
  if (!isFirstConflict.value) {
    currentConflictIndex.value--;
    resolution.value = currentConflict.value?.ours || '';
  }
}

function nextConflict() {
  if (!isLastConflict.value) {
    currentConflictIndex.value++;
    resolution.value = currentConflict.value?.ours || '';
  }
}

// Abort merge
async function abortMerge() {
  if (!confirm('确定要中止合并吗？所有更改将被丢弃。')) {
    return;
  }

  try {
    const result = await GitApi.abortMerge(props.repoPath);
    if (result.success) {
      emit('close');
    } else {
      error.value = result.error || 'Failed to abort merge';
    }
  } catch (err) {
    error.value = `Error: ${err}`;
  }
}

onMounted(() => {
  loadConflicts();
});
</script>

<template>
  <div class="conflict-resolver">
    <div class="resolver-header">
      <div class="header-title">
        <svg width="20" height="20" viewBox="0 0 16 16" fill="currentColor">
          <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/>
          <path d="M7.002 11a1 1 0 1 1 2 0 1 1 0 0 1-2 0zM7.1 4.995a.905.905 0 1 1 1.8 0l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 4.995z"/>
        </svg>
        <h2>解决合并冲突</h2>
      </div>
      <button class="btn-close" @click="emit('close')" title="关闭">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8 2.146 2.854z"/>
        </svg>
      </button>
    </div>

    <div v-if="isLoading" class="loading-state">
      <div class="spinner"></div>
      <p>加载冲突信息...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <svg width="48" height="48" viewBox="0 0 16 16" fill="currentColor">
        <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/>
        <path d="M7.002 11a1 1 0 1 1 2 0 1 1 0 0 1-2 0zM7.1 4.995a.905.905 0 1 1 1.8 0l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 4.995z"/>
      </svg>
      <p class="error-message">{{ error }}</p>
      <button class="btn-retry" @click="loadConflicts">重试</button>
    </div>

    <div v-else-if="!hasConflicts" class="empty-state">
      <svg width="64" height="64" viewBox="0 0 16 16" fill="currentColor">
        <path d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
      </svg>
      <p>没有检测到冲突</p>
      <p class="hint">所有文件都已成功合并</p>
    </div>

    <div v-else class="conflict-content">
      <!-- Progress indicator -->
      <div class="conflict-progress">
        <div class="progress-text">
          冲突 {{ currentConflictIndex + 1 }} / {{ conflicts.length }}
        </div>
        <div class="progress-bar">
          <div
            class="progress-fill"
            :style="{ width: `${((currentConflictIndex + 1) / conflicts.length) * 100}%` }"
          ></div>
        </div>
      </div>

      <!-- Current conflict file -->
      <div class="conflict-file">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M4 1.5a.5.5 0 0 1 .5-.5h5.586a.5.5 0 0 1 .353.146l2.5 2.5a.5.5 0 0 1 .147.354V14.5a.5.5 0 0 1-.5.5h-9a.5.5 0 0 1-.5-.5V1.5zm6 0v2a.5.5 0 0 0 .5.5h2L10 1.5z"/>
        </svg>
        <span class="file-path">{{ currentConflict?.path }}</span>
      </div>

      <!-- Three-panel view -->
      <div class="conflict-panels">
        <!-- Ours (Current Branch) -->
        <div class="panel">
          <div class="panel-header ours">
            <span class="panel-title">Ours (当前分支)</span>
            <button class="btn-accept" @click="acceptOurs">使用此版本</button>
          </div>
          <div class="panel-content">
            <pre><code>{{ currentConflict?.ours }}</code></pre>
          </div>
        </div>

        <!-- Base (Common Ancestor) - Optional -->
        <div v-if="currentConflict?.base" class="panel">
          <div class="panel-header base">
            <span class="panel-title">Base (共同祖先)</span>
          </div>
          <div class="panel-content">
            <pre><code>{{ currentConflict.base }}</code></pre>
          </div>
        </div>

        <!-- Theirs (Merging Branch) -->
        <div class="panel">
          <div class="panel-header theirs">
            <span class="panel-title">Theirs (合并分支)</span>
            <button class="btn-accept" @click="acceptTheirs">使用此版本</button>
          </div>
          <div class="panel-content">
            <pre><code>{{ currentConflict?.theirs }}</code></pre>
          </div>
        </div>
      </div>

      <!-- Resolution editor -->
      <div class="resolution-section">
        <div class="resolution-header">
          <span class="resolution-title">解决方案 (编辑后的最终版本)</span>
          <div class="resolution-actions">
            <button class="btn-action" @click="acceptBoth" title="合并两个版本">
              合并两者
            </button>
          </div>
        </div>
        <textarea
          v-model="resolution"
          class="resolution-editor"
          placeholder="在此编辑最终版本..."
        ></textarea>
      </div>

      <!-- Action buttons -->
      <div class="action-bar">
        <div class="navigation-buttons">
          <button
            class="btn-nav"
            :disabled="isFirstConflict"
            @click="prevConflict"
          >
            ← 上一个
          </button>
          <button
            class="btn-nav"
            :disabled="isLastConflict"
            @click="nextConflict"
          >
            下一个 →
          </button>
        </div>

        <div class="main-actions">
          <button class="btn-abort" @click="abortMerge">
            中止合并
          </button>
          <button
            class="btn-resolve"
            :disabled="!resolution || isResolving"
            @click="resolveCurrentConflict"
          >
            {{ isResolving ? '解决中...' : (isLastConflict ? '完成解决' : '解决并继续') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.conflict-resolver {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.resolver-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md);
  border-bottom: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
}

.header-title {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  color: var(--error-color);
}

.header-title h2 {
  margin: 0;
  font-size: var(--font-size-lg);
  font-weight: 600;
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

.error-message {
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

.empty-state svg {
  color: var(--success-color, #22c55e);
}

.empty-state .hint {
  font-size: var(--font-size-sm);
  opacity: 0.7;
}

.conflict-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding: var(--spacing-md);
  gap: var(--spacing-md);
}

.conflict-progress {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.progress-text {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  white-space: nowrap;
}

.progress-bar {
  flex: 1;
  height: 6px;
  background-color: var(--bg-secondary);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background-color: var(--accent-color);
  transition: width 0.3s ease;
}

.conflict-file {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background-color: var(--bg-secondary);
  border-radius: var(--border-radius);
  color: var(--text-primary);
}

.file-path {
  font-family: monospace;
  font-size: var(--font-size-sm);
}

.conflict-panels {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: var(--spacing-md);
  min-height: 200px;
}

.panel {
  display: flex;
  flex-direction: column;
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius);
  overflow: hidden;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: var(--font-size-sm);
  font-weight: 600;
}

.panel-header.ours {
  background-color: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
  border-bottom: 2px solid #3b82f6;
}

.panel-header.base {
  background-color: rgba(107, 114, 128, 0.1);
  color: #6b7280;
  border-bottom: 2px solid #6b7280;
}

.panel-header.theirs {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
  border-bottom: 2px solid #10b981;
}

.panel-content {
  flex: 1;
  overflow: auto;
  padding: var(--spacing-sm);
  background-color: var(--bg-secondary);
}

.panel-content pre {
  margin: 0;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: var(--font-size-xs);
  white-space: pre-wrap;
  word-wrap: break-word;
}

.panel-content code {
  color: var(--text-primary);
}

.btn-accept {
  padding: 4px 10px;
  background-color: white;
  border: 1px solid currentColor;
  border-radius: 4px;
  font-size: var(--font-size-xs);
  cursor: pointer;
  transition: all 0.2s;
}

.btn-accept:hover {
  opacity: 0.8;
}

.resolution-section {
  display: flex;
  flex-direction: column;
  border: 2px solid var(--accent-color);
  border-radius: var(--border-radius);
  overflow: hidden;
}

.resolution-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--accent-color);
  color: white;
}

.resolution-title {
  font-size: var(--font-size-sm);
  font-weight: 600;
}

.resolution-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.btn-action {
  padding: 4px 10px;
  background-color: white;
  color: var(--accent-color);
  border: none;
  border-radius: 4px;
  font-size: var(--font-size-xs);
  cursor: pointer;
  transition: opacity 0.2s;
}

.btn-action:hover {
  opacity: 0.9;
}

.resolution-editor {
  flex: 1;
  min-height: 150px;
  padding: var(--spacing-md);
  border: none;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: var(--font-size-sm);
  resize: vertical;
  outline: none;
}

.action-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-md);
  padding-top: var(--spacing-md);
  border-top: 1px solid var(--border-color);
}

.navigation-buttons {
  display: flex;
  gap: var(--spacing-sm);
}

.main-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.btn-nav {
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: all 0.2s;
}

.btn-nav:hover:not(:disabled) {
  background-color: var(--bg-hover);
}

.btn-nav:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-abort {
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: transparent;
  color: var(--error-color);
  border: 1px solid var(--error-color);
  border-radius: var(--border-radius);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: all 0.2s;
}

.btn-abort:hover {
  background-color: var(--error-color);
  color: white;
}

.btn-resolve {
  padding: var(--spacing-sm) var(--spacing-lg);
  background-color: var(--accent-color);
  color: white;
  border: none;
  border-radius: var(--border-radius);
  cursor: pointer;
  font-size: var(--font-size-sm);
  font-weight: 600;
  transition: opacity 0.2s;
}

.btn-resolve:hover:not(:disabled) {
  opacity: 0.9;
}

.btn-resolve:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Scrollbar styling */
.panel-content::-webkit-scrollbar,
.resolution-editor::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.panel-content::-webkit-scrollbar-track,
.resolution-editor::-webkit-scrollbar-track {
  background: var(--bg-primary);
}

.panel-content::-webkit-scrollbar-thumb,
.resolution-editor::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 4px;
}

.panel-content::-webkit-scrollbar-thumb:hover,
.resolution-editor::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}
</style>
