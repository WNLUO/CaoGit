<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { listen } from '@tauri-apps/api/event';

const props = defineProps<{
  proxyEnabled: boolean;
  testUrl?: string;
}>();

// 显示模式：speed（速率）或 traffic（流量）
type DisplayMode = 'speed' | 'traffic';
const displayMode = ref<DisplayMode>('speed');

// 实时速率数据
const uploadSpeed = ref(0); // bytes/s
const downloadSpeed = ref(0); // bytes/s

// 累计流量数据
const totalUpload = ref(0); // bytes
const totalDownload = ref(0); // bytes

// 延迟数据
const latency = ref(0); // ms
const isTesting = ref(false);

// 从 localStorage 加载历史流量数据
function loadTrafficData() {
  try {
    const saved = localStorage.getItem('git-traffic-stats');
    if (saved) {
      const data = JSON.parse(saved);
      totalUpload.value = data.totalUpload || 0;
      totalDownload.value = data.totalDownload || 0;
    }
  } catch (error) {
    console.error('Failed to load traffic data:', error);
  }
}

// 保存流量数据到 localStorage
function saveTrafficData() {
  try {
    const data = {
      totalUpload: totalUpload.value,
      totalDownload: totalDownload.value,
      lastUpdated: new Date().toISOString()
    };
    localStorage.setItem('git-traffic-stats', JSON.stringify(data));
  } catch (error) {
    console.error('Failed to save traffic data:', error);
  }
}

// 格式化速率 (bytes/s -> MB/s or KB/s)
function formatSpeed(bytesPerSec: number): string {
  if (bytesPerSec === 0) return '0 B/s';
  if (bytesPerSec < 1024) return `${bytesPerSec.toFixed(0)} B/s`;
  if (bytesPerSec < 1024 * 1024) return `${(bytesPerSec / 1024).toFixed(1)} KB/s`;
  return `${(bytesPerSec / (1024 * 1024)).toFixed(2)} MB/s`;
}

