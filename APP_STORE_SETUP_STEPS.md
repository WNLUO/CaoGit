# CaoGit 上架 Mac App Store - 操作步骤

## 📋 当前进度

✅ **已完成：**
- [x] Bundle Identifier 已改为 `com.caogit.macos`
- [x] 创建了 App Store 专用权限文件 `entitlements-appstore.plist`

⏳ **待完成：**
- [ ] 创建 Mac App Distribution 证书
- [ ] 在 Identifiers 创建 App ID
- [ ] 在 App Store Connect 创建应用
- [ ] 配置并打包
- [ ] 上传审核

---

## 🔐 步骤 1: 创建 Mac App Distribution 证书

### 1.1 生成新的 CSR（或重用之前的）

**使用之前的 CSR（推荐）：**
```
文件位置：~/Desktop/CaoGit_DeveloperID.certSigningRequest
```

**或者生成新的：**
```bash
openssl genrsa -out ~/Desktop/CaoGit_AppStore.key 2048
openssl req -new \
  -key ~/Desktop/CaoGit_AppStore.key \
  -out ~/Desktop/CaoGit_AppStore.certSigningRequest \
  -subj "/emailAddress=admin@wnluo.com/CN=admin@wnluo.com/C=CN"
```

### 1.2 在 Apple Developer 网站创建证书

1. 访问 https://developer.apple.com/account/resources/certificates/list
2. 点击 "+" 创建新证书
3. **重要：选择 "Mac App Distribution"**（不是 Developer ID）
4. 上传 CSR 文件
5. 下载证书（文件名类似：`mac_app.cer`）
6. 双击安装到 Keychain

### 1.3 同时创建 Mac Installer Distribution 证书

**需要两个证书：**
- ✅ Mac App Distribution - 用于签名应用
- ✅ Mac Installer Distribution - 用于签名 .pkg 安装包

**创建 Mac Installer Distribution：**
1. 在同一页面再次点击 "+"
2. 选择 **"Mac Installer Distribution"**
3. 上传相同的 CSR 文件（可以重用）
4. 下载并双击安装

---

## 📝 步骤 2: 在 Identifiers 创建 App ID

### 2.1 访问 Identifiers 页面

https://developer.apple.com/account/resources/identifiers/list

### 2.2 创建新的 Identifier

点击 "+" 按钮

### 2.3 选择类型

- 选择：**"App IDs"**
- 点击 "Continue"

### 2.4 选择平台

- Platform: **macOS**
- 点击 "Continue"

### 2.5 填写信息

**Description（描述）：**
```
CaoGit
```

**Bundle ID：**
- 选择：**Explicit**（显式）
- 输入：`com.caogit.macos`
- ⚠️ 必须与 tauri.conf.json 中的 identifier 完全一致

### 2.6 选择 Capabilities（功能）

勾选以下功能：
- ✅ **Network Extensions**（或 Outgoing Connections）
- ✅ **User Selected Files**（文件访问）

其他功能根据需要选择，但不要选择不需要的（审核时会被质疑）

### 2.7 完成

- 点击 "Continue"
- 确认信息
- 点击 "Register"

---

## 🌐 步骤 3: 在 App Store Connect 创建应用

### 3.1 访问 App Store Connect

https://appstoreconnect.apple.com/

### 3.2 创建新应用

1. 点击 "My Apps"
2. 点击左上角的 "+"
3. 选择 "New App"

### 3.3 填写基本信息

**Platforms（平台）：**
- 选择：**macOS**

**Name（应用名称）：**
```
CaoGit
```

**Primary Language（主要语言）：**
- 推荐：**简体中文** 或 **English (U.S.)**

**Bundle ID：**
- 从下拉菜单选择：**com.caogit.macos**
- 如果没有，说明步骤 2 未完成

**SKU（产品编号）：**
```
caogit-macos-2024
```
（任意唯一字符串，用于内部识别）

**User Access（访问权限）：**
- 选择：**Full Access**

### 3.4 点击 "Create"

---

## 📝 步骤 4: 填写应用信息

创建后会进入应用详情页面，需要填写以下信息：

### 4.1 App Information（应用信息）

**Category（类别）：**
- Primary: **Developer Tools**（开发者工具）
- Secondary: **Productivity**（生产力）（可选）

**Privacy Policy URL（隐私政策）：**
```
https://github.com/WNLUO/CaoGit/blob/master/PRIVACY.md
```
（需要在 GitHub 创建这个文件，内容见下文）

### 4.2 Pricing and Availability（定价与发布）

**Price（价格）：**
- 选择：**Free**（免费）或设置价格

