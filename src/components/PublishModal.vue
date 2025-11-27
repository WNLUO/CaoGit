<script setup lang="ts">
import { ref, watch, computed, onMounted } from 'vue';
import { GitApi } from '../services/gitApi';
import { PlatformApi, type CreateRepoOptions } from '../services/platformApi';
import { settingsStore } from '../stores/settingsStore';

interface Props {
  isOpen: boolean;
  repoPath: string;
}

interface RemoteInfo {
  name: string;
  url: string;
  fetch_url?: string;
  push_url?: string;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'remote-added'): void;
  (e: 'open-settings'): void;
}>();

const mode = ref<'select' | 'add-existing' | 'publish' | 'create-repo'>('select');
const remoteUrl = ref('');
const remoteName = ref('origin');
const existingRemotes = ref<RemoteInfo[]>([]);
const isLoading = ref(false);
const selectedPlatform = ref<'github' | 'gitlab' | 'gitee' | ''>('');

// Create repo form
const repoName = ref('');
const repoDescription = ref('');
const repoPrivate = ref(false);
const repoAutoInit = ref(false);
const autoPush = ref(true);
const useSSH = ref(false);  // 默认使用 HTTPS（Token 认证）

function validateRepoName(name: string): { valid: boolean; error?: string } {
  if (!name || !name.trim()) {
    return { valid: false, error: '仓库名称不能为空' };
  }

  // GitHub/GitLab/Gitee 仓库名称规则：
  // - 只能包含字母、数字、连字符(-)、下划线(_)和点(.)
  // - 不能以点或连字符开头或结尾
  // - 长度限制：1-100 字符
  const nameRegex = /^[a-zA-Z0-9]([a-zA-Z0-9._-]{0,98}[a-zA-Z0-9])?$/;

  if (!nameRegex.test(name)) {
    if (/[\u4e00-\u9fa5]/.test(name)) {
      return {
        valid: false,
        error: '仓库名称不能包含中文字符，只能使用英文字母、数字、连字符(-)、下划线(_)和点(.)'
      };
    }
    if (name.startsWith('.') || name.startsWith('-')) {
      return {
        valid: false,
        error: '仓库名称不能以点(.)或连字符(-)开头'
      };
    }
    if (name.endsWith('.') || name.endsWith('-')) {
      return {
        valid: false,
        error: '仓库名称不能以点(.)或连字符(-)结尾'
      };
    }
    return {
      valid: false,
      error: '仓库名称只能包含字母、数字、连字符(-)、下划线(_)和点(.)'
    };
  }

  if (name.length > 100) {
    return { valid: false, error: '仓库名称长度不能超过 100 个字符' };
  }

  return { valid: true };
}

const repoNameError = computed(() => {
  if (!repoName.value) return '';
  const validation = validateRepoName(repoName.value);
  return validation.valid ? '' : validation.error;
});

const platforms = [
  {
    id: 'github',
    name: 'GitHub',
    createUrl: 'https://github.com/new',
    urlPattern: 'https://github.com/username/repository.git',
    sshPattern: 'git@github.com:username/repository.git'
  },
  {
    id: 'gitlab',
    name: 'GitLab',
    createUrl: 'https://gitlab.com/projects/new',
    urlPattern: 'https://gitlab.com/username/repository.git',
    sshPattern: 'git@gitlab.com:username/repository.git'
  },
  {
    id: 'gitee',
    name: 'Gitee',
    createUrl: 'https://gitee.com/projects/new',
    urlPattern: 'https://gitee.com/username/repository.git',
    sshPattern: 'git@gitee.com:username/repository.git'
  }
];

const currentPlatform = computed(() => {
  return platforms.find(p => p.id === selectedPlatform.value);
});

const configuredPlatforms = computed(() => {
  const configured = [];
  if (settingsStore.settings.gitPlatforms.github.enabled && settingsStore.settings.gitPlatforms.github.token) {
    configured.push({ id: 'github', name: 'GitHub', username: settingsStore.settings.gitPlatforms.github.username });
  }
  if (settingsStore.settings.gitPlatforms.gitlab.enabled && settingsStore.settings.gitPlatforms.gitlab.token) {
    configured.push({ id: 'gitlab', name: 'GitLab', username: settingsStore.settings.gitPlatforms.gitlab.username });
  }
  if (settingsStore.settings.gitPlatforms.gitee.enabled && settingsStore.settings.gitPlatforms.gitee.token) {
    configured.push({ id: 'gitee', name: 'Gitee', username: settingsStore.settings.gitPlatforms.gitee.username });
  }
  return configured;
});

