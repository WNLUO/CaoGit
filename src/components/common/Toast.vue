<script setup lang="ts">
export interface ToastMessage {
  id: number;
  type: 'success' | 'error' | 'info' | 'warning';
  message: string;
  duration?: number;
}

defineProps<{
  messages: ToastMessage[];
}>();

const emit = defineEmits<{
  (e: 'remove', id: number): void;
}>();

function getIcon(type: ToastMessage['type']) {
  switch (type) {
    case 'success':
      return '✓';
    case 'error':
      return '✕';
    case 'warning':
      return '⚠';
    case 'info':
      return 'ℹ';
    default:
      return 'ℹ';
  }
}

function getColorClass(type: ToastMessage['type']) {
  switch (type) {
    case 'success':
      return 'toast-success';
    case 'error':
      return 'toast-error';
    case 'warning':
      return 'toast-warning';
    case 'info':
      return 'toast-info';
    default:
      return 'toast-info';
  }
}

function handleClose(id: number) {
  emit('remove', id);
}
</script>

<template>
  <div class="toast-container">
    <TransitionGroup name="toast">
      <div
        v-for="msg in messages"
        :key="msg.id"
        :class="['toast-item', getColorClass(msg.type)]"
        @click="handleClose(msg.id)"
      >
        <span class="toast-icon">{{ getIcon(msg.type) }}</span>
        <span class="toast-message">{{ msg.message }}</span>
        <button class="toast-close" @click.stop="handleClose(msg.id)">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.toast-container {
  position: fixed;
  top: 80px;
  right: 20px;
  z-index: 10000;
  display: flex;
  flex-direction: column;
  gap: 12px;
  pointer-events: none;
}

.toast-item {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 300px;
  max-width: 500px;
  padding: 14px 16px;
  border-radius: var(--radius-md);
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  cursor: pointer;
  pointer-events: auto;
  transition: all 0.3s ease;
}

.toast-item:hover {
  transform: translateX(-4px);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.2);
}

.toast-icon {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: bold;
  flex-shrink: 0;
}

.toast-message {
  flex: 1;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  line-height: 1.4;
  word-break: break-word;
}

.toast-close {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-tertiary);
  flex-shrink: 0;
  transition: all 0.2s ease;
}

.toast-close:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

/* Success */
.toast-success {
  border-left: 4px solid #22c55e;
}

.toast-success .toast-icon {
  background-color: rgba(34, 197, 94, 0.1);
  color: #22c55e;
}

/* Error */
.toast-error {
  border-left: 4px solid #ef4444;
}

.toast-error .toast-icon {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

/* Warning */
.toast-warning {
  border-left: 4px solid #f59e0b;
}

.toast-warning .toast-icon {
  background-color: rgba(245, 158, 11, 0.1);
  color: #f59e0b;
}

/* Info */
.toast-info {
  border-left: 4px solid #3b82f6;
}

.toast-info .toast-icon {
  background-color: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
}

/* Transition animations */
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100px);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100px) scale(0.8);
}

.toast-move {
  transition: transform 0.3s ease;
}
</style>