**Availability（可用地区）：**
- 选择：**所有地区** 或特定国家/地区

### 4.3 App Privacy（应用隐私）

由于 CaoGit 本地运行，选择：
- **We do not collect data from this app**（不收集数据）

### 4.4 版本信息（Version Information）

点击左侧的版本号（如 1.0 Prepare for Submission）

**Screenshots（截图）：**
- 最少 1 张，推荐 3-5 张
- 尺寸：1280x800, 1440x900, 2560x1600, 或 2880x1800
- 格式：PNG 或 JPG（不超过 5MB）

**Description（描述）：**
```
CaoGit 是一款现代化的 Git 仓库管理工具，专为开发者设计。

主要功能：
• 可视化管理多个 Git 仓库
• 一目了然的仓库状态和分支信息
• 支持常用 Git 操作
• 简洁直观的用户界面
• 完全本地运行，保护您的隐私

适合：
- 管理多个项目的开发者
- 需要快速查看仓库状态的团队
- 追求高效工作流的程序员

CaoGit 不收集任何用户数据，所有操作都在本地完成。
```

**Keywords（关键词）：**
```
git,版本控制,开发工具,仓库管理,developer tools,version control,source control
```
（最多 100 字符，用逗号分隔）

**Support URL（支持网址）：**
```
https://github.com/WNLUO/CaoGit
```

**Marketing URL（营销网址）：**（可选）
```
https://github.com/WNLUO/CaoGit
```

**What's New（更新说明）：**
```
首次发布 CaoGit 1.0

• 多仓库可视化管理
• Git 状态实时显示
• 支持常用 Git 操作
• 简洁现代的界面设计
```

---

## 📄 创建隐私政策文件

在您的 GitHub 仓库创建 `PRIVACY.md`：

```markdown
# CaoGit Privacy Policy / 隐私政策

**Last Updated: 2024-12-02**

## English

CaoGit is a local Git repository management tool that runs entirely on your device.

### Data Collection
- **We do not collect any user data**
- **We do not track user behavior**
- **We do not connect to external servers** (except for accessing GitHub via user's browser)
- All Git operations are performed locally on your device
- All data remains on your device

### Permissions
CaoGit requests the following permissions:
- **File Access**: To read and manage Git repositories you select
- **Network Access**: To allow you to open GitHub URLs in your browser

### Contact
For privacy concerns, please open an issue at:
https://github.com/WNLUO/CaoGit/issues

---

## 中文

CaoGit 是一款完全在本地运行的 Git 仓库管理工具。

### 数据收集
- **不收集任何用户数据**
- **不跟踪用户行为**
- **不连接外部服务器**（除了通过浏览器访问 GitHub）
- 所有 Git 操作都在您的设备上本地执行
- 所有数据都保存在您的设备上

### 权限
CaoGit 请求以下权限：
- **文件访问**：用于读取和管理您选择的 Git 仓库
- **网络访问**：允许您在浏览器中打开 GitHub 链接

### 联系方式
如有隐私问题，请在此提交 issue：
https://github.com/WNLUO/CaoGit/issues
```

---

## 🎯 下一步

完成以上在线配置后，告诉我，我会帮您：
1. 验证证书是否正确安装
2. 配置 Tauri 打包设置
3. 打包 App Store 版本
4. 上传到 App Store Connect

---

## ⚠️ 注意事项

### 关于沙盒限制

App Store 版本必须在沙盒中运行，这意味着：

1. **文件访问限制**
   - ❌ 不能自动扫描整个文件系统
   - ✅ 只能访问用户明确选择的目录
   - 💡 需要在 UI 中添加"选择仓库文件夹"功能

2. **Git 命令执行**
   - ✅ 好消息：您的代码使用 `git2` crate（libgit2）
   - ✅ libgit2 是库调用，不是外部进程，沙盒友好
   - ⚠️ 如果有用到 `Command::new("git")`，需要移除

3. **需要代码调整的部分**

检查以下代码：

```rust
// ❌ 不允许（沙盒限制）
use std::process::Command;
Command::new("git").arg("status").output();

// ✅ 允许（使用 libgit2）
use git2::Repository;
let repo = Repository::open(path)?;
```

---

## 📚 相关文档

- [Tauri 配置文档](https://tauri.app/v1/api/config/)
- [App Store 审核指南](https://developer.apple.com/app-store/review/guidelines/)
- [App Sandbox 指南](https://developer.apple.com/library/archive/documentation/Security/Conceptual/AppSandboxDesignGuide/)

---

**准备好后告诉我，我会继续帮您完成打包步骤！**