watch(() => props.isOpen, async (isOpen) => {
  if (isOpen) {
    await checkExistingRemotes();
    // 自动从路径提取仓库名称作为默认值
    if (props.repoPath) {
      const pathParts = props.repoPath.split('/').filter(Boolean);
      repoName.value = pathParts[pathParts.length - 1] || '';
    }
  }
});

async function checkExistingRemotes() {
  if (!props.repoPath) return;

  isLoading.value = true;
  try {
    const response = await GitApi.getRemotes(props.repoPath);
    if (response.success && response.data) {
      existingRemotes.value = response.data;

      // 如果已有远程，直接显示远程列表
      if (existingRemotes.value.length > 0) {
        mode.value = 'select';
      } else {
        mode.value = 'select';
      }
    }
  } catch (error) {
    console.error('Failed to get remotes:', error);
  } finally {
    isLoading.value = false;
  }
}

function selectAddExisting() {
  mode.value = 'add-existing';
  remoteUrl.value = '';
  selectedPlatform.value = '';
}

function selectPublish() {
  // 如果有已配置的平台，直接进入创建仓库模式
  if (configuredPlatforms.value.length > 0) {
    mode.value = 'create-repo';
    selectedPlatform.value = configuredPlatforms.value[0].id as any;
  } else {
    // 否则引导用户去配置账户
    if (confirm('需要先在全局设置中配置 Git 平台账户才能一键创建仓库。\n\n是否现在前往配置？')) {
      emit('open-settings');
      emit('close');
    }
  }
}

function goBack() {
  if (mode.value === 'create-repo') {
    // 如果有已配置的平台，返回选择模式
    mode.value = 'select';
  } else {
    mode.value = 'select';
  }
  remoteUrl.value = '';
  selectedPlatform.value = '';
  // 重置表单
  repoDescription.value = '';
  repoPrivate.value = false;
  repoAutoInit.value = false;
  autoPush.value = true;
  useSSH.value = false;
}

async function handleAddRemote() {
  if (!remoteUrl.value.trim() || !props.repoPath) {
    alert('请输入远程仓库 URL');
    return;
  }

  isLoading.value = true;
  try {
    const response = await GitApi.addRemote(props.repoPath, remoteName.value, remoteUrl.value.trim());

    if (response.success) {
      alert('添加远程仓库成功！');
      emit('remote-added');
      emit('close');
    } else {
      alert('添加远程仓库失败: ' + response.error);
    }
  } catch (error: any) {
    alert('添加远程仓库失败: ' + error.message);
  } finally {
    isLoading.value = false;
  }
}

function openPlatformCreatePage(platform: typeof platforms[0]) {
  // 在浏览器中打开创建仓库页面
  window.open(platform.createUrl, '_blank');
}

function selectPlatformTemplate(platformId: string) {
  selectedPlatform.value = platformId as any;
  const platform = platforms.find(p => p.id === platformId);
  if (platform) {
    // 不自动填充，让用户自己输入
    remoteUrl.value = '';
  }
}

