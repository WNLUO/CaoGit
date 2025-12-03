<script setup lang="ts">
import { saveToKeychain, KeychainAccounts } from '../../services/keychainApi';

defineProps<{
  show: boolean;
}>();

const emit = defineEmits<{
  (e: 'accept'): void;
  (e: 'decline'): void;
}>();

async function handleAccept() {
  try {
    // Save consent to Keychain
    await saveToKeychain(KeychainAccounts.AI_CONSENT, 'accepted');
    emit('accept');
  } catch (error) {
    console.error('Failed to save AI consent:', error);
    // Still emit accept if saving fails - don't block the user
    emit('accept');
  }
}

function handleDecline() {
  emit('decline');
}
</script>

<template>
  <div v-if="show" class="consent-overlay" @click.self="handleDecline">
    <div class="consent-dialog">
      <div class="consent-header">
        <div class="header-icon">🤖</div>
        <h2>AI 功能使用协议</h2>
      </div>

      <div class="consent-body">
        <div class="consent-section">
          <h3>📤 数据传输说明</h3>
          <p>
            当您使用 AI 功能生成提交信息时，<strong>您的代码变更内容</strong>将被发送到您配置的第三方 AI 服务进行处理。
          </p>
        </div>

        <div class="consent-section">
          <h3>🔒 隐私和安全</h3>
          <ul>
            <li>您的代码变更将发送到：OpenAI、Anthropic Claude 或您自定义的 AI 服务</li>
            <li>这些服务由第三方公司运营，不受我们控制</li>
            <li>您的 API Key 将安全存储在 macOS 钥匙串中</li>
            <li>我们不会收集或存储您的代码内容</li>
          </ul>
        </div>

        <div class="consent-section">
          <h3>⚠️ 您需要确保</h3>
          <ul>
            <li>您有权共享这些代码（不违反公司政策或保密协议）</li>
            <li>代码中不包含敏感信息（API 密钥、密码、商业机密等）</li>
            <li>您了解并同意所选 AI 服务的隐私政策</li>
          </ul>
        </div>

        <div class="consent-section important">
          <h3>📜 第三方服务隐私政策</h3>
          <ul>
            <li>
              <a href="https://openai.com/policies/privacy-policy" target="_blank" rel="noopener">
                OpenAI 隐私政策 ↗
              </a>
            </li>
            <li>
              <a href="https://www.anthropic.com/privacy" target="_blank" rel="noopener">
                Anthropic (Claude) 隐私政策 ↗
              </a>
            </li>
          </ul>
        </div>

        <div class="consent-note">
          💡 提示：您可以随时在设置中禁用 AI 功能，或删除存储的 API Key。
        </div>
      </div>

      <div class="consent-footer">
        <button class="btn-decline" @click="handleDecline">
          我不同意
        </button>
        <button class="btn-accept" @click="handleAccept">
          我已阅读并同意
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.consent-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.consent-dialog {
  background-color: var(--bg-primary);
  border-radius: var(--radius-lg);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  animation: slideUp 0.3s ease;
}

@keyframes slideUp {
  from {
    transform: translateY(20px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.consent-header {
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);
  text-align: center;
}

.header-icon {
  font-size: 48px;
  margin-bottom: var(--spacing-sm);
}

.consent-header h2 {
  margin: 0;
  font-size: var(--font-size-xl);
  font-weight: 600;
  color: var(--text-primary);
}

.consent-body {
  padding: var(--spacing-lg);
  overflow-y: auto;
  flex: 1;
}

.consent-section {
  margin-bottom: var(--spacing-lg);
}

.consent-section h3 {
  margin: 0 0 var(--spacing-sm) 0;
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--text-primary);
}

.consent-section p {
  margin: 0 0 var(--spacing-sm) 0;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.6;
}

.consent-section ul {
  margin: 0;
  padding-left: var(--spacing-lg);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.consent-section li {
  margin-bottom: var(--spacing-xs);
  line-height: 1.6;
}

.consent-section.important {
  background-color: rgba(59, 130, 246, 0.1);
  padding: var(--spacing-md);
  border-radius: var(--radius-md);
  border-left: 4px solid var(--accent-color);
}

.consent-section a {
  color: var(--accent-color);
  text-decoration: none;
  transition: color var(--transition-fast);
}

.consent-section a:hover {
  color: var(--accent-hover);
  text-decoration: underline;
}

.consent-note {
  padding: var(--spacing-md);
  background-color: rgba(251, 146, 60, 0.1);
  border: 1px solid rgba(251, 146, 60, 0.3);
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.5;
}

.consent-footer {
  padding: var(--spacing-lg);
  border-top: 1px solid var(--border-color);
  display: flex;
  gap: var(--spacing-md);
  justify-content: flex-end;
}

.btn-decline,
.btn-accept {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  font-weight: 600;
  cursor: pointer;
  transition: all var(--transition-fast);
  border: none;
}

.btn-decline {
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
}

.btn-decline:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.btn-accept {
  background-color: var(--accent-color);
  color: white;
  box-shadow: 0 2px 4px rgba(59, 130, 246, 0.3);
}

.btn-accept:hover {
  background-color: var(--accent-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(59, 130, 246, 0.4);
}

.btn-accept:active {
  transform: translateY(0);
}
</style>
