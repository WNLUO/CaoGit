<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { toastStore } from '../../stores/toastStore';

const version = ref('0.1.0');
const isChecking = ref(false);
const showMenu = ref(false);

// 从 Tauri 配置读取版本号
async function loadVersion() {
  try {
    const response = await invoke<{ version: string }>('get_app_version');
    if (response) {
      version.value = response.version;
    }
  } catch (error) {
    console.error('Failed to load version:', error);
  }
}

async function checkForUpdates() {
  isChecking.value = true;
  try {
    const response = await invoke<any>('check_for_updates');
    if (response.success) {
      if (response.has_update) {
        toastStore.success(`发现新版本: v${response.latest_version}`);
      } else {
        toastStore.success('已是最新版本');
      }
    } else {
      toastStore.error(`检查更新失败: ${response.error}`);
    }
  } catch (error) {
    toastStore.error(`检查更新错误: ${error}`);
  } finally {
    isChecking.value = false;
    showMenu.value = false;
  }
}

function openReleases() {
  // 打开当前项目的 GitHub Release 页面
  // 需要替换为实际的仓库地址
  const releaseUrl = 'https://github.com/wnluo/caogit/releases';
  window.open(releaseUrl, '_blank');
  showMenu.value = false;
}

onMounted(() => {
  loadVersion();
});
</script>

<template>
  <div class="version-display">
    <button class="version-button" @click="showMenu = !showMenu" title="查看版本信息">
      v{{ version }}
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polyline points="6 9 12 15 18 9"></polyline>
      </svg>
    </button>

    <div v-if="showMenu" class="version-menu">
      <div class="menu-header">
        <span>版本信息</span>
      </div>
      <div class="menu-item" disabled>
        当前版本: v{{ version }}
      </div>
      <hr />
      <button class="menu-item" :disabled="isChecking" @click="checkForUpdates">
        <span v-if="!isChecking">检查更新</span>
        <span v-else>检查中...</span>
      </button>
      <button class="menu-item" @click="openReleases">
        查看发布历史
      </button>
      <div class="menu-footer">
        <span class="build-info">Git管理器</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.version-display {
  position: relative;
  display: flex;
  align-items: center;
  z-index: 101;
}

.version-button {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-medium);
  cursor: pointer;
  transition: all var(--transition-fast);
  white-space: nowrap;
}

.version-button:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.version-button:active {
  transform: translateY(0.5px);
}

.version-button svg {
  transition: transform var(--transition-fast);
}

.version-display.open .version-button svg {
  transform: rotate(180deg);
}

.version-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 8px;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  min-width: 180px;
  z-index: 10000;
  overflow: hidden;
}

.menu-header {
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid var(--border-color);
  font-size: var(--font-size-xs);
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.menu-item {
  display: block;
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background: none;
  border: none;
  text-align: left;
  cursor: pointer;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  transition: background-color var(--transition-fast);
  font-weight: 500;
}

.menu-item:not([disabled]):hover {
  background-color: var(--bg-hover);
}

.menu-item[disabled] {
  color: var(--text-secondary);
  cursor: default;
  font-weight: 400;
  font-size: var(--font-size-xs);
}

.menu-item[disabled]:hover {
  background-color: transparent;
}

.menu-footer {
  padding: var(--spacing-xs) var(--spacing-md);
  border-top: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
  font-size: 10px;
  color: var(--text-tertiary);
  text-align: center;
}

.build-info {
  display: inline-block;
}

hr {
  margin: 4px 0;
  border: none;
  border-top: 1px solid var(--border-color);
}
</style>