async function handleCreateAndPublish() {
  if (!repoName.value.trim()) {
    alert('请输入仓库名称');
    return;
  }

  // 验证仓库名称
  const validation = validateRepoName(repoName.value.trim());
  if (!validation.valid) {
    alert(validation.error);
    return;
  }

  if (!selectedPlatform.value) {
    alert('请选择平台');
    return;
  }

  isLoading.value = true;

  try {
    const options: CreateRepoOptions = {
      name: repoName.value.trim(),
      description: repoDescription.value.trim(),
      private: repoPrivate.value,
      autoInit: repoAutoInit.value
    };

    let result;
    let token;

    // 获取对应平台的 token
    switch (selectedPlatform.value) {
      case 'github':
        token = settingsStore.settings.gitPlatforms.github.token;
        result = await PlatformApi.createGitHubRepo(token, options);
        break;
      case 'gitlab':
        token = settingsStore.settings.gitPlatforms.gitlab.token;
        result = await PlatformApi.createGitLabRepo(token, options);
        break;
      case 'gitee':
        token = settingsStore.settings.gitPlatforms.gitee.token;
        result = await PlatformApi.createGiteeRepo(token, options);
        break;
      default:
        alert('不支持的平台');
        return;
    }

    if (!result.success) {
      alert('创建仓库失败: ' + (result.error || '未知错误'));
      isLoading.value = false;
      return;
    }

    // 创建成功，添加 remote
    // 根据用户选择使用 SSH 或 HTTPS URL
    let repoUrl;
    if (useSSH.value) {
      repoUrl = result.sshUrl || result.url;
    } else {
      // 使用 HTTPS，需要在 URL 中嵌入 Token 以实现自动认证
      let httpsUrl = result.url;

      // 如果 result.url 是 SSH 格式，转换为 HTTPS
      if (httpsUrl?.startsWith('git@')) {
        switch (selectedPlatform.value) {
          case 'github':
            // git@github.com:user/repo.git -> https://github.com/user/repo.git
            httpsUrl = httpsUrl.replace('git@github.com:', 'https://github.com/');
            break;
          case 'gitlab':
            httpsUrl = httpsUrl.replace('git@gitlab.com:', 'https://gitlab.com/');
            break;
          case 'gitee':
            httpsUrl = httpsUrl.replace('git@gitee.com:', 'https://gitee.com/');
            break;
        }
      }

      // 在 HTTPS URL 中嵌入 Token
      // 格式: https://<token>@github.com/user/repo.git
      if (httpsUrl?.startsWith('https://')) {
        // 获取对应平台的 token
        switch (selectedPlatform.value) {
          case 'github':
            token = settingsStore.settings.gitPlatforms.github.token;
            // https://github.com/user/repo.git -> https://<token>@github.com/user/repo.git
            httpsUrl = httpsUrl.replace('https://', `https://${token}@`);
            break;
          case 'gitlab':
            token = settingsStore.settings.gitPlatforms.gitlab.token;
            // GitLab 使用 oauth2 作为用户名
            httpsUrl = httpsUrl.replace('https://', `https://oauth2:${token}@`);
            break;
          case 'gitee':
            token = settingsStore.settings.gitPlatforms.gitee.token;
            httpsUrl = httpsUrl.replace('https://', `https://${token}@`);
            break;
        }
      }

      repoUrl = httpsUrl;
    }

    if (!repoUrl) {
      alert('仓库创建成功，但未获取到 URL');
      isLoading.value = false;
      return;
    }

    // 调试信息（隐藏 Token）
    console.log('选择的 URL 类型:', useSSH.value ? 'SSH' : 'HTTPS');
    const safeUrl = repoUrl?.includes('@') && !repoUrl.startsWith('git@')
      ? repoUrl.replace(/\/\/[^@]+@/, '//***@')  // 隐藏 Token
      : repoUrl;
    console.log('实际使用的 URL:', safeUrl);
    console.log('URL 格式:', repoUrl?.startsWith('git@') ? 'SSH' : 'HTTPS');
    console.log('包含认证信息:', repoUrl?.includes('@') && !repoUrl?.startsWith('git@') ? '是' : '否');

    // 检查是否已存在 origin 远程
    const remotesResponse = await GitApi.getRemotes(props.repoPath);
    const originExists = remotesResponse.success &&
                        remotesResponse.data?.some(r => r.name === 'origin');

    let addRemoteResponse;
    if (originExists) {
      // 如果已存在，先删除再添加（相当于更新）
      await GitApi.removeRemote(props.repoPath, 'origin');
      addRemoteResponse = await GitApi.addRemote(props.repoPath, 'origin', repoUrl);
    } else {
      // 不存在则直接添加
      addRemoteResponse = await GitApi.addRemote(props.repoPath, 'origin', repoUrl);
    }

    if (!addRemoteResponse.success) {
      alert(`仓库创建成功，但添加远程失败: ${addRemoteResponse.error}\n\n仓库 URL: ${repoUrl}`);
      isLoading.value = false;
      return;
    }

    // 如果选择自动推送
    if (autoPush.value) {
      // 获取当前分支
      const currentBranchResponse = await GitApi.getCurrentBranch(props.repoPath);
      let branchToPush = 'main';

      if (currentBranchResponse.success && currentBranchResponse.data) {
        branchToPush = currentBranchResponse.data;
        console.log('当前分支:', branchToPush);
      }

      // 检查是否有提交记录
      const commitsResponse = await GitApi.getCommits(props.repoPath, 1);
      if (!commitsResponse.success || !commitsResponse.data || commitsResponse.data.length === 0) {
        alert(`仓库创建成功！\n\n但本地仓库还没有任何提交记录，无法推送。\n\n请先进行提交：\n1. 在"变更"标签页中暂存文件\n2. 输入提交信息并提交\n3. 然后使用顶部的"推送"按钮推送到远程`);
        emit('remote-added');
        emit('close');
        return;
      }

      const pushResponse = await GitApi.push(props.repoPath, 'origin', branchToPush);

      if (pushResponse.success) {
        alert('仓库创建成功并已推送！');
      } else {
        // 检查具体错误类型
        const isAuthError = pushResponse.error?.includes('authentication') ||
                           pushResponse.error?.includes('Auth') ||
                           pushResponse.error?.includes('credentials');

        const isEmptyRepoError = pushResponse.error?.includes('does not match any') ||
                                pushResponse.error?.includes('src refspec');

        let errorMsg = `仓库创建成功，但推送失败:\n${pushResponse.error}\n\n`;

        if (isEmptyRepoError) {
          errorMsg += '这是因为本地仓库没有提交记录。解决方法：\n\n';
          errorMsg += '1. 先进行至少一次提交，然后再推送\n';
          errorMsg += '2. 或者在终端执行以下命令：\n';
          errorMsg += `   git add .\n`;
          errorMsg += `   git commit -m "Initial commit"\n`;
          errorMsg += `   git push -u origin ${branchToPush}`;
        } else if (isAuthError) {
          errorMsg += '这通常是因为需要身份验证。解决方法：\n\n';
          errorMsg += '1. 检查 Token 是否正确配置\n';
          errorMsg += '2. 或使用 SSH URL（需要配置 SSH 密钥）\n\n';
          errorMsg += '3. 或者在终端手动推送：\n';
          errorMsg += `   git push -u origin ${branchToPush}`;
        } else {
          errorMsg += '请手动推送代码：\n';
          errorMsg += `git push -u origin ${branchToPush}`;
        }

        alert(errorMsg);
      }
    } else {
      alert('仓库创建成功！远程已添加。');
    }

    emit('remote-added');
    emit('close');

  } catch (error: any) {
    alert('操作失败: ' + error.message);
  } finally {
    isLoading.value = false;
  }
}

