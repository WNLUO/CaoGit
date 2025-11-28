# 📊 构建状态监控

## 最新构建信息

**标签**: v0.2.0
**构建改进**: GitHub Actions workflow 已优化，支持多平台并行构建

## 实时监控步骤

### 步骤 1️⃣：查看 Actions 运行状态

1. 打开仓库：https://github.com/WNLUO/CaoGit
2. 点击上方的 **Actions** 标签
3. 左侧选择 **Build and Release** workflow
4. 查看最新运行记录

### 步骤 2️⃣：理解构建流程

```
标签 v0.2.0 推送
        ↓
GitHub Actions 触发
        ↓
并行构建三个平台：
├─ build-macos (macOS aarch64 + x86_64)     ~ 10-15 分钟
├─ build-windows (Windows x86_64)            ~ 10-15 分钟
└─ build-linux (Linux x86_64)                ~ 10-15 分钟
        ↓
所有构建完成
        ↓
publish-release job 发布 Release
        ↓
release 页面更新，显示所有安装包 ✅
```

### 步骤 3️⃣：预期的产物

构建成功后，你会在 Release v0.2.0 中看到：

**macOS**:
- `Git管理器_v0.2.0_aarch64.dmg` (Apple Silicon)
- `Git管理器_v0.2.0_x64.dmg` (Intel)
- `Git管理器_v0.2.0_aarch64.app.tar.gz`
- `Git管理器_v0.2.0_x64.app.tar.gz`

**Windows**:
- `Git管理器_v0.2.0_x64-setup.msi` (安装程序)
- `Git管理器_v0.2.0_x64.exe` (便携版)

**Linux**:
- `git-manager_v0.2.0_amd64.AppImage` (通用 Linux)
- `git-manager_v0.2.0_amd64.deb` (Debian 包)
- `git-manager_v0.2.0_amd64.tar.gz` (压缩包)

## 常见问题

### Q: 多久能看到安装包？

**A**:
- 首次构建：20-25 分钟（需要从头编译）
- 后续构建：15-20 分钟（使用缓存加速）

### Q: 如果构建失败了怎么办？

**A**:
1. 点击失败的 job 查看日志
2. 常见失败原因：
   - ❌ 代码编译错误 → 修复代码
   - ❌ 依赖缺失 → 检查 package.json 或 Cargo.toml
   - ❌ 图标缺失 → 确保 `src-tauri/icons/` 有所有必需文件
   - ❌ 权限错误 → 确保 GitHub Token 有足够权限

### Q: 能看到实时构建日志吗？

**A**: 可以！在 Actions 页面，点击对应的 job，会显示实时日志输出。

### Q: Release 页面显示"Draft"是什么意思？

**A**: 说明构建还在进行中。完成后会自动发布。

### Q: 为什么 release 没有立即出现？

**A**: GitHub 有缓存延迟，通常需要 1-2 分钟才能显示在 release 列表中。刷新页面试试。

## 故障排查清单

- [ ] 访问 Actions 标签页，确保能看到 "Build and Release" workflow
- [ ] 查看 v0.2.0 对应的运行记录
- [ ] 确保有三个平台的 job 在运行（build-macos, build-windows, build-linux）
- [ ] 最后一个 job 是 "publish-release"，它会发布最终 release
- [ ] 所有 job 都应该显示绿色的 ✅ 通过状态

## 手动触发构建（如果标签方式不工作）

如果推送标签后没有触发构建，可以手动触发：

1. 打开 GitHub 仓库
2. 进入 **Actions** 标签页
3. 左侧选择 **Build and Release**
4. 点击 **Run workflow** 按钮
5. 选择分支为 master，点击绿色的 **Run workflow** 按钮

## 查看完整日志

1. 在 Actions 页面点击对应的 workflow run
2. 点击具体的 job（例如 "build-macos"）
3. 点击具体的 step 可以展开查看详细日志

## 下载已构建的产物

### 方式 A: 从 Releases 页面下载（推荐）

1. https://github.com/WNLUO/CaoGit/releases
2. 找到 v0.2.0
3. 点击对应平台的文件下载

### 方式 B: 从 Actions Artifacts 下载

如果 release 还在草稿状态，可以从 artifacts 下载：

1. 进入 Actions 页面
2. 点击 v0.2.0 的运行记录
3. 向下滚动找到 "Artifacts" 部分
4. 下载对应平台的文件

## 后续发布新版本

每次发布新版本只需：

```bash
git push origin master
git tag -a v0.3.0 -m "Your release notes"
git push origin v0.3.0
```

10-20 分钟后就能在 Releases 页面看到新版本的安装包！

---

**开始监控**: https://github.com/WNLUO/CaoGit/actions
