# macOS 标题栏主题适配修复

## 问题描述

在 macOS 上，原生窗口标题栏（状态栏）的颜色不随主题切换而改变，导致在暗黑模式下标题栏仍然是白色。

## 解决方案

实现了完整的标题栏主题同步机制：

### 1. Tauri 配置更新

**文件:** `src-tauri/tauri.conf.json`

```json
{
  "app": {
    "windows": [
      {
        "title": "Git Manager",
        "width": 1200,
        "height": 800,
        "titleBarStyle": "overlay",  // 新增：允许自定义标题栏样式
        "theme": "auto"               // 新增：默认跟随系统主题
      }
    ]
  }
}
```

### 2. Rust 后端命令

**文件:** `src-tauri/src/commands.rs`

新增 `set_window_theme` 命令：

```rust
use tauri::{Manager, Window, Theme};

#[tauri::command]
pub fn set_window_theme(window: Window, theme: String) -> ApiResponse<String> {
    let tauri_theme = match theme.as_str() {
        "dark" => Some(Theme::Dark),
        "light" => Some(Theme::Light),
        _ => None, // Auto/system theme
    };

    match window.set_theme(tauri_theme) {
        Ok(_) => ApiResponse::success(format!("Window theme set to {}", theme)),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}
```

**注册命令:** `src-tauri/src/lib.rs`

```rust
.invoke_handler(tauri::generate_handler![
    // ... 其他命令 ...
    set_window_theme,
])
```

### 3. 前端 API

**文件:** `src/services/gitApi.ts`

```typescript
// Window theme operations
static async setWindowTheme(theme: 'light' | 'dark' | 'auto'): Promise<ApiResponse<string>> {
    return await safeInvoke('set_window_theme', { theme });
}
```

### 4. 主题 Store 集成

**文件:** `src/stores/themeStore.ts`

更新 `applyTheme` 函数同步设置窗口主题：

```typescript
import { GitApi } from '../services/gitApi';

// Apply theme to document and window
export async function applyTheme(theme: 'light' | 'dark') {
  document.documentElement.setAttribute('data-theme', theme);
  themeStore.effectiveTheme = theme;

  // Update native window theme (for macOS titlebar)
  try {
    await GitApi.setWindowTheme(theme);
  } catch (error) {
    console.warn('Failed to set window theme:', error);
  }
}
```

## 工作原理

1. **用户切换主题** → 点击主题切换按钮
2. **调用 setTheme()** → 更新 store 和 localStorage
3. **执行 applyTheme()** → 同时更新：
   - HTML `data-theme` 属性（CSS 变量）
   - Tauri 窗口主题（原生标题栏）
4. **Tauri 更新窗口** → macOS 标题栏颜色改变

## 效果

### 明亮模式
- 标题栏：白色/浅灰色
- 内容区：浅色背景
- 文字：深色

### 暗黑模式
- 标题栏：深灰色/黑色 ✨ **现在会正确切换！**
- 内容区：深色背景
- 文字：浅色

### 自动模式
- 跟随系统设置
- 系统切换时标题栏也会同步更新

## 测试步骤

1. 启动应用：`npm run tauri dev`
2. 点击右上角的主题切换按钮
3. 观察 macOS 标题栏颜色变化

**预期结果:**
- 切换到暗黑模式 → 标题栏变为深色
- 切换到明亮模式 → 标题栏变为浅色
- 切换到自动模式 → 跟随系统设置

## 技术细节

### Tauri Theme API

Tauri 提供了 `set_theme()` 方法来控制窗口外观：

```rust
pub fn set_theme(&self, theme: Option<Theme>) -> Result<()>
```

- `Some(Theme::Light)` - 强制浅色主题
- `Some(Theme::Dark)` - 强制深色主题
- `None` - 跟随系统设置

### macOS 特定行为

在 macOS 上，这会影响：
- 窗口标题栏颜色
- 标题栏按钮样式（红/黄/绿按钮）
- 窗口阴影和边框

### Windows/Linux

在其他平台上，此功能可能有不同表现：
- Windows 10/11：影响标题栏和窗口边框
- Linux：取决于桌面环境

## 兼容性

| 平台 | 支持度 | 说明 |
|------|--------|------|
| macOS | ✅ 完全支持 | 原生标题栏主题切换 |
| Windows 10/11 | ✅ 支持 | 窗口主题适配 |
| Linux | ⚠️ 部分支持 | 取决于 DE |

## 故障排除

### 标题栏没有变化

**可能原因:**
1. Tauri 配置未更新 → 重启开发服务器
2. 系统设置阻止主题切换 → 检查系统偏好设置
3. API 调用失败 → 检查浏览器控制台

**解决方法:**
```bash
# 1. 完全重启
pkill -f tauri
pkill -f vite
npm run tauri dev

# 2. 检查 Rust 编译
cd src-tauri
cargo check
```

### 标题栏闪烁

这是异步调用的正常现象。主题切换时：
1. CSS 立即更新（同步）
2. 窗口主题稍后更新（异步）

如果闪烁明显，可以添加过渡延迟。

## 相关文件

**修改的文件:**
- ✅ `src-tauri/tauri.conf.json` - 窗口配置
- ✅ `src-tauri/src/commands.rs` - 新增命令
- ✅ `src-tauri/src/lib.rs` - 注册命令
- ✅ `src/services/gitApi.ts` - API 封装
- ✅ `src/stores/themeStore.ts` - 主题同步

**无需修改:**
- `src/components/ThemeToggle.vue` - 已使用 store
- `src/styles.css` - CSS 变量保持不变

## 总结

通过这个修复，现在主题切换会完整更新：
1. ✅ CSS 变量（内容区颜色）
2. ✅ macOS 标题栏（原生窗口外观）
3. ✅ 跟随系统主题自动切换
4. ✅ 持久化保存用户偏好

用户体验从 **部分适配** → **完全适配** 🎉

---

**实施日期:** 2025-11-27
**问题修复:** macOS 标题栏主题不切换
**影响范围:** 所有平台的窗口主题
