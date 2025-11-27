# Phase 2 功能使用指南

本指南介绍如何使用 Phase 2 新增的四大功能。

---

## 🌓 1. 暗黑模式主题系统

### 使用方法:

**界面切换:**
- 点击顶部导航栏右侧的主题切换按钮
- 图标会根据当前主题显示：☀️ (明亮) / 🌙 (暗黑) / 🌗 (自动)

**三种模式:**
1. **Light (明亮)** - 始终使用浅色主题
2. **Dark (暗黑)** - 始终使用深色主题
3. **Auto (自动)** - 跟随系统主题设置

**持久化:**
- 主题选择会自动保存到 LocalStorage
- 刷新页面后保持您的主题偏好

**代码集成示例:**
```typescript
import { setTheme, toggleTheme, currentTheme } from '@/stores/themeStore';

// 设置特定主题
setTheme('dark');

// 在三种模式间切换
toggleTheme();

// 获取当前主题
console.log(currentTheme.value); // 'light' | 'dark' | 'auto'
```

---

## 🔍 2. Git Blame 文件历史追踪

### 功能说明:
查看文件每一行的修改历史，包括作者、提交时间和提交哈希。

### 使用 BlameView 组件:

```vue
<template>
  <BlameView
    :repo-path="'/path/to/repo'"
    :file-path="'src/main.ts'"
    @close="handleClose"
    @jump-to-commit="handleJumpToCommit"
  />
</template>

<script setup lang="ts">
import BlameView from '@/components/BlameView.vue';

function handleClose() {
  // 关闭 Blame 视图
}

function handleJumpToCommit(hash: string) {
  // 跳转到对应的提交详情
  console.log('Jump to commit:', hash);
}
</script>
```

### 界面特性:
- **行号列**: 显示代码行号
- **提交列**: 显示提交哈希 (点击可复制)
- **作者列**: 彩色标签显示作者名称
- **日期列**: 相对时间显示 (如 "2天前")
- **代码列**: 显示实际代码内容

### API 调用:
```typescript
import { GitApi } from '@/services/gitApi';

const result = await GitApi.getFileBlame(repoPath, 'src/App.vue');
if (result.success && result.data) {
  result.data.forEach(line => {
    console.log(`Line ${line.line_number}: ${line.author} - ${line.commit_hash}`);
  });
}
```

---

## ⚔️ 3. 冲突解决工具

### 功能说明:
合并分支时出现冲突，使用三栏编辑器可视化对比和解决冲突。

### 使用 ConflictResolver 组件:

```vue
<template>
  <ConflictResolver
    :repo-path="'/path/to/repo'"
    @close="handleClose"
    @resolved="handleResolved"
  />
</template>

<script setup lang="ts">
import ConflictResolver from '@/components/ConflictResolver.vue';

function handleClose() {
  // 关闭冲突解决器
}

function handleResolved() {
  // 所有冲突已解决
  console.log('All conflicts resolved!');
}
</script>
```

### 界面布局:

```
┌──────────────────────────────────────────────────────┐
│  冲突 1/3                                             │
│  src/components/App.vue                               │
├──────────────────┬──────────────────┬─────────────────┤
│  Ours (当前分支)  │  Base (共同祖先)  │  Theirs (合并)  │
│  [使用此版本]     │                  │  [使用此版本]    │
│                  │                  │                 │
│  const x = 1;    │  const x = 0;    │  const x = 2;   │
│  // Feature A    │  // Original     │  // Feature B   │
└──────────────────┴──────────────────┴─────────────────┘
┌──────────────────────────────────────────────────────┐
│  解决方案 (最终版本)            [合并两者]            │
├──────────────────────────────────────────────────────┤
│  const x = 1;                                         │
│  // Feature A                                         │
│  (在此编辑最终版本)                                    │
└──────────────────────────────────────────────────────┘
  [← 上一个]  [下一个 →]    [中止合并]  [解决并继续]
```

### 快速操作:
1. **使用 Ours** - 保留当前分支的版本
2. **使用 Theirs** - 使用合并分支的版本
3. **合并两者** - 同时保留两个版本
4. **手动编辑** - 在底部编辑器中自定义最终版本