// 格式化流量 (bytes -> GB or MB or KB)
function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  if (bytes < 1024) return `${bytes.toFixed(0)} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
}

const formattedUploadSpeed = computed(() => formatSpeed(uploadSpeed.value));
const formattedDownloadSpeed = computed(() => formatSpeed(downloadSpeed.value));
const formattedTotalUpload = computed(() => formatBytes(totalUpload.value));
const formattedTotalDownload = computed(() => formatBytes(totalDownload.value));
const formattedLatency = computed(() => {
  if (latency.value === 0) return '--';
  return `${latency.value}ms`;
});

// 点击处理：切换显示模式 + 触发网络测试
async function handleClick() {
  // 切换显示模式
  displayMode.value = displayMode.value === 'speed' ? 'traffic' : 'speed';

  // 触发延迟测试
  await testNetworkLatency();
}

// 网络延迟测试
async function testNetworkLatency() {
  if (isTesting.value) return;

  isTesting.value = true;
  const testUrl = props.testUrl || 'https://www.github.com';

  try {
    // 测试3次取平均值
    const latencyTests = [];
    for (let i = 0; i < 3; i++) {
      const startTime = performance.now();
      try {
        await fetch(testUrl, {
          method: 'HEAD',
          cache: 'no-cache',
          mode: 'no-cors'
        });
        const endTime = performance.now();
        latencyTests.push(endTime - startTime);
      } catch (e) {
        // 忽略单次失败
      }
    }

    if (latencyTests.length > 0) {
      latency.value = Math.round(latencyTests.reduce((a, b) => a + b, 0) / latencyTests.length);
    } else {
      latency.value = 0;
    }
  } catch (error) {
    console.error('Network test failed:', error);
    latency.value = 0;
  } finally {
    isTesting.value = false;
  }
}

// 监听 Git 进度事件
let unlistenProgress: (() => void) | null = null;

onMounted(async () => {
  // 加载历史流量数据
  loadTrafficData();

  // 监听 Tauri 的 git-progress 事件
  try {
    unlistenProgress = await listen<{
      operation_type: string;
      total_objects: number;
      received_objects: number;
      total_bytes: number;
      received_bytes: number;
      speed_bytes_per_sec: number;
    }>('git-progress', (event) => {
      const progress = event.payload;

      if (progress.operation_type === 'upload') {
        uploadSpeed.value = progress.speed_bytes_per_sec;
        const bytesIncrement = progress.received_bytes;
        if (bytesIncrement > 0) {
          totalUpload.value += bytesIncrement;
          saveTrafficData();
        }
      } else if (progress.operation_type === 'download') {
        downloadSpeed.value = progress.speed_bytes_per_sec;
        const bytesIncrement = progress.received_bytes;
        if (bytesIncrement > 0) {
          totalDownload.value += bytesIncrement;
          saveTrafficData();
        }
      }
    });
  } catch (error) {
    console.error('Failed to listen to git-progress event:', error);
  }
});

onUnmounted(() => {
  if (unlistenProgress) {
    unlistenProgress();
  }

  // 保存最终的流量数据
  saveTrafficData();
});
</script>

<template>
  <div
    class="network-status"
    :class="{ clickable: true, [displayMode]: true, testing: isTesting }"
    @click="handleClick"
    :title="`点击切换显示模式和测速 (当前: ${displayMode === 'speed' ? '速率' : '流量'})`"
  >
    <!-- 速率模式 -->
    <div v-if="displayMode === 'speed'" class="stats">
      <div class="stat-group">
        <div class="stat-item">
          <span class="stat-icon">↑</span>
          <span class="stat-label">上传</span>
          <span class="stat-value">{{ formattedUploadSpeed }}</span>
        </div>

        <div class="stat-item">
          <span class="stat-icon">↓</span>
          <span class="stat-label">下载</span>
          <span class="stat-value">{{ formattedDownloadSpeed }}</span>
        </div>
      </div>

      <div class="divider"></div>

      <div class="latency-item">
        <span class="latency-label">延迟</span>
        <span class="latency-value" :class="{ testing: isTesting }">
          {{ isTesting ? '测试中...' : formattedLatency }}
        </span>
      </div>
    </div>

    <!-- 流量模式 -->
    <div v-else class="stats">
      <div class="stat-group">
        <div class="stat-item">
          <span class="stat-icon">↑</span>
          <span class="stat-label">上传</span>
          <span class="stat-value traffic">{{ formattedTotalUpload }}</span>
        </div>

        <div class="stat-item">
          <span class="stat-icon">↓</span>
          <span class="stat-label">下载</span>
          <span class="stat-value traffic">{{ formattedTotalDownload }}</span>
        </div>
      </div>

      <div class="divider"></div>

      <div class="latency-item">
        <span class="latency-label">延迟</span>
        <span class="latency-value" :class="{ testing: isTesting }">
          {{ isTesting ? '测试中...' : formattedLatency }}
        </span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.network-status {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xs) var(--spacing-sm);
  background-color: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  width: 100%;
  min-width: 0;
  overflow: hidden;
  transition: all var(--transition-fast);
}

.network-status.clickable {
  cursor: pointer;
}

.network-status.clickable:hover {
  background-color: var(--bg-hover);
  transform: translateY(-1px);
}

.network-status.clickable:active {
  transform: translateY(0);
}

.network-status.testing {
  opacity: 0.8;
}

/* 速率模式样式 */
.network-status.speed {
  border-left: 3px solid #3b82f6;
}

/* 流量模式样式 */
.network-status.traffic {
  border-left: 3px solid #22c55e;
}

.stats {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
}

.stat-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.stat-icon {
  font-size: 14px;
  color: var(--text-tertiary);
  font-weight: 600;
  flex-shrink: 0;
}

.stat-label {
  font-size: 10px;
  color: var(--text-tertiary);
  flex-shrink: 0;
  min-width: 28px;
}

.stat-value {
  font-size: 11px;
  font-weight: var(--font-weight-semibold);
  color: var(--accent-color);
  white-space: nowrap;
  font-family: monospace;
  flex: 1;
  text-align: left;
}

.stat-value.traffic {
  color: #22c55e;
}

.divider {
  width: 1px;
  height: 32px;
  background-color: var(--border-color);
  flex-shrink: 0;
}

.latency-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  min-width: 60px;
}

.latency-label {
  font-size: 10px;
  color: var(--text-tertiary);
}

.latency-value {
  font-size: 11px;
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  font-family: monospace;
  white-space: nowrap;
}

.latency-value.testing {
  color: var(--accent-color);
  animation: pulse 1s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}
</style>
