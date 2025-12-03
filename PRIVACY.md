# CaoGit Privacy Policy / 隐私政策

**Last Updated / 最后更新: 2025-12-04**

## English

CaoGit is a Git repository management tool designed for developers. This privacy policy describes how we collect, use, and protect your information.

### 1. Information We Collect

**1.1 Local Data Storage**
CaoGit stores the following data locally on your device:
- Repository paths and locations you add to the application
- Application preferences and settings
- Layout customization preferences
- Git platform credentials (stored securely in macOS Keychain)
- AI service API keys (stored securely in macOS Keychain when provided)

**1.2 No Analytics or Tracking**
- We do NOT collect any analytics data
- We do NOT track your usage behavior
- We do NOT collect device information
- We do NOT use any tracking SDKs or services

### 2. Network Connections and Data Transmission

CaoGit connects to external services for the following purposes:

**2.1 GitHub API Access**
- **Purpose**: To fetch release information, manage repositories, and access GitHub features
- **Data Transmitted**: GitHub authentication tokens (if provided), repository URLs
- **Third Party**: GitHub, Inc. (Microsoft)
- **Privacy Policy**: https://docs.github.com/en/site-policy/privacy-policies/github-privacy-statement

**2.2 AI-Powered Commit Message Generation (Optional Feature)**
- **Purpose**: To generate commit messages based on your code changes
- **Data Transmitted**:
  - Code diff content (the actual changes in your files)
  - Your custom system prompts
  - Your API authentication tokens
- **Third Party Services** (you choose which to use):
  - OpenAI (GPT-4, GPT-3.5)
    - Privacy Policy: https://openai.com/policies/privacy-policy
  - Anthropic (Claude)
    - Privacy Policy: https://www.anthropic.com/privacy
  - Custom API endpoints (configured by you)
- **User Control**:
  - This feature is OPTIONAL and disabled by default
  - You must explicitly enable and configure it
  - You must provide your own API keys
  - You can disable it at any time in settings
- **Important Notice**: When you use this feature, your code changes will be sent to the AI service you configured. Please ensure you have the right to share this code and it does not contain sensitive information.

**2.3 Git Operations**
- **Purpose**: Standard Git operations (push, pull, fetch, clone)
- **Data Transmitted**: Git repository data to your configured remote servers
- **Third Parties**: Determined by your repository configuration (GitHub, GitLab, Gitee, or custom Git servers)

### 3. How We Protect Your Data

**3.1 Secure Credential Storage**
- All sensitive credentials (API keys, tokens, passwords) are stored in **macOS Keychain**
- macOS Keychain provides system-level encryption and security
- Credentials are never stored in plain text
- Other applications cannot access your credentials without your permission

