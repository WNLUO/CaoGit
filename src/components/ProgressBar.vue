<script setup lang="ts">
import { ref, computed, watch } from 'vue';

interface Props {
  show: boolean;
  operation: 'push' | 'pull' | 'fetch' | '';
  progress?: number; // 0-100
  message?: string;
}

const props = withDefaults(defineProps<Props>(), {
  show: false,
  operation: '',
  progress: 0,
  message: ''
});

// 模拟进度增长（因为 Git 命令可能不提供实时进度）
const simulatedProgress = ref(0);
let progressInterval: number | null = null;

watch(() => props.show, (newVal) => {
  if (newVal) {
    simulatedProgress.value = 0;
    startSimulatedProgress();
  } else {
    stopSimulatedProgress();
    simulatedProgress.value = 0;
  }
});

function startSimulatedProgress() {
  stopSimulatedProgress();

  progressInterval = window.setInterval(() => {
    if (simulatedProgress.value < 90) {
      // 进度越高，增长越慢
      const increment = Math.max(1, (90 - simulatedProgress.value) / 10);
      simulatedProgress.value = Math.min(90, simulatedProgress.value + increment);
    }
  }, 200);
}

function stopSimulatedProgress() {
  if (progressInterval !== null) {
    clearInterval(progressInterval);
    progressInterval = null;
  }
}

const displayProgress = computed(() => {
  return props.progress > 0 ? props.progress : simulatedProgress.value;
});

const operationText = computed(() => {
  switch (props.operation) {
    case 'push':
      return '推送中';
    case 'pull':
      return '拉取中';
    case 'fetch':
      return '获取中';
    default:
      return '处理中';
  }
});

const displayMessage = computed(() => {
  return props.message || operationText.value;
});
</script>

<template>
  <Transition name="progress-fade">
    <div v-if="show" class="progress-overlay">
      <div class="progress-container">
        <div class="progress-header">
          <div class="progress-icon">
            <svg class="spinner" viewBox="0 0 24 24" fill="none">
              <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" stroke-dasharray="60" stroke-dashoffset="20" stroke-linecap="round" />
            </svg>
          </div>
          <div class="progress-text">
            <div class="progress-title">{{ displayMessage }}</div>
            <div class="progress-percentage">{{ Math.round(displayProgress) }}%</div>
          </div>
        </div>

        <div class="progress-bar-container">
          <div class="progress-bar-bg">
            <div
              class="progress-bar-fill"
              :style="{ width: displayProgress + '%' }"
            ></div>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.progress-fade-enter-active,
.progress-fade-leave-active {
  transition: opacity 0.3s ease;
}

.progress-fade-enter-from,
.progress-fade-leave-to {
  opacity: 0;
}

.progress-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  backdrop-filter: blur(2px);
}

.progress-container {
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  min-width: 400px;
  box-shadow: var(--shadow-lg);
}

.progress-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-md);
}

.progress-icon {
  width: 40px;
  height: 40px;
  color: var(--accent-color);
}

.spinner {
  width: 100%;
  height: 100%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.progress-text {
  flex: 1;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.progress-title {
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
}

.progress-percentage {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  font-family: monospace;
}

.progress-bar-container {
  width: 100%;
}

.progress-bar-bg {
  width: 100%;
  height: 8px;
  background-color: var(--bg-secondary);
  border-radius: var(--radius-full);
  overflow: hidden;
}

.progress-bar-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--accent-color), var(--accent-hover));
  border-radius: var(--radius-full);
  transition: width 0.3s ease;
  position: relative;
  overflow: hidden;
}

.progress-bar-fill::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.3),
    transparent
  );
  animation: shimmer 1.5s infinite;
}

@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}
</style>
