<script setup lang="ts">
import { ref, computed } from 'vue';
import { settingsStore } from '../../stores/settingsStore';
import { debugStore } from '../../stores/debugStore';
import { toastStore } from '../../stores/toastStore';
import { PlatformApi } from '../../services/platformApi';
import { repoStore } from '../../stores/repoStore';

type SettingsTab =
  | 'appearance'
  | 'git-behavior'
  | 'sync'
  | 'editor'
  | 'network'
  | 'platforms'
  | 'notification'
  | 'performance'
  | 'security'
  | 'advanced';

const activeTab = ref<SettingsTab>('appearance');

// Local state for all settings
const localSettings = ref({ ...settingsStore.settings });

// Verification state
const verifyingToken = ref<'github' | 'gitlab' | 'gitee' | null>(null);

// Watch for changes in Appearance and Editor settings for real-time updates
import { watch } from 'vue';

watch(() => localSettings.value.appearance, (newVal) => {
  settingsStore.updateAppearance(newVal);
}, { deep: true });

watch(() => localSettings.value.editor, (newVal) => {
  settingsStore.updateEditor(newVal);
}, { deep: true });

// 侧边栏菜单配置
const menuItems = [
  { id: 'appearance', label: '外观' },
  { id: 'git-behavior', label: 'Git 行为' },
  { id: 'sync', label: '同步设置' },
  { id: 'editor', label: '编辑器' },
  { id: 'network', label: '网络代理' },
  { id: 'platforms', label: 'Git 平台' },
  { id: 'notification', label: '通知' },
  { id: 'performance', label: '性能' },
  { id: 'security', label: '安全' },
  { id: 'advanced', label: '高级' }
] as const;

// 判断设置是否有变更
const hasChanges = computed(() => {
  return JSON.stringify(localSettings.value) !== JSON.stringify(settingsStore.settings);
});

async function verifyToken(platform: 'github' | 'gitlab' | 'gitee') {
  verifyingToken.value = platform;

  try {
    let result;
    let token;

    switch (platform) {
      case 'github':
        token = localSettings.value.gitPlatforms.github.token;
        result = await PlatformApi.verifyGitHubToken(token);
        if (result.valid && result.username) {
          localSettings.value.gitPlatforms.github.username = result.username;
          toastStore.success(`验证成功！用户名: ${result.username}`);
        } else {
          toastStore.error('Token 验证失败: ' + (result.error || '未知错误'));
        }
        break;
      case 'gitlab':
        token = localSettings.value.gitPlatforms.gitlab.token;
        result = await PlatformApi.verifyGitLabToken(token);
        if (result.valid && result.username) {
          localSettings.value.gitPlatforms.gitlab.username = result.username;
          toastStore.success(`验证成功！用户名: ${result.username}`);
        } else {
          toastStore.error('Token 验证失败: ' + (result.error || '未知错误'));
        }
        break;
      case 'gitee':
        token = localSettings.value.gitPlatforms.gitee.token;
        result = await PlatformApi.verifyGiteeToken(token);
        if (result.valid && result.username) {
          localSettings.value.gitPlatforms.gitee.username = result.username;
          toastStore.success(`验证成功！用户名: ${result.username}`);
        } else {
          toastStore.error('Token 验证失败: ' + (result.error || '未知错误'));
        }
        break;
    }
  } catch (error: any) {
    toastStore.error('验证失败: ' + error.message);
  } finally {
    verifyingToken.value = null;
  }
}

function saveSettings() {
  settingsStore.saveSettings(localSettings.value);

  // 应用同步设置
  if (localSettings.value.sync.refreshOnFocus !== settingsStore.settings.sync.refreshOnFocus) {
    if (localSettings.value.sync.refreshOnFocus) {
      repoStore.startAutoSync();
    } else {
      repoStore.stopAutoSync();
    }
  }

  // 应用调试模式
  debugStore.setDebugMode(localSettings.value.advanced.enableDebugLogging);

  toastStore.success('设置已保存');
}