function handleCancel() {
  emit('close');
  // 重置状态
  setTimeout(() => {
    mode.value = 'select';
    remoteUrl.value = '';
    selectedPlatform.value = '';
    repoDescription.value = '';
    repoPrivate.value = false;
    repoAutoInit.value = false;
    autoPush.value = true;
    useSSH.value = true;
  }, 300);
}
</script>

<template>
  <div v-if="isOpen" class="modal-overlay" @click="handleCancel">
    <div class="modal-content" @click.stop>
      <!-- Header -->
      <div class="modal-header">
        <div class="header-left">
          <button v-if="mode !== 'select'" class="back-btn" @click="goBack">
            ←
          </button>
          <h3>
            <span v-if="mode === 'select'">远程仓库</span>
            <span v-else-if="mode === 'add-existing'">添加现有远程仓库</span>
            <span v-else-if="mode === 'create-repo'">创建并发布仓库</span>
            <span v-else>发布到新仓库</span>
          </h3>
        </div>
        <button class="close-btn" @click="handleCancel">×</button>
      </div>

      <!-- Body -->
      <div class="modal-body">
        <!-- Mode: Select -->
        <div v-if="mode === 'select'" class="select-mode">
          <!-- Existing Remotes -->
          <div v-if="existingRemotes.length > 0" class="existing-remotes">
            <h4>已配置的远程仓库</h4>
            <div class="remote-list">
              <div v-for="remote in existingRemotes" :key="remote.name" class="remote-item">
                <div class="remote-info">
                  <span class="remote-name">{{ remote.name }}</span>
                  <span class="remote-url">{{ remote.url }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- No Remotes - Show Options -->
          <div v-else class="no-remotes">
            <p class="no-remotes-message">此仓库尚未配置远程仓库</p>
            <p class="no-remotes-hint">选择一个选项来开始：</p>
          </div>

          <!-- Action Cards -->
          <div class="action-cards">
            <div class="action-card" @click="selectAddExisting">
              <div class="card-content">
                <h4>添加现有远程仓库</h4>
                <p>如果你已经在 GitHub、GitLab 或 Gitee 创建了仓库</p>
              </div>
              <div class="card-arrow">→</div>
            </div>

            <div class="action-card" @click="selectPublish">
              <div class="card-content">
                <h4>发布到新仓库</h4>
                <p>
                  <span v-if="configuredPlatforms.length > 0">一键创建远程仓库并推送代码</span>
                  <span v-else>在 Git 平台创建新仓库并推送代码</span>
                </p>
              </div>
              <div class="card-arrow">→</div>
            </div>
          </div>
        </div>

        <!-- Mode: Add Existing -->
        <div v-else-if="mode === 'add-existing'" class="add-existing-mode">
          <div class="platform-selector">
            <p class="selector-label">选择你使用的平台（可选）：</p>
            <div class="platform-buttons">
              <button
                v-for="platform in platforms"
                :key="platform.id"
                :class="['platform-btn', { active: selectedPlatform === platform.id }]"
                @click="selectPlatformTemplate(platform.id)"
              >
                <span class="platform-icon">{{ platform.icon }}</span>
                <span>{{ platform.name }}</span>
              </button>
            </div>
          </div>

          <div class="form-group">
            <label for="remote-url">远程仓库 URL <span class="required">*</span></label>
            <input
              id="remote-url"
              v-model="remoteUrl"
              type="text"
              placeholder="https://github.com/username/repository.git"
              @keyup.enter="handleAddRemote"
            />

            <div v-if="currentPlatform" class="url-examples">
              <div class="example-item">
                <span class="example-label">HTTPS:</span>
                <code>{{ currentPlatform.urlPattern }}</code>
              </div>
              <div class="example-item">
                <span class="example-label">SSH:</span>
                <code>{{ currentPlatform.sshPattern }}</code>
              </div>
            </div>
          </div>
        </div>

        <!-- Mode: Create Repo -->
        <div v-else-if="mode === 'create-repo'" class="create-repo-mode">
          <div class="form-section">
            <div class="form-group">
              <label>选择平台</label>
              <select v-model="selectedPlatform">
                <option v-for="platform in configuredPlatforms" :key="platform.id" :value="platform.id">
                  {{ platform.name }} ({{ platform.username }})
                </option>
              </select>
            </div>

            <div class="form-group">
              <label>仓库名称 <span class="required">*</span></label>
              <input
                v-model="repoName"
                type="text"
                placeholder="my-awesome-project"
                :class="{ 'error': repoNameError }"
              />
              <p v-if="repoNameError" class="error-message">{{ repoNameError }}</p>
            </div>

            <div class="form-group">
              <label>描述（可选）</label>
              <textarea
                v-model="repoDescription"
                placeholder="项目的简短描述..."
                rows="3"
              ></textarea>
            </div>

            <div class="form-group-row">
              <label class="checkbox-label">
                <input type="checkbox" v-model="repoPrivate">
                <span>私有仓库</span>
              </label>

              <label class="checkbox-label">
                <input type="checkbox" v-model="repoAutoInit">
                <span>初始化 README</span>
              </label>
            </div>

            <div class="form-group">
              <label>远程 URL 类型</label>
              <div class="radio-group">
                <label class="radio-label">
                  <input type="radio" :value="false" v-model="useSSH">
                  <span>HTTPS（使用 Token 自动认证，推荐）</span>
                </label>
                <label class="radio-label">
                  <input type="radio" :value="true" v-model="useSSH">
                  <span>SSH（需要配置 SSH 密钥）</span>
                </label>
              </div>
            </div>

            <div class="form-group">
              <label class="checkbox-label">
                <input type="checkbox" v-model="autoPush">
                <span>创建后自动推送代码</span>
              </label>
              <p class="hint">如果取消勾选，需要手动执行 git push</p>
            </div>
          </div>

          <div class="info-box">
            <p>创建仓库后将自动添加远程地址并推送代码。</p>
          </div>
        </div>

        <!-- Mode: Publish -->
        <div v-else-if="mode === 'publish'" class="publish-mode">
          <div class="publish-intro">
            <p>要发布到新仓库，请按照以下步骤：</p>
          </div>

          <div class="publish-steps">
            <div class="step">
              <div class="step-number">1</div>
              <div class="step-content">
                <h4>在 Git 平台创建新仓库</h4>
                <p>选择一个平台，在浏览器中创建新仓库</p>
                <div class="platform-buttons">
                  <button
                    v-for="platform in platforms"
                    :key="platform.id"
                    class="platform-btn-large"
                    @click="openPlatformCreatePage(platform)"
                  >
                    <span class="platform-icon">{{ platform.icon }}</span>
                    <div class="platform-info">
                      <span class="platform-name">{{ platform.name }}</span>
                      <span class="platform-hint">在新窗口打开</span>
                    </div>
                    <span class="external-icon">↗</span>
                  </button>
                </div>
              </div>
            </div>

            <div class="step">
              <div class="step-number">2</div>
              <div class="step-content">
                <h4>复制仓库 URL</h4>
                <p>创建完成后，复制仓库的 URL（HTTPS 或 SSH）</p>
              </div>
            </div>

            <div class="step">
              <div class="step-number">3</div>
              <div class="step-content">
                <h4>添加远程仓库并推送</h4>
                <div class="form-group">
                  <input
                    v-model="remoteUrl"
                    type="text"
                    placeholder="粘贴仓库 URL"
                    @keyup.enter="handleAddRemote"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="modal-footer">
        <button class="cancel-btn" @click="handleCancel">取消</button>
        <button
          v-if="mode === 'create-repo'"
          class="submit-btn"
          :disabled="!repoName.trim() || isLoading"
          @click="handleCreateAndPublish"
        >
          {{ isLoading ? '创建中...' : '创建并发布' }}
        </button>
        <button
          v-else-if="mode === 'add-existing' || mode === 'publish'"
          class="submit-btn"
          :disabled="!remoteUrl.trim() || isLoading"
          @click="handleAddRemote"
        >
          {{ isLoading ? '添加中...' : '添加远程仓库' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--bg-primary);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-xl);
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-header {
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-left {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.back-btn {
  font-size: 20px;
  color: var(--text-secondary);
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: var(--radius-sm);
}

.back-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.modal-header h3 {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-bold);
  color: var(--text-primary);
  margin: 0;
}

.close-btn {
  font-size: 28px;
  color: var(--text-tertiary);
  background: none;
  border: none;
  cursor: pointer;
  line-height: 1;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-sm);
}

.close-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.modal-body {
  padding: var(--spacing-lg);
  overflow-y: auto;
  flex: 1;
}

/* Select Mode */
.existing-remotes {
  margin-bottom: var(--spacing-lg);
  padding-bottom: var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);
}

