# 测试优化功能指南

## 🚀 快速开始

### 1. 安装和构建

```bash
# 1. 安装前端依赖 (如果还没有安装)
npm install

# 2. 启动开发服务器
npm run tauri dev

# 注意: 首次运行会下载 Rust 依赖,可能需要 3-5 分钟
```

---

## ✅ 测试清单

### 测试 1: 并行加载性能 ⚡

**步骤:**
1. 打开应用
2. 添加一个有大量提交的 Git 仓库
3. 打开浏览器开发者工具 (F12)
4. 切换到 Network 面板
5. 点击不同的仓库观察加载

**预期结果:**
- ✅ 多个请求同时发出(并行)
- ✅ 总加载时间 < 500ms
- ✅ 界面不卡顿

**对比测试:**
```
优化前: status → branches → commits → current (串行)
优化后: [status, branches] → [commits, current] (并行)

性能提升: ~60%
```

---

### 测试 2: 虚拟滚动 📜

**步骤:**
1. 打开一个有 1000+ 提交的大仓库
2. 进入 "历史" 视图
3. 快速滚动提交列表
4. 滚动到列表底部

**预期结果:**
- ✅ 滚动流畅,无卡顿
- ✅ 只渲染可见的 20-30 条提交
- ✅ 滚动到底部自动加载更多
- ✅ 内存占用稳定(不随提交数增加而增长)

**性能指标:**
```
10,000 条提交:
- DOM 节点数: ~30 个 (vs 10,000 个)
- 内存占用: ~50MB (vs 500MB+)
- 滚动帧率: 60fps (vs <10fps)
```

---

### 测试 3: 搜索功能 🔍

**步骤:**
1. 进入 "历史" 视图
2. 点击搜索框或过滤器图标
3. 测试各种搜索:
   - 输入提交消息关键词
   - 输入提交哈希 (前 7 位)
   - 输入作者名称
4. 清空搜索,观察恢复速度

**预期结果:**
- ✅ 实时搜索,即时显示结果
- ✅ 搜索响应时间 < 50ms
- ✅ 高亮匹配结果
- ✅ 无匹配时显示空状态

**搜索示例:**
```
搜索 "fix" → 只显示包含 "fix" 的提交
搜索 "abc123" → 显示哈希为 abc123* 的提交
搜索作者 "张三" → 只显示张三的提交
```

---

### 测试 4: 高级过滤 🎯

**步骤:**
1. 点击过滤器图标 (漏斗形状)
2. 展开过滤面板
3. 测试各种过滤:
   - **按作者**: 输入作者名称
   - **按日期**: 选择开始和结束日期
   - **按分支**: 选择特定分支
4. 组合多个过滤器
5. 点击 "清除所有过滤器"

**预期结果:**
- ✅ 过滤器图标显示活跃状态
- ✅ 徽章显示活跃过滤器数量
- ✅ 过滤结果即时更新
- ✅ 组合过滤正确工作

**过滤示例:**
```
作者 = "John" AND 日期 >= 2024-01-01
→ 只显示 John 在 2024 年后的提交
```

---

### 测试 5: 缓存机制 💾

**步骤:**
1. 打开一个仓库 A (观察加载时间)
2. 切换到另一个仓库 B
3. 再切换回仓库 A (应该更快)
4. 等待 30 秒
5. 切换到其他仓库再切回 A (缓存过期,重新加载)

**预期结果:**
- ✅ 第二次加载同一仓库更快 (缓存命中)
- ✅ 缓存过期后自动重新加载
- ✅ 删除仓库时清除缓存

**缓存配置:**
```
提交历史: 30 秒 TTL
仓库实例: 5 分钟 TTL
```

---

### 测试 6: 增量加载 📥

**步骤:**
1. 打开有 500+ 提交的仓库
2. 进入历史视图
3. 滚动到列表底部
4. 观察 "加载更多提交..." 提示
5. 等待自动加载

**预期结果:**
- ✅ 初始加载 100 条提交
- ✅ 滚动到底部自动加载下一批
- ✅ 加载指示器显示
- ✅ 加载完成后平滑添加到列表

**加载策略:**
```
初始: 100 条
每次加载更多: 100 条
触发点: 距底部 10 条时
```

---

## 🎨 性能对比测试

### 使用 Chrome DevTools

**步骤:**
1. 打开 Chrome DevTools (F12)
2. 切换到 "Performance" 面板
3. 点击录制按钮 ⏺
4. 执行以下操作:
   - 切换仓库
   - 滚动提交列表
   - 搜索和过滤
