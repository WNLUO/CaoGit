<script setup lang="ts">
import { ref, watch } from 'vue';
import type { Repository } from '../types';
import { settingsStore } from '../stores/settingsStore';
import { debugStore } from '../stores/debugStore';
import { PlatformApi } from '../services/platformApi';

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

// Git platform accounts
const githubEnabled = ref(false);
const githubToken = ref('');
const githubUsername = ref('');
const gitlabEnabled = ref(false);
const gitlabToken = ref('');
const gitlabUsername = ref('');
const giteeEnabled = ref(false);
const giteeToken = ref('');
const giteeUsername = ref('');

const verifyingToken = ref<'github' | 'gitlab' | 'gitee' | null>(null);

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

      // Load platform accounts
      githubEnabled.value = globalSettings.gitPlatforms.github.enabled;
      githubToken.value = globalSettings.gitPlatforms.github.token;
      githubUsername.value = globalSettings.gitPlatforms.github.username || '';
      gitlabEnabled.value = globalSettings.gitPlatforms.gitlab.enabled;
      gitlabToken.value = globalSettings.gitPlatforms.gitlab.token;
      gitlabUsername.value = globalSettings.gitPlatforms.gitlab.username || '';
      giteeEnabled.value = globalSettings.gitPlatforms.gitee.enabled;
      giteeToken.value = globalSettings.gitPlatforms.gitee.token;
      giteeUsername.value = globalSettings.gitPlatforms.gitee.username || '';

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

