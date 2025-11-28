# 贡献指南

感谢你有兴趣为 CaoGit 做出贡献！本文档将指导你如何参与项目开发。

## 行为准则

请在参与项目时保持友好和尊重。我们欢迎所有背景和经验水平的贡献者。

## 如何贡献

### 报告 Bug

1. 在 [GitHub Issues](https://github.com/WNLUO/CaoGit/issues) 中搜索是否已有相似问题
2. 如果没有，创建一个新的 Issue，包含以下信息：
   - 清晰的标题和描述
   - 复现步骤
   - 期望行为 vs 实际行为
   - 操作系统和版本
   - CaoGit 版本
   - 相关的错误日志或截图

### 提出新功能

1. 在 [GitHub Discussions](https://github.com/WNLUO/CaoGit/discussions) 中讨论你的想法
2. 如果得到正面反馈，创建一个 Feature Request Issue
3. 等待维护者确认后再开始开发

### 提交代码

#### 环境设置

```bash
# 1. Fork 项目到你的 GitHub 账户

# 2. 克隆你的 Fork
git clone https://github.com/你的用户名/CaoGit.git
cd CaoGit

# 3. 添加上游仓库
git remote add upstream https://github.com/WNLUO/CaoGit.git

# 4. 安装依赖
npm install

# 5. 启动开发模式
npm run tauri dev
```

#### 开发流程

1. **创建分支**
   ```bash
   git checkout -b feature/你的功能名称
   # 或
   git checkout -b fix/你要修复的问题
   ```

2. **进行开发**
   - 遵循现有的代码风格
   - 添加必要的测试
   - 更新文档（如果需要）

3. **运行测试**
   ```bash
   # 前端测试
   npm run test

   # Rust 测试
   cd src-tauri && cargo test

   # 构建检查
   npm run build
   ```

4. **提交更改**
   ```bash
   git add .
   git commit -m "feat: 添加了某个功能"
   ```

5. **推送并创建 PR**
   ```bash
   git push origin feature/你的功能名称
   ```
   然后在 GitHub 上创建 Pull Request

## 代码规范

### Git 提交信息

我们遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**类型 (type):**
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式（不影响功能）
- `refactor`: 重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建/工具相关

**示例:**
```
feat(branch): 添加分支重命名功能

实现了本地分支重命名功能，支持右键菜单操作。

Closes #123
```

### TypeScript / Vue 代码规范

- 使用 TypeScript 严格模式
- 组件使用 `<script setup>` 语法
- 使用 `defineProps` 和 `defineEmits` 定义类型
- 导入顺序：Vue -> 第三方库 -> 本地模块
- 使用有意义的变量和函数名

```typescript
// 好的示例
const isLoading = ref(false);
const handleSubmit = async () => { ... };

// 不好的示例
const x = ref(false);
const fn = async () => { ... };
```

### Rust 代码规范

- 遵循 Rust 官方风格指南
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 为公共 API 添加文档注释

```rust
/// 打开一个 Git 仓库
///
/// # Arguments
///
/// * `path` - 仓库路径
///
/// # Returns
///
/// 返回 `GitRepository` 实例或错误
pub fn open(path: &str) -> Result<Self> {
    // ...
}
```

## 项目结构

```
├── src/                        # Vue 前端
│   ├── components/             # 组件
│   │   ├── common/            # 通用组件
│   │   ├── layout/            # 布局组件
│   │   ├── modals/            # 弹窗组件
│   │   └── views/             # 视图组件
│   ├── i18n/                  # 国际化
│   ├── services/              # 服务层
│   ├── stores/                # 状态管理
│   └── types/                 # 类型定义
│
├── src-tauri/                  # Rust 后端
│   └── src/
│       ├── git_ops/           # Git 操作
│       └── commands/          # Tauri 命令
│
└── tests/                      # 测试文件
```

## 测试指南

### 前端测试

```bash
# 运行所有测试
npm run test

# 运行特定测试文件
npm run test -- tests/unit/i18n.test.ts

# 生成覆盖率报告
npm run test:coverage
```

### Rust 测试

```bash
cd src-tauri

# 运行所有测试
cargo test

# 运行特定测试
cargo test test_open_repository
```

## Pull Request 检查清单

提交 PR 前，请确保：

- [ ] 代码通过所有测试
- [ ] 代码格式正确（`npm run build` 无错误）
- [ ] 添加了必要的测试
- [ ] 更新了相关文档
- [ ] 提交信息遵循规范
- [ ] PR 描述清晰说明了更改内容

## 发布流程

维护者负责发布新版本：

1. 更新版本号
   ```bash
   npm run bump:patch  # 或 bump:minor, bump:major
   ```

2. 创建标签并推送
3. GitHub Actions 自动构建和发布

## 获取帮助

如果你有任何问题：

- 查看 [文档](https://github.com/WNLUO/CaoGit/wiki)
- 在 [Discussions](https://github.com/WNLUO/CaoGit/discussions) 中提问
- 加入我们的社区

感谢你的贡献！ 🎉
