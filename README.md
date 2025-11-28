<p align="center">
  <img src="/doc/b1.png" alt="CaoGit Logo">
</p>

<h1 align="center">CaoGit</h1>

<p align="center">
  <strong>现代化跨平台 Git 图形化客户端</strong>
</p>

<p align="center">
  基于 Tauri 2.0 + Vue 3 + TypeScript + Rust 构建的高性能 Git GUI 工具
</p>

<p align="center">
  <a href="https://github.com/WNLUO/CaoGit/releases"><img src="https://img.shields.io/github/v/release/WNLUO/CaoGit?style=flat-square" alt="Release"></a>
  <a href="https://github.com/WNLUO/CaoGit/blob/main/LICENSE"><img src="https://img.shields.io/github/license/WNLUO/CaoGit?style=flat-square" alt="License"></a>
  <a href="https://github.com/WNLUO/CaoGit/issues"><img src="https://img.shields.io/github/issues/WNLUO/CaoGit?style=flat-square" alt="Issues"></a>
  <a href="https://github.com/WNLUO/CaoGit/stargazers"><img src="https://img.shields.io/github/stars/WNLUO/CaoGit?style=flat-square" alt="Stars"></a>
</p>

---

## 界面预览

<table>
  <tr>
    <td colspan="2" align="center">
      <img src="/doc/1.png" alt="主界面" width="100%">
      <p align="center"><strong>主界面</strong></p>
    </td>
  </tr>
  <tr>
    <td colspan="2" align="center">
      <img src="/doc/diff.png" alt="Diff 查看" width="100%">
      <p align="center"><strong>Diff 查看</strong></p>
    </td>
  </tr>
  <tr>
    <td width="50%" align="center">
      <img src="/doc/2.png" alt="全局网络代理" width="80%">
      <p align="center"><strong>支持全局网络代理</strong></p>
    </td>
    <td width="50%" align="center">
      <img src="/doc/3.png" alt="单仓库网络代理" width="80%">
      <p align="center"><strong>支持单个仓库的网络代理</strong></p>
    </td>
  </tr>
  <tr>
    <td colspan="2" align="center">
      <img src="/doc/4.png" alt="一键发布" width="70%">
      <p align="center"><strong>支持一键发布新版本</strong></p>
    </td>
  </tr>
</table>


## 特性

### 核心功能
- **多仓库管理** - 轻松管理多个 Git 仓库，一键切换
- **完整分支管理** - 创建、切换、删除、合并、重命名分支
- **智能提交** - 可视化 stage/unstage 文件，智能差异对比
- **远程同步** - 支持 Push、Pull、Fetch 等所有远程操作
- **提交历史** - 优雅的提交历史展示，支持搜索和过滤

### 可视化体验
- **分支可视化图** - Canvas 绘制的交互式分支树
- **高级 Diff 查看器** - 支持并排/内联两种模式，语法高亮
- **实时状态监控** - 工作区和暂存区实时状态更新
- **主题定制** - 暗黑/明亮/自动主题切换

### 高级功能
- **AI 提交消息** - 基于 OpenAI API 智能生成提交信息
- **Merge & Rebase** - 智能合并，自动冲突检测
- **Cherry-pick** - 精选提交到当前分支
- **Stash 管理** - 保存和恢复工作状态
- **Tag 管理** - 创建、查看、删除标签
- **Git Blame** - 代码行责任追踪
- **冲突解决** - 三栏可视化冲突编辑器
- **Clone 仓库** - 从 GitHub、GitLab 等克隆远程仓库
- **快捷键支持** - 高效的键盘操作
- **国际化** - 中文/英文界面切换

---

## 下载和安装

### 下载最新版本

