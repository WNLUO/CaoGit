<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { toastStore } from '../stores/toastStore';

interface UpdateInfo {
  currentVersion: string;
  latestVersion: string;
  releaseNotes: string;
  downloadUrl: string;
  releasedAt: string;
}

const showDialog = ref(false);
const isDownloading = ref(false);
const downloadProgress = ref(0);
const updateInfo = ref<UpdateInfo | null>(null);
const isMacOS = ref(false);

const formattedDate = computed(() => {
  if (!updateInfo.value) return '';
  try {
    const date = new Date(updateInfo.value.releasedAt);
    return date.toLocaleDateString('zh-CN', { year: 'numeric', month: 'long', day: 'numeric' });
  } catch {
    return updateInfo.value.releasedAt;
  }
});

onMounted(() => {
  // 检测操作系统
  isMacOS.value = navigator.userAgent.includes('Macintosh');

  // 监听 update-available 事件
  listen<void>('update-available', async () => {
    await loadUpdateInfo();
    showDialog.value = true;
  });
});

async function loadUpdateInfo() {
  try {
    const result = await invoke<any>('check_for_updates');
    if (result.success && result.has_update) {
      // 获取发布说明（这里需要从某处获取，可能需要额外的后端支持）
      updateInfo.value = {
        currentVersion: result.current_version,
        latestVersion: result.latest_version,
        releaseNotes: '点击"查看日志"查看完整的发布说明',
        downloadUrl: result.download_url,
        releasedAt: result.released_at,
      };
    }
  } catch (error) {
    console.error('Failed to load update info:', error);
    toastStore.error('加载更新信息失败');
  }
}

async function handleInstallNow() {
  isDownloading.value = true;
  downloadProgress.value = 0;

  try {
    // 模拟下载进度
    const progressInterval = setInterval(() => {
      downloadProgress.value += Math.random() * 30;
      if (downloadProgress.value >= 100) {
        downloadProgress.value = 100;
        clearInterval(progressInterval);
      }
    }, 200);

    // 调用安装命令
    await invoke<any>('install_update');

    // 如果是 macOS，显示引导信息
    if (isMacOS.value) {
      toastStore.success('更新已下载，请打开 Downloads 文件夹完成安装');
      showDialog.value = false;
    } else {
      // 其他系统：重启应用
      setTimeout(() => {
        toastStore.success('更新已安装，应用将在 3 秒后重启');
        setTimeout(() => {
          invoke('restart_app').catch(console.error);
        }, 3000);
      }, 500);
    }
  } catch (error) {
    console.error('Failed to install update:', error);
    toastStore.error(`更新安装失败: ${error}`);
  } finally {
    isDownloading.value = false;
  }
}

function handleLaterRemind() {
  showDialog.value = false;
  toastStore.info('提醒已关闭');
}

function handleViewRelease() {
  if (updateInfo.value) {
    // 使用 opener 插件打开 URL
    invoke('plugin:opener|open', { path: updateInfo.value.downloadUrl }).catch(console.error);
  }
}
</script>

<template>
  <div v-if="showDialog" class="update-overlay">
    <div class="update-dialog">
      <!-- 标题 -->
      <div class="dialog-header">
        <h2>发现新版本</h2>
        <button class="close-btn" @click="handleLaterRemind">×</button>
      </div>

      <!-- 内容 -->
      <div class="dialog-content">
        <!-- 版本信息 -->
        <div class="version-info">
          <div class="version-display">
            <span class="current">v{{ updateInfo?.currentVersion }}</span>
            <span class="arrow">→</span>
            <span class="latest">v{{ updateInfo?.latestVersion }}</span>
          </div>
          <div class="release-date">{{ formattedDate }}</div>
        </div>

        <!-- 发布说明 -->
        <div class="release-notes">
          <h3>更新说明</h3>
          <p>{{ updateInfo?.releaseNotes }}</p>
        </div>

        <!-- 下载进度 -->
        <div v-if="isDownloading" class="download-progress">
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: `${downloadProgress}%` }"></div>
          </div>
          <div class="progress-text">{{ Math.floor(downloadProgress) }}%</div>
        </div>

        <!-- macOS 提示 -->
        <div v-if="isMacOS && !isDownloading" class="platform-info">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="12" y1="8" x2="12" y2="12"></line>
            <line x1="12" y1="16" x2="12.01" y2="16"></line>
          </svg>
          <span>macOS 需要手动安装，下载完成后请打开 Downloads 文件夹</span>
        </div>
      </div>

      <!-- 按钮 -->
      <div class="dialog-footer">
        <button class="btn btn-secondary" @click="handleLaterRemind" :disabled="isDownloading">
          稍后提醒
        </button>
        <button class="btn btn-secondary" @click="handleViewRelease" :disabled="isDownloading">
          查看日志
        </button>
        <button
          class="btn btn-primary"
          @click="handleInstallNow"
          :disabled="isDownloading"
          :class="{ loading: isDownloading }"
        >
          <span v-if="!isDownloading">立即更新</span>
          <span v-else>更新中...</span>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.update-overlay {
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
}

.update-dialog {
  background-color: var(--bg-primary);
  border-radius: var(--radius-lg);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  width: 90%;
  max-width: 500px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.dialog-header {
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.dialog-header h2 {
  margin: 0;
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.close-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.dialog-content {
  padding: var(--spacing-lg);
  flex: 1;
  overflow-y: auto;
}

.version-info {
  margin-bottom: var(--spacing-lg);
}

.version-display {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-sm);
}

.current {
  font-size: var(--font-size-base);
  color: var(--text-secondary);
  padding: 4px 8px;
  background-color: var(--bg-secondary);
  border-radius: var(--radius-sm);
}

.arrow {
  color: var(--text-secondary);
  font-weight: 600;
}

.latest {
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--color-accent);
  padding: 4px 8px;
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: var(--radius-sm);
}

.release-date {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

.release-notes {
  margin-bottom: var(--spacing-lg);
}

.release-notes h3 {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-secondary);
  margin: 0 0 var(--spacing-sm) 0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.release-notes p {
  margin: 0;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  line-height: 1.5;
}

.download-progress {
  margin-bottom: var(--spacing-lg);
}

.progress-bar {
  width: 100%;
  height: 6px;
  background-color: var(--bg-secondary);
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: var(--spacing-sm);
}

.progress-fill {
  height: 100%;
  background-color: var(--color-accent);
  transition: width 0.3s ease;
}

.progress-text {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  text-align: right;
}

.platform-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-md);
  background-color: var(--bg-secondary);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  margin-bottom: var(--spacing-lg);
}

.platform-info svg {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
  color: var(--color-accent);
}

.dialog-footer {
  padding: var(--spacing-lg);
  border-top: 1px solid var(--border-color);
  display: flex;
  gap: var(--spacing-sm);
  justify-content: flex-end;
}

.btn {
  padding: 8px 16px;
  border-radius: var(--radius-md);
  border: none;
  font-size: var(--font-size-sm);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  white-space: nowrap;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background-color: var(--color-accent);
  color: white;
}

.btn-primary:not(:disabled):hover {
  background-color: var(--color-accent-dark, #2563eb);
  transform: translateY(-1px);
}

.btn-primary:not(:disabled):active {
  transform: translateY(0);
}

.btn-secondary {
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn-secondary:not(:disabled):hover {
  background-color: var(--bg-hover);
}

.btn.loading {
  position: relative;
}
</style>
