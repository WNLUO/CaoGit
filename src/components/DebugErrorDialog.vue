<script setup lang="ts">
import { computed } from 'vue';
import { debugStore, type ErrorLog } from '../stores/debugStore';

const isOpen = computed(() => debugStore.showErrorDialog.value);
const currentError = computed(() => debugStore.currentError);

function handleClose() {
  debugStore.closeErrorDialog();
}

function handleCopy() {
  if (currentError.value) {
    debugStore.copyErrorToClipboard(currentError.value);
  }
}

function getTypeColor(type: ErrorLog['type']) {
  switch (type) {
    case 'error': return '#ef4444';
    case 'warning': return '#f59e0b';
    case 'info': return '#3b82f6';
    default: return '#6b7280';
  }
}

function getTypeLabel(type: ErrorLog['type']) {
  switch (type) {
    case 'error': return '错误';
    case 'warning': return '警告';
    case 'info': return '信息';
    default: return '未知';
  }
}
</script>

<template>
  <div v-if="isOpen && currentError" class="modal-overlay" @click="handleClose">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>
          <span class="type-badge" :style="{ backgroundColor: getTypeColor(currentError.type) }">
            {{ getTypeLabel(currentError.type) }}
          </span>
          调试信息
        </h3>
        <button class="close-btn" @click="handleClose">×</button>
      </div>

      <div class="modal-body">
        <div class="info-section">
          <div class="info-item">
            <span class="label">时间:</span>
            <span class="value">{{ new Date(currentError.timestamp).toLocaleString('zh-CN') }}</span>
          </div>
          <div v-if="currentError.context" class="info-item">
            <span class="label">上下文:</span>
            <span class="value">{{ currentError.context }}</span>
          </div>
        </div>

        <div class="error-section">
          <div class="section-title">错误消息</div>
          <div class="error-message">{{ currentError.message }}</div>
        </div>

        <div v-if="currentError.stack" class="stack-section">
          <div class="section-title">堆栈追踪</div>
          <pre class="stack-trace">{{ currentError.stack }}</pre>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-secondary" @click="handleClose">关闭</button>
        <button class="btn-primary" @click="handleCopy">复制错误信息</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.6);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 9999;
  backdrop-filter: blur(2px);
}

.modal-content {
  background-color: var(--bg-primary);
  border-radius: var(--radius-lg);
  width: 700px;
  max-width: 90vw;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-color);
}

.modal-header {
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-shrink: 0;
}

.modal-header h3 {
  font-size: var(--font-size-lg);
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.type-badge {
  padding: 2px 8px;
  border-radius: var(--radius-sm);
  color: white;
  font-size: var(--font-size-xs);
  font-weight: 600;
}

.close-btn {
  font-size: 1.5rem;
  line-height: 1;
  color: var(--text-secondary);
  padding: 4px 8px;
}

.close-btn:hover {
  color: var(--text-primary);
}

.modal-body {
  padding: var(--spacing-lg);
  overflow-y: auto;
  flex: 1;
}

.info-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  margin-bottom: var(--spacing-md);
  padding: var(--spacing-sm);
  background-color: var(--bg-secondary);
  border-radius: var(--radius-md);
}

.info-item {
  display: flex;
  gap: var(--spacing-sm);
  font-size: var(--font-size-sm);
}

.info-item .label {
  color: var(--text-secondary);
  font-weight: 600;
  min-width: 60px;
}

.info-item .value {
  color: var(--text-primary);
}

.error-section,
.stack-section {
  margin-bottom: var(--spacing-md);
}

.section-title {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.error-message {
  padding: var(--spacing-md);
  background-color: #fee;
  border-left: 4px solid #ef4444;
  border-radius: var(--radius-md);
  color: #991b1b;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: var(--font-size-sm);
  word-break: break-word;
}

.stack-trace {
  padding: var(--spacing-md);
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 11px;
  line-height: 1.5;
  overflow-x: auto;
  white-space: pre-wrap;
  word-break: break-all;
  max-height: 300px;
  overflow-y: auto;
}

.modal-footer {
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  flex-shrink: 0;
}

.btn-secondary {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
}

.btn-secondary:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.btn-primary {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  background-color: var(--accent-color);
  color: white;
  font-weight: 600;
}

.btn-primary:hover {
  background-color: var(--accent-hover);
}
</style>