### API 调用:
```typescript
import { GitApi } from '@/services/gitApi';

// 1. 获取冲突列表
const conflicts = await GitApi.getConflicts(repoPath);
if (conflicts.success && conflicts.data) {
  console.log(`Found ${conflicts.data.length} conflicts`);
}

// 2. 解决冲突
const resolution = "const x = 1;\n// Final version";
await GitApi.resolveConflict(repoPath, 'src/App.vue', resolution);

// 3. 中止合并 (如果需要)
await GitApi.abortMerge(repoPath);
```

---

## 🍒 4. Cherry-pick 功能

### 功能说明:
将指定的提交应用到当前分支，无需完整合并。

### 使用方法:

**在 HistoryView 中使用:**
1. 在提交历史中右键点击一个提交
2. 选择 "Cherry-pick 此提交"
3. 等待操作完成，查看结果提示

**批量 Cherry-pick:**
1. 按住 Ctrl (Windows) 或 Cmd (Mac)
2. 点击选择多个提交
3. 右键菜单会显示 "Cherry-pick N 提交"
4. 确认执行批量操作

### 右键菜单选项:
```
┌─────────────────────────────┐
│ 🍒 Cherry-pick 此提交        │
│ 📋 复制提交哈希              │
│ 📝 复制提交信息              │
└─────────────────────────────┘
```

### API 调用:

**单个提交:**
```typescript
import { GitApi } from '@/services/gitApi';

const result = await GitApi.cherryPick(repoPath, 'abc123def');
if (result.success) {
  console.log('Cherry-pick 成功:', result.data);
} else {
  console.error('Cherry-pick 失败:', result.error);
}
```

**批量提交:**
```typescript
const commits = ['abc123', 'def456', 'ghi789'];
const result = await GitApi.cherryPickBatch(repoPath, commits);

if (result.success && result.data) {
  result.data.forEach(msg => console.log(msg));
  // 输出示例:
  // abc123: Cherry-pick successful
  // def456: Cherry-pick successful
  // ghi789: Failed - conflict detected
}
```

### 冲突处理:
如果 cherry-pick 遇到冲突:
1. 系统会提示 "Cherry-pick has conflicts"
2. 自动打开冲突解决工具
3. 解决冲突后，操作自动继续

---

## 📚 完整工作流示例

### 场景: 从功能分支选择性合并提交到主分支

```typescript
import { GitApi } from '@/services/gitApi';

async function selectiveMerge(repoPath: string) {
  // 1. 切换到主分支
  await GitApi.checkoutBranch(repoPath, 'main');

  // 2. Cherry-pick 特定的功能提交
  const commits = ['abc123', 'def456'];
  const result = await GitApi.cherryPickBatch(repoPath, commits);

  // 3. 检查是否有冲突
  if (result.error?.includes('conflicts')) {
    // 打开冲突解决器
    const conflicts = await GitApi.getConflicts(repoPath);
    // ... 用户手动解决冲突
  }

  // 4. 查看 Blame 确认更改
  const blame = await GitApi.getFileBlame(repoPath, 'src/feature.ts');

  // 5. 完成！
  console.log('Selective merge completed!');
}
```

---

## 🔧 故障排除

### Blame 显示失败
- 确保文件路径正确
- 检查文件是否在 Git 版本控制中
- 验证仓库路径是否有效

### 冲突解决器无法打开
- 确保当前确实处于合并冲突状态
- 检查 Git 索引是否损坏
- 尝试使用 `git status` 命令验证状态

### Cherry-pick 失败
- 检查提交哈希是否正确
- 确保工作目录干净 (无未提交更改)
- 查看错误消息获取详细信息

---

## 🎯 快捷键 (计划中)

未来版本将支持:
- `Cmd/Ctrl + B` - 打开当前文件的 Blame 视图
- `Cmd/Ctrl + Shift + C` - Cherry-pick 选中的提交
- `Cmd/Ctrl + Shift + R` - 打开冲突解决器

---

## 📞 获取帮助

如果遇到问题:
1. 查看浏览器控制台错误信息
2. 检查 Rust 后端日志
3. 参考 `PHASE2_IMPLEMENTATION_SUMMARY.md` 了解技术细节

---

**最后更新:** 2025-11-27
**版本:** Phase 2.0