.existing-remotes h4 {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: var(--spacing-sm);
}

.remote-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.remote-item {
  padding: var(--spacing-sm);
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
}

.remote-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.remote-name {
  font-weight: 600;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
}

.remote-url {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  font-family: monospace;
}

.no-remotes {
  text-align: center;
  padding: var(--spacing-lg) var(--spacing-lg);
  margin-bottom: var(--spacing-lg);
}

.no-remotes-message {
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.no-remotes-hint {
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
}

.action-cards {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.action-card {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-md);
  border: 2px solid var(--border-color);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--transition-fast);
  background-color: var(--bg-primary);
}

.action-card:hover {
  border-color: var(--accent-color);
  background-color: var(--bg-hover);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.card-content {
  flex: 1;
}

.card-content h4 {
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.card-content p {
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
  margin: 0;
}

.card-arrow {
  font-size: 20px;
  color: var(--text-tertiary);
  flex-shrink: 0;
}

/* Platform Selector */
.platform-selector {
  margin-bottom: var(--spacing-lg);
}

.selector-label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-sm);
}

.platform-buttons {
  display: flex;
  gap: var(--spacing-sm);
}

.platform-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm);
  border: 2px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.platform-btn:hover {
  border-color: var(--text-tertiary);
  background-color: var(--bg-hover);
}

