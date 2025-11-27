<script setup lang="ts">
import { ref, watch } from 'vue';
import type { Repository } from '../types';
import { settingsStore } from '../stores/settingsStore';
import { debugStore } from '../stores/debugStore';

const props = defineProps<{
  isOpen: boolean;
  mode: 'global' | 'repo';
  repo?: Repository;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

// Local state for global proxy form
const proxyEnabled = ref(false);
const proxyHost = ref('127.0.0.1');
const proxyPort = ref('7890');
const proxyType = ref<'http' | 'https' | 'socks5'>('http');
const proxyUsername = ref('');
const proxyPassword = ref('');

// Network test settings
const testUrl = ref('https://github.com');
const testInterval = ref(60); // seconds

// Debug mode setting
const debugModeEnabled = ref(false);

const repoProtocol = ref<'http' | 'https' | 'ssh'>('https');
const repoAuthType = ref<'none' | 'token' | 'password'>('none');
const repoToken = ref('');
const repoUsername = ref('');
const repoPassword = ref('');

const repoProxyEnabled = ref(false);
const repoProxyHost = ref('');
const repoProxyPort = ref('');
const repoProxyType = ref<'http' | 'https' | 'socks5'>('http');
const repoProxyUsername = ref('');
const repoProxyPassword = ref('');

// Sync local state with props when modal opens
watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    if (props.mode === 'global') {
      // Load global settings
      const globalSettings = settingsStore.settings;
      proxyEnabled.value = globalSettings.proxy.enabled;
      proxyType.value = globalSettings.proxy.type;
      proxyHost.value = globalSettings.proxy.host;
      proxyPort.value = globalSettings.proxy.port;
      proxyUsername.value = globalSettings.proxy.username || '';
      proxyPassword.value = globalSettings.proxy.password || '';
      testUrl.value = globalSettings.networkTest.testUrl;
      testInterval.value = globalSettings.networkTest.testInterval;
      debugModeEnabled.value = debugStore.enabled;
    } else if (props.mode === 'repo' && props.repo) {
      // Load repo settings
      repoProtocol.value = props.repo.protocol;
      repoAuthType.value = props.repo.authType;
      repoToken.value = props.repo.token || '';
      repoUsername.value = props.repo.username || '';
      repoPassword.value = props.repo.password || '';

      if (props.repo.proxy) {
        repoProxyEnabled.value = props.repo.proxy.enabled;
        repoProxyHost.value = props.repo.proxy.host;
        repoProxyPort.value = props.repo.proxy.port;
        repoProxyType.value = props.repo.proxy.type as any;
      } else {
        repoProxyEnabled.value = false;
        repoProxyHost.value = '';
        repoProxyPort.value = '';
        repoProxyType.value = 'http';
      }
    }
  }
});

function save() {
  if (props.mode === 'global') {
    // Save global settings
    settingsStore.updateProxy({
      enabled: proxyEnabled.value,
      type: proxyType.value,
      host: proxyHost.value,
      port: proxyPort.value,
      username: proxyUsername.value,
      password: proxyPassword.value
    });

    settingsStore.updateNetworkTest({
      testUrl: testUrl.value,
      testInterval: testInterval.value
    });

    // Save debug mode
    debugStore.setDebugMode(debugModeEnabled.value);

    alert('全局设置已保存');
  } else {
    // TODO: Save repo settings
    alert('仓库设置保存功能即将完善');
  }

  emit('close');
}
</script>

