# App Store 审核说明 / App Store Review Notes

## 应用概述 / App Overview

CaoGit 是一个 macOS 原生的 Git 可视化管理工具，帮助开发者更高效地管理 Git 仓库。

## 网络权限说明 / Network Permissions

### com.apple.security.network.client

应用需要网络客户端权限用于：

1. **Git 操作**（核心功能）
   - 连接到 Git 远程服务器（GitHub、GitLab、Bitbucket 等）
   - 执行 push、pull、fetch、clone 操作
   - 测试方法：打开任意 Git 仓库，点击"同步"按钮

2. **GitHub API 集成**（可选功能）
   - 获取仓库的 Release 信息
   - 管理 GitHub Releases
   - 需要用户提供 GitHub Personal Access Token
   - 测试方法：在设置中配置 GitHub Token，打开"发布管理"

3. **AI 功能**（可选功能，需用户明确同意）
   - 连接到 OpenAI API 或 Anthropic API
   - 发送代码变更内容以生成提交信息建议
   - 需要用户配置 API Key 并同意隐私协议
   - 测试方法：在设置中配置 AI API Key，在提交界面点击"AI 生成"

## 文件访问权限说明 / File Access Permissions

### com.apple.security.files.user-selected.read-write

用于访问用户通过文件选择器选择的 Git 仓库目录，执行 Git 操作（读取状态、提交、创建分支等）。

### com.apple.security.files.downloads.read-write

用于将 Git 仓库克隆到用户的下载文件夹。这是用户常用的默认位置。

### com.apple.security.files.bookmarks.app-scope

用于记住用户授权的仓库目录，避免每次启动都需要重新选择。

## 测试账号 / Test Account

不需要测试账号。应用的核心功能可以使用本地 Git 仓库测试。

如需测试完整功能：
- GitHub Token：可以使用任意 GitHub 账号创建 Personal Access Token
- AI 功能：可选功能，不影响核心 Git 操作

## 特殊说明 / Special Notes

1. **App Sandbox**：应用完全在沙盒中运行
2. **无自动更新**：App Store 版本不包含自动更新功能，通过 App Store 更新
3. **本地优先**：所有数据存储在本地，不上传到我们的服务器
4. **开源依赖**：使用 libgit2 库处理 Git 操作，不依赖系统 git 命令

## 隐私合规 / Privacy Compliance

- 完整隐私政策：见 PRIVACY_POLICY.md
- AI 功能使用前需要用户明确同意
- 所有第三方服务的隐私政策链接已在应用内提供
- 敏感凭证使用 macOS Keychain 安全存储
