<script setup lang="ts">
import { ref, watch } from 'vue';
import type { Repository } from '../types';
import { settingsStore } from '../stores/settingsStore';
import { debugStore } from '../stores/debugStore';
import { PlatformApi } from '../services/platformApi';
import { GitApi } from '../services/gitApi';
import { repoStore } from '../stores/repoStore';

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

// 远程仓库配置
const remotes = ref<Array<{name: string, url: string}>>([]);
const selectedRemote = ref('');

// 自动加载远程仓库配置
async function loadRemoteConfig() {
  if (!props.repo) return;

  try {
    const response = await GitApi.getRemotes(props.repo.path);
    if (response.success && response.data) {
      remotes.value = response.data;

      // 选择origin或第一个remote
      if (remotes.value.length > 0) {
        const origin = remotes.value.find(r => r.name === 'origin');
        selectedRemote.value = origin ? origin.name : remotes.value[0].name;

        // 自动识别协议
        const remote = origin || remotes.value[0];
        if (remote.url.startsWith('git@') || remote.url.startsWith('ssh://')) {
          repoProtocol.value = 'ssh';
        } else if (remote.url.startsWith('https://')) {
          repoProtocol.value = 'https';
        } else if (remote.url.startsWith('http://')) {
          repoProtocol.value = 'http';
        }
      }
    }
  } catch (error) {
    console.error('Failed to load remote config:', error);
  }
}

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

      // 加载远程仓库配置
      loadRemoteConfig();
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

// 远程仓库管理
const newRemoteName = ref('');
const newRemoteUrl = ref('');
const editingRemote = ref<{name: string, url: string} | null>(null);
const showAddRemoteForm = ref(false);

// @ts-ignore
function toggleAddRemoteForm() {
  showAddRemoteForm.value = !showAddRemoteForm.value;
  if (!showAddRemoteForm.value) {
    // 关闭时清空输入
    newRemoteName.value = '';
    newRemoteUrl.value = '';
  }
}

async function addRemote() {
  if (!props.repo || !newRemoteName.value || !newRemoteUrl.value) {
    alert('请填写远程仓库名称和URL');
    return;
  }

  try {
    const response = await GitApi.addRemote(props.repo.path, newRemoteName.value, newRemoteUrl.value);
    if (response.success) {
      alert('添加远程仓库成功!');
      newRemoteName.value = '';
      newRemoteUrl.value = '';
      showAddRemoteForm.value = false;
      await loadRemoteConfig();
    } else {
      alert('添加失败: ' + response.error);
    }
  } catch (error: any) {
    alert('添加失败: ' + error.message);
  }
}

async function removeRemote(remoteName: string) {
  if (!props.repo) return;

  if (!confirm(`确定要删除远程仓库 "${remoteName}" 吗？`)) {
    return;
  }

  try {
    const response = await GitApi.removeRemote(props.repo.path, remoteName);
    if (response.success) {
      alert('删除成功!');
      await loadRemoteConfig();
    } else {
      alert('删除失败: ' + response.error);
    }
  } catch (error: any) {
    alert('删除失败: ' + error.message);
  }
}

function startEditRemote(remote: {name: string, url: string}) {
  editingRemote.value = { ...remote };
}

function cancelEditRemote() {
  editingRemote.value = null;
}

