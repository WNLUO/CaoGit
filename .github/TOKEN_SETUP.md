# 🔐 GitHub Token 权限配置

## 问题

GitHub 拒绝推送 workflow 文件，原因是当前使用的 Personal Access Token 缺少 `workflow` 权限。

```
refusing to allow a Personal Access Token to create or update workflow
`.github/workflows/build.yml` without `workflow` scope
```

## 解决方案

### 步骤 1: 创建新的 Personal Access Token

1. 打开 GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
   - 链接：https://github.com/settings/tokens

2. 点击 **Generate new token**

3. 填写信息：
   - **Token name**: `GitHub Actions Full Access`（或其他名称）
   - **Expiration**: 按需选择（建议 90 天或自定义）

4. **勾选以下权限**：
   ```
   ✅ repo (Full control of private repositories)
   ✅ workflow (Update GitHub Action workflows)
   ✅ read:user (Read user profile data)
   ```

5. 点击 **Generate token**

6. 复制生成的 token（只显示一次！⚠️）

### 步骤 2: 更新本地 Git 凭证

#### 方案 A：使用 git config（推荐）

```bash
# 为当前仓库设置凭证
git config credential.helper store

# 或全局设置（所有仓库）
git config --global credential.helper osxkeychain  # macOS
git config --global credential.helper cache        # Linux
git config --global credential.helper wincred      # Windows
```

然后重新推送，会提示输入用户名和密码：
```bash
git push origin master
```

输入：
- **username**: 你的 GitHub 用户名
- **password**: 刚刚复制的 Personal Access Token

#### 方案 B：直接在 URL 中使用 Token

```bash
# 临时推送一次
git push https://你的用户名:你的token@github.com/WNLUO/CaoGit.git master

# 或修改 remote URL（不建议在公共环境）
git remote set-url origin https://你的用户名:你的token@github.com/WNLUO/CaoGit.git
git push origin master
```

⚠️ **警告**：不要将 token 提交到代码中或公开分享！

### 步骤 3: 验证推送成功

```bash
# 查看远程状态
git log --oneline -3 origin/master

# 应该看到新的提交：
# abc1234 ci: Add GitHub Actions CI/CD for multi-platform builds
```

### 步骤 4: 验证 Workflows 已加载

1. 打开 GitHub 仓库
2. 进入 **Actions** 标签页
3. 应该看到 **Build and Release** 和 **Create Release on Tag** 两个 workflows

如果没有看到，可能需要：
- 刷新页面
- 检查 `.github/workflows/` 目录是否上传成功
- 检查 YAML 语法是否正确

## 测试 CI/CD

### 方法 1：创建测试标签（推荐）

```bash
# 创建测试版本标签
git tag -a v0.1.0 -m "Test build"

# 推送标签
git push origin v0.1.0

# 查看进度
# 打开 GitHub → Actions 标签页
```

### 方法 2：手动触发（无需创建标签）

1. 打开 GitHub 仓库
2. 进入 **Actions** 标签页
3. 左侧选择 **Build and Release**
4. 点击 **Run workflow** 按钮
5. 保持默认设置，点击 **Run workflow**

## 安全建议

✅ **好的做法**：
- 定期轮换 token
- 为不同用途创建不同的 token
- 设置较短的过期时间
- 删除不使用的 token

❌ **不要做的事**：
- 不要将 token 提交到仓库
- 不要在日志中打印 token
- 不要在公共环境中使用 token
- 不要与他人分享 token

## 故障排查

### Q: Token 已创建但推送仍然失败？

**A**: 可能的原因：
1. Token 权限不足 → 重新创建时勾选 `workflow`
2. Token 已过期 → 创建新的 token
3. Keychain/凭证管理器有缓存的旧 token → 清除并重新输入

清除 macOS 的缓存凭证：
```bash
git credential-osxkeychain erase
# 输入：
# host=github.com
# (按 Enter)
```

### Q: 如何检查当前使用的 Token 权限？

**A**: 推送失败时会显示权限错误。如果成功了，可以在 GitHub 设置中查看 Token 的最后使用时间。

### Q: 能否用 SSH Key 替代？

**A**: 可以，但需要额外配置。优点是更安全，缺点是初次设置较复杂。建议使用 Personal Access Token 快速开始。

## 需要帮助？

- [GitHub Personal Access Tokens 文档](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)
- [Git Credentials 文档](https://git-scm.com/book/en/v2/Git-Tools-Credential-Storage)
