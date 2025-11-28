#!/bin/bash

# 重新打包DMG，添加背景图和美化效果
# 这个脚本用于在GitHub Actions中修复DMG外观问题

set -e

# 设置更激进的清理策略
trap 'cleanup' EXIT

cleanup() {
  echo "🧹 清理临时文件..."
  rm -rf "$TEMP_CONTENT_DIR" 2>/dev/null || true
  rm -rf "$MOUNT_DIR" 2>/dev/null || true
  rm -f "${DMG_FILE}.bak" 2>/dev/null || true
  # 卸载任何挂载的DMG
  hdiutil detach /Volumes/CaoGit 2>/dev/null || true
}

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DMG_DIR="$PROJECT_ROOT/src-tauri/target/release/bundle/dmg"
ASSETS_DIR="$PROJECT_ROOT/src-tauri/assets"

echo "📍 查找DMG文件在: $DMG_DIR"
DMG_FILE=$(find "$DMG_DIR" -name "*.dmg" -type f | head -n 1)
if [ ! -f "$DMG_FILE" ]; then
  echo "❌ DMG file not found in $DMG_DIR"
  ls -la "$DMG_DIR" 2>/dev/null || echo "   目录不存在"
  exit 1
fi

DMG_NAME=$(basename "$DMG_FILE")
echo "✅ 找到DMG文件: $DMG_NAME"
echo "   大小: $(du -h "$DMG_FILE" | cut -f1)"

# 检查bundle_dmg.sh工具是否存在
BUNDLE_DMG_SCRIPT="$DMG_DIR/bundle_dmg.sh"
if [ ! -f "$BUNDLE_DMG_SCRIPT" ]; then
  echo "❌ bundle_dmg.sh script not found at $BUNDLE_DMG_SCRIPT"
  ls -la "$DMG_DIR"
  exit 1
fi

echo "📦 使用bundle_dmg.sh脚本重新打包..."

# 创建临时目录用于存储App
TEMP_CONTENT_DIR=$(mktemp -d)
MOUNT_DIR=$(mktemp -d)

echo "📂 临时目录: $TEMP_CONTENT_DIR"

# 挂载原始DMG并复制内容
echo "📂 挂载DMG..."
if ! hdiutil attach -nobrowse -readonly "$DMG_FILE" -mountpoint "$MOUNT_DIR" 2>/dev/null; then
  echo "❌ 无法挂载DMG"
  cleanup
  exit 1
fi

echo "📋 复制DMG内容..."
cp -r "$MOUNT_DIR"/* "$TEMP_CONTENT_DIR/" 2>/dev/null || true

# 立即卸载DMG，释放磁盘空间
echo "💾 卸载原始DMG..."
hdiutil detach "$MOUNT_DIR" 2>/dev/null || true

# 删除原始DMG以节省空间
echo "🗑️  删除原始DMG以释放空间..."
rm -f "$DMG_FILE"

# 现在重新创建DMG，带上美化参数
echo "🔨 重新创建DMG，添加背景图和配置..."
chmod +x "$BUNDLE_DMG_SCRIPT"

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

if [ -f "$DMG_FILE" ]; then
  echo "✅ DMG重新打包完成！"
  echo "   文件: $DMG_FILE"
  echo "   大小: $(du -h "$DMG_FILE" | cut -f1)"
else
  echo "❌ DMG重新打包失败"
  exit 1
fi