function resetSettings() {
  if (confirm('确定要重置所有设置为默认值吗？此操作不可撤销。')) {
    settingsStore.resetToDefaults();
    localSettings.value = { ...settingsStore.settings };
    toastStore.success('已重置为默认设置');
  }
}

function discardChanges() {
  localSettings.value = { ...settingsStore.settings };
  toastStore.info('已放弃更改');
}

function exportSettings() {
  const dataStr = settingsStore.exportSettings();
  const dataBlob = new Blob([dataStr], { type: 'application/json' });
  const url = URL.createObjectURL(dataBlob);
  const link = document.createElement('a');
  link.href = url;
  link.download = 'caogit-settings.json';
  link.click();
  URL.revokeObjectURL(url);
  toastStore.success('设置已导出');
}

function importSettings() {
  const input = document.createElement('input');
  input.type = 'file';
  input.accept = 'application/json';
  input.onchange = (e: any) => {
    const file = e.target.files[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (event: any) => {
        try {
          const success = settingsStore.importSettings(event.target.result);
          if (success) {
            localSettings.value = { ...settingsStore.settings };
            toastStore.success('设置已导入');
          } else {
            toastStore.error('导入失败：文件格式不正确');
          }
        } catch (error) {
          toastStore.error('导入失败：' + error);
        }
      };
      reader.readAsText(file);
    }
  };
  input.click();
}

function clearCache() {
  if (confirm('确定要清除所有缓存吗？这将刷新所有仓库数据。')) {
    settingsStore.clearCache();
    repoStore.clearCache();
    toastStore.success('缓存已清除');
  }
}

function selectSshKey() {
  // TODO: 实现文件选择器
  toastStore.info('文件选择功能开发中');
}
</script>