前往 [Releases 页面](https://github.com/WNLUO/CaoGit/releases/latest) 下载适合你操作系统的安装包：

| 平台 | 文件格式 | 说明 |
|------|---------|------|
| **macOS (Apple Silicon)** | `CaoGit_x.x.x_aarch64.dmg` | M1/M2/M3 芯片 |
| **macOS (Intel)** | `CaoGit_x.x.x_x64.dmg` | Intel 芯片 |
| **Windows** | `CaoGit_x.x.x_x64_en-US.msi` | 64位系统 |
| **Linux (AppImage)** | `caogit_x.x.x_amd64.AppImage` | 通用格式 |
| **Linux (Debian)** | `caogit_x.x.x_amd64.deb` | Ubuntu/Debian系 |

---

### macOS 安装指南

由于应用未经过 Apple 公证（没钱注册），首次打开时可能会遇到 **"CaoGit 已损坏，无法打开"** 的提示。

#### 解决方法（推荐）

**使用终端命令**

```bash
# 打开终端，执行以下命令（将路径替换为实际安装路径）
xattr -cr /Applications/CaoGit.app
```

> **注意**：每次下载新版本都需要执行一次上述操作。

---

### Windows 安装指南

1. 双击 `.msi/.exe` 安装包
2. 如果出现 SmartScreen 警告，点击 **"更多信息"** → **"仍要运行"**
3. 按照安装向导完成安装

---

### Linux 安装指南

**AppImage 用户**：
```bash
# 1. 添加执行权限
chmod +x caogit_*.AppImage

# 2. 运行应用
./caogit_*.AppImage
```

**Debian/Ubuntu 用户**：
```bash
# 安装 .deb 包
sudo dpkg -i caogit_*.deb

# 如果缺少依赖，运行
sudo apt-get install -f
```


## 使用指南

### 添加仓库

1. 点击侧边栏的 **"+"** 按钮
2. 选择本地 Git 仓库路径
3. 仓库会自动添加到列表并加载

### 提交代码

1. 在 **Changes** 视图查看修改的文件
2. 勾选文件将其添加到暂存区
3. 输入提交信息（可使用 AI 生成）
4. 点击 **"Commit"** 按钮

### AI 提交消息

1. 打开设置（`Ctrl/Cmd + ,`）
2. 配置 OpenAI API Key
3. 在提交界面点击 **"AI 生成"** 按钮
4. 自动分析代码变更并生成提交信息

### 分支管理

- **创建分支**：右键点击提交 → "Create Branch"
- **切换分支**：点击顶部栏的分支名称
- **合并分支**：右键点击分支 → "Merge into current"
- **删除分支**：右键点击分支 → "Delete"

### 快捷键

| 快捷键 | 功能 | 说明 |
|--------|------|------|
| `Ctrl/Cmd + S` | 保存/提交 | 快速提交代码 |
| `Ctrl/Cmd + R` | 刷新仓库 | 重新加载仓库状态 |
| `Ctrl/Cmd + Shift + P` | 推送 | Push 到远程仓库 |
| `Ctrl/Cmd + F` | 搜索 | 搜索提交历史 |
| `Ctrl/Cmd + B` | 切换分支 | 打开分支选择器 |
| `Ctrl/Cmd + ,` | 打开设置 | 配置应用选项 |
| `Escape` | 关闭对话框 | 关闭当前弹窗 |

---

## 技术栈

**后端 (Rust)** - 高性能原生操作
- [Tauri 2.0](https://tauri.app/) - 跨平台桌面应用框架
- [git2-rs](https://github.com/rust-lang/git2-rs) - Git 操作库 (libgit2 绑定)
- [tokio](https://tokio.rs/) - 异步运行时
- [serde](https://serde.rs/) - 序列化/反序列化
- [anyhow](https://github.com/dtolnay/anyhow) - 错误处理
- [chrono](https://github.com/chronotope/chrono) - 时间处理

**前端 (Vue 3)** - 现代化响应式 UI
- [Vue 3.5](https://vuejs.org/) - 组合式 API，响应式系统
- [TypeScript 5.6](https://www.typescriptlang.org/) - 类型安全
- [Vite 6.0](https://vitejs.dev/) - 极速构建工具
- [Vitest 4.0](https://vitest.dev/) - 单元测试框架

---

## 与其他工具对比

| 功能 | CaoGit | GitKraken | GitHub Desktop | SourceTree | GitExtensions |
|------|:------:|:---------:|:--------------:|:----------:|:-------------:|
| **分支可视化** | ✅ | ✅ | ❌ | ✅ | ✅ |
| **并排 Diff** | ✅ | ✅ | 部分 | ✅ | ✅ |
| **多仓库管理** | ✅ | ✅ | ❌ | ✅ | ✅ |
| **完全免费** | ✅ | ❌ (部分收费) | ✅ | ✅ | ✅ |
| **跨平台** | ✅ | ✅ | ✅ | macOS/Win | Windows |
| **AI 功能** | ✅ | ✅ | ❌ | ❌ | ❌ |
| **性能** | 轻量 | 资源占用高 | 中等 | 较慢 | 中等 |
| **国际化** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **冲突解决器** | ✅ | ✅ | 基础 | ✅ | ✅ |
| **开源** | MIT | ❌ | ❌ | ❌ | ✅ GPL |
| **Cherry-pick** | ✅ | ✅ | ❌ | ✅ | ✅ |
| **Git Blame** | ✅ | ✅ | ❌ | ✅ | ✅ |

---

## 贡献

欢迎贡献代码、报告 Bug 或提出新功能建议！

### 快速贡献流程

1. **Fork** 项目到你的 GitHub 账户
2. **Clone** 你的 Fork: `git clone https://github.com/你的用户名/CaoGit.git`
3. **创建分支**: `git checkout -b feature/amazing-feature`
4. **提交更改**: `git commit -m 'feat: add amazing feature'`
5. **推送分支**: `git push origin feature/amazing-feature`
6. **提交 Pull Request**

### 提交规范

遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式
- `refactor`: 重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建/工具相关

详细指南请查看 [CONTRIBUTING.md](CONTRIBUTING.md)

---

## 📄 许可证

本项目采用 **MIT 许可证** - 详见 [LICENSE](LICENSE) 文件

这意味着你可以自由地使用、修改、分发本项目，包括商业用途，只需保留原始版权声明。

---

## 🙏 致谢

- [Tauri](https://tauri.app/) - 强大的跨平台桌面应用框架
- [git2-rs](https://github.com/rust-lang/git2-rs) - 优秀的 Rust Git 绑定库
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [libgit2](https://libgit2.org/) - 可移植的 Git 核心实现
- 所有开源贡献者和社区成员

---

<p align="center">
  <strong>如果这个项目对你有帮助，请给个 ⭐ Star！</strong>
</p>

<p align="center">
  Made with ❤️ by <a href="https://github.com/WNLUO">WNLUO</a>
</p>
