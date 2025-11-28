<script setup lang="ts">
import { computed, ref } from 'vue';
import { debugStore, type ErrorLog } from '../../stores/debugStore';

const isOpen = computed(() => debugStore.showErrorDialog);
const currentError = computed(() => debugStore.currentError);

// 日志列表模式状态
const showLogsList = ref(false);
const filterType = ref<'all' | 'error' | 'warning' | 'info'>('all');
const searchText = ref('');

const errorStats = computed(() => debugStore.getErrorStats());
const filteredLogs = computed(() => {
  const type = filterType.value === 'all' ? undefined : (filterType.value as 'error' | 'warning' | 'info');
  return debugStore.getFilteredErrors(type, searchText.value);
});

function handleClose() {
  debugStore.closeErrorDialog();
  showLogsList.value = false;
}

function handleCopy() {
  if (currentError.value) {
    debugStore.copyErrorToClipboard(currentError.value);
  }
}

function toggleLogsList() {
  showLogsList.value = !showLogsList.value;
}

function exportLogs(format: 'json' | 'csv' | 'text') {
  if (format === 'json') {
    debugStore.exportLogsAsJson();
  } else if (format === 'csv') {
    debugStore.exportLogsAsCSV();
  } else if (format === 'text') {
    debugStore.exportLogsAsText();
  }
}

function clearAllLogs() {
  if (confirm('确定要清除所有日志吗？此操作无法撤销。')) {
    debugStore.clearErrors();
    handleClose();
  }
}

