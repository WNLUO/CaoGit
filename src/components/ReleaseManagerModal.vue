<template>
  <div v-if="show" class="modal-overlay" @click.self="close">
    <div class="modal-container">
      <div class="modal-header">
        <h2>发布管理</h2>
        <button class="close-btn" @click="close">×</button>
      </div>

      <div class="modal-body">
        <!-- Loading State -->
        <div v-if="loading" class="loading">
          <div class="spinner"></div>
          <p>加载中...</p>
        </div>

        <!-- Error State -->
        <div v-else-if="error" class="error-box">
          <p>{{ error }}</p>
          <button @click="loadReleaseInfo">重试</button>
        </div>

        <!-- Main Content -->
        <div v-else-if="releaseInfo" class="content">
          <!-- Publish Section -->
          <div class="section">
            <h3>发布新版本</h3>
            <div class="publish-form">
              <div class="form-group">
                <label>当前版本:</label>
                <span class="current-version">{{ releaseInfo.current_version }}</span>
              </div>

              <div class="form-group">
                <label>新版本号:</label>
                <div class="version-input">
                  <input v-model="newVersion" type="text" placeholder="v0.2.2" />
                  <button @click="incrementPatch">+Patch</button>
                  <button @click="incrementMinor">+Minor</button>
                  <button @click="incrementMajor">+Major</button>
                </div>
              </div>

              <div class="form-group">
                <label>发布说明:</label>
                <textarea v-model="releaseMessage" rows="3" placeholder="Release notes..."></textarea>
              </div>

              <button class="publish-btn" :disabled="publishing" @click="publishRelease">
                <span v-if="publishing">发布中...</span>
                <span v-else>发布到 GitHub</span>
              </button>
            </div>
          </div>

          <!-- Releases List -->
          <div class="section">
            <h3>最近发布 ({{ releaseInfo.releases.length }})</h3>
            <div class="releases-list">
              <div v-for="release in releaseInfo.releases.slice(0, 5)" :key="release.id" class="release-item">
                <div class="release-header">
                  <span class="release-tag">{{ release.tag_name }}</span>
                  <span class="release-date">{{ formatDate(release.created_at) }}</span>
                </div>
                <div class="release-body">
                  <p>{{ release.name }}</p>
                  <div class="release-assets">
                    <span>
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
                      </svg>
                      {{ release.assets.length }} 个文件
                    </span>
                    <span>
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                        <polyline points="7 10 12 15 17 10"></polyline>
                        <line x1="12" y1="15" x2="12" y2="3"></line>
                      </svg>
                      {{ totalDownloads(release.assets) }} 次下载
                    </span>
                  </div>
                </div>
                <a :href="release.html_url" target="_blank" class="view-link">查看详情 →</a>
              </div>
            </div>
          </div>

          <!-- Workflow Runs -->
          <div class="section">
            <div class="section-header">
              <h3>构建状态</h3>
              <button class="refresh-btn" @click="loadReleaseInfo" :disabled="loading" title="刷新构建状态">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" :class="{ spinning: loading }">
                  <polyline points="23 4 23 10 17 10"></polyline>
                  <polyline points="1 20 1 14 7 14"></polyline>
                  <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
                </svg>
              </button>
            </div>
            <div class="workflows-list">
              <div v-for="run in releaseInfo.workflow_runs.slice(0, 5)" :key="run.id" class="workflow-item">
                <div class="workflow-status" :class="getStatusClass(run.status, run.conclusion)">
                  <!-- Success -->
                  <svg v-if="run.conclusion === 'success'" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                    <polyline points="22 4 12 14.01 9 11.01"></polyline>
                  </svg>
                  <!-- Failure -->
                  <svg v-else-if="run.conclusion === 'failure'" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="10"></circle>
                    <line x1="15" y1="9" x2="9" y2="15"></line>
                    <line x1="9" y1="9" x2="15" y2="15"></line>
                  </svg>
                  <!-- Running -->
                  <svg v-else-if="run.status === 'in_progress'" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="spinning">
                    <circle cx="12" cy="12" r="10"></circle>
                    <path d="M12 6v6l4 2"></path>
                  </svg>
                  <!-- Pending/Other -->
                  <svg v-else width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="10"></circle>
                    <line x1="12" y1="8" x2="12" y2="12"></line>
                    <line x1="12" y1="16" x2="12.01" y2="16"></line>
                  </svg>
                </div>
                <div class="workflow-info">
                  <div class="workflow-name">{{ run.name }}</div>
                  <div class="workflow-meta">
                    <span>{{ formatDate(run.created_at) }}</span>
                    <span v-if="run.conclusion">{{ run.conclusion }}</span>
                  </div>
                </div>
                <div class="workflow-actions">
                  <a :href="run.html_url" target="_blank" class="view-link-small">查看</a>
                  <button v-if="run.conclusion === 'failure'" @click="rerunWorkflow(run.id)" class="rerun-btn">
                    重新运行
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { openUrl } from '@tauri-apps/plugin-opener'

const props = defineProps<{
  show: boolean
  repoPath: string | null
  githubToken: string | null
}>()

const emit = defineEmits<{
  close: []
  success: [message: string]
}>()

const loading = ref(false)
const error = ref<string | null>(null)
const releaseInfo = ref<any>(null)
const newVersion = ref('')
const releaseMessage = ref('')
const publishing = ref(false)

// Watch for modal open
watch(() => props.show, (show) => {
  if (show && props.repoPath) {
    loadReleaseInfo()
  }
})

async function loadReleaseInfo() {
  if (!props.repoPath) return

  loading.value = true
  error.value = null

  try {
    releaseInfo.value = await invoke('get_release_info', {
      repoPath: props.repoPath,
      githubToken: props.githubToken || undefined
    })

    // Set default new version
    newVersion.value = releaseInfo.value.current_version
  } catch (e: any) {
    error.value = e.toString()
  } finally {
    loading.value = false
  }
}

