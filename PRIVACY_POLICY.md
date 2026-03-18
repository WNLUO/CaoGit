# CaoGit 隐私政策 / Privacy Policy

最后更新：2026-03-17

## 数据收集和使用 / Data Collection and Use

### 本地数据存储 / Local Data Storage

CaoGit 在您的设备上本地存储以下信息：
- Git 仓库路径和配置
- 用户偏好设置
- GitHub Personal Access Token（存储在 macOS Keychain）
- AI API 密钥（存储在 macOS Keychain）

**这些数据仅存储在您的本地设备，我们不会收集或上传到任何服务器。**

### 网络连接 / Network Connections

CaoGit 会在以下情况下连接到网络：

1. **Git 操作**：连接到您配置的 Git 远程服务器（如 GitHub、GitLab 等）进行 push、pull、fetch 操作
2. **GitHub API**：使用您的 Personal Access Token 访问 GitHub API 获取仓库信息和发布管理
3. **AI 功能（可选）**：当您启用并使用 AI 功能时，您的代码变更内容会发送到您选择的 AI 服务

### AI 功能数据共享 / AI Feature Data Sharing

**重要：AI 功能完全可选，需要您明确同意后才能使用。**

当您使用 AI 功能生成提交信息时：
- **发送的数据**：您当前的代码变更内容（git diff）
- **接收方**：您配置的第三方 AI 服务（OpenAI、Anthropic Claude 或自定义端点）
- **用途**：生成提交信息建议
- **控制权**：您可以随时在设置中禁用此功能

**第三方 AI 服务隐私政策：**
- OpenAI: https://openai.com/policies/privacy-policy
- Anthropic: https://www.anthropic.com/privacy

### 我们不收集的数据 / Data We Do NOT Collect

- 您的代码内容
- 您的 Git 提交历史
- 您的个人身份信息
- 使用统计或分析数据
- 崩溃报告或诊断数据

## 数据安全 / Data Security

- 所有敏感凭证（GitHub Token、AI API Key）使用 macOS Keychain 安全存储
- 应用使用 App Sandbox 隔离运行
- 所有网络连接使用 HTTPS 加密

## 您的权利 / Your Rights

- 您可以随时删除存储在 Keychain 中的凭证
- 您可以随时禁用 AI 功能
- 您完全控制应用访问的 Git 仓库

## 联系我们 / Contact Us

如有隐私相关问题，请通过 GitHub Issues 联系我们。
