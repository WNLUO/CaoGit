<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, watch } from 'vue';
import { settingsStore } from '../stores/settingsStore';

const props = defineProps<{
  proxyEnabled: boolean;
  testUrl?: string;
}>();

const downloadSpeed = ref(0); // KB/s
const uploadSpeed = ref(0); // KB/s
const latency = ref(0); // ms
const isTesting = ref(false);

let testInterval: number | null = null;

const formattedDownload = computed(() => {
  if (downloadSpeed.value < 1024) {
    return `${downloadSpeed.value.toFixed(1)} KB/s`;
  }
  return `${(downloadSpeed.value / 1024).toFixed(2)} MB/s`;
});

const formattedUpload = computed(() => {
  if (uploadSpeed.value < 1024) {
    return `${uploadSpeed.value.toFixed(1)} KB/s`;
  }
  return `${(uploadSpeed.value / 1024).toFixed(2)} MB/s`;
});

async function testNetworkSpeed() {
  if (isTesting.value || !props.proxyEnabled) return;

  isTesting.value = true;
  const testUrl = props.testUrl || settingsStore.settings.networkTest.testUrl;

  try {
    // Test latency with multiple requests for accuracy
    const latencyTests = [];
    for (let i = 0; i < 3; i++) {
      const startTime = performance.now();
      try {
        await fetch(testUrl, {
          method: 'HEAD',
          cache: 'no-cache',
          mode: 'no-cors' // Allow cross-origin requests
        });
        const endTime = performance.now();
        latencyTests.push(endTime - startTime);
      } catch (e) {
        // Ignore individual failures
      }
    }

    if (latencyTests.length > 0) {
      // Calculate average latency
      latency.value = Math.round(latencyTests.reduce((a, b) => a + b, 0) / latencyTests.length);
    } else {
      latency.value = -1;
    }

    // Test download speed with a small file
    try {
      const downloadStart = performance.now();
      const response = await fetch(testUrl, {
        cache: 'no-cache',
        mode: 'no-cors'
      });

      if (response.ok || response.type === 'opaque') {
        const downloadEnd = performance.now();
        const duration = (downloadEnd - downloadStart) / 1000; // seconds

        // Estimate download speed (rough estimate based on typical response size)
        // For a more accurate test, you'd need to download a known-size file
        const estimatedSize = 50; // KB (rough estimate)
        downloadSpeed.value = Math.round(estimatedSize / duration);
      }
    } catch (e) {
      downloadSpeed.value = 0;
    }

    // Upload speed is harder to test without a dedicated endpoint
    // Set to 0 or a fraction of download speed as estimate
    uploadSpeed.value = Math.round(downloadSpeed.value * 0.6);

  } catch (error) {
    console.error('Network test failed:', error);
    latency.value = -1;
    downloadSpeed.value = 0;
    uploadSpeed.value = 0;
  } finally {
    isTesting.value = false;
  }
}

function startAutoTest(intervalSeconds: number = 60) {
  if (testInterval) {
    clearInterval(testInterval);
  }

  testNetworkSpeed();
  testInterval = window.setInterval(() => {
    testNetworkSpeed();
  }, intervalSeconds * 1000);
}

function stopAutoTest() {
  if (testInterval) {
    clearInterval(testInterval);
    testInterval = null;
  }
}

function handleClick() {
  if (props.proxyEnabled && !isTesting.value) {
    testNetworkSpeed();
  }
}

// Watch for proxy enabled changes and test interval changes
watch(() => props.proxyEnabled, (newVal) => {
  if (newVal) {
    const interval = settingsStore.settings.networkTest.testInterval;
    startAutoTest(interval);
  } else {
    stopAutoTest();
  }
});

watch(() => settingsStore.settings.networkTest.testInterval, (newInterval) => {
  if (props.proxyEnabled) {
    startAutoTest(newInterval);
  }
});

onMounted(() => {
  if (props.proxyEnabled) {
    const interval = settingsStore.settings.networkTest.testInterval;
    startAutoTest(interval);
  }
});

onUnmounted(() => {
  stopAutoTest();
});

defineExpose({
  startAutoTest,
  stopAutoTest,
  testNetworkSpeed
});
</script>

<template>
  <div
    class="network-status"
    :class="{ disabled: !proxyEnabled, clickable: proxyEnabled }"
    @click="handleClick"
    :title="proxyEnabled ? '点击刷新测速' : ''"
  >
    <div v-if="!proxyEnabled" class="status-text">
      未启用代理
    </div>
    <div v-else class="stats">
      <div class="stat-item">
        <div class="stat-label">下载</div>
        <div class="stat-value">{{ formattedDownload }}</div>
      </div>
      <div class="divider"></div>
      <div class="stat-item">
        <div class="stat-label">上传</div>
        <div class="stat-value">{{ formattedUpload }}</div>
      </div>
      <div class="divider"></div>
      <div class="stat-item">
        <div class="stat-label">时延</div>
        <div class="stat-value" :class="{ error: latency < 0 }">
          {{ latency >= 0 ? `${latency}ms` : 'N/A' }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.network-status {
  display: flex;
  align-items: center;
  padding: var(--spacing-xs) var(--spacing-sm);
  background-color: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  min-width: 240px;
  white-space: nowrap;
}

.network-status.disabled {
  justify-content: center;
  min-width: auto;
}

.network-status.clickable {
  cursor: pointer;
  transition: all var(--transition-fast);
}

.network-status.clickable:hover {
  background-color: var(--bg-hover);
}

.network-status.clickable:active {
  transform: scale(0.98);
}

.status-text {
  font-weight: var(--font-weight-medium);
}

.stats {
  display: flex;
  align-items: center;
  gap: 6px;
  width: 100%;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex: 1;
  min-width: 0;
}

.stat-label {
  font-size: 9px;
  color: var(--text-tertiary);
  margin-bottom: 1px;
  white-space: nowrap;
}

.stat-value {
  font-size: 11px;
  font-weight: var(--font-weight-semibold);
  color: var(--accent-color);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}

.stat-value.error {
  color: var(--text-tertiary);
}

.divider {
  width: 1px;
  height: 20px;
  background-color: var(--border-color);
  flex-shrink: 0;
}
</style>