<template>
  <div class="settings-view">
    <!-- 侧边栏导航 -->
    <aside class="settings-sidebar">
      <div class="sidebar-header">
        <h2>设置</h2>
      </div>
      <nav class="settings-nav">
        <button
          v-for="item in menuItems"
          :key="item.id"
          :class="['nav-item', { active: activeTab === item.id }]"
          @click="activeTab = item.id as SettingsTab"
        >
          <span class="nav-label">{{ item.label }}</span>
        </button>
      </nav>
    </aside>

    <!-- 主内容区 -->
    <main class="settings-content">
      <div class="content-header">
        <div class="header-actions">
          <button v-if="hasChanges" class="btn-discard" @click="discardChanges">
            放弃更改
          </button>
          <button v-if="hasChanges" class="btn-save" @click="saveSettings">
            保存设置
          </button>
        </div>
      </div>

      <div class="content-body">
        <!-- 外观设置 -->
        <section v-if="activeTab === 'appearance'" class="settings-section">
          <h3>外观设置</h3>
          <p class="section-desc">自定义应用的外观和显示效果</p>

          <div class="form-group">
            <label>主题</label>
            <select v-model="localSettings.appearance.theme">
              <option value="light">亮色</option>
              <option value="dark">暗色</option>
              <option value="system">跟随系统</option>
            </select>
            <span class="hint">更改主题将影响整个应用的配色方案</span>
          </div>

          <div class="form-group">
            <label>代码字体大小</label>
            <input
              v-model.number="localSettings.appearance.fontSize"
              type="number"
              min="10"
              max="24"
            />
            <span class="hint">{{ localSettings.appearance.fontSize }}px</span>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.appearance.compactMode" />
              紧凑模式
            </label>
            <span class="hint">减少界面元素的间距，显示更多内容</span>
          </div>

          <div class="form-group">
            <label>语言</label>
            <select v-model="localSettings.appearance.language">
              <option value="zh-CN">简体中文</option>
              <option value="en-US">English</option>
            </select>
          </div>
        </section>

        <!-- Git 行为设置 -->
        <section v-if="activeTab === 'git-behavior'" class="settings-section">
          <h3>Git 行为设置</h3>
          <p class="section-desc">配置 Git 操作的默认行为</p>

          <div class="form-group">
            <label>默认提交行为</label>
            <select v-model="localSettings.gitBehavior.defaultCommitAction">
              <option value="commit">仅提交</option>
              <option value="commit-and-push">提交并推送</option>
            </select>
            <span class="hint">点击提交按钮时的默认操作</span>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.gitBehavior.autoStageModified" />
              自动暂存已修改文件
            </label>
            <span class="hint">提交时自动暂存所有已修改的文件</span>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.gitBehavior.checkBeforeCommit" />
              提交前检查
            </label>
            <span class="hint">执行代码格式化和 Lint 检查（如果配置）</span>
          </div>

          <div class="form-group">
            <label>默认分支名称</label>
            <input
              v-model="localSettings.gitBehavior.defaultBranchName"
              type="text"
              placeholder="main"
            />
            <span class="hint">创建新仓库时的默认分支名</span>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.gitBehavior.autoFetch" />
              自动 Fetch
            </label>
            <span class="hint">定期从远程获取更新（不合并）</span>
          </div>

          <div v-if="localSettings.gitBehavior.autoFetch" class="form-group">
            <label>Fetch 间隔（分钟）</label>
            <input
              v-model.number="localSettings.gitBehavior.autoFetchInterval"
              type="number"
              min="1"
              max="60"
            />
          </div>
        </section>

        <!-- 同步设置 -->
        <section v-if="activeTab === 'sync'" class="settings-section">
          <h3>同步设置</h3>
          <p class="section-desc">控制数据刷新和同步行为</p>

          <div class="form-group">
            <label>自动刷新间隔（秒）</label>
            <input
              v-model.number="localSettings.sync.autoRefreshInterval"
              type="number"
              min="5"
              max="300"
            />
            <span class="hint">每隔 {{ localSettings.sync.autoRefreshInterval }} 秒自动刷新仓库状态</span>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.sync.refreshOnFocus" />
              窗口聚焦时刷新
            </label>
            <span class="hint">切换回应用时自动刷新数据</span>
          </div>

          <div class="form-group">
            <label>Push 超时时间（秒）</label>
            <input
              v-model.number="localSettings.sync.pushTimeout"
              type="number"
              min="10"
              max="300"
            />
          </div>

          <div class="form-group">
            <label>Pull 超时时间（秒）</label>
            <input
              v-model.number="localSettings.sync.pullTimeout"
              type="number"
              min="10"
              max="300"
            />
          </div>
        </section>

        <!-- 编辑器设置 -->
        <section v-if="activeTab === 'editor'" class="settings-section">
          <h3>编辑器设置</h3>
          <p class="section-desc">自定义代码查看和编辑体验</p>

          <div class="form-group">
            <label>Diff 视图样式</label>
            <select v-model="localSettings.editor.diffViewStyle">
              <option value="side-by-side">并排对比</option>
              <option value="unified">统一视图</option>
            </select>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.editor.showLineNumbers" />
              显示行号
            </label>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.editor.showWhitespace" />
              显示空白字符
            </label>
            <span class="hint">显示空格和制表符</span>
          </div>

          <div class="form-group">
            <label>Tab 大小</label>
            <input
              v-model.number="localSettings.editor.tabSize"
              type="number"
              min="2"
              max="8"
            />
            <span class="hint">{{ localSettings.editor.tabSize }} 个空格</span>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.editor.wordWrap" />
              自动换行
            </label>
          </div>
        </section>

        <!-- 网络代理设置 -->
        <section v-if="activeTab === 'network'" class="settings-section">
          <h3>网络代理</h3>
          <p class="section-desc">配置 Git 操作的网络代理</p>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.proxy.enabled" />
              启用代理
            </label>
          </div>

          <div class="form-group" :class="{ disabled: !localSettings.proxy.enabled }">
            <label>代理类型</label>
            <select v-model="localSettings.proxy.type" :disabled="!localSettings.proxy.enabled">
              <option value="http">HTTP</option>
              <option value="https">HTTPS</option>
              <option value="socks5">SOCKS5</option>
            </select>
          </div>

          <div class="form-row" :class="{ disabled: !localSettings.proxy.enabled }">
            <div class="form-group">
              <label>主机</label>
              <input
                v-model="localSettings.proxy.host"
                type="text"
                placeholder="127.0.0.1"
                :disabled="!localSettings.proxy.enabled"
              />
            </div>
            <div class="form-group">
              <label>端口</label>
              <input
                v-model="localSettings.proxy.port"
                type="text"
                placeholder="7890"
                :disabled="!localSettings.proxy.enabled"
              />
            </div>
          </div>

          <div v-if="localSettings.proxy.type === 'socks5'" class="form-row" :class="{ disabled: !localSettings.proxy.enabled }">
            <div class="form-group">
              <label>用户名（可选）</label>
              <input
                v-model="localSettings.proxy.username"
                type="text"
                :disabled="!localSettings.proxy.enabled"
              />
            </div>
            <div class="form-group">
              <label>密码（可选）</label>
              <input
                v-model="localSettings.proxy.password"
                type="password"
                :disabled="!localSettings.proxy.enabled"
              />
            </div>
          </div>

          <div class="divider"></div>

          <h4>网络测速</h4>
          <div class="form-group">
            <label>测速地址</label>
            <input v-model="localSettings.networkTest.testUrl" type="text" placeholder="https://github.com" />
          </div>

          <div class="form-group">
            <label>测速间隔（秒）</label>
            <input v-model.number="localSettings.networkTest.testInterval" type="number" min="10" max="600" />
          </div>
        </section>

        <!-- Git 平台账户 -->
        <section v-if="activeTab === 'platforms'" class="settings-section">
          <h3>Git 平台账户</h3>
          <p class="section-desc">配置 GitHub、GitLab、Gitee 账户</p>

          <!-- GitHub -->
          <div class="platform-card">
            <div class="platform-header">
              <label class="checkbox-label">
                <input type="checkbox" v-model="localSettings.gitPlatforms.github.enabled" />
                <span class="platform-name">GitHub</span>
              </label>
            </div>
            <div v-if="localSettings.gitPlatforms.github.enabled" class="platform-body">
              <div class="form-group">
                <label>Personal Access Token</label>
                <div class="token-input-group">
                  <input
                    v-model="localSettings.gitPlatforms.github.token"
                    type="password"
                    placeholder="ghp_..."
                  />
                  <button
                    class="btn-verify"
                    @click="verifyToken('github')"
                    :disabled="!localSettings.gitPlatforms.github.token || verifyingToken === 'github'"
                  >
                    {{ verifyingToken === 'github' ? '验证中...' : '验证' }}
                  </button>
                </div>
                <a
                  href="https://github.com/settings/tokens/new?scopes=repo&description=CaoGit"
                  target="_blank"
                  class="help-link"
                >
                  如何获取 Token？
                </a>
              </div>
              <div v-if="localSettings.gitPlatforms.github.username" class="username-badge">
                <span class="label">用户名:</span>
                <span class="username">{{ localSettings.gitPlatforms.github.username }}</span>
              </div>
            </div>
          </div>

          <!-- GitLab -->
          <div class="platform-card">
            <div class="platform-header">
              <label class="checkbox-label">
                <input type="checkbox" v-model="localSettings.gitPlatforms.gitlab.enabled" />
                <span class="platform-name">GitLab</span>
              </label>
            </div>
            <div v-if="localSettings.gitPlatforms.gitlab.enabled" class="platform-body">
              <div class="form-group">
                <label>Personal Access Token</label>
                <div class="token-input-group">
                  <input
                    v-model="localSettings.gitPlatforms.gitlab.token"
                    type="password"
                    placeholder="glpat-..."
                  />
                  <button
                    class="btn-verify"
                    @click="verifyToken('gitlab')"
                    :disabled="!localSettings.gitPlatforms.gitlab.token || verifyingToken === 'gitlab'"
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
              <div v-if="localSettings.gitPlatforms.gitlab.username" class="username-badge">
                <span class="label">用户名:</span>
                <span class="username">{{ localSettings.gitPlatforms.gitlab.username }}</span>
              </div>
            </div>
          </div>

          <!-- Gitee -->
          <div class="platform-card">
            <div class="platform-header">
              <label class="checkbox-label">
                <input type="checkbox" v-model="localSettings.gitPlatforms.gitee.enabled" />
                <span class="platform-name">Gitee</span>
              </label>
            </div>
            <div v-if="localSettings.gitPlatforms.gitee.enabled" class="platform-body">
              <div class="form-group">
                <label>私人令牌</label>
                <div class="token-input-group">
                  <input
                    v-model="localSettings.gitPlatforms.gitee.token"
                    type="password"
                    placeholder="..."
                  />
                  <button
                    class="btn-verify"
                    @click="verifyToken('gitee')"
                    :disabled="!localSettings.gitPlatforms.gitee.token || verifyingToken === 'gitee'"
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
              <div v-if="localSettings.gitPlatforms.gitee.username" class="username-badge">
                <span class="label">用户名:</span>
                <span class="username">{{ localSettings.gitPlatforms.gitee.username }}</span>
              </div>
            </div>
          </div>

          <div class="divider"></div>

          <h4>发布管理</h4>
          <p class="section-desc">配置 GitHub Token 用于 Release Manager</p>
          <div class="form-group">
            <label>GitHub Personal Access Token</label>
            <input
              v-model="localSettings.githubToken"
              type="password"
              placeholder="ghp_..."
            />
            <span class="hint">
              需要权限: <code>repo</code>, <code>workflow</code>
            </span>
          </div>
        </section>

        <!-- 通知设置 -->
        <section v-if="activeTab === 'notification'" class="settings-section">
          <h3>通知设置</h3>
          <p class="section-desc">配置应用通知行为</p>

          <div class="form-group">
            <label>Toast 显示时长（毫秒）</label>
            <input
              v-model.number="localSettings.notification.toastDuration"
              type="number"
              min="1000"
              max="10000"
              step="500"
            />
            <span class="hint">{{ localSettings.notification.toastDuration / 1000 }} 秒</span>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.notification.errorSound" />
              错误提示音
            </label>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.notification.successSound" />
              成功提示音
            </label>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.notification.desktopNotifications" />
              桌面通知
            </label>
            <span class="hint">操作完成时发送系统通知</span>
          </div>
        </section>

        <!-- 性能设置 -->
        <section v-if="activeTab === 'performance'" class="settings-section">
          <h3>性能设置</h3>
          <p class="section-desc">优化应用性能和资源使用</p>

          <div class="form-group">
            <label>提交缓存时长（秒）</label>
            <input
              v-model.number="localSettings.performance.commitCacheTTL"
              type="number"
              min="60"
              max="3600"
            />
            <span class="hint">{{ Math.floor(localSettings.performance.commitCacheTTL / 60) }} 分钟</span>
          </div>

          <div class="form-group">
            <label>最大缓存大小（MB）</label>
            <input
              v-model.number="localSettings.performance.maxCacheSize"
              type="number"
              min="50"
              max="500"
            />
          </div>

          <div class="form-group">
            <label>日志保留天数</label>
            <input
              v-model.number="localSettings.performance.logRetentionDays"
              type="number"
              min="1"
              max="30"
            />
          </div>

          <div class="form-group">
            <button class="btn-action" @click="clearCache">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="1 4 1 10 7 10"></polyline>
                <path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"></path>
              </svg>
              清除缓存
            </button>
            <span class="hint">清除所有缓存数据，释放存储空间</span>
          </div>
        </section>

        <!-- 安全设置 -->
        <section v-if="activeTab === 'security'" class="settings-section">
          <h3>安全设置</h3>
          <p class="section-desc">保护您的凭据和敏感信息</p>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.security.rememberCredentials" />
              记住凭据
            </label>
            <span class="hint">保存 Token 和密码以便下次使用</span>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.security.encryptPasswords" />
              加密存储密码
            </label>
            <span class="hint">使用加密算法保护存储的密码</span>
          </div>

          <div class="form-group">
            <label>SSH 私钥路径</label>
            <div class="file-input-group">
              <input
                v-model="localSettings.security.sshKeyPath"
                type="text"
                placeholder="~/.ssh/id_rsa"
                readonly
              />
              <button class="btn-browse" @click="selectSshKey">浏览</button>
            </div>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.security.enableGPGSign" />
              启用 GPG 签名
            </label>
          </div>

          <div v-if="localSettings.security.enableGPGSign" class="form-group">
            <label>GPG 签名密钥 ID</label>
            <input
              v-model="localSettings.security.gpgSigningKey"
              type="text"
              placeholder="ABCD1234"
            />
          </div>
        </section>

        <!-- 高级设置 -->
        <section v-if="activeTab === 'advanced'" class="settings-section">
          <h3>高级设置</h3>
          <p class="section-desc">高级功能和实验性特性</p>

          <div class="form-group">
            <label>自定义 Git 路径</label>
            <input
              v-model="localSettings.advanced.customGitPath"
              type="text"
              placeholder="/usr/bin/git"
            />
            <span class="hint">留空使用系统默认 Git</span>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.advanced.experimentalFeatures" />
              启用实验性功能
            </label>
            <span class="hint">⚠️ 可能不稳定，请谨慎使用</span>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="localSettings.advanced.enableDebugLogging" />
              启用调试日志
            </label>
            <span class="hint">记录详细的调试信息，用于问题排查</span>
          </div>

          <div class="divider"></div>

          <h4>数据管理</h4>
          <div class="form-group">
            <div class="button-group">
              <button class="btn-action" @click="exportSettings">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                  <polyline points="7 10 12 15 17 10"></polyline>
                  <line x1="12" y1="15" x2="12" y2="3"></line>
                </svg>
                导出设置
              </button>
              <button class="btn-action" @click="importSettings">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                  <polyline points="17 8 12 3 7 8"></polyline>
                  <line x1="12" y1="3" x2="12" y2="15"></line>
                </svg>
                导入设置
              </button>
            </div>
          </div>

          <div class="form-group">
            <button class="btn-danger" @click="resetSettings">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="1 4 1 10 7 10"></polyline>
                <polyline points="23 20 23 14 17 14"></polyline>
                <path d="M20.49 9A9 9 0 0 0 5.64 5.64L1 10m22 4l-4.64 4.36A9 9 0 0 1 3.51 15"></path>
              </svg>
              重置所有设置
            </button>
            <span class="hint">⚠️ 此操作将恢复所有设置为默认值</span>
          </div>
        </section>
      </div>
    </main>
  </div>
