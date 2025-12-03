<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { repoStore } from '../../stores/repoStore';
import { toastStore } from '../../stores/toastStore';
import type { FileChange } from '../../types';
import AISettingsModal, { type AISettings } from '../modals/AISettingsModal.vue';
import FileIcon from '../common/FileIcon.vue';
import Resizer from '../layout/Resizer.vue';
import DiffStats from '../common/DiffStats.vue';
import { GitApi } from '../../services/gitApi';
import { settingsStore } from '../../stores/settingsStore';

const commitMessage = ref('');
const isCommitting = ref(false);
const isGeneratingAI = ref(false);
const showAISettings = ref(false);
const commitSectionHeight = ref(200); // 提交区域的高度
const showFileActions = ref(false); // 是否显示文件操作按钮
const aiGeneratedCommit = ref(false); // 标记提交信息是否由 AI 生成

onMounted(() => {
  // 从设置中恢复提交区域高度
  commitSectionHeight.value = settingsStore.settings.layout.commitSectionHeight ?? 200;
});

const stagedFiles = computed(() => repoStore.fileChanges.filter(f => f.staged));
const unstagedFiles = computed(() => repoStore.fileChanges.filter(f => !f.staged));

// 计算左侧按钮的配置（暂存或提交）
const leftButtonConfig = computed(() => {
  const hasStaged = stagedFiles.value.length > 0;
  const hasUnstaged = unstagedFiles.value.length > 0;

  // 如果是 AI 生成的提交信息，且有未暂存文件 -> 显示"暂存所有"
  if (aiGeneratedCommit.value && hasUnstaged) {
    return {
      text: '暂存所有',
      title: `暂存所有未暂存的文件 (${unstagedFiles.value.length} 个)`,
      disabled: false,
      action: 'stage-all'
    };
  }

  // AI 生成后，所有文件都已暂存 -> 显示"提交"且可用
  if (aiGeneratedCommit.value && !hasUnstaged && hasStaged) {
    return {
      text: `提交至 ${repoStore.currentBranch}`,
      title: `提交暂存的文件 (${stagedFiles.value.length} 个)`,
      disabled: !commitMessage.value || isCommitting.value,
      action: 'commit'
    };
  }

  // 普通情况：提交按钮
  return {
    text: `提交至 ${repoStore.currentBranch}`,
    title: hasStaged ? '提交暂存的文件' : '请先暂存文件',
    disabled: !commitMessage.value || isCommitting.value || !hasStaged,
    action: 'commit'
  };
});

async function toggleStage(file: FileChange) {
  try {
    // 如果当前文件被选中，操作后清除选择
    const shouldClearSelection = repoStore.selectedFile?.path === file.path;

    if (file.staged) {
      await repoStore.unstageFile(file.path);
    } else {
      await repoStore.stageFile(file.path);
    }

    // 清除diff视图
    if (shouldClearSelection) {
      repoStore.selectedFile = null;
    }
  } catch (error: any) {
    toastStore.error('操作失败: ' + error.message);
  }
}

async function stageAll() {
  if (unstagedFiles.value.length === 0) return;

  try {
    // 批量暂存所有文件
    await Promise.all(
      unstagedFiles.value.map(file => repoStore.stageFile(file.path))
    );
  } catch (error: any) {
    toastStore.error('暂存失败: ' + error.message);
  }
}

async function unstageAll() {
  if (stagedFiles.value.length === 0) return;

  try {
    // 批量取消暂存所有文件
    await Promise.all(
      stagedFiles.value.map(file => repoStore.unstageFile(file.path))
    );
  } catch (error: any) {
    toastStore.error('取消暂存失败: ' + error.message);
  }
}

async function discardFile(file: FileChange, event?: Event) {
  // 阻止事件冒泡，避免触发其他操作
  if (event) {
    event.preventDefault();
    event.stopPropagation();
  }

  // 确保确认对话框在执行前显示
  const confirmed = window.confirm(
    `确定要丢弃 ${file.path} 的所有更改吗？\n\n此操作不可撤销！`
  );

  // 如果用户点击取消，立即返回
  if (!confirmed) {
    console.log('用户取消了丢弃操作');
    return;
  }

  console.log('用户确认丢弃文件:', file.path);

  // 如果当前文件被选中，操作后清除选择
  const shouldClearSelection = repoStore.selectedFile?.path === file.path;

  try {
    await repoStore.discardFile(file.path);
    console.log('文件丢弃成功:', file.path);

    // 清除diff视图
    if (shouldClearSelection) {
      repoStore.selectedFile = null;
    }
  } catch (error: any) {
    console.error('丢弃文件失败:', error);
    toastStore.error('丢弃更改失败: ' + error.message);
  }
}