async function saveEditRemote() {
  if (!props.repo || !editingRemote.value) return;

  const oldName = editingRemote.value.name;
  const newUrl = editingRemote.value.url;

  try {
    // 先删除旧的
    await GitApi.removeRemote(props.repo.path, oldName);
    // 再添加新的
    await GitApi.addRemote(props.repo.path, oldName, newUrl);

    alert('修改成功!');
    editingRemote.value = null;
    await loadRemoteConfig();
  } catch (error: any) {
    alert('修改失败: ' + error.message);
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
  } else if (props.repo) {
    // Save repo settings
    const updates: Partial<Repository> = {
      protocol: repoProtocol.value,
      authType: repoAuthType.value,
      token: repoToken.value || undefined,
      username: repoUsername.value || undefined,
      password: repoPassword.value || undefined,
    };

    // Save proxy settings
    if (repoProxyEnabled.value) {
      updates.proxy = {
        enabled: true,
        host: repoProxyHost.value,
        port: repoProxyPort.value,
        type: repoProxyType.value as 'http' | 'socks5',
        username: repoProxyUsername.value || undefined,
        password: repoProxyPassword.value || undefined,
      };
    } else {
      updates.proxy = undefined;
    }

    repoStore.updateRepository(props.repo.id, updates);
    alert('仓库设置已保存');
  }

  emit('close');
}
</script>

<template>
  <div v-if="isOpen" class="modal-overlay">
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

          <!-- 远程仓库管理 -->
          <div class="remote-section">
            <h5>远程仓库 (Remotes)</h5>

            <div v-if="remotes.length > 0" class="remote-list">
              <div v-for="remote in remotes" :key="remote.name" class="remote-item">
                <div v-if="editingRemote?.name === remote.name" class="remote-edit">
                  <div class="input-group">
                    <label>名称</label>
                    <input type="text" :value="remote.name" disabled />
                  </div>
                  <div class="input-group">
                    <label>URL</label>
                    <input type="text" v-model="editingRemote.url" />
                  </div>
                  <div class="remote-actions">
                    <button class="btn-save-small" @click="saveEditRemote">保存</button>
                    <button class="btn-cancel-small" @click="cancelEditRemote">取消</button>
                  </div>
                </div>
                <div v-else class="remote-display">
                  <div class="remote-info">
                    <span class="remote-name">{{ remote.name }}</span>
                    <span class="remote-url">{{ remote.url }}</span>
                  </div>
                  <div class="remote-actions">
                    <button class="icon-btn" @click="startEditRemote(remote)" title="编辑">
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
                        <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
                      </svg>
                    </button>
                    <button class="icon-btn danger" @click="removeRemote(remote.name)" title="删除">
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <polyline points="3 6 5 6 21 6"></polyline>
                        <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
                      </svg>
                    </button>
                  </div>
                </div>
              </div>
            </div>
            <div v-else class="no-remotes">
              <p>暂无远程仓库配置</p>
            </div>

            <!-- 添加远程仓库按钮 -->
            <button v-if="!showAddRemoteForm" class="btn-show-add" @click="toggleAddRemoteForm">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="12" y1="5" x2="12" y2="19"></line>
                <line x1="5" y1="12" x2="19" y2="12"></line>
              </svg>
              添加远程仓库
            </button>

            <!-- 添加远程仓库表单 -->
            <div v-if="showAddRemoteForm" class="add-remote">
              <div class="add-remote-header">
                <h6>添加远程仓库</h6>
                <button class="close-form-btn" @click="toggleAddRemoteForm" title="取消">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                  </svg>
                </button>
              </div>
              <div class="input-row">
                <div class="input-group">
                  <label>名称 (如: origin)</label>
                  <input type="text" v-model="newRemoteName" placeholder="origin" />
                </div>
                <div class="input-group" style="flex: 2;">
                  <label>URL</label>
                  <input type="text" v-model="newRemoteUrl" placeholder="https://github.com/user/repo.git" />
                </div>
              </div>
              <div class="form-actions">
                <button class="btn-cancel" @click="toggleAddRemoteForm">取消</button>
                <button class="btn-add" @click="addRemote">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="12" y1="5" x2="12" y2="19"></line>
                    <line x1="5" y1="12" x2="19" y2="12"></line>
                  </svg>
                  添加
                </button>
              </div>
            </div>
          </div>

          <div class="divider"></div>

          <h5>认证配置</h5>
          <p class="hint auth-hint">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="display: inline; vertical-align: text-top;">
              <circle cx="12" cy="12" r="10"></circle>
              <line x1="12" y1="16" x2="12" y2="12"></line>
              <line x1="12" y1="8" x2="12.01" y2="8"></line>
            </svg>
            <strong>重要说明:</strong> 远程仓库URL不包含认证信息。这些设置保存在本地,用于Git推送/拉取时的身份验证。
          </p>

          <div class="input-group">
            <label>传输协议 (已根据远程URL自动识别)</label>
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
  width: 90vw;
  max-width: 450px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
  margin: var(--spacing-md);
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

