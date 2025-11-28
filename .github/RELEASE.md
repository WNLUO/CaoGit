# 📦 发布新版本指南

## 🚀 方式一：手动发布（推荐新手）

### 步骤 1: 提交代码

```bash
git add .
git commit -m "feat: 你的改动描述"
git push origin master
```

### 步骤 2: 创建版本标签

```bash
# 创建标签（修改版本号）
git tag -a v0.2.2 -m "Release v0.2.2"

# 推送标签（自动触发构建）
git push origin v0.2.2
```

### 步骤 3: 等待构建完成

- 打开 Actions 页面监控进度
- 15-20 分钟后在 Releases 页面查看安装包

---

## ⚡ 方式二：一键发布（推荐熟练用户）

使用 `release.sh` 脚本：

```bash
# 发布新版本
./release.sh v0.2.2 "Release v0.2.2 - 新功能描述"
```

脚本会自动：
1. ✅ 检查并提交未保存的改动
2. ✅ 创建版本标签
3. ✅ 推送到 GitHub
4. ✅ 触发三平台自动构建
5. ✅ 显示监控链接

---

## 📋 版本号规范

遵循语义化版本（Semantic Versioning）：

```
v主版本号.次版本号.修订号

示例:
v0.2.2    修复 bug
v0.3.0    添加新功能
v1.0.0    正式稳定版本
```

**规则**:
- **修订号**: 修复 bug，向后兼容
- **次版本号**: 添加新功能，向后兼容
- **主版本号**: 重大更新，可能不兼容

---

## 🎯 实际例子

### 例子 1: 修复 bug

```bash
# 修改代码...
git add .
git commit -m "fix: 修复提交历史显示错误"
git push origin master

# 发布修订版本
git tag -a v0.2.2 -m "Release v0.2.2 - 修复 bug"
git push origin v0.2.2
```

### 例子 2: 添加新功能

```bash
# 修改代码...
git add .
git commit -m "feat: 添加暗黑模式支持"
git push origin master

# 发布次版本
git tag -a v0.3.0 -m "Release v0.3.0 - 添加暗黑模式"
git push origin v0.3.0
```

### 例子 3: 使用脚本发布

```bash
# 一键发布
./release.sh v0.3.0 "Release v0.3.0 - 添加暗黑模式"
```

---

## 🔍 查看构建进度

### Actions 页面（构建日志）

https://github.com/WNLUO/CaoGit/actions

可以看到：
- ✅ build-macos - macOS 构建状态
- ✅ build-windows - Windows 构建状态
- ✅ build-linux - Linux 构建状态

### Release 页面（下载安装包）

https://github.com/WNLUO/CaoGit/releases

完成后可以下载：
- 🍎 macOS: `GitManager_x.x.x_aarch64.dmg`
- 🪟 Windows: `GitManager_x.x.x_x64_en-US.msi`
- 🐧 Linux: `git-manager_x.x.x_amd64.AppImage`

---

## ❌ 常见问题

### Q: 如何删除错误的标签？

```bash
# 删除本地标签
git tag -d v0.2.2

# 删除远程标签
git push origin :refs/tags/v0.2.2
```

### Q: 构建失败了怎么办？

1. 点击失败的 job 查看日志
2. 修复错误后重新推送代码
3. 删除旧标签，重新创建

### Q: 如何只重新构建某个平台？

1. 打开 Actions 页面
2. 选择 "Build and Release" workflow
3. 点击 "Run workflow"
4. 手动触发（会重新构建所有平台）

### Q: 能否在不创建标签的情况下测试构建？

可以！手动触发 workflow：
1. Actions → Build and Release → Run workflow
2. 但不会创建 Release，只能从 Artifacts 下载

---

## 🎉 完成发布后

1. ✅ 验证所有平台的安装包都已上传
2. ✅ 下载测试安装包是否正常工作
3. ✅ 在 Release 页面编辑说明（可选）
4. ✅ 分享给用户使用

---

## 📝 提交信息规范（可选）

使用 Conventional Commits 格式：

```
feat:     新功能
fix:      修复 bug
docs:     文档修改
style:    代码格式（不影响功能）
refactor: 重构
test:     测试
chore:    构建工具或辅助工具

示例:
feat: 添加深色模式支持
fix: 修复文件删除时的崩溃问题
docs: 更新 README 安装说明
```

---

**祝发布顺利！** 🚀
