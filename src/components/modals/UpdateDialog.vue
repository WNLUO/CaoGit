<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { toastStore } from '../../stores/toastStore';

interface UpdateInfo {
  currentVersion: string;
  latestVersion: string;
  releaseNotes: string;
  downloadUrl: string;
  releasedAt: string;
}

interface AppStoreUpdateInfo {
  has_update: boolean;
  current_version: string;
  latest_version: string;
  update_message: string;
}

interface PlatformDownloadInfo {
  url: string;
  filename: string;
  platform: string;
}

interface UpdateInstallResult {
  status: string;
  file_path: string;
  message: string;
}

interface DownloadProgress {
  downloaded: number;
  total: number;
  progress: number;
}

type Platform = 'windows' | 'macos' | 'linux' | 'unknown';

const showDialog = ref(false);
const isDownloading = ref(false);
const downloadProgress = ref(0);
const downloadedSize = ref(0);
const totalSize = ref(0);
const updateInfo = ref<UpdateInfo | null>(null);
const platform = ref<Platform>('unknown');
const updateStatus = ref<'idle' | 'downloading' | 'installing' | 'success' | 'ready_to_install' | 'error'>('idle');
const errorMessage = ref('');
const resultMessage = ref('');

// 检测是否为 App Store 版本
const isAppStore = import.meta.env.VITE_APPSTORE === 'true';

let unlistenProgress: UnlistenFn | null = null;
let unlistenUpdateAvailable: UnlistenFn | null = null;

const formattedDate = computed(() => {
  if (!updateInfo.value) return '';
  try {
    const date = new Date(updateInfo.value.releasedAt);
    return date.toLocaleDateString('zh-CN', { year: 'numeric', month: 'long', day: 'numeric' });
  } catch {
    return updateInfo.value.releasedAt;
  }
});

const formattedProgress = computed(() => {
  if (totalSize.value === 0) return '';
  const downloaded = (downloadedSize.value / 1024 / 1024).toFixed(1);
  const total = (totalSize.value / 1024 / 1024).toFixed(1);
  return `${downloaded} MB / ${total} MB`;
});

const platformInfo = computed(() => {
  // App Store 版本特殊处理
  if (isAppStore && platform.value === 'macos') {
    return {
      icon: 'apple',
      title: 'Mac App Store',
      description: '点击"前往更新"将打开 Mac App Store，请在 App Store 中完成更新',
      buttonText: '前往 App Store 更新',
    };
  }

  switch (platform.value) {
    case 'windows':
      return {
        icon: 'windows',
        title: 'Windows',
        description: '点击"立即更新"将自动下载并安装新版本',
        buttonText: '立即更新',
      };
    case 'macos':
      return {
        icon: 'apple',
        title: 'macOS',
        description: '点击"立即更新"将下载安装包并生成自动安装脚本',
        buttonText: '立即更新',
      };
    case 'linux':
      return {
        icon: 'linux',
        title: 'Linux',
        description: '点击"立即更新"将下载 AppImage 到 Downloads 文件夹',
        buttonText: '立即更新',
      };
    default:
      return {
        icon: 'unknown',
        title: '更新',
        description: '请访问 Release 页面下载最新版本',
        buttonText: '前往下载',
      };
  }
});

onMounted(async () => {
  // 检测操作系统
  detectPlatform();

  // 监听 update-available 事件
  unlistenUpdateAvailable = await listen<void>('update-available', async () => {
    await loadUpdateInfo();
    if (updateInfo.value) {
      showDialog.value = true;
    }
  });

  // 监听下载进度事件
  unlistenProgress = await listen<DownloadProgress>('update-download-progress', (event) => {
    downloadedSize.value = event.payload.downloaded;
    totalSize.value = event.payload.total;
    downloadProgress.value = event.payload.progress;
  });
});

onUnmounted(() => {
  if (unlistenProgress) unlistenProgress();
  if (unlistenUpdateAvailable) unlistenUpdateAvailable();
});

function detectPlatform() {
  const ua = navigator.userAgent.toLowerCase();
  if (ua.includes('win')) {
    platform.value = 'windows';
  } else if (ua.includes('mac')) {
    platform.value = 'macos';
  } else if (ua.includes('linux')) {
    platform.value = 'linux';
  } else {
    platform.value = 'unknown';
  }
}

