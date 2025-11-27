<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'save', settings: AISettings): void;
}>();

export interface AISettings {
  apiEndpoint: string;
  apiKey: string;
  model: string;
  systemPrompt: string;
  language: string;
  temperature: number;
  maxTokens: number;
}

const apiEndpoint = ref('https://api.openai.com/v1/chat/completions');
const apiKey = ref('');
const modelSelection = ref('gpt-4');
const customModel = ref('');
const model = ref('gpt-4');
const systemPrompt = ref('你是一个专业的Git提交信息生成助手。请基于代码变更内容，生成简洁、清晰、符合规范的提交信息。提交信息应该：1. 使用祈使句 2. 首字母小写 3. 不超过50个字符 4. 描述做了什么，而不是怎么做的');
const language = ref('zh-CN');
const temperature = ref(0.7);
const maxTokens = ref(200);

// Load saved settings
watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    loadSettings();
  }
});

function loadSettings() {
  // TODO: Load from local storage or backend
  const saved = localStorage.getItem('ai_settings');
  if (saved) {
    try {
      const settings = JSON.parse(saved);
      apiEndpoint.value = settings.apiEndpoint || apiEndpoint.value;
      apiKey.value = settings.apiKey || '';
      model.value = settings.model || model.value;

      // 检查是否是预设模型
      const presetModels = ['gpt-4', 'gpt-4-turbo', 'gpt-3.5-turbo', 'claude-3-opus', 'claude-3-sonnet'];
      if (presetModels.includes(settings.model)) {
        modelSelection.value = settings.model;
        customModel.value = '';
      } else {
        modelSelection.value = 'custom';
        customModel.value = settings.model || '';
      }

      systemPrompt.value = settings.systemPrompt || systemPrompt.value;
      language.value = settings.language || language.value;
      temperature.value = settings.temperature || temperature.value;
      maxTokens.value = settings.maxTokens || maxTokens.value;
    } catch (error) {
      console.error('Failed to load AI settings:', error);
    }
  }
}

function save() {
  // 确定最终使用的模型
  const finalModel = modelSelection.value === 'custom' ? customModel.value : modelSelection.value;

  const settings: AISettings = {
    apiEndpoint: apiEndpoint.value,
    apiKey: apiKey.value,
    model: finalModel,
    systemPrompt: systemPrompt.value,
    language: language.value,
    temperature: temperature.value,
    maxTokens: maxTokens.value,
  };

  // Save to local storage
  localStorage.setItem('ai_settings', JSON.stringify(settings));

  emit('save', settings);
  emit('close');
}
</script>

<template>
  <div v-if="isOpen" class="modal-overlay">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>AI 设置</h3>
        <button class="close-btn" @click="emit('close')">×</button>
      </div>

      <div class="modal-body">
        <div class="form-group">
          <h4>API 配置</h4>

          <div class="input-group">
            <label>API 端点</label>
            <input v-model="apiEndpoint" type="text" placeholder="https://api.openai.com/v1/chat/completions">
            <p class="hint">支持 OpenAI API 或兼容的端点 (如 Azure OpenAI, 本地部署等)</p>
          </div>

          <div class="input-group">
            <label>API Key</label>
            <input v-model="apiKey" type="password" placeholder="sk-...">
          </div>

          <div class="input-group">
            <label>模型</label>
            <select v-model="modelSelection">
              <option value="gpt-4">GPT-4</option>
              <option value="gpt-4-turbo">GPT-4 Turbo</option>
              <option value="gpt-3.5-turbo">GPT-3.5 Turbo</option>
              <option value="claude-3-opus">Claude 3 Opus</option>
              <option value="claude-3-sonnet">Claude 3 Sonnet</option>
              <option value="custom">自定义</option>
            </select>
          </div>

          <div v-if="modelSelection === 'custom'" class="input-group">
            <label>自定义模型名称</label>
            <input v-model="customModel" type="text" placeholder="your-model-name">
          </div>
        </div>

        <div class="divider"></div>

        <div class="form-group">
          <h4>生成配置</h4>

          <div class="input-group">
            <label>系统提示词</label>
            <textarea
              v-model="systemPrompt"
              rows="5"
              placeholder="定义 AI 如何生成提交信息..."
            ></textarea>
            <p class="hint">定义 AI 生成提交信息的规则和风格</p>
          </div>

          <div class="input-group">
            <label>默认语言</label>
            <select v-model="language">
              <option value="zh-CN">简体中文</option>
              <option value="zh-TW">繁体中文</option>
              <option value="en">English</option>
              <option value="ja">日本語</option>
              <option value="ko">한국어</option>
            </select>
          </div>

          <div class="input-group">
            <label>Temperature ({{ temperature }})</label>
            <input
              v-model.number="temperature"
              type="range"
              min="0"
              max="2"
              step="0.1"
            >
            <p class="hint">控制生成的随机性。较低值更确定性，较高值更随机</p>
          </div>

          <div class="input-group">
            <label>最大令牌数</label>
            <input
              v-model.number="maxTokens"
              type="number"
              min="50"
              max="1000"
              step="50"
            >
            <p class="hint">限制生成的提交信息长度</p>
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
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  margin: var(--spacing-md);
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

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.form-group h4 {
  margin-bottom: var(--spacing-xs);
  font-size: var(--font-size-base);
  color: var(--text-primary);
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.input-group label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
}

input[type="text"],
input[type="password"],
input[type="number"],
textarea,
select {
  padding: var(--spacing-sm);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  width: 100%;
  font-family: inherit;
}

textarea {
  resize: vertical;
  font-size: var(--font-size-sm);
  line-height: 1.5;
}

input[type="range"] {
  padding: 0;
}

input:focus,
textarea:focus,
select:focus {
  outline: 2px solid var(--accent-color);
  border-color: transparent;
}

.hint {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  margin-top: 2px;
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
</style>
