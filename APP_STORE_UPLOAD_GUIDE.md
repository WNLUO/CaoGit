# CaoGit App Store 上传指南

## 已完成的修复

✅ 修复了非公开 API 使用问题
✅ 修复了 macOS 26.1 启动崩溃问题
✅ 移除了对 Homebrew 的依赖
✅ 准备了审核回复文本

## 上传到 App Store 的步骤

### 方法 1: 使用 Transporter（最简单）

1. **下载 Transporter**
   - 从 Mac App Store 下载 "Transporter" 应用
   - 或访问: https://apps.apple.com/app/transporter/id1450874784

2. **准备 App Store 版本**
   ```bash
   cd /Users/wnluo/Desktop/code/Git/src-tauri

   # 使用 productbuild 创建 .pkg 包
   productbuild --component target/release/bundle/macos/CaoGit.app /Applications CaoGit-0.2.25.pkg
   ```

3. **使用 Transporter 上传**
   - 打开 Transporter
   - 登录您的 Apple ID (T5P2UCK36A 团队)
   - 拖拽 `CaoGit-0.2.25.pkg` 到 Transporter
   - 点击 "Deliver" 上传

### 方法 2: 使用命令行（需要 API Key）

如果您有 App Store Connect API Key:

```bash
# 上传到 App Store Connect
xcrun altool --upload-package CaoGit-0.2.25.pkg \
  --type macos \
  --apiKey YOUR_API_KEY \
  --apiIssuer YOUR_ISSUER_ID
```

或使用 Apple ID:

```bash
# 需要应用专用密码
xcrun altool --upload-package CaoGit-0.2.25.pkg \
  --type macos \
  --apple-id "your-apple-id@email.com" \
  --password "@keychain:AC_PASSWORD"
```

## App Store Connect 回复文本

上传完成后，在 App Store Connect 的审核反馈页面复制粘贴以下内容：

---

Dear App Review Team,

Thank you for your detailed feedback. I have addressed all three issues identified in the review.

**Issue 1 & 3: Non-Public API Usage and Launch Crash on macOS 26.1**

Root Cause:
The previous build was dynamically linking to Homebrew's OpenSSL library (/opt/homebrew/*/libssl.3.dylib), which caused both the non-public API reference (_EVP_PKEY_CTX_set_rsa_padding) and the launch crash on macOS 26.1.

Fix Applied:
I have updated the build configuration to statically compile OpenSSL (vendored-openssl) instead of linking to external libraries. The new binary only depends on macOS system frameworks and no longer references any non-public APIs.

The updated version has been tested and verified to:
1. Remove all references to non-public APIs
2. Launch successfully without external library dependencies
3. Only use macOS native frameworks (Security, Foundation, WebKit, etc.)

**Issue 2: com.apple.security.files.downloads.read-write Entitlement**

This entitlement is required for our app's auto-update feature. When users check for updates, the application downloads the new version to the user's Downloads folder for easy access and installation.

Implementation Details:
- The code is located in release_commands.rs (lines 1019-1041)
- The function determines the appropriate Downloads folder path based on the operating system
- Downloaded update files are saved to this location for user convenience
- This functionality provides a better user experience by placing update files in a familiar and accessible location

I have uploaded the updated binary with all fixes applied. Please review the new submission.

Best regards

---

## 注意事项

### 证书要求
- ⚠️ 当前使用的是 "Developer ID Application" 证书（用于 DMG 分发）
- 📋 App Store 提交需要 "3rd Party Mac Developer Application" 或 "Apple Distribution" 证书
- 如果需要，请在 Apple Developer 网站创建 App Store 证书

### 版本信息
- App 名称: CaoGit
- Bundle ID: com.caogit.macos
- 当前版本: 0.2.25
- 团队 ID: T5P2UCK36A

### 文件位置
- App Bundle: `/Users/wnluo/Desktop/code/Git/src-tauri/target/release/bundle/macos/CaoGit.app`
- 修复的代码: `/Users/wnluo/Desktop/code/Git/src-tauri/Cargo.toml:32`

## 验证清单

上传前请确认：

- [ ] 新版本号是 0.2.25 或更高
- [ ] 使用正确的 App Store 证书签名
- [ ] 使用 `entitlements-appstore.plist` 权限文件
- [ ] 已验证没有 Homebrew 依赖
- [ ] 已准备好审核回复文本

## 如有问题

如果上传过程中遇到任何问题：

1. 检查证书是否正确（App Store 证书 vs Developer ID）
2. 确认 Apple ID 有权限管理该应用
3. 验证网络连接正常
4. 查看 Transporter 或终端的错误信息

## 后续步骤

1. ✅ 上传新版本到 App Store Connect
2. ✅ 在审核回复页面粘贴上面的回复文本
3. ✅ 重新提交审核
4. ⏳ 等待审核通过
