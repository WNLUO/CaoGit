<script setup lang="ts">
interface Props {
  currentChange: number;
  totalChanges: number;
  currentFile: number;
  totalFiles: number;
  canAcceptAll?: boolean;
  canRejectAll?: boolean;
}

withDefaults(defineProps<Props>(), {
  canAcceptAll: true,
  canRejectAll: true
});

const emit = defineEmits<{
  prevChange: [];
  nextChange: [];
  prevFile: [];
  nextFile: [];
  acceptAll: [];
  rejectAll: [];
}>();
</script>

<template>
  <div class="diff-navigator">
    <div class="nav-section">
      <span class="nav-label">变更</span>
      <div class="nav-controls">
        <button
          class="nav-btn"
          :disabled="currentChange <= 1"
          @click="emit('prevChange')"
          title="上一个变更 (K)"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="18 15 12 9 6 15"></polyline>
          </svg>
        </button>
        <span class="nav-counter">{{ currentChange }}/{{ totalChanges }}</span>
        <button
          class="nav-btn"
          :disabled="currentChange >= totalChanges"
          @click="emit('nextChange')"
          title="下一个变更 (J)"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="6 9 12 15 18 9"></polyline>
          </svg>
        </button>
      </div>
    </div>

    <div class="nav-divider"></div>

    <div class="nav-section">
      <span class="nav-label">文件</span>
      <div class="nav-controls">
        <button
          class="nav-btn"
          :disabled="currentFile <= 1"
          @click="emit('prevFile')"
          title="上一个文件"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="15 18 9 12 15 6"></polyline>
          </svg>
        </button>
        <span class="nav-counter">{{ currentFile }}/{{ totalFiles }}</span>
        <button
          class="nav-btn"
          :disabled="currentFile >= totalFiles"
          @click="emit('nextFile')"
          title="下一个文件"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="9 18 15 12 9 6"></polyline>
          </svg>
        </button>
      </div>
    </div>

    <div class="nav-divider"></div>

    <div class="nav-section actions">
      <button
        class="action-btn accept"
        :disabled="!canAcceptAll"
        @click="emit('acceptAll')"
        title="接受所有变更 (A)"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
        <span>接受全部</span>
      </button>
      <button
        class="action-btn reject"
        :disabled="!canRejectAll"
        @click="emit('rejectAll')"
        title="拒绝所有变更 (R)"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
        <span>拒绝全部</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.diff-navigator {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 8px 12px;
  background-color: var(--bg-secondary);
  border-radius: 8px;
  font-size: 13px;
}

.nav-section {
  display: flex;
  align-items: center;
  gap: 8px;
}

.nav-section.actions {
  margin-left: auto;
  gap: 6px;
}

.nav-label {
  color: var(--text-tertiary);
  font-size: 12px;
  font-weight: 500;
}

.nav-controls {
  display: flex;
  align-items: center;
  gap: 4px;
}

.nav-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  padding: 0;
  border-radius: 6px;
  background-color: transparent;
  color: var(--text-secondary);
  transition: all 0.2s;
}

.nav-btn:hover:not(:disabled) {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.nav-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.nav-counter {
  min-width: 50px;
  text-align: center;
  color: var(--text-secondary);
  font-weight: 500;
  font-family: ui-monospace, 'SF Mono', monospace;
  font-size: 12px;
}

.nav-divider {
  width: 1px;
  height: 24px;
  background-color: var(--border-color);
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  transition: all 0.2s;
}

.action-btn.accept {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
  border: 1px solid rgba(16, 185, 129, 0.2);
}

.action-btn.accept:hover:not(:disabled) {
  background-color: rgba(16, 185, 129, 0.15);
  border-color: rgba(16, 185, 129, 0.3);
}

.action-btn.reject {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
  border: 1px solid rgba(239, 68, 68, 0.2);
}

.action-btn.reject:hover:not(:disabled) {
  background-color: rgba(239, 68, 68, 0.15);
  border-color: rgba(239, 68, 68, 0.3);
}

.action-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
</style>
