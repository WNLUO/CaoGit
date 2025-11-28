# CaoGit

<p align="center">
  <img src="public/icon.png" alt="CaoGit Logo" width="128" height="128">
</p>

<p align="center">
  <strong>一个功能强大的跨平台 Git 图形化客户端</strong>
</p>

<p align="center">
  基于 Tauri 2.0 + Vue 3 + TypeScript + Rust 构建
</p>

<p align="center">
  <a href="https://github.com/WNLUO/CaoGit/releases"><img src="https://img.shields.io/github/v/release/WNLUO/CaoGit" alt="Release"></a>
  <a href="https://github.com/WNLUO/CaoGit/blob/main/LICENSE"><img src="https://img.shields.io/github/license/WNLUO/CaoGit" alt="License"></a>
  <a href="https://github.com/WNLUO/CaoGit/issues"><img src="https://img.shields.io/github/issues/WNLUO/CaoGit" alt="Issues"></a>
</p>

---

## 特性

### 核心功能
- **多仓库管理** - 同时管理多个 Git 仓库，快速切换
- **完整分支管理** - 创建、切换、删除、合并分支
- **智能提交** - 可视化 stage/unstage，支持 AI 生成提交信息
- **远程同步** - Push、Pull、Fetch 操作，完整远程仓库支持
- **提交历史** - 美观的提交历史展示，支持搜索和过滤

### 可视化
- **分支可视化图** - Canvas 绘制的交互式分支图
- **高级 Diff 查看器** - 并排/内联两种模式，语法高亮
- **实时状态** - 工作区和暂存区实时状态展示

### 高级操作
- **Tag 管理** - 创建、查看、删除标签
- **Stash 功能** - 保存和恢复工作状态
- **Merge** - 智能合并，自动检测冲突
- **Clone** - 克隆远程仓库
- **AI 提交消息** - 基于 OpenAI API 分析代码变更自动生成提交信息
- **冲突解决工具** - 三栏可视化冲突编辑器
- **Cherry-pick** - 精选提交功能
- **Git Blame** - 代码行责任追踪
- **主题定制** - 暗黑/明亮/自动主题切换
- **国际化** - 支持中文和英文界面
- **快捷键** - 支持常用操作快捷键

## 下载和安装

### 📥 下载最新版本