.platform-btn.active {
  border-color: var(--accent-color);
  background-color: rgba(59, 130, 246, 0.1);
  color: var(--accent-color);
}

.platform-icon {
  font-size: 18px;
}

/* Form Group */
.form-group {
  margin-bottom: var(--spacing-md);
}

.form-group label {
  display: block;
  margin-bottom: var(--spacing-xs);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
}

.required {
  color: #ef4444;
}

.form-group input {
  width: 100%;
  padding: var(--spacing-sm);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  transition: all var(--transition-fast);
}

.form-group input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.form-group input.error {
  border-color: #ef4444;
}

.form-group input.error:focus {
  border-color: #ef4444;
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
}

.error-message {
  color: #ef4444;
  font-size: var(--font-size-xs);
  margin-top: 4px;
  margin-bottom: 0;
}

.url-examples {
  margin-top: var(--spacing-sm);
  padding: var(--spacing-sm);
  background-color: var(--bg-secondary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
}

.example-item {
  display: flex;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-xs);
  font-size: var(--font-size-xs);
}

.example-item:last-child {
  margin-bottom: 0;
}

.example-label {
  color: var(--text-tertiary);
  font-weight: var(--font-weight-medium);
  min-width: 50px;
}

code {
  flex: 1;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  color: var(--text-primary);
  background-color: var(--bg-tertiary);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
  font-size: 11px;
}