</template>

<style scoped>
.settings-view {
  display: flex;
  height: 100%;
  background-color: var(--bg-primary);
}

/* 侧边栏 */
.settings-sidebar {
  width: 220px;
  background-color: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

.sidebar-header {
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);
}

.sidebar-header h2 {
  font-size: var(--font-size-xl);
  font-weight: 600;
  margin: 0;
}

.settings-nav {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-sm);
}

.nav-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--text-secondary);
  text-align: left;
  transition: all var(--transition-fast);
  margin-bottom: 2px;
}

.nav-item:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.nav-item.active {
  background-color: var(--accent-color);
  color: white;
}

.nav-icon {
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.nav-icon::before {
  content: '⚙️';
}

.nav-icon[data-icon="palette"]::before { content: '🎨'; }
.nav-icon[data-icon="git"]::before { content: '🔀'; }
.nav-icon[data-icon="refresh"]::before { content: '🔄'; }
.nav-icon[data-icon="code"]::before { content: '💻'; }
.nav-icon[data-icon="globe"]::before { content: '🌐'; }
.nav-icon[data-icon="cloud"]::before { content: '☁️'; }
.nav-icon[data-icon="bell"]::before { content: '🔔'; }
.nav-icon[data-icon="zap"]::before { content: '⚡'; }
.nav-icon[data-icon="shield"]::before { content: '🛡️'; }
.nav-icon[data-icon="settings"]::before { content: '⚙️'; }

.nav-label {
  font-size: var(--font-size-sm);
  font-weight: 500;
}

/* 主内容区 */
.settings-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.content-header {
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);
  background-color: var(--bg-primary);
  display: flex;
  justify-content: flex-end;
  align-items: center;
  min-height: 60px;
}

