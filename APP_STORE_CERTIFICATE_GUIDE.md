# App Store 证书创建指南

## 当前问题

您使用的是 "Developer ID Application" 证书，这个证书只能用于独立分发（DMG）。
App Store 提交需要不同的证书类型。

## 需要的证书

App Store 提交需要两个证书：

1. **3rd Party Mac Developer Application** - 用于签名应用
2. **3rd Party Mac Developer Installer** - 用于签名 .pkg 安装包

或者使用新的：

1. **Apple Distribution** - 用于签名应用和安装包

## 如何创建证书

### 方法 1: 使用 Xcode（推荐）

1. **打开 Xcode**
   - 启动 Xcode
   - 菜单：Settings (或 Preferences) → Accounts

2. **添加 Apple ID**
   - 点击左下角 "+" 添加 Apple ID
   - 登录您的开发者账号：admin@wnluo.com

3. **管理证书**
   - 选择您的团队 (T5P2UCK36A)
   - 点击 "Manage Certificates..."
   - 点击左下角 "+"
   - 选择 "Apple Distribution"

4. **创建 Installer 证书**
   - 在同一个界面
   - 点击左下角 "+"
   - 选择 "Mac Installer Distribution"

### 方法 2: 使用 Apple Developer 网站

1. **访问证书页面**
   - 打开：https://developer.apple.com/account/resources/certificates/list
   - 登录您的开发者账号

2. **创建 Application 证书**
   - 点击 "+" 创建新证书
   - 选择 "Apple Distribution"
   - 跟随向导完成创建
   - 下载证书并双击安装到钥匙串

3. **创建 Installer 证书**
   - 再次点击 "+" 创建新证书
   - 选择 "Mac Installer Distribution"
   - 跟随向导完成创建
   - 下载证书并双击安装到钥匙串

## 证书创建后

创建并安装证书后，运行以下命令验证：

```bash
# 验证证书已安装
security find-identity -v -p codesigning

# 您应该看到类似这样的输出：
# "Apple Distribution: luo changyi (T5P2UCK36A)"
# "Mac Installer Distribution: luo changyi (T5P2UCK36A)"
```

## 使用证书重新签名和打包

证书安装后，运行以下命令：

```bash
cd /Users/wnluo/Desktop/code/Git

# 1. 使用 Apple Distribution 签名应用
codesign --force --deep \
  --sign "Apple Distribution: luo changyi (T5P2UCK36A)" \
  --options runtime \
  --entitlements src-tauri/entitlements-appstore.plist \
  src-tauri/target/release/bundle/macos/CaoGit.app

# 2. 创建 .pkg 包
productbuild \
  --component src-tauri/target/release/bundle/macos/CaoGit.app \
  /Applications \
  CaoGit-unsigned.pkg

# 3. 使用 Mac Installer Distribution 签名 .pkg
productsign \
  --sign "Mac Installer Distribution: luo changyi (T5P2UCK36A)" \
  CaoGit-unsigned.pkg \
  CaoGit-0.2.25.pkg

# 4. 清理临时文件
rm CaoGit-unsigned.pkg

# 5. 验证签名
pkgutil --check-signature CaoGit-0.2.25.pkg
```

## 常见问题

### Q: 我没有付费的 Apple Developer 账号怎么办？
A: App Store 提交需要付费的 Apple Developer Program 会员资格（$99/年）。
   免费的账号只能用于开发测试，不能提交到 App Store。

### Q: 证书创建失败？
A: 确保：
   - 您的 Apple Developer 账号是激活的付费会员
   - 您的团队 ID 是 T5P2UCK36A
   - 您有创建证书的权限（Account Holder 或 Admin 角色）

### Q: 找不到 "Apple Distribution" 选项？
A: 旧版本可能显示为：
   - "3rd Party Mac Developer Application"（应用签名）
   - "3rd Party Mac Developer Installer"（安装包签名）

   这两种证书类型功能相同，都可以使用。

## 当前状态

- ✅ 已修复：OpenSSL 依赖问题
- ✅ 已修复：LSApplicationCategoryType
- ✅ 已修复：文件权限
- ✅ 已修复：App Sandbox
- ⏳ 待处理：使用正确的证书签名

完成证书创建后，告诉我，我会帮您重新签名和打包。

## 替代方案：使用 Xcode 打包

如果证书配置比较复杂，您也可以：

1. 在 Xcode 中创建一个新的 macOS 项目
2. 将已构建的 CaoGit.app 添加进去
3. 使用 Xcode 的 Archive 功能打包
4. 使用 Xcode 的 Organizer 上传到 App Store Connect

这样 Xcode 会自动处理证书和签名问题。