/* Publish Mode */
.publish-intro {
  margin-bottom: var(--spacing-lg);
  padding: var(--spacing-md);
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: var(--radius-md);
  border-left: 3px solid var(--accent-color);
}

.publish-intro p {
  margin: 0;
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
}

.publish-steps {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.step {
  display: flex;
  gap: var(--spacing-md);
}

.step-number {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background-color: var(--accent-color);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  flex-shrink: 0;
}

.step-content {
  flex: 1;
}

.step-content h4 {
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.step-content p {
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
  margin-bottom: var(--spacing-sm);
}

.platform-btn-large {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  transition: all var(--transition-fast);
  margin-bottom: var(--spacing-xs);
  width: 100%;
}

.platform-btn-large:hover {
  border-color: var(--accent-color);
  background-color: var(--bg-hover);
}

.platform-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}

.platform-name {
  font-weight: 600;
  font-size: var(--font-size-sm);
}

.platform-hint {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

.external-icon {
  color: var(--text-tertiary);
  font-size: 16px;
}

/* Footer */
.modal-footer {
  padding: var(--spacing-lg);
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
}

.cancel-btn,
.submit-btn {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.cancel-btn {
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
}

.cancel-btn:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.submit-btn {
  background-color: var(--accent-color);
  color: white;
  border: none;
}

.submit-btn:hover:not(:disabled) {
  background-color: var(--accent-hover);
}

.submit-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Create Repo Mode */
.create-repo-mode {
  padding: var(--spacing-md);
}

.form-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.form-group-row {
  display: flex;
  gap: var(--spacing-lg);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  cursor: pointer;
}

.radio-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  margin-top: var(--spacing-xs);
}

.radio-label {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  cursor: pointer;
  padding: var(--spacing-sm);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
  transition: all var(--transition-fast);
}

.radio-label:hover {
  background-color: var(--bg-hover);
}

.radio-label input[type="radio"] {
  cursor: pointer;
}

.required {
  color: #ef4444;
}

textarea {
  width: 100%;
  padding: var(--spacing-sm);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  font-family: inherit;
  resize: vertical;
}

textarea:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.info-box {
  padding: var(--spacing-sm);
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: var(--radius-md);
  border-left: 3px solid var(--accent-color);
}

.info-box p {
  margin: 0;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

select {
  width: 100%;
  padding: var(--spacing-sm);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  cursor: pointer;
}

select:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}
</style>