async function publishRelease() {
  if (!props.repoPath || !newVersion.value) return

  publishing.value = true
  try {
    const actionsUrl = await invoke('publish_new_release', {
      repoPath: props.repoPath,
      config: {
        version: newVersion.value,
        message: releaseMessage.value || `Release ${newVersion.value}`,
        createTag: true,
        pushTag: true
      },
      githubToken: props.githubToken || undefined
    })

    emit('success', `发布成功！构建已触发`)

    // Open Actions page
    if (actionsUrl) {
      await openUrl(actionsUrl as string)
    }

    // Reload release info
    await loadReleaseInfo()
  } catch (e: any) {
    error.value = e.toString()
  } finally {
    publishing.value = false
  }
}

async function incrementVersion(part: string) {
  try {
    newVersion.value = await invoke('increment_version', {
      version: newVersion.value || releaseInfo.value.current_version,
      part
    })
  } catch (e) {
    console.error('Failed to increment version:', e)
  }
}

function incrementPatch() { incrementVersion('patch') }
function incrementMinor() { incrementVersion('minor') }
function incrementMajor() { incrementVersion('major') }

async function rerunWorkflow(runId: number) {
  if (!props.githubToken) {
    error.value = '需要 GitHub Token 才能重新运行构建'
    return
  }

  try {
    await invoke('rerun_failed_build', {
      repoPath: props.repoPath,
      runId,
      githubToken: props.githubToken
    })

    emit('success', '重新触发构建成功')
    await loadReleaseInfo()
  } catch (e: any) {
    error.value = e.toString()
  }
}

function formatDate(dateStr: string) {
  const date = new Date(dateStr)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)

  if (diffMins < 60) return `${diffMins} 分钟前`
  if (diffMins < 1440) return `${Math.floor(diffMins / 60)} 小时前`
  return `${Math.floor(diffMins / 1440)} 天前`
}

function totalDownloads(assets: any[]) {
  return assets.reduce((sum, asset) => sum + asset.download_count, 0)
}

function getStatusClass(status: string, conclusion: string | null) {
  if (status === 'in_progress') return 'status-running'
  if (conclusion === 'success') return 'status-success'
  if (conclusion === 'failure') return 'status-failure'
  return 'status-pending'
}

function close() {
  emit('close')
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-container {
  background: var(--bg-primary);
  border-radius: 12px;
  width: 90%;
  max-width: 900px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.modal-header {
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.modal-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 28px;
  cursor: pointer;
  color: var(--text-secondary);
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
}

.close-btn:hover {
  background: var(--bg-secondary);
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--border-color);
  border-top-color: var(--accent-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-box {
  padding: 20px;
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.3);
  border-radius: 8px;
  color: #ef4444;
  text-align: center;
}

.section {
  margin-bottom: 32px;
}

.section h3 {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.refresh-btn {
  padding: 6px;
  border-radius: 6px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.refresh-btn:hover:not(:disabled) {
  background: var(--bg-hover);
  border-color: var(--accent-color);
}

.refresh-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.publish-form {
  background: var(--bg-secondary);
  padding: 20px;
  border-radius: 8px;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary);
}

.current-version {
  display: inline-block;
  padding: 6px 12px;
  background: var(--accent-color);
  color: white;
  border-radius: 6px;
  font-weight: 600;
}

.version-input {
  display: flex;
  gap: 8px;
}

.version-input input {
  flex: 1;
  padding: 8px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 14px;
}

.version-input button {
  padding: 8px 16px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.version-input button:hover {
  border-color: var(--accent-color);
  color: var(--accent-color);
}

textarea {
  width: 100%;
  padding: 8px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 14px;
  resize: vertical;
  font-family: inherit;
}

.publish-btn {
  width: 100%;
  padding: 12px;
  background: var(--accent-color);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.2s;
}

.publish-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.publish-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.releases-list, .workflows-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.release-item, .workflow-item {
  padding: 16px;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.release-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.release-tag {
  font-weight: 600;
  color: var(--accent-color);
}

.release-date {
  font-size: 13px;
  color: var(--text-secondary);
}

.release-body p {
  margin: 8px 0;
  color: var(--text-primary);
  font-size: 14px;
}

.release-assets {
  display: flex;
  gap: 16px;
  font-size: 13px;
  color: var(--text-secondary);
  margin-top: 8px;
}

.release-assets span {
  display: flex;
  align-items: center;
  gap: 4px;
}

.release-assets svg {
  flex-shrink: 0;
}

.workflow-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.workflow-status {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  font-size: 18px;
}

.status-running { background: rgba(59, 130, 246, 0.1); }
.status-success { background: rgba(34, 197, 94, 0.1); }
.status-failure { background: rgba(239, 68, 68, 0.1); }
.status-pending { background: var(--bg-tertiary); }

.workflow-info {
  flex: 1;
}

.workflow-name {
  font-weight: 500;
  color: var(--text-primary);
  font-size: 14px;
}

.workflow-meta {
  display: flex;
  gap: 12px;
  font-size: 13px;
  color: var(--text-secondary);
  margin-top: 4px;
}

.workflow-actions {
  display: flex;
  gap: 8px;
}

.view-link, .view-link-small {
  color: var(--accent-color);
  text-decoration: none;
  font-size: 14px;
  font-weight: 500;
}

.view-link-small {
  font-size: 13px;
}

.view-link:hover, .view-link-small:hover {
  text-decoration: underline;
}

.rerun-btn {
  padding: 6px 12px;
  background: var(--accent-color);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.rerun-btn:hover {
  opacity: 0.9;
}
</style>