function clearLogsByType(type: 'error' | 'warning' | 'info') {
  const typeLabel = { error: '错误', warning: '警告', info: '信息' }[type];
  if (confirm(`确定要清除所有${typeLabel}日志吗？`)) {
    debugStore.clearErrorsByType(type);
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
  <div v-if="isOpen" class="modal-overlay">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>
          <span v-if="!showLogsList" class="type-badge" :style="{ backgroundColor: getTypeColor(currentError?.type || 'error') }">
            {{ getTypeLabel(currentError?.type || 'error') }}
          </span>
          <span v-else class="type-badge" style="background-color: #3b82f6;">日志</span>
          调试信息
        </h3>
        <div class="header-actions">
          <button v-if="!showLogsList" class="icon-btn" @click="toggleLogsList" title="查看所有日志">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
              <polyline points="9 22 9 12 15 12 15 22"></polyline>
            </svg>
          </button>
          <button class="close-btn" @click="handleClose">×</button>
        </div>
      </div>

      <!-- 单条错误详情视图 -->
      <div v-if="!showLogsList && currentError" class="modal-body">
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

      <!-- 日志列表视图 -->
      <div v-else class="modal-body logs-view">
        <div class="logs-toolbar">
          <div class="search-box">
            <input v-model="searchText" type="text" placeholder="搜索日志..." />
          </div>
          <div class="filter-buttons">
            <button
              v-for="type in ['all', 'error', 'warning', 'info']"
              :key="type"
              :class="['filter-btn', { active: filterType === type }]"
              @click="filterType = type as any"
            >
              {{ type === 'all' ? '全部' : ({ error: '错误', warning: '警告', info: '信息' }[type] || type) }}
              <span class="count">{{
                type === 'all' ? errorStats.total :
                type === 'error' ? errorStats.errors :
                type === 'warning' ? errorStats.warnings :
                errorStats.infos
              }}</span>
            </button>
          </div>
        </div>

        <div class="logs-list">
          <div v-if="filteredLogs.length === 0" class="empty-logs">
            没有匹配的日志
          </div>
          <div v-for="log in filteredLogs" :key="log.id" class="log-item" :class="{ [log.type]: true }">
            <div class="log-header">
              <span class="log-time">{{ new Date(log.timestamp).toLocaleTimeString('zh-CN') }}</span>
              <span class="log-type" :style="{ color: getTypeColor(log.type) }">{{ getTypeLabel(log.type) }}</span>
            </div>
            <div class="log-message">{{ log.message }}</div>
            <div v-if="log.context" class="log-context">{{ log.context }}</div>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <div v-if="!showLogsList" class="footer-left">
          <div class="export-menu">
            <button class="export-btn" title="导出日志">
              导出 ▼
            </button>
            <div class="export-options">
              <button @click="exportLogs('json')">导出为 JSON</button>
              <button @click="exportLogs('csv')">导出为 CSV</button>
              <button @click="exportLogs('text')">导出为 TXT</button>
              <hr />
              <button @click="clearAllLogs" class="danger">清除所有日志</button>
            </div>
          </div>
        </div>
        <div v-else class="footer-left">
          <button class="btn-secondary" @click="clearLogsByType('error')">清除错误</button>
          <button class="btn-secondary" @click="clearLogsByType('warning')">清除警告</button>
          <button class="btn-secondary" @click="clearLogsByType('info')">清除信息</button>
        </div>
        <div class="footer-right">
          <button class="btn-secondary" @click="handleClose">关闭</button>
          <button v-if="!showLogsList && currentError" class="btn-primary" @click="handleCopy">复制错误信息</button>
        </div>
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

.header-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.icon-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 4px 8px;
  display: flex;
  align-items: center;
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.icon-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

/* 日志列表样式 */
.logs-view {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.logs-toolbar {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.search-box {
  display: flex;
}

.search-box input {
  flex: 1;
  padding: var(--spacing-sm) var(--spacing-md);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: var(--font-size-sm);
}

.search-box input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.filter-buttons {
  display: flex;
  gap: var(--spacing-xs);
  flex-wrap: wrap;
}

.filter-btn {
  padding: 4px 12px;
  border-radius: var(--radius-full);
  border: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  font-size: var(--font-size-xs);
  cursor: pointer;
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  gap: 6px;
}

.filter-btn:hover {
  border-color: var(--accent-color);
  color: var(--text-primary);
}

.filter-btn.active {
  background-color: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.filter-btn .count {
  background-color: rgba(0, 0, 0, 0.2);
  padding: 0 6px;
  border-radius: 10px;
  font-size: 10px;
  font-weight: 600;
}

.logs-list {
  flex: 1;
  overflow-y: auto;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
}

.log-item {
  padding: var(--spacing-md);
  border-bottom: 1px solid var(--border-color);
  font-size: var(--font-size-sm);
}

.log-item:last-child {
  border-bottom: none;
}

.log-item.error {
  border-left: 4px solid #ef4444;
}

.log-item.warning {
  border-left: 4px solid #f59e0b;
}

.log-item.info {
  border-left: 4px solid #3b82f6;
}

.log-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 6px;
  font-size: 11px;
}

.log-time {
  color: var(--text-tertiary);
}

.log-type {
  font-weight: 600;
  text-transform: uppercase;
}

.log-message {
  color: var(--text-primary);
  margin-bottom: 4px;
  word-break: break-word;
}

.log-context {
  color: var(--text-secondary);
  font-size: 11px;
  margin-top: 4px;
  padding-top: 4px;
  border-top: 1px solid var(--border-color);
}

.empty-logs {
  padding: var(--spacing-lg);
  text-align: center;
  color: var(--text-secondary);
}

/* 导出菜单 */
.export-menu {
  position: relative;
}

.export-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  cursor: pointer;
  font-size: var(--font-size-sm);
  transition: all var(--transition-fast);
}

.export-btn:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.export-options {
  position: absolute;
  bottom: 100%;
  left: 0;
  margin-bottom: 8px;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  display: none;
  flex-direction: column;
  min-width: 120px;
  z-index: 10000;
}

.export-menu:hover .export-options {
  display: flex;
}

.export-options button {
  padding: var(--spacing-sm) var(--spacing-md);
  background: none;
  border: none;
  text-align: left;
  cursor: pointer;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  transition: background-color var(--transition-fast);
}

.export-options button:hover {
  background-color: var(--bg-hover);
}

.export-options button.danger {
  color: #ef4444;
}

.export-options button.danger:hover {
  background-color: rgba(239, 68, 68, 0.1);
}

.export-options hr {
  margin: 4px 0;
  border: none;
  border-top: 1px solid var(--border-color);
}

/* 页脚布局 */
.modal-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--border-color);
  flex-shrink: 0;
  gap: var(--spacing-sm);
}

.footer-left {
  display: flex;
  gap: var(--spacing-xs);
}

.footer-right {
  display: flex;
  gap: var(--spacing-sm);
}
</style>
