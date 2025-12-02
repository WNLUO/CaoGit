# CaoGit 构建指南

本文档说明如何构建 CaoGit 的不同发行版本。

## 版本说明

CaoGit 支持两种发行渠道：

1. **DMG 版本**（独立分发）
   - 支持自动下载和安装更新
   - 使用 Developer ID Application 证书签名
   - 需要经过 Apple 公证（notarization）

2. **App Store 版本**
   - 仅提示用户有新版本，引导至 App Store 更新
   - 使用 Mac App Distribution 证书签名
   - 启用沙盒（Sandbox）
   - 需要上传到 App Store Connect

## 构建 DMG 版本

### 前提条件

- 已安装 Xcode 和 Xcode Command Line Tools
- 拥有有效的 Developer ID Application 证书
- 已配置公证（notarization）密钥链配置文件

### 构建步骤

```bash
# 1. 构建前端
npm run build

# 2. 构建 Tauri 应用（默认使用 DMG 配置）
npm run tauri:build
```

### 构建输出

构建完成后，会生成以下文件：

- DMG 安装包：`src-tauri/target/release/bundle/dmg/CaoGit_<version>_aarch64.dmg`
- macOS 应用：`src-tauri/target/release/bundle/macos/CaoGit.app`

### 公证（Notarization）

```bash
# 提交公证
xcrun notarytool submit \
  src-tauri/target/release/bundle/dmg/CaoGit_<version>_aarch64.dmg \
  --keychain-profile "caogit-notary-profile" \
  --wait

# 检查公证状态
xcrun notarytool info <submission-id> \
  --keychain-profile "caogit-notary-profile"
```

## 构建 App Store 版本

### 前提条件

- 已安装 Xcode 和 Xcode Command Line Tools
- 拥有有效的 Mac App Distribution 证书
- 拥有有效的 Mac Installer Distribution 证书
- 已在 App Store Connect 中创建应用

### 构建步骤

```bash
# 1. 清理之前的构建（重要！）
rm -rf src-tauri/target/release

# 2. 构建前端（App Store 模式）
npm run build:appstore

# 3. 构建 Rust 后端（App Store feature）
cd src-tauri
cargo build --release --features appstore --no-default-features
cd ..

# 4. 构建 Tauri 应用
npm run tauri build

# 5. 打包为 PKG（需要手动执行）
productbuild --component \
  src-tauri/target/release/bundle/macos/CaoGit.app \
  /Applications \
  --sign "3rd Party Mac Developer Installer: luo changyi (T5P2UCK36A)" \
  src-tauri/target/release/bundle/CaoGit_AppStore.pkg
```

或者使用一键命令（实验性）：

```bash
npm run tauri:build:appstore
```

### 构建输出

- PKG 安装包：`src-tauri/target/release/bundle/CaoGit_AppStore.pkg`

### 上传到 App Store Connect

#### 方法 1: 使用 Xcode Transporter

1. 打开 Xcode Transporter 应用
2. 选择 PKG 文件
3. 点击 "Deliver" 上传

#### 方法 2: 使用命令行

```bash
xcrun altool --upload-app \
  --type macos \
  --file src-tauri/target/release/bundle/CaoGit_AppStore.pkg \
  --username "your-apple-id@example.com" \
  --password "app-specific-password"
```

## 代码差异说明

### Rust 后端

**DMG 版本**（使用 `auto-update` feature）：
- 包含 `install_update` 命令，可自动下载和安装更新
- 不包含 App Store 相关命令

**App Store 版本**（使用 `appstore` feature）：
- 不包含 `install_update` 命令
- 包含 `check_update_appstore` 和 `open_app_store` 命令
- 使用硬编码的 GitHub 仓库信息检查更新

### 前端

通过环境变量 `VITE_APPSTORE` 区分：

**DMG 版本**（`VITE_APPSTORE` 未设置或为 false）：
- 显示"立即更新"按钮
- 点击后自动下载并安装更新
- 显示下载进度

**App Store 版本**（`VITE_APPSTORE=true`）：
- 显示"前往 App Store 更新"按钮
- 点击后打开 Mac App Store
- 不显示下载进度

## 配置文件说明

### Cargo.toml

```toml
[features]
default = ["auto-update"]
auto-update = []  # DMG 版本
appstore = []     # App Store 版本
```

### tauri.conf.json

App Store 版本需要的特殊配置：

- `identifier`: `com.caogit.macos`
- `macOS.signingIdentity`: 使用 Mac App Distribution 证书
- `macOS.entitlements`: 指向 `entitlements-appstore.plist`
- `macOS.hardenedRuntime`: false（App Store 版本不需要）

### entitlements-appstore.plist

App Store 版本需要的权限：

- `com.apple.security.app-sandbox`: 启用沙盒
- `com.apple.security.network.client`: 网络访问
- `com.apple.security.files.user-selected.read-write`: 用户选择的文件读写

## 本地化支持

应用已配置支持中文和英文本地化:

- 默认语言：中文（zh_CN）
- 支持语言：中文（zh_CN）、英文（en）
- 文件对话框语言：根据 macOS 系统语言自动显示

本地化文件位于：
- `src-tauri/resources/zh_CN.lproj/InfoPlist.strings`
- `src-tauri/resources/en.lproj/InfoPlist.strings`

构建时会自动应用本地化配置并编译为二进制格式。

## 注意事项

1. **版本号同步**：确保 `Cargo.toml`、`package.json` 和 `tauri.conf.json` 中的版本号一致
2. **证书选择**：DMG 和 App Store 版本使用不同的证书，构建前请确认
3. **清理构建**：切换版本时建议清理之前的构建输出
4. **测试**：构建完成后应在本地测试应用是否正常运行
5. **App Store 审核**：App Store 版本需要通过 Apple 的审核才能发布
6. **本地化**：修改本地化文件后需要重新构建应用

## 常见问题

### Q: 构建 App Store 版本时出现证书错误？

A: 确保：
1. 在 Keychain Access 中导入了正确的证书和私钥
2. `tauri.conf.json` 中的 `signingIdentity` 配置正确
3. 使用 `security find-identity -v -p codesigning` 查看可用证书

### Q: App Store 版本更新检查失败？

A: 检查：
1. 应用是否有网络访问权限
2. GitHub API 是否可访问
3. 后端硬编码的仓库信息是否正确（`wnluo/caogit`）

### Q: DMG 版本公证失败？

A: 常见原因：
1. 使用了错误的证书类型（应使用 Developer ID Application）
2. 未配置公证密钥链配置文件
3. 应用缺少必要的 entitlements

## 发布流程

### DMG 版本发布

1. 更新版本号：`npm run bump:patch`（或 minor/major）
2. 构建 DMG：`npm run tauri:build`
3. 提交公证：使用 `xcrun notarytool submit`
4. 创建 GitHub Release
5. 上传 DMG 文件到 Release

### App Store 版本发布

1. 更新版本号（与 DMG 版本保持一致）
2. 构建 App Store 版本
3. 上传到 App Store Connect
4. 填写版本信息和截图
5. 提交审核
6. 审核通过后发布

## 参考资料

- [Tauri 官方文档](https://tauri.app/)
- [Apple 代码签名指南](https://developer.apple.com/documentation/security/code_signing)
- [App Store Connect 指南](https://developer.apple.com/app-store-connect/)
