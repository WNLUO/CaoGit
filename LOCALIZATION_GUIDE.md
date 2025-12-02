# macOS 本地化配置指南

本文档说明如何为 CaoGit 应用配置 macOS 本地化支持，使文件对话框和系统界面显示为中文。

## 背景

macOS 应用的文件选择器对话框语言由应用的本地化配置决定。如果应用没有正确配置本地化，系统对话框会默认显示为英文，即使用户的系统语言设置为中文。

## 解决方案

我们通过以下步骤实现了本地化支持：

### 1. 创建本地化资源文件

创建了两个本地化目录：

```
src-tauri/resources/
├── zh_CN.lproj/
│   └── InfoPlist.strings
└── en.lproj/
    └── InfoPlist.strings
```

**zh_CN.lproj/InfoPlist.strings** (中文)：
```
CFBundleName = "CaoGit";
CFBundleDisplayName = "CaoGit";
CFBundleGetInfoString = "CaoGit Git 管理器";
NSHumanReadableCopyright = "Copyright © 2024 CaoGit. All rights reserved.";
```

**en.lproj/InfoPlist.strings** (英文)：
```
CFBundleName = "CaoGit";
CFBundleDisplayName = "CaoGit";
CFBundleGetInfoString = "CaoGit Git Manager";
NSHumanReadableCopyright = "Copyright © 2024 CaoGit. All rights reserved.";
```

### 2. 创建构建后脚本

创建了 `src-tauri/scripts/add-localization.sh` 脚本，在构建完成后自动：

1. 修改 `Info.plist` 设置 `CFBundleDevelopmentRegion` 为 `zh_CN`
2. 添加 `CFBundleLocalizations` 数组包含 `zh_CN` 和 `en`
3. 复制本地化资源文件到应用包
4. 将 `.strings` 文件编译为二进制格式

### 3. 创建 DMG 重新打包脚本

创建了 `src-tauri/scripts/fix-dmg-localization.sh` 脚本，该脚本：

1. 在 Tauri 构建完成后运行
2. 对 `target/release/bundle/macos/CaoGit.app` 应用本地化
3. 使用 Tauri 的 `bundle_dmg.sh` 重新创建 DMG
4. 保留所有 DMG 自定义设置（背景图、布局等）

### 4. 更新构建脚本

修改了 `package.json` 添加构建后本地化脚本：

```json
{
  "scripts": {
    "tauri:build": "tauri build && npm run post-build:localize-dmg",
    "tauri:build:appstore": "npm run build:appstore && ... && npm run post-build:localize-pkg",
    "post-build:localize-dmg": "cd src-tauri && bash scripts/fix-dmg-localization.sh",
    "post-build:localize-pkg": "cd src-tauri && bash scripts/add-localization.sh target/release/bundle/macos/CaoGit.app"
  }
}
```

**工作流程**:
1. `tauri build` 创建应用和 DMG
2. `fix-dmg-localization.sh` 应用本地化到 macos bundle
3. 使用相同参数重新创建 DMG（包含本地化的应用）

### 5. 更新资源配置

修改了 `tauri.conf.json` 包含本地化资源：

```json
{
  "bundle": {
    "resources": [
      "resources/fix-macos.command",
      "resources/README-macOS.txt",
      "resources/zh_CN.lproj",
      "resources/en.lproj"
    ]
  }
}
```

## 工作原理

1. **CFBundleDevelopmentRegion**: 设置为 `zh_CN`，告诉 macOS 应用的主要开发语言是中文
2. **CFBundleLocalizations**: 声明应用支持的语言列表
3. **.lproj 目录**: macOS 标准的本地化资源目录结构
4. **InfoPlist.strings**: 包含本地化的应用元数据

当用户的 macOS 系统语言设置为中文时：
- 系统会查找应用是否有 `zh_CN.lproj` 目录
- 如果找到，系统对话框（如文件选择器）会显示为中文
- 如果系统语言是英文，则使用 `en.lproj`