async function verifyToken(platform: 'github' | 'gitlab' | 'gitee') {
  verifyingToken.value = platform;

  try {
    let result;
    let token;

    switch (platform) {
      case 'github':
        token = githubToken.value;
        result = await PlatformApi.verifyGitHubToken(token);
        if (result.valid && result.username) {
          githubUsername.value = result.username;
          alert(`验证成功！用户名: ${result.username}`);
        } else {
          alert('Token 验证失败: ' + (result.error || '未知错误'));
        }
        break;
      case 'gitlab':
        token = gitlabToken.value;
        result = await PlatformApi.verifyGitLabToken(token);
        if (result.valid && result.username) {
          gitlabUsername.value = result.username;
          alert(`验证成功！用户名: ${result.username}`);
        } else {
          alert('Token 验证失败: ' + (result.error || '未知错误'));
        }
        break;
      case 'gitee':
        token = giteeToken.value;
        result = await PlatformApi.verifyGiteeToken(token);
        if (result.valid && result.username) {
          giteeUsername.value = result.username;
          alert(`验证成功！用户名: ${result.username}`);
        } else {
          alert('Token 验证失败: ' + (result.error || '未知错误'));
        }
        break;
    }
  } catch (error: any) {
    alert('验证失败: ' + error.message);
  } finally {
    verifyingToken.value = null;
  }
}

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

    // Save platform accounts
    settingsStore.updateGitPlatforms({
      github: {
        enabled: githubEnabled.value,
        token: githubToken.value,
        username: githubUsername.value
      },
      gitlab: {
        enabled: gitlabEnabled.value,
        token: gitlabToken.value,
        username: gitlabUsername.value
      },
      gitee: {
        enabled: giteeEnabled.value,
        token: giteeToken.value,
        username: giteeUsername.value
      }
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

          <h4>Git 平台账户</h4>
          <p class="hint">配置后可以直接在应用内创建远程仓库</p>

          <!-- GitHub -->
          <div class="platform-section">
            <label class="checkbox-label">
              <input type="checkbox" v-model="githubEnabled">
              <span class="platform-name">🐙 GitHub</span>
            </label>

            <div v-if="githubEnabled" class="platform-config">
              <div class="input-group">
                <label>Personal Access Token</label>
                <div class="token-input-wrapper">
                  <input
                    v-model="githubToken"
                    type="password"
                    placeholder="ghp_..."
                  />
                  <button
                    class="verify-btn"
                    @click="verifyToken('github')"
                    :disabled="!githubToken || verifyingToken === 'github'"
                  >
                    {{ verifyingToken === 'github' ? '验证中...' : '验证' }}
                  </button>
                </div>
                <a
                  href="https://github.com/settings/tokens/new?scopes=repo&description=Git%20Manager"
                  target="_blank"
                  class="help-link"
                >
                  如何获取 Token？
                </a>
              </div>

              <div v-if="githubUsername" class="username-display">
                <span class="label">用户名:</span>
                <span class="username">{{ githubUsername }}</span>
              </div>
            </div>
          </div>

          <!-- GitLab -->
          <div class="platform-section">
            <label class="checkbox-label">
              <input type="checkbox" v-model="gitlabEnabled">
              <span class="platform-name">🦊 GitLab</span>
            </label>

            <div v-if="gitlabEnabled" class="platform-config">
              <div class="input-group">
                <label>Personal Access Token</label>
                <div class="token-input-wrapper">
                  <input
                    v-model="gitlabToken"
                    type="password"
                    placeholder="glpat-..."
                  />
                  <button
                    class="verify-btn"
                    @click="verifyToken('gitlab')"
                    :disabled="!gitlabToken || verifyingToken === 'gitlab'"
                  >
                    {{ verifyingToken === 'gitlab' ? '验证中...' : '验证' }}
                  </button>
                </div>
                <a
                  href="https://gitlab.com/-/profile/personal_access_tokens"
                  target="_blank"
                  class="help-link"
                >
                  如何获取 Token？
                </a>
              </div>

              <div v-if="gitlabUsername" class="username-display">
                <span class="label">用户名:</span>
                <span class="username">{{ gitlabUsername }}</span>
              </div>
            </div>
          </div>

          <!-- Gitee -->
          <div class="platform-section">
            <label class="checkbox-label">
              <input type="checkbox" v-model="giteeEnabled">
              <span class="platform-name">🔴 Gitee</span>
            </label>

            <div v-if="giteeEnabled" class="platform-config">
              <div class="input-group">
                <label>私人令牌 (Private Token)</label>
                <div class="token-input-wrapper">
                  <input
                    v-model="giteeToken"
                    type="password"
                    placeholder="..."
                  />
                  <button
                    class="verify-btn"
                    @click="verifyToken('gitee')"
                    :disabled="!giteeToken || verifyingToken === 'gitee'"
                  >
                    {{ verifyingToken === 'gitee' ? '验证中...' : '验证' }}
                  </button>
                </div>
                <a
                  href="https://gitee.com/profile/personal_access_tokens"
                  target="_blank"
                  class="help-link"
                >
                  如何获取令牌？
                </a>
              </div>

              <div v-if="giteeUsername" class="username-display">
                <span class="label">用户名:</span>
                <span class="username">{{ giteeUsername }}</span>
              </div>
            </div>
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

.platform-section {
  margin-bottom: var(--spacing-md);
  padding: var(--spacing-md);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
}

.platform-name {
  font-weight: 600;
  font-size: var(--font-size-base);
}

.platform-config {
  margin-top: var(--spacing-md);
  padding-top: var(--spacing-md);
  border-top: 1px solid var(--border-color);
}

.token-input-wrapper {
  display: flex;
  gap: var(--spacing-sm);
}

.token-input-wrapper input {
  flex: 1;
}

.verify-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background-color: var(--accent-color);
  color: white;
  font-size: var(--font-size-sm);
  white-space: nowrap;
}

.verify-btn:hover:not(:disabled) {
  background-color: var(--accent-hover);
}

.verify-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.help-link {
  font-size: var(--font-size-xs);
  color: var(--accent-color);
  text-decoration: none;
  margin-top: 4px;
  display: inline-block;
}

.help-link:hover {
  text-decoration: underline;
}

.username-display {
  margin-top: var(--spacing-sm);
  padding: var(--spacing-sm);
  background-color: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-sm);
  display: flex;
  gap: var(--spacing-xs);
}

.username-display .label {
  color: var(--text-tertiary);
}

.username-display .username {
  color: var(--text-primary);
  font-weight: 600;
}
</style>