.header-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.btn-discard {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  font-size: var(--font-size-sm);
  font-weight: 500;
  transition: all var(--transition-fast);
}

.btn-discard:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.btn-save {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  background-color: var(--accent-color);
  color: white;
  font-size: var(--font-size-sm);
  font-weight: 500;
  transition: all var(--transition-fast);
}

.btn-save:hover {
  background-color: var(--accent-hover);
}

.content-body {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-xl);
}

/* 设置区块 */
.settings-section {
  max-width: 700px;
}

.settings-section h3 {
  font-size: var(--font-size-xl);
  font-weight: 600;
  margin-bottom: var(--spacing-xs);
}

.settings-section h4 {
  font-size: var(--font-size-lg);
  font-weight: 600;
  margin-top: var(--spacing-lg);
  margin-bottom: var(--spacing-md);
}

.section-desc {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xl);
}

.form-group {
  margin-bottom: var(--spacing-lg);
}

.form-group label {
  display: block;
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.form-group input[type="text"],
.form-group input[type="password"],
.form-group input[type="number"],
.form-group select {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  transition: all var(--transition-fast);
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  cursor: pointer;
  font-weight: 500;
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.hint {
  display: block;
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  margin-top: var(--spacing-xs);
}

.hint code {
  background-color: var(--bg-tertiary);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
  font-family: monospace;
  font-size: var(--font-size-xs);
}

.form-row {
  display: flex;
  gap: var(--spacing-md);
}

.form-row .form-group {
  flex: 1;
}

.disabled {
  opacity: 0.5;
  pointer-events: none;
}

.divider {
  height: 1px;
  background-color: var(--border-color);
  margin: var(--spacing-xl) 0;
}

/* 平台卡片 */
.platform-card {
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--spacing-md);
  margin-bottom: var(--spacing-md);
  background-color: var(--bg-secondary);
}

.platform-header {
  margin-bottom: var(--spacing-md);
}

.platform-name {
  font-size: var(--font-size-base);
  font-weight: 600;
}

.platform-body {
  padding-top: var(--spacing-md);
  border-top: 1px solid var(--border-color);
}

.token-input-group {
  display: flex;
  gap: var(--spacing-sm);
}

.token-input-group input {
  flex: 1;
}

.btn-verify {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background-color: var(--accent-color);
  color: white;
  font-size: var(--font-size-sm);
  font-weight: 500;
  white-space: nowrap;
  transition: all var(--transition-fast);
}

.btn-verify:hover:not(:disabled) {
  background-color: var(--accent-hover);
}

.btn-verify:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.help-link {
  display: inline-block;
  margin-top: var(--spacing-xs);
  font-size: var(--font-size-xs);
  color: var(--accent-color);
  text-decoration: none;
}

.help-link:hover {
  text-decoration: underline;
}

.username-badge {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background-color: var(--bg-tertiary);
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  margin-top: var(--spacing-sm);
}

.username-badge .label {
  color: var(--text-secondary);
}

.username-badge .username {
  color: var(--text-primary);
  font-weight: 600;
  font-family: monospace;
}

/* 按钮 */
.btn-action {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  font-size: var(--font-size-sm);
  font-weight: 500;
  transition: all var(--transition-fast);
}

.btn-action:hover {
  background-color: var(--bg-hover);
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.btn-danger {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background-color: transparent;
  color: #ef4444;
  border: 1px solid #ef4444;
  font-size: var(--font-size-sm);
  font-weight: 500;
  transition: all var(--transition-fast);
}

.btn-danger:hover {
  background-color: rgba(239, 68, 68, 0.1);
}

.button-group {
  display: flex;
  gap: var(--spacing-sm);
}

.file-input-group {
  display: flex;
  gap: var(--spacing-sm);
}

.file-input-group input {
  flex: 1;
}

.btn-browse {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  font-size: var(--font-size-sm);
  white-space: nowrap;
  transition: all var(--transition-fast);
}

.btn-browse:hover {
  background-color: var(--bg-hover);
}
</style>