<template>
  <div v-if="isOpen" class="modal-overlay" @click="emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>{{ mode === 'global' ? '全局设置' : '仓库设置' }}</h3>
        <button class="close-btn" @click="emit('close')">×</button>
      </div>

      <div class="modal-body">
        <!-- Global Proxy Settings -->
        <div v-if="mode === 'global'" class="form-group">
          <h4>网络代理</h4>
          <p class="hint">此设置仅影响本软件内的 Git 操作</p>

          <label class="checkbox-label">
            <input type="checkbox" v-model="proxyEnabled">
            启用代理
          </label>

          <div class="input-group" :class="{ disabled: !proxyEnabled }">
            <label>代理类型</label>
            <select v-model="proxyType" :disabled="!proxyEnabled">
              <option value="http">HTTP</option>
              <option value="https">HTTPS</option>
              <option value="socks5">SOCKS5</option>
            </select>
          </div>

          <div class="input-row" :class="{ disabled: !proxyEnabled }">
            <div class="input-group">
              <label>主机 (Host)</label>
              <input v-model="proxyHost" type="text" placeholder="127.0.0.1" :disabled="!proxyEnabled">
            </div>
            <div class="input-group">
              <label>端口 (Port)</label>
              <input v-model="proxyPort" type="text" placeholder="7890" :disabled="!proxyEnabled">
            </div>
          </div>

          <div v-if="proxyType === 'socks5'" class="input-row" :class="{ disabled: !proxyEnabled }">
            <div class="input-group">
              <label>用户名（可选）</label>
              <input v-model="proxyUsername" type="text" :disabled="!proxyEnabled">
            </div>
            <div class="input-group">
              <label>密码（可选）</label>
              <input v-model="proxyPassword" type="password" :disabled="!proxyEnabled">
            </div>
          </div>

          <div class="divider"></div>

          <h4>网络测速</h4>
          <p class="hint">定期测试代理网络速度和延迟</p>

          <div class="input-group">
            <label>测速地址</label>
            <input v-model="testUrl" type="text" placeholder="https://github.com">
          </div>

          <div class="input-group">
            <label>测速间隔 (秒)</label>
            <input v-model.number="testInterval" type="number" min="10" max="600" placeholder="60">
          </div>

          <div class="divider"></div>

          <h4>开发调试</h4>
          <p class="hint">启用后，应用中的错误会自动弹出调试窗口</p>

          <label class="checkbox-label">
            <input type="checkbox" v-model="debugModeEnabled">
            启用调试模式
          </label>
        </div>

        <!-- Repository Settings -->
        <div v-if="mode === 'repo'" class="form-group">
          <h4>{{ repo?.name }} 配置</h4>

          <div class="input-group">
            <label>传输协议</label>
            <select v-model="repoProtocol">
              <option value="https">HTTPS</option>
              <option value="ssh">SSH</option>
              <option value="http">HTTP</option>
            </select>
          </div>

          <div class="input-group">
            <label>认证方式</label>
            <select v-model="repoAuthType">
              <option value="none">无 (公开仓库/SSH Key)</option>
              <option value="token">Token (推荐)</option>
              <option value="password">用户名/密码</option>
            </select>
          </div>

          <div v-if="repoAuthType === 'token'" class="input-group">
            <label>Access Token</label>
            <input v-model="repoToken" type="password" placeholder="ghp_...">
          </div>

          <div v-if="repoAuthType === 'password'" class="input-row">
            <div class="input-group">
              <label>用户名</label>
              <input v-model="repoUsername" type="text">
            </div>
            <div class="input-group">
              <label>密码</label>
              <input v-model="repoPassword" type="password">
            </div>
          </div>

          <div class="divider"></div>

          <h4>仓库独立代理</h4>
          <p class="hint">如果不启用，将使用全局代理设置</p>

          <label class="checkbox-label">
            <input type="checkbox" v-model="repoProxyEnabled">
            启用独立代理
          </label>

          <div class="input-group" :class="{ disabled: !repoProxyEnabled }">
            <label>代理类型</label>
            <select v-model="repoProxyType" :disabled="!repoProxyEnabled">
              <option value="http">HTTP</option>
              <option value="https">HTTPS</option>
              <option value="socks5">SOCKS5</option>
            </select>
          </div>

          <div class="input-row" :class="{ disabled: !repoProxyEnabled }">
            <div class="input-group">
              <label>主机 (Host)</label>
              <input v-model="repoProxyHost" type="text" placeholder="127.0.0.1" :disabled="!repoProxyEnabled">
            </div>
            <div class="input-group">
              <label>端口 (Port)</label>
              <input v-model="repoProxyPort" type="text" placeholder="7890" :disabled="!repoProxyEnabled">
            </div>
          </div>

          <div v-if="repoProxyType === 'socks5'" class="input-row" :class="{ disabled: !repoProxyEnabled }">
            <div class="input-group">
              <label>用户名（可选）</label>
              <input v-model="repoProxyUsername" type="text" :disabled="!repoProxyEnabled">
            </div>
            <div class="input-group">
              <label>密码（可选）</label>
              <input v-model="repoProxyPassword" type="password" :disabled="!repoProxyEnabled">
            </div>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-cancel" @click="emit('close')">取消</button>
        <button class="btn-save" @click="save">保存</button>
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
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--bg-primary);
  border-radius: var(--radius-lg);
  width: 450px;
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

.form-group h4 {
  margin-bottom: var(--spacing-xs);
  font-size: var(--font-size-base);
}

.hint {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-md);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
  cursor: pointer;
}

.input-group {
  margin-bottom: var(--spacing-md);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.input-group label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.input-row {
  display: flex;
  gap: var(--spacing-md);
}

.input-row .input-group {
  flex: 1;
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

.disabled {
  opacity: 0.5;
  pointer-events: none;
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

.divider {
  height: 1px;
  background-color: var(--border-color);
  margin: var(--spacing-lg) 0;
}
</style>
