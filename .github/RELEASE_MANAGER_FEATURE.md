# 🚀 发布管理功能说明

## 功能概述

在 Git管理器 中添加了完整的发布管理功能，允许用户在 GUI 界面中一键发布新版本到 GitHub，并监控构建状态。

## 核心功能

### 1. 📦 发布新版本
- 自动检测当前版本号
- 支持语义化版本自动递增（Patch/Minor/Major）
- 自动创建 Git 标签
- 自动推送标签到 GitHub
- 自动触发 CI/CD 构建

### 2. 📊 查看发布历史
- 显示最近的所有 Releases
- 查看每个 Release 的下载次数
- 直接链接到 GitHub Release 页面

### 3. 🔄 监控构建状态
- 实时显示 GitHub Actions 构建状态
- 支持重新运行失败的构建
- 直接链接到 Actions 页面查看日志

## 技术实现

### 后端（Rust）

#### 新增模块
1. **`github_api.rs`** - GitHub API 客户端
   - `GitHubClient`: HTTP 客户端封装
   - `parse_repo_url()`: 解析 GitHub 仓库 URL
   - `list_releases()`: 获取所有 Releases
   - `list_workflow_runs()`: 获取构建记录
   - `rerun_workflow()`: 重新运行构建

2. **`release_commands.rs`** - Tauri 命令
   - `get_release_info()`: 获取发布信息
   - `publish_new_release()`: 发布新版本
   - `rerun_failed_build()`: 重新运行构建
   - `increment_version()`: 版本号递增

#### 新增命令
```rust
#[tauri::command]
async fn get_release_info(repo_path: String, github_token: Option<String>) -> Result<ReleaseInfo>

#[tauri::command]
async fn publish_new_release(repo_path: String, config: PublishConfig, github_token: Option<String>) -> Result<String>

#[tauri::command]
async fn rerun_failed_build(repo_path: String, run_id: u64, github_token: String) -> Result<()>

#[tauri::command]
fn increment_version(version: String, part: String) -> Result<String>
```

### 前端（Vue 3）

#### 新增组件
- **`ReleaseManagerModal.vue`**: 发布管理主界面
  - 版本发布表单
  - Releases 列表
  - Workflow Runs 列表
  - 构建状态显示

#### 数据流
```
用户操作 → Tauri Command → Rust 后端 → GitHub API → 返回结果 → 更新 UI
```

## 使用说明

### 集成到现有界面

在 TopBar 或 SettingsModal 中添加按钮：

```vue
<script setup>
import { ref } from 'vue'
import ReleaseManagerModal from './ReleaseManagerModal.vue'

const showReleaseManager = ref(false)
const githubToken = ref(null) // 从设置中读取

function openReleaseManager() {
  showReleaseManager.value = true
}
</script>

<template>
  <!-- 在工具栏添加按钮 -->
  <button @click="openReleaseManager">
    🚀 发布管理
  </button>

  <!-- 模态框 -->
  <ReleaseManagerModal
    :show="showReleaseManager"
    :repo-path="currentRepoPath"
    :github-token="githubToken"
    @close="showReleaseManager = false"
    @success="handleSuccess"
  />
</template>
```

### GitHub Token 配置

用户需要配置 GitHub Personal Access Token（可选，但推荐）：

**权限要求**：
- ✅ `repo` - 访问私有仓库
- ✅ `workflow` - 触发和重新运行 workflow

**配置位置**：建议在 `SettingsModal.vue` 中添加：

```vue
<div class="setting-item">
  <label>GitHub Personal Access Token</label>
  <input type="password" v-model="githubToken" placeholder="ghp_xxxxxxxxxxxx" />
  <span class="hint">用于访问私有仓库和触发构建（可选）</span>
</div>
```

## API 使用示例

### 发布新版本

```javascript
import { invoke } from '@tauri-apps/api/core'

async function publishRelease() {
  try {
    const actionsUrl = await invoke('publish_new_release', {
      repoPath: '/path/to/repo',
      config: {
        version: 'v0.2.2',
        message: 'Release v0.2.2 - Bug fixes',
        createTag: true,
        pushTag: true
      },
      githubToken: 'ghp_xxxx' // 可选
    })

    console.log('构建触发成功！', actionsUrl)
  } catch (error) {
    console.error('发布失败:', error)
  }
}
```

### 获取发布信息

```javascript
const releaseInfo = await invoke('get_release_info', {
  repoPath: '/path/to/repo',
  githubToken: 'ghp_xxxx' // 可选
})

console.log('当前版本:', releaseInfo.currentVersion)
console.log('Releases:', releaseInfo.releases)
console.log('构建记录:', releaseInfo.workflowRuns)
```

### 版本号递增

```javascript
// v0.2.1 → v0.2.2
const newVersion = await invoke('increment_version', {
  version: 'v0.2.1',
  part: 'patch' // 或 'minor', 'major'
})
```

## 工作流程

```
1. 用户点击 "发布管理" 按钮
      ↓
2. 加载当前仓库的 Releases 和 Workflow Runs
      ↓
3. 用户输入新版本号和发布说明
      ↓
4. 点击 "发布到 GitHub"
      ↓
5. 后端创建标签并推送到 GitHub
      ↓
6. GitHub Actions 自动触发三平台构建
      ↓
7. 用户可以在界面中实时查看构建状态
      ↓
8. 构建失败时可以直接重新运行
```

## 优势

### 对用户
- ✅ 无需离开应用即可发布新版本
- ✅ 直观的构建状态监控
- ✅ 自动化的版本管理
- ✅ 降低发布流程的学习成本

### 对开发者
- ✅ 复用现有的 Git 操作代码
- ✅ 类型安全的 API 调用
- ✅ 完整的错误处理
- ✅ 可扩展的架构设计

## 后续优化方向

1. **缓存优化**
   - 缓存 GitHub API 响应
   - 减少网络请求

2. **WebSocket 实时更新**
   - 实时监控构建进度
   - 构建完成自动通知

3. **多平台支持**
   - 支持 GitLab CI/CD
   - 支持其他 CI 平台

4. **发布模板**
   - 预设发布说明模板
   - Changelog 自动生成

5. **安全增强**
   - Token 加密存储
   - 敏感信息脱敏

## 依赖

无需添加新的依赖，完全使用现有的 Rust crates：
- ✅ `reqwest` - 已存在，用于 HTTP 请求
- ✅ `serde` - 已存在，用于序列化
- ✅ `git2` - 已存在，用于 Git 操作

## 测试

手动测试流程：
1. 确保当前仓库有 GitHub remote
2. 配置 GitHub Token（可选）
3. 打开发布管理界面
4. 验证显示当前 Releases
5. 发布测试版本
6. 验证 GitHub Actions 是否触发
7. 测试重新运行失败的构建

## 总结

这个功能将 Git 管理器从单纯的本地 Git 工具提升为完整的 DevOps 工作台，用户可以在一个界面中完成从代码提交到多平台发布的全流程。

---

**实现完成日期**: 2025-11-28
**作者**: Claude Code
**状态**: ✅ 已完成，待集成测试
