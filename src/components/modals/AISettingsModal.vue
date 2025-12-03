<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { saveToKeychain, getFromKeychain, KeychainAccounts } from '../../services/keychainApi';
import { toastStore } from '../../stores/toastStore';

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
const maxTokens = ref(300);

// Load saved settings
watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    loadSettings();
  }
});

// Auto-migrate from localStorage on component mount
onMounted(async () => {
  await autoMigrateFromLocalStorage();
});

async function loadSettings() {
  try {
    // Load non-sensitive settings from localStorage
    const saved = localStorage.getItem('ai_settings_v2');
    if (saved) {
      const settings = JSON.parse(saved);
      apiEndpoint.value = settings.apiEndpoint || apiEndpoint.value;
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
    }

    // Load API key from Keychain (secure storage)
    const keyResponse = await getFromKeychain(KeychainAccounts.AI_API_KEY);
    if (keyResponse.success && keyResponse.data) {
      apiKey.value = keyResponse.data;
    }
  } catch (error) {
    console.error('Failed to load AI settings:', error);
    toastStore.error('加载 AI 设置失败');
  }
}

/**
 * Auto-migrate API key from old localStorage to Keychain
 */
async function autoMigrateFromLocalStorage() {
  try {
    const oldSettings = localStorage.getItem('ai_settings');
    if (oldSettings) {
      const settings = JSON.parse(oldSettings);
      if (settings.apiKey) {
        // Migrate API key to Keychain
        const migrateResponse = await saveToKeychain(KeychainAccounts.AI_API_KEY, settings.apiKey);
        if (migrateResponse.success) {
          // Remove API key from old settings
          delete settings.apiKey;
          // Save migrated settings under new key
          localStorage.setItem('ai_settings_v2', JSON.stringify(settings));
          // Remove old settings
          localStorage.removeItem('ai_settings');
          console.log('Successfully migrated API key to Keychain');
        }
      }
    }
  } catch (error) {
    console.error('Failed to migrate AI settings:', error);
  }
}

async function save() {
  try {
    // 确定最终使用的模型
    const finalModel = modelSelection.value === 'custom' ? customModel.value : modelSelection.value;

    // Save API key to Keychain (secure storage)
    if (apiKey.value) {
      const keyResponse = await saveToKeychain(KeychainAccounts.AI_API_KEY, apiKey.value);
      if (!keyResponse.success) {
        toastStore.error('保存 API Key 失败：' + (keyResponse.error || '未知错误'));
        return;
      }
    }

    // Save non-sensitive settings to localStorage
    const settings = {
      apiEndpoint: apiEndpoint.value,
      model: finalModel,
      systemPrompt: systemPrompt.value,
      language: language.value,
      temperature: temperature.value,
      maxTokens: maxTokens.value,
    };

    localStorage.setItem('ai_settings_v2', JSON.stringify(settings));

    // Emit with full settings (including API key for immediate use)
    const fullSettings: AISettings = {
      ...settings,
      apiKey: apiKey.value,
    };

    toastStore.success('AI 设置已保存');
    emit('save', fullSettings);
    emit('close');
  } catch (error) {
    console.error('Failed to save AI settings:', error);
    toastStore.error('保存设置失败');
  }
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
        <!-- Privacy Warning -->
        <div class="privacy-warning">
          <div class="warning-icon">⚠️</div>
          <div class="warning-content">
            <h4>隐私提示</h4>
            <p>
              当您使用 AI 功能生成提交信息时，您的<strong>代码变更内容</strong>将被发送到您配置的 AI 服务（OpenAI、Claude 等）进行处理。
            </p>
            <p>
              请确保：
            </p>
            <ul>
              <li>您有权共享这些代码</li>
              <li>代码中不包含敏感信息（密钥、密码、商业机密等）</li>
              <li>您了解并同意所选 AI 服务的隐私政策</li>
            </ul>
            <p class="warning-note">
              💡 您的 API Key 将安全存储在 macOS 钥匙串中，而不是明文存储。
            </p>
          </div>
        </div>

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

.privacy-warning {
  display: flex;
  gap: var(--spacing-md);
  padding: var(--spacing-md);
  background-color: rgba(251, 146, 60, 0.1);
  border: 1px solid rgba(251, 146, 60, 0.3);
  border-radius: var(--radius-md);
  margin-bottom: var(--spacing-lg);
}

.warning-icon {
  font-size: 24px;
  flex-shrink: 0;
}

.warning-content {
  flex: 1;
}

.warning-content h4 {
  margin: 0 0 var(--spacing-xs) 0;
  font-size: var(--font-size-base);
  color: var(--text-primary);
  font-weight: 600;
}

.warning-content p {
  margin: 0 0 var(--spacing-sm) 0;
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  line-height: 1.5;
}

.warning-content ul {
  margin: var(--spacing-sm) 0;
  padding-left: var(--spacing-lg);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
}

.warning-content li {
  margin-bottom: var(--spacing-xs);
  line-height: 1.5;
}

.warning-note {
  margin-top: var(--spacing-sm);
  padding: var(--spacing-sm);
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
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
