<script setup lang="ts">
import { ref, computed } from 'vue';
import { repoStore } from '../stores/repoStore';
import type { FileChange } from '../types';
import AISettingsModal, { type AISettings } from './AISettingsModal.vue';
import FileIcon from './FileIcon.vue';
import { GitApi } from '../services/gitApi';

const commitMessage = ref('');
const isCommitting = ref(false);
const isGeneratingAI = ref(false);
const showAISettings = ref(false);

const stagedFiles = computed(() => repoStore.fileChanges.filter(f => f.staged));
const unstagedFiles = computed(() => repoStore.fileChanges.filter(f => !f.staged));

async function toggleStage(file: FileChange) {
  try {
    if (file.staged) {
      await repoStore.unstageFile(file.path);
    } else {
      await repoStore.stageFile(file.path);
    }
  } catch (error: any) {
    alert('操作失败: ' + error.message);
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
    alert('暂存失败: ' + error.message);
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
    alert('取消暂存失败: ' + error.message);
  }
}

function selectFile(file: FileChange) {
  repoStore.selectedFile = file;
}

async function doCommit() {
  if (!commitMessage.value.trim()) {
    alert('请输入提交信息');
    return;
  }

  if (!repoStore.activeRepo) {
    alert('请先打开一个仓库');
    return;
  }

  // 如果没有暂存的文件，自动暂存所有变更
  if (stagedFiles.value.length === 0 && unstagedFiles.value.length > 0) {
    try {
      // 暂存所有文件
      await Promise.all(
        unstagedFiles.value.map(file => repoStore.stageFile(file.path))
      );
    } catch (error: any) {
      alert('自动暂存文件失败: ' + error.message);
      return;
    }
  }

  // 再次检查是否有暂存的文件
  if (stagedFiles.value.length === 0) {
    alert('没有文件需要提交');
    return;
  }

  isCommitting.value = true;
  try {
    await repoStore.commit(commitMessage.value);
    commitMessage.value = '';
    alert('提交成功!');
  } catch (error: any) {
    alert('提交失败: ' + error.message);
  } finally {
    isCommitting.value = false;
  }
}

async function generateAICommitMessage() {
  if (!repoStore.activeRepo) {
    alert('请先打开一个仓库');
    return;
  }

  // 检查是否有变更的文件（暂存或未暂存）
  if (repoStore.fileChanges.length === 0) {
    alert('没有文件变更，无需生成提交信息');
    return;
  }

  // Load AI settings
  const savedSettings = localStorage.getItem('ai_settings');
  if (!savedSettings) {
    alert('请先配置 AI 设置');
    showAISettings.value = true;
    return;
  }

  isGeneratingAI.value = true;

  try {
    const settings: AISettings = JSON.parse(savedSettings);

    if (!settings.apiKey) {
      alert('请先配置 API Key');
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
              // 只包含变更行，限制长度
              const changes = hunk.lines
                .filter(line => line.origin === '+' || line.origin === '-')
                .slice(0, 20) // 限制每个 hunk 最多 20 行
                .map(line => `${line.origin}${line.content}`)
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

    const diffContext = diffs.join('\n---\n').slice(0, 8000); // 限制总长度

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
          content: `请基于以下代码变更生成简洁的提交信息（${settings.language}）：\n\n${diffContext}`
        }
      ],
      settings.temperature,
      settings.maxTokens
    );

    if (!response.success || !response.data) {
      throw new Error(response.error || 'AI API 调用失败');
    }

    const generatedMessage = response.data.trim();

    if (generatedMessage) {
      commitMessage.value = generatedMessage;
    } else {
      throw new Error('AI 返回了空的提交信息');
    }

  } catch (error: any) {
    console.error('AI 生成失败:', error);
    commitMessage.value = '';
    alert('AI 生成失败: ' + error.message);
  } finally {
    isGeneratingAI.value = false;
  }
}