/* 远程仓库管理样式 */
.remote-section {
  margin-bottom: var(--spacing-lg);
}

.remote-section h5 {
  font-size: var(--font-size-base);
  font-weight: 600;
  margin-bottom: var(--spacing-md);
  color: var(--text-primary);
}

.remote-section h6 {
  font-size: var(--font-size-sm);
  font-weight: 600;
  margin-bottom: var(--spacing-sm);
  color: var(--text-secondary);
}

.remote-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
}

.remote-item {
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--bg-secondary);
}

.remote-display {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--spacing-md);
}

.remote-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.remote-name {
  font-weight: 600;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
}

.remote-url {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-family: monospace;
  word-break: break-all;
}

.remote-actions {
  display: flex;
  gap: var(--spacing-xs);
  flex-shrink: 0;
}

.icon-btn {
  padding: 6px;
  border-radius: var(--radius-sm);
  background-color: transparent;
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  justify-content: center;
}

.icon-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.icon-btn.danger:hover {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.remote-edit {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.btn-save-small,
.btn-cancel-small {
  padding: 4px 12px;
  border-radius: var(--radius-sm);
  font-size: var(--font-size-xs);
}

.btn-save-small {
  background-color: var(--accent-color);
  color: white;
}

.btn-save-small:hover {
  background-color: var(--accent-hover);
}

.btn-cancel-small {
  background-color: var(--bg-tertiary);
  color: var(--text-secondary);
}

.btn-cancel-small:hover {
  background-color: var(--bg-hover);
}

.no-remotes {
  text-align: center;
  padding: var(--spacing-lg);
  color: var(--text-tertiary);
  background-color: var(--bg-secondary);
  border-radius: var(--radius-md);
  border: 1px dashed var(--border-color);
  margin-bottom: var(--spacing-md);
}

.add-remote {
  padding: var(--spacing-md);
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
}

.btn-add {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background-color: var(--accent-color);
  color: white;
  font-size: var(--font-size-sm);
  font-weight: 500;
  width: 100%;
  justify-content: center;
  transition: all var(--transition-fast);
}

.btn-add:hover {
  background-color: var(--accent-hover);
}

.btn-show-add {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  font-size: var(--font-size-sm);
  font-weight: 500;
  width: 100%;
  justify-content: center;
  transition: all var(--transition-fast);
}

.btn-show-add:hover {
  background-color: var(--bg-hover);
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.add-remote-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-sm);
}

.close-form-btn {
  padding: 4px;
  border-radius: var(--radius-sm);
  background-color: transparent;
  color: var(--text-secondary);
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-form-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.form-actions {
  display: flex;
  gap: var(--spacing-sm);
  justify-content: flex-end;
}

.form-actions .btn-add {
  width: auto;
  flex: 1;
}

.form-actions .btn-cancel {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  border: 1px solid var(--border-color);
}

.form-actions .btn-cancel:hover {
  background-color: var(--bg-hover);
}

.auth-hint {
  background-color: rgba(59, 130, 246, 0.1);
  border-left: 3px solid var(--accent-color);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-sm);
  display: flex;
  gap: var(--spacing-sm);
  align-items: flex-start;
}

.auth-hint svg {
  flex-shrink: 0;
  color: var(--accent-color);
  margin-top: 2px;
}
</style>
