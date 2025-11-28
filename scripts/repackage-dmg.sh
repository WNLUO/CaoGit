#!/bin/bash

# 重新打包DMG，添加背景图和美化效果
# 这个脚本用于在GitHub Actions中修复DMG外观问题

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DMG_DIR="$PROJECT_ROOT/src-tauri/target/release/bundle/dmg"
APP_DIR="$PROJECT_ROOT/src-tauri/target/release/bundle/macos/CaoGit.app"
ASSETS_DIR="$PROJECT_ROOT/src-tauri/assets"

# 查找原始DMG文件
echo "📍 查找DMG文件在: $DMG_DIR"
DMG_FILE=$(find "$DMG_DIR" -name "*.dmg" -type f | head -n 1)
if [ ! -f "$DMG_FILE" ]; then
  echo "❌ DMG file not found in $DMG_DIR"
  echo "   目录内容:"
  ls -la "$DMG_DIR" 2>/dev/null || echo "   目录不存在"
  exit 1
fi

DMG_NAME=$(basename "$DMG_FILE")
TEMP_DMG="$DMG_DIR/temp_${DMG_NAME}"

echo "✅ 找到DMG文件: $DMG_NAME"
echo "🔄 正在重新打包DMG..."

# 检查bundle_dmg.sh工具是否存在（它就是create-dmg）
BUNDLE_DMG_SCRIPT="$DMG_DIR/bundle_dmg.sh"

if [ ! -f "$BUNDLE_DMG_SCRIPT" ]; then
  echo "❌ bundle_dmg.sh script not found at $BUNDLE_DMG_SCRIPT"
  ls -la "$DMG_DIR"
  exit 1
fi

echo "📦 使用bundle_dmg.sh脚本: $BUNDLE_DMG_SCRIPT"

# 挂载原始DMG以提取其内容
MOUNT_DIR=$(mktemp -d)
echo "📂 挂载原始DMG到: $MOUNT_DIR"

hdiutil attach -nobrowse -readonly "$DMG_FILE" -mountpoint "$MOUNT_DIR" 2>/dev/null || {
  echo "❌ 无法挂载DMG"
  exit 1
}

# 复制DMG内容到临时目录
TEMP_CONTENT_DIR="/tmp/CaoGit_content_$$"
mkdir -p "$TEMP_CONTENT_DIR"
cp -r "$MOUNT_DIR"/* "$TEMP_CONTENT_DIR/"

# 卸载原始DMG
hdiutil detach "$MOUNT_DIR" 2>/dev/null || true
rm -rf "$MOUNT_DIR"

# 备份原始DMG
cp "$DMG_FILE" "${DMG_FILE}.bak"

# 删除原始DMG，我们将重新创建它
rm -f "$DMG_FILE"

# 使用bundle_dmg.sh（create-dmg）重新创建DMG，添加背景图和其他参数
chmod +x "$BUNDLE_DMG_SCRIPT"

echo "🔨 使用bundle_dmg.sh重新打包DMG..."

"$BUNDLE_DMG_SCRIPT" \
  --volname "CaoGit" \
  --background "$ASSETS_DIR/dmg-background.jpg" \
  --window-pos 100 100 \
  --window-size 1025 678 \
  --icon-size 128 \
  --icon "CaoGit.app" 240 300 \
  --app-drop-link 790 300 \
  "$DMG_FILE" \
  "$TEMP_CONTENT_DIR"

# 清理临时文件
rm -rf "$TEMP_CONTENT_DIR"

if [ -f "$DMG_FILE" ]; then
  echo "✅ DMG重新打包完成！"
  echo "   文件: $DMG_FILE"
  ls -lh "$DMG_FILE"
else
  echo "❌ DMG重新打包失败"
  # 恢复备份
  cp "${DMG_FILE}.bak" "$DMG_FILE"
  exit 1
fi

# 删除备份
rm -f "${DMG_FILE}.bak"
