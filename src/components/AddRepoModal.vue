<script setup lang="ts">
import { ref, computed } from 'vue';
import { GitApi } from '../services/gitApi';
import { open } from '@tauri-apps/plugin-dialog';
import { debugStore } from '../stores/debugStore';
import InitRepoModal from './InitRepoModal.vue';

const props = defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'repo-added', path: string): void;
}>();

type AddMode = 'local' | 'clone';
const addMode = ref<AddMode>('local');

// Local path
const localPath = ref('');

// Clone settings
const cloneUrl = ref('');
const cloneTargetPath = ref('');

// Proxy settings
type ProxyMode = 'none' | 'global' | 'custom';
const proxyMode = ref<ProxyMode>('global');

const customProxyEnabled = ref(false);
const customProxyType = ref<'http' | 'https' | 'socks5'>('http');
const customProxyHost = ref('127.0.0.1');
const customProxyPort = ref('7890');
const customProxyUsername = ref('');
const customProxyPassword = ref('');

const isProcessing = ref(false);
const isInitModalOpen = ref(false);
const pathToInit = ref('');

async function selectLocalPath() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择本地Git仓库路径'
    });

    if (selected && typeof selected === 'string') {
      localPath.value = selected;
    }
  } catch (error) {
    debugStore.logError('选择路径失败', error as Error, 'AddRepoModal.selectLocalPath');
  }
}

async function selectCloneTarget() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择克隆目标路径'
    });

    if (selected && typeof selected === 'string') {
      cloneTargetPath.value = selected;
    }
  } catch (error) {
    debugStore.logError('选择路径失败', error as Error, 'AddRepoModal.selectCloneTarget');
  }
}

async function handleAddRepo() {
  isProcessing.value = true;

  try {
    if (addMode.value === 'local') {
      await addLocalRepo();
    } else {
      await cloneRemoteRepo();
    }
  } catch (error: any) {
    alert('操作失败: ' + error.message);
  } finally {
    isProcessing.value = false;
  }
}

async function addLocalRepo() {
  if (!localPath.value) {
    debugStore.logWarning('未选择路径', 'AddRepoModal.addLocalRepo');
    return;
  }

  try {
    // Check if it's a git repository
    const response = await GitApi.openRepository(localPath.value);

    if (response.success) {
      emit('repo-added', localPath.value);
      emit('close');
      // 重置表单
      localPath.value = '';
    } else {
      // Not a git repo, open init modal directly
      debugStore.logInfo(
        `该目录不是Git仓库，打开初始化对话框: ${localPath.value}`,
        'AddRepoModal.addLocalRepo'
      );

      pathToInit.value = localPath.value;
      isInitModalOpen.value = true;
    }
  } catch (error) {
    debugStore.logError('添加本地仓库失败', error as Error, 'AddRepoModal.addLocalRepo');
  }
}

async function cloneRemoteRepo() {
  if (!cloneUrl.value || !cloneTargetPath.value) {
    alert('请填写克隆URL和目标路径');
    return;
  }

  // TODO: Apply proxy settings
  // TODO: Use GitApi.cloneRepository with proxy settings

  const response = await GitApi.cloneRepository(cloneUrl.value, cloneTargetPath.value);

  if (response.success) {
    emit('repo-added', cloneTargetPath.value);
    emit('close');
  } else {
    throw new Error(response.error || 'Clone failed');
  }
}

function handleInitSuccess(path: string) {
  emit('repo-added', path);
  emit('close');
  localPath.value = '';
  isInitModalOpen.value = false;
  pathToInit.value = '';
}

function handleInitCancel() {
  isInitModalOpen.value = false;
  pathToInit.value = '';
  localPath.value = '';
}
</script>