**3.2 Local-First Architecture**
- All Git operations are performed locally on your device
- No code or repository data is sent to our servers (we don't have any servers)
- Data is only transmitted to services YOU explicitly configure (GitHub, AI services, etc.)

**3.3 No Third-Party Analytics**
- We do not integrate any analytics or tracking services
- No user behavior data is collected or transmitted

### 4. Your Rights and Choices

**4.1 Data Control**
- You have full control over what repositories you add to the application
- You can delete any stored repository references at any time
- You can clear all application data from Settings

**4.2 AI Feature Control**
- You choose whether to use AI features
- You choose which AI service to use
- You provide your own API keys
- You can disable AI features at any time

**4.3 Credential Management**
- You can view and delete stored credentials from macOS Keychain:
  - Open "Keychain Access" app
  - Search for "CaoGit"
  - Delete any items you want to remove

### 5. Data Retention

- **Local Data**: Stored on your device until you delete it
- **Keychain Data**: Stored until you explicitly delete it from Keychain Access
- **Transmitted Data**: Subject to the privacy policies of the services you use (GitHub, OpenAI, etc.)

### 6. Children's Privacy

CaoGit is intended for developers and is not directed to children under 13. We do not knowingly collect information from children.

### 7. Changes to This Policy

We may update this privacy policy from time to time. The "Last Updated" date at the top indicates when the policy was last revised.

### 8. Required Permissions

**8.1 File Access Permission**
- **Why we need it**: To read and manage Git repositories you select
- **What we access**: Only folders and files you explicitly select
- **Security**: Protected by macOS App Sandbox

**8.2 Network Access Permission**
- **Why we need it**: To access GitHub API and optional AI services
- **What we connect to**:
  - api.github.com (GitHub API)
  - api.openai.com (if you enable OpenAI integration)
  - api.anthropic.com (if you enable Anthropic Claude integration)
  - Custom endpoints (if you configure them)
  - Your Git remote servers (for push/pull operations)

### 9. Contact

For privacy concerns or questions, please open an issue at:
https://github.com/WNLUO/CaoGit/issues

Or email: [Your Email]

---

## 中文

CaoGit 是一款为开发者设计的 Git 仓库管理工具。本隐私政策说明我们如何收集、使用和保护您的信息。

### 1. 我们收集的信息

**1.1 本地数据存储**
CaoGit 在您的设备上本地存储以下数据：
- 您添加到应用中的仓库路径和位置
- 应用偏好设置
- 布局自定义设置
- Git 平台凭证（安全存储在 macOS 钥匙串中）
- AI 服务 API 密钥（如果您提供，安全存储在 macOS 钥匙串中）

**1.2 不收集分析或跟踪数据**
- 我们不收集任何分析数据
- 我们不跟踪您的使用行为
- 我们不收集设备信息
- 我们不使用任何跟踪 SDK 或服务

### 2. 网络连接和数据传输

CaoGit 出于以下目的连接到外部服务：

**2.1 GitHub API 访问**
- **目的**：获取发布信息、管理仓库、访问 GitHub 功能
- **传输的数据**：GitHub 认证令牌（如果提供）、仓库 URL
- **第三方**：GitHub, Inc. (Microsoft)
- **隐私政策**：https://docs.github.com/zh/site-policy/privacy-policies/github-privacy-statement

**2.2 AI 驱动的提交信息生成（可选功能）**
- **目的**：基于您的代码变更生成提交信息
- **传输的数据**：
  - 代码差异内容（文件中的实际变更）
  - 您的自定义系统提示词
  - 您的 API 认证令牌
- **第三方服务**（由您选择使用）：
  - OpenAI (GPT-4, GPT-3.5)
    - 隐私政策：https://openai.com/policies/privacy-policy
  - Anthropic (Claude)
    - 隐私政策：https://www.anthropic.com/privacy
  - 自定义 API 端点（由您配置）
- **用户控制**：
  - 此功能是**可选的**，默认禁用
  - 您必须明确启用并配置它
  - 您必须提供自己的 API 密钥
  - 您可以随时在设置中禁用它
- **重要提示**：当您使用此功能时，您的代码变更将被发送到您配置的 AI 服务。请确保您有权共享此代码，且不包含敏感信息。

**2.3 Git 操作**
- **目的**：标准 Git 操作（推送、拉取、获取、克隆）
- **传输的数据**：Git 仓库数据到您配置的远程服务器
- **第三方**：由您的仓库配置决定（GitHub、GitLab、Gitee 或自定义 Git 服务器）

### 3. 我们如何保护您的数据

**3.1 安全的凭证存储**
- 所有敏感凭证（API 密钥、令牌、密码）都存储在 **macOS 钥匙串**中
- macOS 钥匙串提供系统级加密和安全保护
- 凭证永不以明文存储
- 其他应用无法在未经您许可的情况下访问您的凭证

**3.2 本地优先架构**
- 所有 Git 操作都在您的设备上本地执行
- 不会将代码或仓库数据发送到我们的服务器（我们没有服务器）
- 数据仅传输到您明确配置的服务（GitHub、AI 服务等）

**3.3 无第三方分析**
- 我们不集成任何分析或跟踪服务
- 不收集或传输用户行为数据

### 4. 您的权利和选择

**4.1 数据控制**
- 您完全控制向应用添加哪些仓库
- 您可以随时删除任何存储的仓库引用
- 您可以从设置中清除所有应用数据

**4.2 AI 功能控制**
- 您选择是否使用 AI 功能
- 您选择使用哪个 AI 服务
- 您提供自己的 API 密钥
- 您可以随时禁用 AI 功能

**4.3 凭证管理**
- 您可以从 macOS 钥匙串中查看和删除存储的凭证：
  - 打开"钥匙串访问"应用
  - 搜索"CaoGit"
  - 删除您想要移除的任何项目

### 5. 数据保留

- **本地数据**：存储在您的设备上，直到您删除它
- **钥匙串数据**：存储直到您从钥匙串访问中明确删除它
- **传输的数据**：受您使用的服务（GitHub、OpenAI 等）的隐私政策约束

### 6. 儿童隐私

CaoGit 面向开发者，不针对 13 岁以下的儿童。我们不会故意收集儿童的信息。

### 7. 政策变更

我们可能会不时更新本隐私政策。顶部的"最后更新"日期表示政策的最后修订时间。

### 8. 所需权限

**8.1 文件访问权限**
- **为何需要**：读取和管理您选择的 Git 仓库
- **访问内容**：仅您明确选择的文件夹和文件
- **安全性**：受 macOS App Sandbox 保护

**8.2 网络访问权限**
- **为何需要**：访问 GitHub API 和可选的 AI 服务
- **连接的服务**：
  - api.github.com（GitHub API）
  - api.openai.com（如果您启用 OpenAI 集成）
  - api.anthropic.com（如果您启用 Anthropic Claude 集成）
  - 自定义端点（如果您配置）
  - 您的 Git 远程服务器（用于推送/拉取操作）

### 9. 联系方式

如有隐私问题或疑问，请在此提交 issue：
https://github.com/WNLUO/CaoGit/issues

或发送邮件至：[您的邮箱]

---

## Changelog / 更新记录

### 2025-12-04
- Complete rewrite of privacy policy
- Added detailed information about AI features and data transmission
- Added information about secure credential storage in macOS Keychain
- Added comprehensive disclosure of all network connections
- Added user rights and control information
- 完全重写隐私政策
- 添加了关于 AI 功能和数据传输的详细信息
- 添加了关于 macOS 钥匙串中安全凭证存储的信息
- 添加了所有网络连接的全面披露
- 添加了用户权利和控制信息

### 2024-12-02
- Initial privacy policy / 初始隐私政策
