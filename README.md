# CaoGit

一个功能强大的跨平台 Git 图形化客户端，基于 Tauri 2.0 + Vue 3 + TypeScript 构建。

## 特性

### 核心功能
- **多仓库管理** - 同时管理多个 Git 仓库,快速切换
- **完整分支管理** - 创建、切换、删除、合并分支
- **智能提交** - 可视化 stage/unstage, 支持 AI 生成提交信息
- **远程同步** - Push、Pull、Fetch 操作,完整远程仓库支持
- **提交历史** - 美观的提交历史展示,支持搜索和过滤

### 可视化
- **分支可视化图** - Canvas 绘制的交互式分支图
- **高级 Diff 查看器** - 并排/内联两种模式,语法高亮
- **实时状态** - 工作区和暂存区实时状态展示

### 高级操作
- **Tag 管理** - 创建、查看、删除标签
- **Stash 功能** - 保存和恢复工作状态
- **Merge** - 智能合并,自动检测冲突
- **Clone** - 克隆远程仓库
- **AI 提交消息** - 基于 OpenAI API 分析代码变更自动生成提交信息
- **冲突解决工具** - 三栏可视化冲突编辑器
- **Cherry-pick** - 精选提交功能
- **Git Blame** - 代码行责任追踪
- **主题定制** - 暗黑/明亮/自动主题切换

### 未来计划
- **性能优化** - 大仓库虚拟滚动优化,增量加载
- **Rebase** - 交互式变基

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

首次运行会下载 Rust 依赖,可能需要几分钟。

### 构建

```bash
# 构建生产版本
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`

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

### 切换分支

1. 点击顶栏的当前分支名称
2. 在下拉菜单中选择目标分支
3. 或点击"新建"创建新分支

### 推送代码

1. 确保有未推送的提交
2. 点击顶栏的 "Push" 按钮
3. 等待推送完成

### 查看 Diff

1. 在文件列表中点击文件
2. 右侧会显示详细差异
3. 可切换"内联"或"并排"模式

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

### 目录结构

```
├── src/                    # Vue 前端代码
│   ├── components/         # Vue 组件
│   │   ├── Sidebar.vue    # 仓库侧边栏
│   │   ├── TopBar.vue     # 顶部操作栏
│   │   ├── ChangesView.vue # 文件变更视图
│   │   ├── HistoryView.vue # 提交历史
│   │   ├── EnhancedDiffView.vue # Diff 查看器
│   │   ├── BranchGraph.vue # 分支图
│   │   ├── BlameView.vue  # Git Blame 视图
│   │   ├── ConflictResolver.vue # 冲突解决器
│   │   └── ...            # 更多组件
│   ├── services/
│   │   └── gitApi.ts      # API 调用服务
│   ├── stores/
│   │   └── repoStore.ts   # 状态管理
│   ├── types.ts           # TypeScript 类型
│   └── main.ts            # 入口文件
│
├── src-tauri/              # Rust 后端代码
│   ├── src/
│   │   ├── git_ops.rs     # Git 核心操作
│   │   ├── commands.rs    # Tauri 命令
│   │   ├── repo_cache.rs  # 仓库缓存
│   │   └── lib.rs         # 主入口
│   └── Cargo.toml         # Rust 依赖
│
└── README.md              # 本文件
```

### 核心实现

**Git 操作层** (`git_ops.rs`)
- 封装 git2-rs 库
- 提供高层 API 接口
- 处理认证和代理

**命令层** (`commands.rs`)
- Tauri IPC 接口
- 参数验证
- 错误处理

**前端 API** (`gitApi.ts`)
- 调用 Tauri 命令
- 类型安全封装
- Promise 化接口

**状态管理** (`repoStore.ts`)
- 响应式仓库状态
- 自动刷新机制
- 错误处理

## 与其他工具对比

| 功能 | CaoGit | GitKraken | GitHub Desktop | SourceTree |
|------|-------------|-----------|----------------|------------|
| 分支可视化 | 是 | 是 | 否 | 是 |
| 并排 Diff | 是 | 是 | 部分 | 是 |
| 多仓库管理 | 是 | 是 | 否 | 是 |
| 免费 | 是 | 部分收费 | 是 | 是 |
| 跨平台 | 是 | 是 | 是 | macOS/Win |
| AI 功能 | 是 | 是 | 否 | 否 |
| 性能 | 轻量 | 资源占用高 | 中等 | 慢 |
| 本土化 | 中文 | 是 | 是 | 是 |

## 贡献

欢迎贡献代码、报告 Bug 或提出新功能建议!

### 开发流程

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 提交 Pull Request

### 代码规范

- Rust: `cargo fmt` + `cargo clippy`
- TypeScript/Vue: ESLint + Prettier
- 提交信息遵循 [Conventional Commits](https://www.conventionalcommits.org/)

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

如果这个项目对你有帮助,请给个 Star!