function handleAISettingsSave(settings: AISettings) {
  console.log('AI settings saved:', settings);
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
                <FileIcon :fileName="file.path.split('/').pop() || file.path" />
                <span class="file-path">{{ file.path }}</span>
                <span class="status-badge" :style="{ backgroundColor: getStatusColor(file.status) }">
                  {{ getStatusIcon(file.status) }}
                </span>
                <button class="file-action" @click.stop="toggleStage(file)" title="取消暂存">
                  <span class="action-icon">−</span>
                </button>
              </li>
            </ul>
          </div>

          <!-- Unstaged -->
          <div v-if="unstagedFiles.length > 0" class="file-group">
            <div class="group-header">
              <span class="group-title">工作区 ({{ unstagedFiles.length }})</span>
              <button class="group-action" @click="stageAll" title="暂存所有文件">
                <span class="action-icon">+</span>
              </button>
            </div>
            <ul class="file-list">
              <li
                v-for="file in unstagedFiles"
                :key="file.path"
                :class="{ selected: repoStore.selectedFile?.path === file.path }"
                @click="selectFile(file)"
              >
                <FileIcon :fileName="file.path.split('/').pop() || file.path" />
                <span class="file-path">{{ file.path }}</span>
                <span class="status-badge" :style="{ backgroundColor: getStatusColor(file.status) }">
                  {{ getStatusIcon(file.status) }}
                </span>
                <button class="file-action" @click.stop="toggleStage(file)" title="暂存">
                  <span class="action-icon">+</span>
                </button>
              </li>
            </ul>
          </div>
        </div>
      </div>

      <!-- Commit Section -->
      <div class="commit-section">
        <div class="section-header">
          <span>提交信息</span>
          <div class="ai-actions">
            <button
              class="ai-btn"
              :disabled="isGeneratingAI || repoStore.fileChanges.length === 0"
              @click="generateAICommitMessage"
            >
              {{ isGeneratingAI ? '生成中...' : 'AI生成' }}
            </button>
            <button class="ai-settings-btn" @click="showAISettings = true" title="AI设置">
              <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51h.15a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>
            </button>
          </div>
        </div>
        
        <div class="commit-input-wrapper">
          <textarea 
            v-model="commitMessage" 
            placeholder="输入提交信息 (Commit Message)..."
          ></textarea>
        </div>

        <button
          class="commit-btn"
          :disabled="!commitMessage || isCommitting || repoStore.fileChanges.length === 0"
          @click="doCommit"
        >
          {{ isCommitting ? '提交中...' : (stagedFiles.length === 0 ? '暂存并提交至 ' : '提交至 ') + repoStore.currentBranch }}
        </button>
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

.group-action {
  opacity: 0;
  padding: 2px 8px;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  background-color: transparent;
  transition: all 0.2s;
}

.group-header:hover .group-action {
  opacity: 1;
}

.group-action:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.action-icon {
  font-size: 16px;
  font-weight: bold;
  line-height: 1;
}

.file-list {
  list-style: none;
  margin-bottom: var(--spacing-md);
  padding: 0;
}

.file-list li {
  display: flex;
  align-items: center;
  padding: var(--spacing-xs) var(--spacing-sm);
  cursor: pointer;
  border-radius: var(--radius-sm);
  font-size: var(--font-size-sm);
  gap: var(--spacing-sm);
}

.file-list li:hover {
  background-color: var(--bg-tertiary);
}

.file-list li.selected {
  background-color: rgba(59, 130, 246, 0.1);
  color: var(--accent-color);
}

.file-action {
  opacity: 0;
  padding: 2px 6px;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  background-color: transparent;
  transition: all 0.2s;
  margin-left: var(--spacing-xs);
}

.file-list li:hover .file-action {
  opacity: 1;
}

.file-action:hover {
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
  margin-left: auto;
  padding: 2px 6px;
  border-radius: 3px;
  color: white;
  line-height: 1;
}

.file-path {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
}

.commit-section {
  padding: var(--spacing-md);
  border-top: 1px solid var(--border-color);
  background-color: var(--bg-primary);
  flex-shrink: 0;
}

.ai-actions {
  display: flex;
  gap: var(--spacing-xs);
  align-items: center;
}

.ai-btn {
  font-size: 0.75rem;
  color: #8b5cf6;
  background-color: #f3e8ff;
  padding: 2px 8px;
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
}

.ai-btn:hover:not(:disabled) {
  background-color: #e9d5ff;
}

.ai-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.ai-settings-btn {
  padding: 4px;
  color: var(--text-tertiary);
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
}

.ai-settings-btn:hover {
  color: var(--text-primary);
  background-color: var(--bg-hover);
}

.commit-input-wrapper {
  margin: var(--spacing-sm) 0;
}

textarea {
  width: 100%;
  height: 100px;
  resize: none;
  padding: var(--spacing-sm);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  font-family: inherit;
}

textarea:focus {
  outline: 2px solid var(--accent-color);
  border-color: transparent;
}

.commit-btn {
  width: 100%;
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
</style>