## 验证本地化

构建完成后，可以验证本地化配置：

```bash
# 挂载 DMG
hdiutil attach src-tauri/target/release/bundle/dmg/CaoGit_*.dmg -nobrowse -readonly

# 检查 DMG 中应用的 Info.plist
plutil -p /Volumes/CaoGit/CaoGit.app/Contents/Info.plist | grep -E "CFBundleDevelopmentRegion|CFBundleLocalizations"

# 应该看到:
# "CFBundleDevelopmentRegion" => "zh_CN"
# "CFBundleLocalizations" => [
#   0 => "zh_CN"
#   1 => "en"
# ]

# 检查本地化目录
ls -la /Volumes/CaoGit/CaoGit.app/Contents/Resources/*.lproj

# 检查 .strings 文件格式（应该是二进制）
file /Volumes/CaoGit/CaoGit.app/Contents/Resources/zh_CN.lproj/InfoPlist.strings

# 卸载 DMG
hdiutil detach /Volumes/CaoGit
```

## 测试

1. 确保你的 macOS 系统语言设置为中文
2. 运行应用并打开文件选择对话框
3. 对话框应该显示中文按钮（"打开"、"取消"等）

如果系统语言是英文，对话框会显示英文。

## 注意事项

1. **构建时自动应用**: 每次运行 `npm run tauri:build` 时会自动应用本地化配置
2. **需要重新签名**: 修改应用包后需要重新签名（构建流程已自动处理）
3. **系统语言优先**: 对话框语言由用户的系统语言设置决定，不是应用内设置
4. **支持多语言**: 可以通过添加更多 `.lproj` 目录支持其他语言

## 添加新语言

要添加新语言支持（例如日语）：

1. 创建 `src-tauri/resources/ja.lproj/InfoPlist.strings`
2. 在脚本中添加复制和编译逻辑
3. 在 `add-localization.sh` 中添加到 `CFBundleLocalizations` 数组

```bash
/usr/libexec/PlistBuddy -c "Add :CFBundleLocalizations:2 string ja" "$INFO_PLIST"
```

## DMG 本地化的特殊处理

由于 Tauri 在构建时会立即将 app bundle 打包成 DMG，我们采用了"构建后重新打包"的策略：

1. **问题**: Tauri 的 `tauri build` 命令会：
   - 创建 `target/release/bundle/macos/CaoGit.app`
   - 立即使用 `bundle_dmg.sh` 将其打包成 DMG
   - 无法在中间插入本地化步骤

2. **解决方案**:
   - 让 Tauri 完成正常构建
   - 运行 `fix-dmg-localization.sh` 脚本：
     - 对 macos bundle 中的 app 应用本地化
     - 删除未本地化的 DMG
     - 使用 Tauri 的 `bundle_dmg.sh` 重新创建 DMG
     - 保留所有 DMG 自定义设置（背景、图标位置、窗口大小等）

3. **优点**:
   - 不修改 Tauri 的构建流程
   - 保留 DMG 的所有自定义配置
   - 可以在已有的构建上独立运行
   - 不破坏代码签名流程

## 相关文件

- `src-tauri/resources/zh_CN.lproj/InfoPlist.strings` - 中文本地化
- `src-tauri/resources/en.lproj/InfoPlist.strings` - 英文本地化
- `src-tauri/scripts/add-localization.sh` - 本地化配置脚本
- `src-tauri/scripts/fix-dmg-localization.sh` - DMG 重新打包脚本
- `package.json` - 构建脚本配置
- `tauri.conf.json` - 资源文件配置

## 参考资料

- [Apple 本地化文档](https://developer.apple.com/documentation/xcode/localizing-your-app)
- [Info.plist 键值参考](https://developer.apple.com/library/archive/documentation/General/Reference/InfoPlistKeyReference/)
- [Tauri 本地化指南](https://tauri.app/v1/guides/features/internationalization/)