async function discardAllUnstaged(event?: Event) {
  // 阻止事件冒泡
  if (event) {
    event.preventDefault();
    event.stopPropagation();
  }

  if (unstagedFiles.value.length === 0) return;

  const fileCount = unstagedFiles.value.length;
  const confirmed = window.confirm(
    `确定要丢弃所有工作区的更改吗？\n\n这将丢弃 ${fileCount} 个文件的更改。\n此操作不可撤销！`
  );

  if (!confirmed) {
    console.log('用户取消了批量丢弃操作');
    return;
  }

  console.log(`用户确认丢弃 ${fileCount} 个文件`);

  try {
    await Promise.all(
      unstagedFiles.value.map(file => repoStore.discardFile(file.path))
    );
    console.log('批量丢弃成功');

    // 清除diff视图
    repoStore.selectedFile = null;
  } catch (error: any) {
    console.error('批量丢弃失败:', error);
    toastStore.error('丢弃更改失败: ' + error.message);
  }
}

function selectFile(file: FileChange) {
  repoStore.selectedFile = file;
}

// 处理左侧按钮点击（暂存所有 或 提交）
async function handleLeftButtonClick() {
  if (leftButtonConfig.value.action === 'stage-all') {
    // 暂存所有文件
    await stageAll();
    // 暂存后，AI 生成标记保持，但按钮会自动变为"提交"
  } else {
    // 提交
    await doCommit();
  }
}

async function doCommit() {
  if (!commitMessage.value.trim()) {
    toastStore.warning('请输入提交信息');
    return;
  }

  if (!repoStore.activeRepo) {
    toastStore.warning('请先打开一个仓库');
    return;
  }

  // 检查是否有暂存的文件
  if (stagedFiles.value.length === 0) {
    toastStore.warning('请先暂存文件');
    return;
  }

  isCommitting.value = true;
  try {
    // 执行提交
    await repoStore.commit(commitMessage.value);
    commitMessage.value = '';
    aiGeneratedCommit.value = false; // 重置 AI 生成标记
    toastStore.success('提交成功!');
  } catch (error: any) {
    toastStore.error('提交失败: ' + error.message);
  } finally {
    isCommitting.value = false;
  }
}

async function doCommitAndPush() {
  // 先执行提交前的检查
  if (!repoStore.activeRepo) {
    toastStore.warning('请先打开一个仓库');
    return;
  }

  // 自动暂存所有未暂存的文件
  if (stagedFiles.value.length === 0 && unstagedFiles.value.length > 0) {
    try {
      for (const file of unstagedFiles.value) {
        await repoStore.stageFile(file.path);
      }
    } catch (error: any) {
      toastStore.error('暂存文件失败: ' + error.message);
      return;
    }
  }

  // 再次检查是否有暂存的文件
  if (stagedFiles.value.length === 0) {
    toastStore.warning('没有文件需要提交');
    return;
  }

  isCommitting.value = true;
  try {
    // 执行提交
    await repoStore.commit(commitMessage.value);
    commitMessage.value = '';
    aiGeneratedCommit.value = false; // 重置 AI 生成标记
    toastStore.success('提交成功!');

    // 直接推送，不询问
    try {
      await repoStore.push();
      toastStore.success('推送成功!');
    } catch (error: any) {
      toastStore.error('推送失败: ' + error.message);
    }
  } catch (error: any) {
    toastStore.error('提交失败: ' + error.message);
  } finally {
    isCommitting.value = false;
  }
}