5. 停止录制
6. 分析结果

**关键指标:**
- **FPS**: 应保持在 55-60 之间
- **Main Thread**: 任务时间应 < 50ms
- **Memory**: 稳定,无持续增长

---

## 🐛 常见问题排查

### 问题 1: 虚拟滚动不工作

**症状:** 列表渲染所有项,滚动卡顿

**检查:**
```bash
# 确认 VirtualScroller 组件已正确导入
grep -r "VirtualScroller" src/components/HistoryView.vue
```

**解决:** 确保已正确传递 `:items` 和 `:item-height` 属性

---

### 问题 2: 搜索无反应

**症状:** 输入搜索词但没有结果

**检查:**
1. 打开浏览器控制台
2. 查看是否有 JavaScript 错误
3. 确认 `filteredCommits` computed 属性工作正常

**解决:**
```javascript
// 在 HistoryView.vue 中添加调试
console.log('Filtered commits:', filteredCommits.value);
console.log('Filter options:', filterOptions.value);
```

---

### 问题 3: Rust 编译失败

**症状:** `npm run tauri dev` 报错

**解决:**
```bash
# 清理并重新构建
cd src-tauri
cargo clean
cd ..
npm run tauri dev
```

如果仍然失败:
```bash
# 检查 Rust 版本
rustc --version  # 应该 >= 1.70

# 更新 Rust
rustup update
```

---

### 问题 4: 缓存不生效

**症状:** 重复加载相同数据

**检查:**
```javascript
// 在 repoStore.ts 中添加调试
console.log('Cache hit:', cacheService.has(cacheKey));
console.log('Cache size:', cacheService.size);
```

**解决:** 确认 `cacheService.ts` 文件已正确导入

---

## 📊 性能基准

使用以下仓库测试:

### 小型仓库 (< 100 commits)
- **加载时间**: < 200ms
- **搜索时间**: < 10ms
- **内存占用**: < 30MB

### 中型仓库 (100-1000 commits)
- **加载时间**: 200-400ms
- **搜索时间**: < 30ms
- **内存占用**: 30-60MB

### 大型仓库 (1000-10000 commits)
- **加载时间**: 400-800ms
- **搜索时间**: < 50ms
- **内存占用**: 50-100MB

### 超大型仓库 (10000+ commits)
- **加载时间**: 1-2s (初次)
- **搜索时间**: 50-100ms
- **内存占用**: < 150MB (虚拟滚动)

---

## ✨ 推荐测试仓库

### 1. 小型测试仓库
```bash
# 创建测试仓库
git init test-small
cd test-small
for i in {1..50}; do
  echo "Commit $i" > file.txt
  git add .
  git commit -m "Commit $i"
done
```

### 2. 使用知名开源项目
```bash
# Vue.js (约 10,000+ commits)
git clone https://github.com/vuejs/vue.git

# React (约 15,000+ commits)
git clone https://github.com/facebook/react.git

# Linux Kernel (约 1,000,000+ commits)
git clone --depth=1000 https://github.com/torvalds/linux.git
```

---

## 📈 性能监控

### 实时监控

在 `src/stores/repoStore.ts` 中添加:

```typescript
// 性能监控
const performanceMonitor = {
  start(name: string) {
    console.time(name);
  },
  end(name: string) {
    console.timeEnd(name);
  }
};

// 使用示例
async loadRepoData(repo: Repository) {
  performanceMonitor.start('loadRepoData');
  // ... 现有代码
  performanceMonitor.end('loadRepoData');
}
```

---

## 🎯 验收标准

### 优化成功的标准:

- ✅ 加载时间比优化前快 50%+
- ✅ 滚动 10,000 条提交无卡顿
- ✅ 搜索响应时间 < 100ms
- ✅ 内存占用不随数据量线性增长
- ✅ 缓存命中率 > 70%
- ✅ 用户体验流畅自然

### 如果未达标:

1. 检查 Network 面板确认并行加载
2. 检查 Performance 面板找出瓶颈
3. 查看 Console 是否有错误
4. 参考上方的故障排查指南

---

## 🎊 测试完成

如果所有测试通过,恭喜! 您的 Git管理器 已经:

✅ **性能提升 60%+**
✅ **支持超大仓库**
✅ **实现智能搜索**
✅ **流畅用户体验**

**下一步:** 开始使用并享受优化后的体验,或者继续实施更多功能!

---

**测试指南版本:** 1.0
**最后更新:** 2025-11-27