async function loadUpdateInfo() {
  try {
    if (isAppStore) {
      // App Store 版本：调用专用的检查更新命令
      // 注意：这里使用固定的 GitHub 仓库路径，因为我们在检查应用本身的更新
      // 后端会使用硬编码的仓库信息 (wnluo/caogit)
      const result = await invoke<AppStoreUpdateInfo>('check_update_appstore', {
        repoPath: 'https://github.com/wnluo/caogit',
        githubToken: null
      });

      if (result.has_update) {
        updateInfo.value = {
          currentVersion: result.current_version,
          latestVersion: result.latest_version,
          releaseNotes: result.update_message,
          downloadUrl: '', // App Store 版本不需要下载链接
          releasedAt: new Date().toISOString(),
        };
      } else {
        updateInfo.value = null;
      }
    } else {
      // DMG 版本：使用原有的更新检查逻辑
      const result = await invoke<any>('check_for_updates');
      if (result.success && result.has_update) {
        updateInfo.value = {
          currentVersion: result.current_version,
          latestVersion: result.latest_version,
          releaseNotes: '点击"查看日志"查看完整的发布说明',
          downloadUrl: result.download_url,
          releasedAt: result.released_at,
        };
      } else if (result.success && !result.has_update) {
        updateInfo.value = null;
      }
    }
  } catch (error) {
    console.error('Failed to load update info:', error);
  }
}

async function handleInstallNow() {
  if (!updateInfo.value) return;

  // App Store 版本：直接打开 App Store
  if (isAppStore) {
    try {
      await invoke('open_app_store');
      toastStore.success('已打开 Mac App Store，请在 App Store 中完成更新');
      showDialog.value = false;
    } catch (error) {
      console.error('Failed to open App Store:', error);
      toastStore.error(`打开 App Store 失败: ${error}`);
    }
    return;
  }

  // DMG 版本：原有的自动下载安装逻辑
  isDownloading.value = true;
  updateStatus.value = 'downloading';
  downloadProgress.value = 0;
  downloadedSize.value = 0;
  totalSize.value = 0;
  errorMessage.value = '';
  resultMessage.value = '';

  try {
    // 获取平台特定的下载信息
    const downloadInfo = await invoke<PlatformDownloadInfo>('get_platform_download_url', {
      baseUrl: updateInfo.value.downloadUrl,
      version: updateInfo.value.latestVersion
    });

    // 调用安装命令
    const result = await invoke<UpdateInstallResult>('install_update', {
      downloadUrl: downloadInfo.url,
      platform: platform.value,
      version: updateInfo.value.latestVersion
    });

    downloadProgress.value = 100;

    if (result.status === 'installing') {
      // Windows: 安装程序已启动
      updateStatus.value = 'installing';
      resultMessage.value = result.message;
      toastStore.success(result.message);

      // 延迟退出应用，让安装程序接管
      setTimeout(async () => {
        await invoke('exit_app');
      }, 3000);
    } else if (result.status === 'ready_to_install') {
      // macOS: DMG 已挂载并打开，准备拖拽安装
      updateStatus.value = 'ready_to_install';
      resultMessage.value = result.message;
      toastStore.success('DMG 已打开，请拖拽安装');
    } else if (result.status === 'downloaded') {
      // Linux: 下载完成，已打开文件夹
      updateStatus.value = 'success';
      resultMessage.value = result.message;
      toastStore.success('下载完成，已打开文件夹');
    }
  } catch (error) {
    console.error('Failed to install update:', error);
    updateStatus.value = 'error';
    errorMessage.value = String(error);
    toastStore.error(`更新失败: ${error}`);
  } finally {
    isDownloading.value = false;
  }
}

function handleLaterRemind() {
  showDialog.value = false;
  updateStatus.value = 'idle';
  resultMessage.value = '';
  errorMessage.value = '';
}

function handleViewRelease() {
  if (updateInfo.value) {
    // 打开 Release 页面
    invoke('install_update', {
      downloadUrl: updateInfo.value.downloadUrl,
      platform: 'browser',
      version: updateInfo.value.latestVersion
    }).catch(() => {
      // 如果失败，尝试使用 opener
      window.open(updateInfo.value?.downloadUrl, '_blank');
    });
  }
}

// 暴露方法供外部调用（手动检查更新）
async function checkForUpdates() {
  try {
    await loadUpdateInfo();
    if (updateInfo.value) {
      showDialog.value = true;
    } else {
      toastStore.success('已是最新版本');
    }
  } catch (error) {
    toastStore.error('检查更新失败');
  }
}

defineExpose({ checkForUpdates });
</script>

