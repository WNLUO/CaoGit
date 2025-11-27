<script setup lang="ts">
import { ref, watch } from 'vue';
import { GitApi } from '../services/gitApi';
import { debugStore } from '../stores/debugStore';

const props = defineProps<{
  isOpen: boolean;
  initialPath: string;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'success', path: string): void;
}>();

// Form data
const repoName = ref('');
const description = ref('');
const createReadme = ref(false);
const gitignoreTemplate = ref('none');
const defaultBranch = ref('main');

const isProcessing = ref(false);

// Common .gitignore templates
const gitignoreTemplates = [
  { value: 'none', label: '无' },
  { value: 'node', label: 'Node.js' },
  { value: 'python', label: 'Python' },
  { value: 'java', label: 'Java' },
  { value: 'go', label: 'Go' },
  { value: 'rust', label: 'Rust' },
  { value: 'cpp', label: 'C++' },
  { value: 'csharp', label: 'C#' },
  { value: 'ruby', label: 'Ruby' },
  { value: 'php', label: 'PHP' },
  { value: 'swift', label: 'Swift' },
  { value: 'kotlin', label: 'Kotlin' },
];

async function handleInit() {
  isProcessing.value = true;

  try {
    // Initialize the repository
    const response = await GitApi.initRepository(
      props.initialPath,
      defaultBranch.value
    );

    if (!response.success) {
      throw new Error(response.error || '初始化失败');
    }

    // TODO: Create initial files if requested
    // - Create README.md if createReadme is true
    // - Create .gitignore from template if selected
    // - Add repository name and description to a config file

    emit('success', props.initialPath);
    emit('close');
    resetForm();
  } catch (error: any) {
    debugStore.logError('初始化仓库失败', error, 'InitRepoModal.handleInit');
  } finally {
    isProcessing.value = false;
  }
}

function resetForm() {
  repoName.value = '';
  description.value = '';
  createReadme.value = false;
  gitignoreTemplate.value = 'none';
  defaultBranch.value = 'main';
}

function handleClose() {
  resetForm();
  emit('close');
}

// Auto-detect gitignore template based on directory files
async function detectGitignoreTemplate(path: string): Promise<void> {
  try {
    const response = await GitApi.detectProjectType(path);
    if (response.success && response.data) {
      gitignoreTemplate.value = response.data;
      debugStore.logInfo(
        `检测到项目类型: ${response.data}`,
        'InitRepoModal.detectGitignoreTemplate'
      );
    }
  } catch (error) {
    debugStore.logError(
      '项目类型检测失败',
      error as Error,
      'InitRepoModal.detectGitignoreTemplate'
    );
    // Fallback to 'none' if detection fails
    gitignoreTemplate.value = 'none';
  }
}

// Watch for initialPath changes and auto-detect template
watch(() => props.initialPath, async (newPath) => {
  if (newPath) {
    // Detect project type from files
    await detectGitignoreTemplate(newPath);

    // Also extract and suggest repo name
    const pathParts = newPath.split('/').filter(Boolean);
    const suggestedName = pathParts[pathParts.length - 1] || '';
    if (suggestedName && !repoName.value) {
      repoName.value = suggestedName;
    }
  }
}, { immediate: true });
</script>

<template>
  <div v-if="isOpen" class="modal-overlay" @click="handleClose">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>初始化 Git 仓库</h3>
        <button class="close-btn" @click="handleClose">×</button>
      </div>

      <div class="modal-body">
        <div class="path-display">
          <label>仓库路径</label>
          <div class="path">{{ initialPath }}</div>
        </div>

        <div class="form-section">
          <div class="input-group">
            <label>仓库名称（可选）</label>
            <input
              v-model="repoName"
              type="text"
              placeholder="my-project"
            >
            <span class="hint">用于标识此仓库，不会影响文件夹名称</span>
          </div>

          <div class="input-group">
            <label>仓库描述（可选）</label>
            <textarea
              v-model="description"
              placeholder="这个项目是关于..."
              rows="3"
            ></textarea>
          </div>

          <div class="divider"></div>

          <div class="input-group">
            <label class="checkbox-label">
              <input
                v-model="createReadme"
                type="checkbox"
              >
              <span>创建 README.md 文件</span>
            </label>
            <span class="hint">建议创建 README 文件来描述项目</span>
          </div>

          <div class="input-group">
            <label>选择 .gitignore 模板</label>
            <select v-model="gitignoreTemplate">
              <option
                v-for="template in gitignoreTemplates"
                :key="template.value"
                :value="template.value"
              >
                {{ template.label }}
              </option>
            </select>
            <span class="hint">.gitignore 用于忽略不需要版本控制的文件</span>
          </div>

          <div class="input-group">
            <label>默认分支名称</label>
            <div class="branch-options">
              <label class="radio-label">
                <input
                  v-model="defaultBranch"
                  type="radio"
                  value="main"
                >
                <span>main</span>
              </label>
              <label class="radio-label">
                <input
                  v-model="defaultBranch"
                  type="radio"
                  value="master"
                >
                <span>master</span>
              </label>
              <label class="radio-label">
                <input
                  v-model="defaultBranch"
                  type="radio"
                  value="custom"
                >
                <span>自定义</span>
              </label>
            </div>
            <input
              v-if="defaultBranch === 'custom'"
              v-model="defaultBranch"
              type="text"
              placeholder="输入分支名称"
              class="custom-branch-input"
            >
            <span class="hint">GitHub 和 GitLab 现在默认使用 "main" 作为主分支名称</span>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-cancel" @click="handleClose">取消</button>
        <button
          class="btn-save"
          @click="handleInit"
          :disabled="isProcessing"
        >
          {{ isProcessing ? '初始化中...' : '初始化仓库' }}
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
  max-height: 85vh;
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

.path-display {
  margin-bottom: var(--spacing-lg);
  padding: var(--spacing-md);
  background-color: var(--bg-secondary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
}

.path-display label {
  display: block;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.path {
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  word-break: break-all;
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

.input-group > label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: 500;
}

.hint {
  font-size: 11px;
  color: var(--text-secondary);
  opacity: 0.8;
}

input[type="text"],
textarea,
select {
  padding: var(--spacing-sm);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  width: 100%;
  font-size: var(--font-size-sm);
}

textarea {
  resize: vertical;
  font-family: inherit;
}

input:focus,
textarea:focus,
select:focus {
  outline: 2px solid var(--accent-color);
  border-color: transparent;
}

.checkbox-label,
.radio-label {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
}

.checkbox-label input[type="checkbox"],
.radio-label input[type="radio"] {
  width: auto;
  cursor: pointer;
}

.branch-options {
  display: flex;
  gap: var(--spacing-md);
  padding: var(--spacing-sm) 0;
}

.custom-branch-input {
  margin-top: var(--spacing-xs);
}

.divider {
  height: 1px;
  background-color: var(--border-color);
  margin: var(--spacing-sm) 0;
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