async function generateAICommitMessage() {
  if (!repoStore.activeRepo) {
    toastStore.warning('请先打开一个仓库');
    return;
  }

  // 检查是否有变更的文件（暂存或未暂存）
  if (repoStore.fileChanges.length === 0) {
    toastStore.info('没有文件变更，无需生成提交信息');
    return;
  }

  // Load AI settings
  const savedSettings = localStorage.getItem('ai_settings');
  if (!savedSettings) {
    toastStore.warning('请先配置 AI 设置');
    showAISettings.value = true;
    return;
  }

  isGeneratingAI.value = true;

  try {
    const settings: AISettings = JSON.parse(savedSettings);

    if (!settings.apiKey) {
      toastStore.warning('请先配置 API Key');
      showAISettings.value = true;
      isGeneratingAI.value = false;
      return;
    }

    // 显示加载状态
    commitMessage.value = '正在生成提交信息...';

    // 获取所有变更文件的 diff
    const diffs: string[] = [];
    for (const file of repoStore.fileChanges) {
      try {
        const response = await GitApi.getFileDiff(
          repoStore.activeRepo.path,
          file.path,
          file.staged
        );

        if (response.success && response.data) {
          // 构建简化的 diff 信息
          let fileDiff = `File: ${file.path} (${file.status})\n`;

          if (response.data.hunks && response.data.hunks.length > 0) {
            for (const hunk of response.data.hunks) {
              fileDiff += `${hunk.header}\n`;
              // 包含所有变更行
              const changes = hunk.lines
                .filter((line: any) => line.origin === '+' || line.origin === '-')
                .map((line: any) => `${line.origin}${line.content}`)
                .join('\n');
              fileDiff += changes + '\n';
            }
          }

          diffs.push(fileDiff);
        }
      } catch (error) {
        console.error(`Failed to get diff for ${file.path}:`, error);
      }
    }

    const diffContext = diffs.join('\n---\n').slice(0, 16000); // 增加内容长度限制

    // 通过 Rust 后端调用 AI API
    const response = await GitApi.callAIApi(
      settings.apiEndpoint,
      settings.apiKey,
      settings.model,
      [
        {
          role: 'system',
          content: settings.systemPrompt
        },
        {
          role: 'user',
          content: `请基于以下代码变更，生成一条简洁清晰的 Git 提交信息。语言：${settings.language}\n\n代码变更：\n${diffContext}`
        }
      ],
      settings.temperature,
      settings.maxTokens
    );

    if (!response.success || !response.data) {
      throw new Error(response.error || 'AI API 调用失败');
    }

    let generatedMessage = response.data.trim();

    // 清理 AI 生成的 Markdown 格式标记
    // 移除代码块标记 ```
    generatedMessage = generatedMessage.replace(/```[\w]*\n?/g, '');
    // 移除可能的引用块标记
    generatedMessage = generatedMessage.replace(/^>\s*/gm, '');
    // 清理多余的空行
    generatedMessage = generatedMessage.replace(/\n{3,}/g, '\n\n').trim();

    if (generatedMessage) {
      commitMessage.value = generatedMessage;
      aiGeneratedCommit.value = true; // 标记为 AI 生成
    } else {
      throw new Error('AI 返回了空的提交信息');
    }

  } catch (error: any) {
    console.error('AI 生成失败:', error);
    commitMessage.value = '';
    aiGeneratedCommit.value = false;
    toastStore.error('AI 生成失败: ' + error.message);
  } finally {
    isGeneratingAI.value = false;
  }
}

function handleAISettingsSave(settings: AISettings) {
  console.log('AI settings saved:', settings);
}

// 监听手动编辑提交信息
function handleCommitMessageInput() {
  // 用户手动编辑时，重置 AI 生成标记
  if (aiGeneratedCommit.value) {
    aiGeneratedCommit.value = false;
  }
}

function handleCommitSectionResize(delta: number) {
  const newHeight = Math.max(150, Math.min(500, commitSectionHeight.value - delta));
  commitSectionHeight.value = newHeight;
  settingsStore.updateLayoutSettings({ commitSectionHeight: newHeight });
}

function getStatusIcon(status: FileChange['status']) {
  switch (status) {
    case 'modified': return 'M';
    case 'added': return 'A';
    case 'deleted': return 'D';
    case 'renamed': return 'R';
    case 'untracked': return '?';
    default: return '';
  }
}

function getStatusColor(status: FileChange['status']) {
  switch (status) {
    case 'modified': return '#eab308'; // yellow
    case 'added': return '#10b981'; // green
    case 'deleted': return '#ef4444'; // red
    case 'renamed': return '#3b82f6'; // blue
    case 'untracked': return '#9ca3af'; // gray
    default: return 'var(--text-secondary)';
  }
}

function getDiffStatusLabel(status?: FileChange['diffStatus']) {
  switch (status) {
    case 'generating': return '生成中';
    case 'applying': return '应用中';
    case 'applied': return '已应用';
    default: return null;
  }
}

function getDiffStatusColor(status?: FileChange['diffStatus']) {
  switch (status) {
    case 'generating': return '#8b5cf6'; // purple
    case 'applying': return '#f59e0b'; // amber
    case 'applied': return '#10b981'; // green
    default: return 'transparent';
  }
}


</script>

<template>
  <div class="changes-view">
    <div class="changes-panel">
      <!-- File List Section -->
      <div class="files-section">
        <div class="section-header">
          <span>变更文件 ({{ repoStore.fileChanges.length }})</span>
        </div>
        
        <div class="file-groups">
          <!-- Staged -->
          <div v-if="stagedFiles.length > 0" class="file-group">
            <div class="group-header">
              <span class="group-title">暂存区 ({{ stagedFiles.length }})</span>
              <button class="group-action" @click="unstageAll" title="取消暂存所有文件">
                <span class="action-icon">−</span>
              </button>
            </div>
            <ul class="file-list">
              <li
                v-for="file in stagedFiles"
                :key="file.path"
                :class="{ selected: repoStore.selectedFile?.path === file.path }"
                @click="selectFile(file)"
              >
                <div class="file-main-info">
                  <FileIcon :fileName="file.path.split('/').pop() || file.path" />
                  <span class="file-path">{{ file.path }}</span>
                  <span class="status-badge" :style="{ backgroundColor: getStatusColor(file.status) }">
                    {{ getStatusIcon(file.status) }}
                  </span>
                  <span
                    v-if="file.diffStatus && file.diffStatus !== 'idle'"
                    class="diff-status-badge"
                    :style="{ backgroundColor: getDiffStatusColor(file.diffStatus) }"
                  >
                    {{ getDiffStatusLabel(file.diffStatus) }}
                  </span>
                  <button class="file-action" @click.stop="toggleStage(file)" title="取消暂存">
                    <span class="action-icon">−</span>
                  </button>
                </div>
                <div v-if="file.stats" class="file-stats">
                  <DiffStats :stats="file.stats" compact />
                </div>
              </li>
            </ul>
          </div>

          <!-- Unstaged -->
          <div v-if="unstagedFiles.length > 0" class="file-group">
            <div class="group-header">
              <span class="group-title">工作区 ({{ unstagedFiles.length }})</span>
              <div class="group-actions">
                <button
                  class="group-action toggle-actions"
                  @click="showFileActions = !showFileActions"
                  :title="showFileActions ? '隐藏操作按钮' : '显示操作按钮'"
                >
                  <svg v-if="showFileActions" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                    <circle cx="12" cy="12" r="3"></circle>
                  </svg>
                  <svg v-else width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path>
                    <line x1="1" y1="1" x2="23" y2="23"></line>
                  </svg>
                </button>
                <button class="group-action discard-all" @click="discardAllUnstaged($event)" title="丢弃所有更改">
                  <span class="action-icon">×</span>
                </button>
                <button class="group-action" @click="stageAll" title="暂存所有文件">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="12" y1="5" x2="12" y2="19"></line>
                    <line x1="5" y1="12" x2="19" y2="12"></line>
                  </svg>
                </button>
              </div>
            </div>
            <ul class="file-list">
              <li
                v-for="file in unstagedFiles"
                :key="file.path"
                :class="{ selected: repoStore.selectedFile?.path === file.path }"
                @click="selectFile(file)"
              >
                <div class="file-main-info">
                  <FileIcon :fileName="file.path.split('/').pop() || file.path" />
                  <span class="file-path">{{ file.path }}</span>
                  <span class="status-badge" :style="{ backgroundColor: getStatusColor(file.status) }">
                    {{ getStatusIcon(file.status) }}
                  </span>
                  <span
                    v-if="file.diffStatus && file.diffStatus !== 'idle'"
                    class="diff-status-badge"
                    :style="{ backgroundColor: getDiffStatusColor(file.diffStatus) }"
                  >
                    {{ getDiffStatusLabel(file.diffStatus) }}
                  </span>
                  <button
                    v-if="showFileActions"
                    class="file-action discard"
                    @click.stop="discardFile(file, $event)"
                    title="丢弃更改"
                  >
                    <span class="action-icon">×</span>
                  </button>
                  <button
                    v-if="showFileActions"
                    class="file-action stage"
                    @click.stop="toggleStage(file)"
                    title="暂存"
                  >
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <line x1="12" y1="5" x2="12" y2="19"></line>
                      <line x1="5" y1="12" x2="19" y2="12"></line>
                    </svg>
                  </button>
                </div>
                <div v-if="file.stats" class="file-stats">
                  <DiffStats :stats="file.stats" compact />
                </div>
              </li>
            </ul>
          </div>
        </div>
      </div>

      <!-- Resizer for commit section -->
      <Resizer direction="vertical" @resize="handleCommitSectionResize" />

      <!-- Commit Section -->
      <div class="commit-section" :style="{ height: commitSectionHeight + 'px' }">
        <div class="section-header">
          <span>提交信息</span>
          <div class="ai-button-group">
            <button
              class="ai-generate-btn primary"
              :disabled="isGeneratingAI || repoStore.fileChanges.length === 0"
              @click="generateAICommitMessage"
              title="AI生成提交信息"
            >
              <svg class="ai-generate-icon" :class="{ generating: isGeneratingAI }" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="16" height="16" fill="none">
                <path d="M512 348.416a742.4 742.4 0 0 1 265.386667 43.904C853.888 422.741333 896 465.237333 896 512s-42.112 89.258667-118.613333 119.68A742.4 742.4 0 0 1 512 675.584a742.4 742.4 0 0 1-265.386667-43.904C170.112 601.258667 128 558.762667 128 512s42.112-89.301333 118.613333-119.68A742.4 742.4 0 0 1 512 348.416z m0 290.816a705.322667 705.322667 0 0 0 251.733333-41.301333c60.757333-24.149333 95.616-55.466667 95.616-85.930667s-34.773333-61.781333-95.616-85.930667a705.322667 705.322667 0 0 0-251.733333-41.301333 705.322667 705.322667 0 0 0-251.733333 41.301333c-60.842667 24.149333-95.701333 55.466667-95.701334 85.930667s34.858667 61.781333 95.701334 85.930667a705.322667 705.322667 0 0 0 251.733333 41.301333z" fill="currentColor"/>
                <path d="M362.453333 170.666667a238.933333 238.933333 0 0 1 145.92 73.898666 775.210667 775.210667 0 0 1 146.133334 185.642667 730.922667 730.922667 0 0 1 94.592 250.410667c11.733333 81.066667-4.266667 138.538667-44.970667 162.133333a84.096 84.096 0 0 1-42.666667 10.794667 238.677333 238.677333 0 0 1-145.92-73.898667 775.424 775.424 0 0 1-146.176-185.856 730.922667 730.922667 0 0 1-94.336-250.410667C263.253333 262.357333 279.253333 204.8 320 181.461333A84.138667 84.138667 0 0 1 362.453333 170.666667z m299.093334 646.314666a47.914667 47.914667 0 0 0 24.192-5.930666c26.538667-15.232 36.394667-60.885333 27.093333-125.269334a694.442667 694.442667 0 0 0-89.984-237.397333 749.312 749.312 0 0 0-139.008-176.853333 209.066667 209.066667 0 0 0-121.386667-64.512 47.914667 47.914667 0 0 0-24.149333 5.930666c-26.538667 15.189333-36.437333 60.885333-27.093333 125.269334a694.442667 694.442667 0 0 0 89.941333 237.397333 749.482667 749.482667 0 0 0 138.88 176.853333 209.066667 209.066667 0 0 0 121.514667 64.512z" fill="currentColor"/>
                <path d="M362.453333 853.333333a84.138667 84.138667 0 0 1-42.453333-10.794666c-40.746667-23.381333-56.746667-81.066667-44.970667-162.133334a730.965333 730.965333 0 0 1 94.336-250.197333 775.381333 775.381333 0 0 1 146.218667-185.642667A238.677333 238.677333 0 0 1 661.546667 170.666667a84.096 84.096 0 0 1 42.666666 10.794666c40.746667 23.381333 56.704 81.066667 44.970667 162.133334a730.965333 730.965333 0 0 1-94.677333 250.197333 775.210667 775.210667 0 0 1-146.133334 185.642667A238.933333 238.933333 0 0 1 362.453333 853.333333z m299.093334-646.314666a209.066667 209.066667 0 0 0-121.514667 64.512 749.482667 749.482667 0 0 0-138.88 176.853333 694.442667 694.442667 0 0 0-89.941333 237.397333c-9.344 64.384 0.554667 110.037333 27.050666 125.269334a47.914667 47.914667 0 0 0 24.149334 5.930666 209.066667 209.066667 0 0 0 121.386666-64.512 749.269333 749.269333 0 0 0 139.050667-176.853333 694.442667 694.442667 0 0 0 89.941333-237.397333c9.344-64.384-0.554667-110.037333-27.093333-125.269334a47.914667 47.914667 0 0 0-24.149333-5.930666z" fill="currentColor"/>
                <path d="M512 457.472A54.741333 54.741333 0 0 1 566.826667 512a54.869333 54.869333 0 0 1-109.696 0A54.741333 54.741333 0 0 1 512 457.472z m0 72.533333a18.176 18.176 0 1 0-18.304-18.005333 18.261333 18.261333 0 0 0 18.304 18.005333z" fill="currentColor"/>
              </svg>
              <span class="btn-text">生成</span>
            </button>
            <div class="button-divider"></div>
            <button class="ai-settings-btn" @click="showAISettings = true" title="AI设置">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51h.15a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>
            </button>
          </div>
        </div>
        
        <div class="commit-input-wrapper">
          <textarea
            v-model="commitMessage"
            @input="handleCommitMessageInput"
            placeholder="输入提交信息 (Commit Message)..."
          ></textarea>
        </div>

        <div class="commit-actions">
          <button
            class="commit-btn primary"
            :disabled="leftButtonConfig.disabled"
            @click="handleLeftButtonClick"
            :title="leftButtonConfig.title"
          >
            {{ isCommitting ? '提交中...' : leftButtonConfig.text }}
          </button>
          <button
            class="commit-and-push-btn"
            :disabled="!commitMessage || isCommitting || stagedFiles.length === 0"
            @click="doCommitAndPush"
            title="提交后立即推送到远程仓库"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
              <polyline points="17 8 12 3 7 8"></polyline>
              <line x1="12" y1="3" x2="12" y2="15"></line>
            </svg>
            提交并推送
          </button>
        </div>
      </div>
    </div>

    <!-- AI Settings Modal -->
    <AISettingsModal
      :is-open="showAISettings"
      @close="showAISettings = false"
      @save="handleAISettingsSave"
    />
  </div>
</template>

<style scoped>
.changes-view {
  display: flex;
  height: 100%;
  overflow: hidden;
  min-width: 0;
}

.changes-panel {
  width: 100%;
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--bg-secondary);
  min-width: 0;
}

.files-section {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.section-header {
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-secondary);
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-shrink: 0;
}

.file-groups {
  padding: var(--spacing-sm);
  overflow-y: auto;
  flex: 1;
}

.group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-xs);
  padding: var(--spacing-xs);
}

.group-title {
  font-size: 0.75rem;
  color: var(--text-tertiary);
  font-weight: 600;
}

.group-actions {
  display: flex;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.2s;
}

.group-header:hover .group-actions,
.group-actions:has(.toggle-actions) {
  opacity: 1;
}

.group-action {
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  background-color: transparent;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 28px;
  min-height: 24px;
}

.group-action:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.group-action.discard-all {
  color: #ef4444;
}

.group-action.discard-all:hover {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.group-action.toggle-actions {
  color: var(--accent-color);
}

.group-action.toggle-actions:hover {
  background-color: rgba(59, 130, 246, 0.1);
  color: var(--accent-color);
}

.action-icon {
  font-size: 14px;
  font-weight: bold;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.file-list {
  list-style: none;
  margin-bottom: var(--spacing-md);
  padding: 0;
}

.file-list li {
  display: flex;
  flex-direction: column;
  padding: var(--spacing-xs) var(--spacing-sm);
  cursor: pointer;
  border-radius: var(--radius-sm);
  font-size: var(--font-size-sm);
  gap: 4px;
}

.file-main-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  width: 100%;
}

.file-list li:hover {
  background-color: var(--bg-tertiary);
}

.file-list li.selected {
  background-color: rgba(59, 130, 246, 0.1);
  color: var(--accent-color);
}

.file-action {
  padding: 4px 6px;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  background-color: transparent;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 1;
  min-width: 24px;
  min-height: 24px;
}

.file-action.discard {
  margin-left: auto;
}

.file-action .action-icon {
  font-size: 14px;
}

.file-action:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.file-action.discard {
  color: #ef4444;
}

.file-action.discard:hover {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.file-action.stage {
  color: var(--text-secondary);
}

.file-action.stage:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.status-icon {
  font-family: monospace;
  font-weight: bold;
  flex-shrink: 0;
}

.status-badge {
  font-family: monospace;
  font-weight: bold;
  font-size: 10px;
  flex-shrink: 0;
  padding: 2px 6px;
  border-radius: 3px;
  color: white;
  line-height: 1;
}

.diff-status-badge {
  font-size: 10px;
  font-weight: 600;
  padding: 2px 6px;
  border-radius: 3px;
  color: white;
  line-height: 1;
  flex-shrink: 0;
  margin-left: 4px;
}

.file-stats {
  padding-left: 28px;
  display: flex;
  align-items: center;
}

.file-path {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
}

.commit-section {
  padding: var(--spacing-md);
  background-color: var(--bg-primary);
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  min-height: 150px;
  max-height: 500px;
  overflow: hidden;
}

/* AI Button Group Container */
.ai-button-group {
  display: flex;
  align-items: center;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

/* Button Divider */
.button-divider {
  width: 1px;
  height: 24px;
  background: linear-gradient(
    to bottom,
    transparent,
    var(--border-color) 20%,
    var(--border-color) 80%,
    transparent
  );
}

/* AI Generate Button - Primary Action */
.ai-generate-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 6px 12px;
  background: linear-gradient(135deg, #8b5cf6 0%, #6366f1 100%);
  color: white;
  border: none;
  transition: all 0.2s ease;
  font-size: 0.813rem;
  font-weight: 500;
  position: relative;
  overflow: hidden;
}

.ai-generate-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%);
  opacity: 0;
  transition: opacity 0.2s ease;
}

.ai-generate-btn:hover:not(:disabled)::before {
  opacity: 1;
}

.ai-generate-btn:disabled {
  background: var(--bg-tertiary);
  color: var(--text-tertiary);
  cursor: not-allowed;
}

.ai-generate-btn:disabled::before {
  display: none;
}

.ai-generate-btn .ai-generate-icon,
.ai-generate-btn .btn-text {
  position: relative;
  z-index: 1;
}

.btn-text {
  line-height: 1;
  user-select: none;
}

/* AI Settings Button - Secondary Action */
.ai-settings-btn {
  padding: 6px 10px;
  color: var(--text-secondary);
  background-color: transparent;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.ai-settings-btn:hover {
  color: var(--text-primary);
  background-color: var(--bg-hover);
}

/* AI Generate Icon */
.ai-generate-icon {
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.ai-generate-icon.generating {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.commit-input-wrapper {
  margin: var(--spacing-sm) 0;
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

textarea {
  width: 100%;
  flex: 1;
  resize: none;
  padding: var(--spacing-sm);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-family: inherit;
  min-height: 60px;
}

textarea:focus {
  outline: 2px solid var(--accent-color);
  border-color: transparent;
}

.commit-actions {
  display: flex;
  gap: var(--spacing-sm);
  margin-top: auto;
}

.commit-btn {
  flex: 1;
  padding: var(--spacing-sm);
  background-color: var(--accent-color);
  color: white;
  border-radius: var(--radius-md);
  font-weight: 600;
  transition: background-color 0.2s;
}

.commit-btn:disabled {
  background-color: var(--bg-tertiary);
  color: var(--text-tertiary);
  cursor: not-allowed;
}

.commit-btn:not(:disabled):hover {
  background-color: var(--accent-hover);
}

.commit-and-push-btn {
  flex: 1;
  padding: var(--spacing-sm);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  font-weight: 600;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
}

.commit-and-push-btn:disabled {
  background-color: var(--bg-tertiary);
  color: var(--text-tertiary);
  cursor: not-allowed;
  opacity: 0.5;
}

.commit-and-push-btn:not(:disabled):hover {
  background-color: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.commit-and-push-btn svg {
  flex-shrink: 0;
}
</style>