<template>
  <div v-if="isOpen" class="modal-overlay" @click="emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>添加仓库</h3>
        <button class="close-btn" @click="emit('close')">×</button>
      </div>

      <div class="modal-body">
        <!-- Mode Selection -->
        <div class="mode-tabs">
          <button
            :class="['tab', { active: addMode === 'local' }]"
            @click="addMode = 'local'"
          >
            本地路径
          </button>
          <button
            :class="['tab', { active: addMode === 'clone' }]"
            @click="addMode = 'clone'"
          >
            克隆远程仓库
          </button>
        </div>

        <!-- Local Path Mode -->
        <div v-if="addMode === 'local'" class="form-section">
          <div class="input-group">
            <label>本地路径</label>
            <div class="path-input">
              <input
                v-model="localPath"
                type="text"
                placeholder="/path/to/repository"
                readonly
              >
              <button class="browse-btn" @click="selectLocalPath">浏览</button>
            </div>
          </div>
        </div>

        <!-- Clone Mode -->
        <div v-if="addMode === 'clone'" class="form-section">
          <div class="input-group">
            <label>远程仓库URL</label>
            <input
              v-model="cloneUrl"
              type="text"
              placeholder="https://github.com/user/repo.git"
            >
          </div>

          <div class="input-group">
            <label>克隆到</label>
            <div class="path-input">
              <input
                v-model="cloneTargetPath"
                type="text"
                placeholder="/path/to/target"
                readonly
              >
              <button class="browse-btn" @click="selectCloneTarget">浏览</button>
            </div>
          </div>

          <div class="divider"></div>

          <!-- Proxy Settings -->
          <h4>代理设置</h4>

          <div class="input-group">
            <label>代理模式</label>
            <select v-model="proxyMode">
              <option value="none">不使用代理</option>
              <option value="global">使用全局代理</option>
              <option value="custom">自定义代理</option>
            </select>
          </div>

          <div v-if="proxyMode === 'custom'" class="custom-proxy-settings">
            <div class="input-group">
              <label>代理类型</label>
              <select v-model="customProxyType">
                <option value="http">HTTP</option>
                <option value="https">HTTPS</option>
                <option value="socks5">SOCKS5</option>
              </select>
            </div>

            <div class="input-row">
              <div class="input-group">
                <label>主机</label>
                <input v-model="customProxyHost" type="text" placeholder="127.0.0.1">
              </div>
              <div class="input-group">
                <label>端口</label>
                <input v-model="customProxyPort" type="text" placeholder="7890">
              </div>
            </div>

            <div v-if="customProxyType === 'socks5'" class="input-row">
              <div class="input-group">
                <label>用户名（可选）</label>
                <input v-model="customProxyUsername" type="text">
              </div>
              <div class="input-group">
                <label>密码（可选）</label>
                <input v-model="customProxyPassword" type="password">
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-cancel" @click="emit('close')">取消</button>
        <button
          class="btn-save"
          @click="handleAddRepo"
          :disabled="isProcessing"
        >
          {{ isProcessing ? '处理中...' : (addMode === 'local' ? '添加' : '克隆') }}
        </button>
      </div>
    </div>

    <!-- Init Repository Modal -->
    <InitRepoModal
      :is-open="isInitModalOpen"
      :initial-path="pathToInit"
      @close="handleInitCancel"
      @success="handleInitSuccess"
    />
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--bg-primary);
  border-radius: var(--radius-lg);
  width: 550px;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
}

.modal-header {
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
  position: sticky;
  top: 0;
  background-color: var(--bg-primary);
  z-index: 1;
}

.modal-header h3 {
  font-size: var(--font-size-lg);
  font-weight: 600;
}

.close-btn {
  font-size: 1.5rem;
  line-height: 1;
  color: var(--text-secondary);
}

.close-btn:hover {
  color: var(--text-primary);
}

.modal-body {
  padding: var(--spacing-lg);
}

.mode-tabs {
  display: flex;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);
}

.tab {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: var(--font-size-base);
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: all var(--transition-fast);
}

.tab.active {
  color: var(--accent-color);
  border-bottom-color: var(--accent-color);
}

.form-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.input-group label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.path-input {
  display: flex;
  gap: var(--spacing-sm);
}

.path-input input {
  flex: 1;
}

.browse-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
}

.browse-btn:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

input[type="text"],
input[type="password"],
select {
  padding: var(--spacing-sm);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  width: 100%;
}

input:focus,
select:focus {
  outline: 2px solid var(--accent-color);
  border-color: transparent;
}

.input-row {
  display: flex;
  gap: var(--spacing-md);
}

.input-row .input-group {
  flex: 1;
}

h4 {
  margin-bottom: var(--spacing-xs);
  font-size: var(--font-size-base);
  color: var(--text-primary);
}

.custom-proxy-settings {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
  padding: var(--spacing-md);
  background-color: var(--bg-secondary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
}

.divider {
  height: 1px;
  background-color: var(--border-color);
  margin: var(--spacing-md) 0;
}

.modal-footer {
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  position: sticky;
  bottom: 0;
  background-color: var(--bg-primary);
}

.btn-cancel {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  color: var(--text-secondary);
}

.btn-cancel:hover {
  background-color: var(--bg-secondary);
}

.btn-save {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  background-color: var(--accent-color);
  color: white;
}

.btn-save:hover {
  background-color: var(--accent-hover);
}

.btn-save:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