前往 [Releases 页面](https://github.com/WNLUO/CaoGit/releases/latest) 下载适合你操作系统的安装包：

- **macOS**: `CaoGit_x.x.x_aarch64.dmg` (Apple Silicon) 或 `CaoGit_x.x.x_x64.dmg` (Intel)
- **Windows**: `CaoGit_x.x.x_x64_en-US.msi`
- **Linux**: `caogit_x.x.x_amd64.AppImage` 或 `caogit_x.x.x_amd64.deb`

### 🍎 macOS 用户重要提示

由于应用未经过 Apple 公证，首次打开时可能会遇到 **"CaoGit 已损坏，无法打开"** 的提示。

**最简单的解决方法**：

**方法 1：使用自动修复脚本（推荐）✨**

在 Releases 页面下载 `fix-macos.command` 文件，然后：
1. 双击运行 `fix-macos.command` 脚本
2. 按照提示操作
3. 完成！现在可以正常打开 CaoGit 了

**方法 2：使用终端命令**
```bash
# 打开终端，执行以下命令（将路径替换为实际安装路径）
xattr -cr /Applications/CaoGit.app
```

**说明**：方法 2 和方法 3（右键打开、系统设置）经测试可能无效，建议使用方法 1 或方法 2。

💡 **提示**：每次下载新版本都需要执行一次修复脚本。

### 🪟 Windows 用户提示

首次安装时可能会出现 SmartScreen 警告，点击"更多信息" → "仍要运行"即可。

### 🐧 Linux 用户提示

**AppImage 用户**：
```bash
# 添加执行权限
chmod +x caogit_*.AppImage

# 运行
./caogit_*.AppImage
```

**DEB 包用户**：
```bash
sudo dpkg -i caogit_*.deb
```

## 截图

| 主界面 | Diff 查看器 |
|:---:|:---:|
| ![主界面](docs/screenshots/main.png) | ![Diff](docs/screenshots/diff.png) |

| 分支图 | 设置 |
|:---:|:---:|
| ![分支图](docs/screenshots/branch-graph.png) | ![设置](docs/screenshots/settings.png) |

## 快速开始

### 前置要求

- Node.js 16+
- Rust 1.70+
- Git 2.0+

### 安装

```bash
# 克隆项目
git clone https://github.com/WNLUO/CaoGit.git
cd CaoGit

# 安装依赖
npm install
```

### 开发

```bash
# 启动开发模式 (同时启动前端和后端)
npm run tauri dev
```

首次运行会下载 Rust 依赖，可能需要几分钟。

### 构建

```bash
# 构建生产版本
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`

### 测试

```bash
# 运行测试
npm run test

# 运行测试并生成覆盖率报告
npm run test:coverage
```

## 使用指南

### 添加仓库

1. 点击侧边栏的 "+" 按钮
2. 选择本地 Git 仓库路径
3. 仓库会自动添加到列表

### 提交代码

1. 在左侧面板选择要 stage 的文件
2. 点击复选框将文件添加到暂存区
3. 输入提交信息
4. 点击"提交"按钮

### 快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+S` | 保存/提交 |
| `Ctrl+R` | 刷新 |
| `Ctrl+Shift+P` | 推送 |
| `Ctrl+F` | 搜索 |
| `Ctrl+B` | 切换分支 |
| `Ctrl+,` | 打开设置 |
| `Escape` | 关闭对话框 |

## 技术架构

### 技术栈

**后端 (Rust)**
- [Tauri 2.0](https://tauri.app/) - 桌面应用框架
- [git2-rs](https://github.com/rust-lang/git2-rs) - Git 操作库
- [tokio](https://tokio.rs/) - 异步运行时
- [serde](https://serde.rs/) - 序列化

**前端 (Vue 3)**
- [Vue 3.5](https://vuejs.org/) - 渐进式框架
- [TypeScript 5.6](https://www.typescriptlang.org/) - 类型安全
- [Vite 6.0](https://vitejs.dev/) - 构建工具
- [Vitest](https://vitest.dev/) - 单元测试

### 目录结构

```
├── src/                        # Vue 前端代码
│   ├── components/             # Vue 组件
│   │   ├── common/            # 通用组件
│   │   ├── layout/            # 布局组件
│   │   ├── modals/            # 弹窗组件
│   │   └── views/             # 视图组件
│   ├── i18n/                  # 国际化
│   │   └── locales/           # 语言包
│   ├── services/              # 服务层
│   │   ├── gitApi.ts          # Git API
│   │   ├── errorHandler.ts    # 错误处理
│   │   └── keyboardShortcuts.ts # 快捷键
│   ├── stores/                # 状态管理
│   ├── types/                 # TypeScript 类型
│   └── main.ts                # 入口文件
│
├── src-tauri/                  # Rust 后端代码
│   ├── src/
│   │   ├── git_ops/           # Git 操作模块
│   │   │   ├── mod.rs
│   │   │   ├── types.rs       # 类型定义
│   │   │   ├── repository.rs  # 仓库操作
│   │   │   ├── branch.rs      # 分支操作
│   │   │   ├── remote.rs      # 远程操作
│   │   │   ├── diff.rs        # Diff 操作
│   │   │   ├── stash.rs       # Stash 操作
│   │   │   ├── tag.rs         # Tag 操作
│   │   │   ├── merge.rs       # 合并操作
│   │   │   └── blame.rs       # Blame 操作
│   │   ├── commands/          # Tauri 命令模块
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── branch.rs
│   │   │   └── ...
│   │   └── lib.rs             # 主入口
│   └── Cargo.toml             # Rust 依赖
│
├── tests/                      # 测试文件
│   └── unit/                  # 单元测试
│
└── README.md                   # 本文件
```

## 与其他工具对比

| 功能 | CaoGit | GitKraken | GitHub Desktop | SourceTree |
|------|--------|-----------|----------------|------------|
| 分支可视化 | ✅ | ✅ | ❌ | ✅ |
| 并排 Diff | ✅ | ✅ | 部分 | ✅ |
| 多仓库管理 | ✅ | ✅ | ❌ | ✅ |
| 免费 | ✅ | 部分收费 | ✅ | ✅ |
| 跨平台 | ✅ | ✅ | ✅ | macOS/Win |
| AI 功能 | ✅ | ✅ | ❌ | ❌ |
| 性能 | 轻量 | 资源占用高 | 中等 | 慢 |
| 国际化 | ✅ | ✅ | ✅ | ✅ |

## 贡献

欢迎贡献代码、报告 Bug 或提出新功能建议！

请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详细的贡献指南。

### 快速贡献流程

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 提交 Pull Request

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面框架
- [git2-rs](https://github.com/rust-lang/git2-rs) - Rust Git 绑定
- [Vue.js](https://vuejs.org/) - 渐进式框架
- 所有开源贡献者

## 联系方式

- Issue Tracker: [GitHub Issues](https://github.com/WNLUO/CaoGit/issues)
- 讨论区: [GitHub Discussions](https://github.com/WNLUO/CaoGit/discussions)

---

如果这个项目对你有帮助，请给个 ⭐ Star！
