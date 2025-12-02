# macOS 签名打包指南

本文档说明如何使用 Apple 开发者证书对 CaoGit 进行签名和公证打包。

## 📋 前置要求

### 1. Apple 开发者账号
- ✅ 已有 Apple Developer Program 账号（$99/年）
- ✅ 证书：Apple Development: admin@wnluo.com (QP42859HP4)

### 2. 开发环境
- macOS 系统
- Xcode Command Line Tools
- Rust 和 Tauri CLI
- Node.js 和 npm

### 3. 公证所需凭证
需要从 Apple ID 账号获取 App-specific password（应用专用密码）

## 🔐 配置 Apple 公证凭证

### 步骤 1: 创建 App-specific password

1. 访问 https://appleid.apple.com/
2. 登录您的 Apple ID (admin@wnluo.com)
3. 在"安全"部分，点击"App-specific passwords"（应用专用密码）
4. 点击"+"创建新密码
5. 输入名称（如 "CaoGit Notarization"）
6. 复制生成的密码（格式：xxxx-xxxx-xxxx-xxxx）

### 步骤 2: 配置环境变量

在终端中设置环境变量（建议添加到 `~/.zshrc` 或 `~/.bash_profile`）：

```bash
# Apple 公证配置
export APPLE_ID="admin@wnluo.com"
export APPLE_PASSWORD="你的-应用专用-密码"  # 从上一步获取
export APPLE_TEAM_ID="你的团队ID"  # 从开发者账号获取

# 或者使用 Keychain 存储密码（更安全）
xcrun notarytool store-credentials "caogit-notary-profile" \
  --apple-id "admin@wnluo.com" \
  --team-id "你的团队ID" \
  --password "你的-应用专用-密码"
```

### 获取 Team ID

```bash
# 方法 1: 通过证书查看
security find-identity -v -p codesigning

# 方法 2: 登录 https://developer.apple.com/account
# 在 Membership 页面查看 Team ID
```

## 🚀 打包流程

### 标准签名打包（推荐）

```bash
# 1. 安装依赖
npm install

# 2. 构建应用（自动签名）
npm run tauri:build

# 打包完成后，会在以下目录生成文件：
# src-tauri/target/release/bundle/dmg/CaoGit_0.2.21_x64.dmg
```

配置已自动包含签名设置：
- `signingIdentity`: 自动使用您的开发者证书
- `hardenedRuntime`: 启用强化运行时
- `entitlements`: 使用自定义权限文件

### 带公证的完整打包

如果需要 Apple 公证（用户无需额外确认即可打开）：

```bash
# 1. 先进行标准打包
npm run tauri:build

# 2. 对 DMG 进行公证
xcrun notarytool submit \
  src-tauri/target/release/bundle/dmg/CaoGit_*.dmg \
  --keychain-profile "caogit-notary-profile" \
  --wait

# 3. 公证成功后，装订票据到 DMG
xcrun stapler staple src-tauri/target/release/bundle/dmg/CaoGit_*.dmg

# 4. 验证公证
xcrun stapler validate src-tauri/target/release/bundle/dmg/CaoGit_*.dmg
```

## 🔍 验证签名

打包完成后，可以验证签名是否正确：

```bash
# 验证 App 签名
codesign -dvv src-tauri/target/release/bundle/macos/CaoGit.app

# 验证 DMG 签名
codesign -dvv src-tauri/target/release/bundle/dmg/CaoGit_*.dmg

# 检查强化运行时
codesign -d --entitlements - src-tauri/target/release/bundle/macos/CaoGit.app
```

预期输出应包含：
```
Authority=Apple Development: admin@wnluo.com (QP42859HP4)
Sealed Resources version=2 rules=13 files=...
```

## 📦 发布流程

### 不公证版本（当前配置）

1. 打包：`npm run tauri:build`
2. 测试 DMG 文件
3. 上传到 GitHub Releases 或其他平台
4. 用户下载后首次打开需要右键 > 打开（或在系统设置中允许）

### 公证版本（推荐）

1. 打包：`npm run tauri:build`
2. 公证：运行上述公证命令
3. 装订票据
4. 验证
5. 发布
6. 用户下载后可直接双击打开，无需额外确认

## ⚠️ 常见问题

### 1. 签名失败：找不到证书

```bash
# 检查可用证书
security find-identity -v -p codesigning

# 如果没有证书，需要在 Xcode 中下载
# 或访问 https://developer.apple.com/account/resources/certificates
```

### 2. 公证失败：Invalid credentials

```bash
# 确认 Apple ID 和密码正确
# 确认使用的是 App-specific password，不是 Apple ID 密码
# 重新配置 keychain profile
xcrun notarytool store-credentials --reset "caogit-notary-profile"
```

### 3. 公证失败：Invalid binary

检查 entitlements.plist 配置是否正确，某些权限可能不被公证接受。

### 4. 用户反馈"应用已损坏"

这通常是因为：
- 未签名（已解决）
- 未公证（可选）
- 文件传输过程中损坏（检查 SHA256）

建议用户执行（如果未公证）：
```bash
xattr -cr /Applications/CaoGit.app
```

## 🔧 配置文件说明

### tauri.conf.json - macOS 配置

```json
"macOS": {
  "signingIdentity": "Apple Development: admin@wnluo.com (QP42859HP4)",
  "hardenedRuntime": true,
  "entitlements": "entitlements.plist",
  "dmg": { /* DMG 样式配置 */ }
}
```

### entitlements.plist - 权限配置

关键权限：
- `com.apple.security.cs.allow-jit`: 允许 JIT 编译
- `com.apple.security.network.client`: 网络访问
- `com.apple.security.files.user-selected.read-write`: 文件读写

## 📚 参考资料

- [Tauri Bundle 文档](https://tauri.app/v1/guides/distribution/sign-macos)
- [Apple Notarization 指南](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution)
- [Code Signing 最佳实践](https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution)

## 🎯 快速开始（TL;DR）

```bash
# 仅签名（当前配置即可）
npm run tauri:build

# 签名 + 公证（需要配置凭证）
npm run tauri:build
xcrun notarytool submit src-tauri/target/release/bundle/dmg/*.dmg \
  --apple-id "admin@wnluo.com" \
  --password "应用专用密码" \
  --team-id "团队ID" \
  --wait
xcrun stapler staple src-tauri/target/release/bundle/dmg/*.dmg
```

---

最后更新：2024-12-02