<template>
  <div v-if="showDialog" class="update-overlay">
    <div class="update-dialog">
      <!-- 标题 -->
      <div class="dialog-header">
        <h2>发现新版本</h2>
        <button class="close-btn" @click="handleLaterRemind" :disabled="isDownloading && updateStatus === 'downloading'">×</button>
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

        <!-- 平台信息 -->
        <div class="platform-section" v-if="updateStatus === 'idle'">
          <div class="platform-header">
            <span class="platform-title">{{ platformInfo.title }}</span>
          </div>
          <p class="platform-description">{{ platformInfo.description }}</p>
        </div>

        <!-- 下载进度 -->
        <div v-if="updateStatus === 'downloading'" class="download-section">
          <div class="download-status">
            <span class="status-icon spinning">↻</span>
            <span>正在下载...</span>
          </div>
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: `${downloadProgress}%` }"></div>
          </div>
          <div class="progress-info">
            <span>{{ downloadProgress }}%</span>
            <span v-if="formattedProgress">{{ formattedProgress }}</span>
          </div>
        </div>

        <!-- 安装中 -->
        <div v-if="updateStatus === 'installing'" class="result-section installing">
          <div class="result-icon">⏳</div>
          <div class="result-title">正在安装</div>
          <p class="result-message">{{ resultMessage }}</p>
          <p class="result-hint">应用将在几秒后退出...</p>
        </div>

        <!-- 准备安装 (macOS DMG 已打开) -->
        <div v-if="updateStatus === 'ready_to_install'" class="result-section ready">
          <div class="result-icon">📦</div>
          <div class="result-title">准备安装</div>
          <p class="result-message" v-html="resultMessage.replace(/\n/g, '<br>')"></p>

          <!-- macOS 拖拽提示 -->
          <div class="macos-tip">
            <strong>安装步骤：</strong>
            <p>1. 将 CaoGit 图标拖到 Applications 文件夹</p>
            <p>2. 首次打开时如遇"已损坏"提示，右键点击应用选择"打开"即可</p>
          </div>
        </div>

        <!-- 下载成功 -->
        <div v-if="updateStatus === 'success'" class="result-section success">
          <div class="result-icon">✅</div>
          <div class="result-title">下载完成</div>
          <p class="result-message" v-html="resultMessage.replace(/\n/g, '<br>')"></p>
        </div>

        <!-- 错误信息 -->
        <div v-if="updateStatus === 'error'" class="result-section error">
          <div class="result-icon">❌</div>
          <div class="result-title">更新失败</div>
          <p class="result-message">{{ errorMessage }}</p>
        </div>
      </div>

      <!-- 按钮 -->
      <div class="dialog-footer">
        <button
          class="btn btn-secondary"
          @click="handleLaterRemind"
          :disabled="updateStatus === 'downloading'"
        >
          {{ updateStatus === 'success' || updateStatus === 'error' ? '关闭' : '稍后提醒' }}
        </button>
        <button
          v-if="updateStatus === 'idle' || updateStatus === 'error'"
          class="btn btn-secondary"
          @click="handleViewRelease"
        >
          查看日志
        </button>
        <button
          v-if="updateStatus === 'idle' || updateStatus === 'error'"
          class="btn btn-primary"
          @click="handleInstallNow"
          :disabled="isDownloading"
        >
          {{ platformInfo.buttonText }}
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
  max-width: 480px;
  max-height: 90vh;
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

.close-btn:hover:not(:disabled) {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.close-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.dialog-content {
  padding: var(--spacing-lg);
  flex: 1;
  overflow-y: auto;
}

.version-info {
  margin-bottom: var(--spacing-lg);
  text-align: center;
}

.version-display {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-sm);
}

.current {
  font-size: var(--font-size-base);
  color: var(--text-secondary);
  padding: 4px 12px;
  background-color: var(--bg-secondary);
  border-radius: var(--radius-sm);
}

.arrow {
  color: var(--text-secondary);
  font-weight: 600;
  font-size: 18px;
}

.latest {
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--color-accent);
  padding: 4px 12px;
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: var(--radius-sm);
}

.release-date {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

.platform-section {
  padding: var(--spacing-md);
  background-color: var(--bg-secondary);
  border-radius: var(--radius-md);
}

.platform-header {
  margin-bottom: var(--spacing-sm);
}

.platform-title {
  font-weight: 600;
  color: var(--text-primary);
}

.platform-description {
  margin: 0;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.5;
}

.download-section {
  padding: var(--spacing-lg);
  background-color: var(--bg-secondary);
  border-radius: var(--radius-md);
}

.download-status {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
}

.status-icon {
  font-size: 18px;
}

.status-icon.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.progress-bar {
  width: 100%;
  height: 8px;
  background-color: var(--bg-primary);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: var(--spacing-sm);
}

.progress-fill {
  height: 100%;
  background-color: var(--color-accent);
  transition: width 0.3s ease;
  border-radius: 4px;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
}

.result-section {
  padding: var(--spacing-lg);
  background-color: var(--bg-secondary);
  border-radius: var(--radius-md);
  text-align: center;
}

.result-icon {
  font-size: 48px;
  margin-bottom: var(--spacing-md);
}

.result-title {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: var(--spacing-sm);
}

.result-message {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.6;
  margin: 0;
}

.result-hint {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  margin-top: var(--spacing-md);
}

.result-section.success .result-icon {
  color: var(--color-success, #22c55e);
}

.result-section.error .result-icon {
  color: var(--color-danger, #ef4444);
}

.result-section.installing .result-icon {
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.macos-tip {
  margin-top: var(--spacing-lg);
  padding: var(--spacing-md);
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: var(--radius-sm);
  text-align: left;
}

.macos-tip strong {
  display: block;
  color: var(--color-accent);
  margin-bottom: var(--spacing-xs);
  font-size: var(--font-size-sm);
}

.macos-tip p {
  margin: 0;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
}

.macos-tip code {
  background-color: var(--bg-primary);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
  font-family: monospace;
  font-size: var(--font-size-xs);
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
</style>
