# CaoGit 上架 Mac App Store 完整指南

## 📋 目录
1. [前置准备](#前置准备)
2. [证书和配置](#证书和配置)
3. [修改项目配置](#修改项目配置)
4. [打包和签名](#打包和签名)
5. [App Store Connect 配置](#app-store-connect-配置)
6. [提交审核](#提交审核)
7. [注意事项](#注意事项)

---

## 🎯 前置准备

### 1. Apple Developer Program
- ✅ 您已有账号（$99/年）
- ✅ Team ID: T5P2UCK36A

### 2. 需要了解的重要区别

| 分发方式 | 证书类型 | 适用场景 | 优缺点 |
|---------|---------|---------|--------|
| **Developer ID** | Developer ID Application | 独立分发（网站、GitHub） | ✅ 自由分发 ✅ 无需审核 ⚠️ 需公证 |
| **Mac App Store** | Mac App Distribution | App Store 上架 | ✅ 官方渠道 ✅ 自动更新 ⚠️ 严格审核 ⚠️ 沙盒限制 |

---

## 🔐 证书和配置

### 步骤 1: 创建 Mac App Store 证书

1. 访问 https://developer.apple.com/account/resources/certificates/list
2. 点击 "+" 创建新证书
3. 选择 **"Mac App Distribution"**（不是 Developer ID）
4. 上传 CSR 文件（可以重用之前生成的，或创建新的）
5. 下载并安装证书

### 步骤 2: 创建 App ID

1. 访问 https://developer.apple.com/account/resources/identifiers/list
2. 点击 "+" 创建新 Identifier
3. 选择 "App IDs"
4. Platform: **macOS**
5. Bundle ID: `com.caogit.macos`（注意：不能以 .app 结尾！）
6. App Name: `CaoGit`
7. Capabilities: 勾选需要的权限（如 Network、File Access 等）

### 步骤 3: 创建 Provisioning Profile

1. 访问 https://developer.apple.com/account/resources/profiles/list
2. 点击 "+" 创建新 Profile
3. 选择 **"Mac App Store"**
4. 选择刚创建的 App ID
5. 选择 Mac App Distribution 证书
6. 下载并双击安装

---

## ⚙️ 修改项目配置

### 1. 修改 Bundle Identifier

编辑 `src-tauri/tauri.conf.json`:

```json
{
  "identifier": "com.caogit.macos",  // 改为新的 Bundle ID，不能以 .app 结尾
  "productName": "CaoGit"
}
```

### 2. 配置 macOS 签名

编辑 `src-tauri/tauri.conf.json`:

```json
{
  "bundle": {
    "macOS": {
      "signingIdentity": "3rd Party Mac Developer Application: luo changyi (T5P2UCK36A)",
      "provisioningProfile": "path/to/your/profile.provisionprofile",
      "hardenedRuntime": false,  // App Store 不需要
      "entitlements": "entitlements-appstore.plist"
    }
  }
}
```

### 3. 创建 App Store 权限文件

创建 `src-tauri/entitlements-appstore.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <!-- App Sandbox (必需) -->
    <key>com.apple.security.app-sandbox</key>
    <true/>

    <!-- 网络访问 -->
    <key>com.apple.security.network.client</key>
    <true/>

    <!-- 用户选择的文件读写 -->
    <key>com.apple.security.files.user-selected.read-write</key>
    <true/>

    <!-- 如果需要访问 Git 仓库 -->
    <key>com.apple.security.files.bookmarks.app-scope</key>
    <true/>

    <!-- 如果需要运行 git 命令 -->
    <key>com.apple.security.temporary-exception.sbpl</key>
    <string>(allow process-exec* (literal "/usr/bin/git"))</string>
</dict>
</plist>
```

### 4. 关键差异说明

**App Store 版本必须启用沙盒（Sandbox）：**
- ❌ 不能随意访问文件系统
- ❌ 不能执行任意外部命令
- ✅ 只能访问用户选择的文件
- ✅ 需要明确声明所有权限

**这对 CaoGit 的影响：**
- ⚠️ 可能需要用户手动选择 Git 仓库目录
- ⚠️ 执行 git 命令可能需要特殊处理
- ⚠️ 某些功能可能受限

---

## 📦 打包和签名

### 方法 1: 使用 Tauri CLI（推荐）

```bash
# 1. 配置环境变量
export TAURI_SIGNING_IDENTITY="3rd Party Mac Developer Application: luo changyi (T5P2UCK36A)"
export TAURI_PROVISIONING_PROFILE="path/to/profile.provisionprofile"

# 2. 打包
npm run tauri:build -- --target mas  # mas = Mac App Store

# 3. 生成 .pkg 文件
productbuild --component \
  src-tauri/target/release/bundle/macos/CaoGit.app \
  /Applications \
  --sign "3rd Party Mac Developer Installer: luo changyi (T5P2UCK36A)" \
  CaoGit.pkg
```

### 方法 2: 手动打包

```bash
# 1. 构建应用
npm run build
cd src-tauri
cargo build --release

# 2. 创建 .app bundle
# (Tauri 会自动完成)

# 3. 签名应用
codesign --deep --force --verify --verbose \
  --sign "3rd Party Mac Developer Application: luo changyi (T5P2UCK36A)" \
  --entitlements entitlements-appstore.plist \
  target/release/bundle/macos/CaoGit.app

# 4. 创建 Installer Package
productbuild --component target/release/bundle/macos/CaoGit.app /Applications \
  --sign "3rd Party Mac Developer Installer: luo changyi (T5P2UCK36A)" \
  CaoGit.pkg
```

---

## 🌐 App Store Connect 配置

### 步骤 1: 创建 App

1. 访问 https://appstoreconnect.apple.com/
2. 点击 "My Apps" → "+" → "New App"
3. 填写信息：
   - Platform: **macOS**
   - Name: **CaoGit**
   - Primary Language: **简体中文** 或 **English**
   - Bundle ID: **com.caogit.macos**
   - SKU: `caogit-mac`（唯一标识符）

### 步骤 2: 填写 App 信息

**基本信息：**
- App 名称: CaoGit
- 副标题: Git 仓库管理工具
- 类别: 开发者工具（Developer Tools）

**描述：**
```
CaoGit 是一款现代化的 Git 仓库管理工具，帮助开发者更高效地管理多个 Git 项目。

主要功能：
• 可视化管理多个 Git 仓库
• 快速查看仓库状态和分支信息
• 批量操作多个仓库
• 简洁直观的用户界面
• 完全本地运行，保护隐私

适合：
- 管理多个项目的开发者
- 需要快速查看仓库状态的团队
- 喜欢简洁工具的程序员
```

**关键词：**
```
git, 版本控制, 开发工具, 仓库管理, developer, source control
```

**截图要求：**
- 至少 1 张（推荐 3-5 张）
- 尺寸：1280x800, 1440x900, 2560x1600, 或 2880x1800
- 格式：PNG 或 JPG

**隐私政策：**
由于 CaoGit 完全本地运行，可以使用简单的隐私政策：
```
CaoGit 完全在您的设备上本地运行，不收集任何用户数据，
不连接外部服务器，所有数据均保存在您的设备上。
```

### 步骤 3: 定价和发布

- **价格：** 免费（或设置价格）
- **可用性：** 选择发布地区
- **年龄分级：** 4+（无限制内容）

---

## 📤 提交审核

### 使用 Xcode Transporter

1. 从 App Store 下载 **Transporter** 应用
2. 打开 Transporter
3. 点击 "+" 添加您的 `.pkg` 文件
4. 点击 "Deliver" 上传到 App Store Connect

### 或使用命令行

```bash
xcrun altool --upload-app \
  --type macos \
  --file CaoGit.pkg \
  --username "admin@wnluo.com" \
  --password "应用专用密码"
```

### 提交审核

1. 在 App Store Connect 中选择您的 App
2. 创建新版本（如 1.0）
3. 填写 "What's New"（版本更新说明）
4. 选择上传的构建版本
5. 点击 "Submit for Review"

---

## ⚠️ 注意事项和常见问题

### 1. 沙盒限制

**问题：** CaoGit 需要访问 Git 仓库和执行 git 命令

**解决方案：**
- 使用 Security-Scoped Bookmarks 保存用户授权的目录
- 在 entitlements 中申请必要权限
- 可能需要修改代码以适应沙盒环境

### 2. Git 命令执行

App Store 沙盒中执行外部命令有限制：

**选项 1：** 使用 libgit2（已在项目中使用 git2 crate）✅
```rust
// 使用 git2 crate，无需执行外部命令
use git2::Repository;
```

**选项 2：** 申请临时例外（不推荐，可能被拒）
```xml
<key>com.apple.security.temporary-exception.sbpl</key>
<string>(allow process-exec* (literal "/usr/bin/git"))</string>
```

### 3. Bundle Identifier 问题

- ❌ `com.caogit.app` - 以 .app 结尾会冲突
- ✅ `com.caogit.macos` - 推荐使用

### 4. 审核可能被拒的原因

常见拒审原因：
1. **沙盒权限不当** - 申请了不需要的权限
2. **功能不完整** - 崩溃或功能缺失
3. **隐私说明不清** - 没有说明为什么需要某些权限
4. **违反指南** - 使用私有 API 或不当内容

### 5. 审核时间

- 首次提交：通常 1-2 周
- 后续更新：3-7 天
- 被拒后重新提交：1-3 天

---

## 🆚 App Store vs 独立分发对比

### 推荐策略：**双轨发布**

#### App Store 版本
**优点：**
- ✅ 官方渠道，用户信任度高
- ✅ 自动更新机制
- ✅ 统一支付（如果收费）
- ✅ 全球分发

**缺点：**
- ⚠️ 严格的沙盒限制
- ⚠️ 审核周期长
- ⚠️ 需要 30% 分成（如果收费）
- ⚠️ 功能可能受限

#### 独立分发版本（当前）
**优点：**
- ✅ 功能无限制
- ✅ 快速迭代
- ✅ 无需审核
- ✅ 100% 收入（如果收费）

**缺点：**
- ⚠️ 用户需要公证确认
- ⚠️ 更新需要手动
- ⚠️ 分发渠道受限

---

## 📝 总结

### 对于 CaoGit 项目的建议：

**短期（当前）：**
1. ✅ 继续使用 Developer ID 分发（GitHub Releases）
2. ✅ 完成公证，提供最佳用户体验
3. ✅ 建立用户基础

**中期（未来）：**
1. 🔄 评估 App Store 价值（用户需求、收入潜力）
2. 🔄 适配沙盒环境（如果需要）
3. 🔄 准备 App Store 版本

**App Store 是否值得？**
- ✅ 如果目标用户是普通开发者，App Store 可提升信任度
- ⚠️ 但需要投入时间适配沙盒限制
- ⚠️ Git 工具的沙盒适配可能较复杂

**建议优先级：**
1. **高优先级：** 完成公证，优化当前分发方式
2. **中优先级：** 测试沙盒环境兼容性
3. **低优先级：** 上架 App Store（需求明确后）

---

## 🔗 相关资源

- [App Store Review Guidelines](https://developer.apple.com/app-store/review/guidelines/)
- [App Sandbox Design Guide](https://developer.apple.com/library/archive/documentation/Security/Conceptual/AppSandboxDesignGuide/)
- [Mac App Store Submission](https://developer.apple.com/macos/submit/)
- [Tauri Mac App Store Guide](https://tauri.app/distribute/app-stores/mac-app-store/)

---

最后更新：2024-12-02
