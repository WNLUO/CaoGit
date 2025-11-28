#!/bin/bash

# 重新打包DMG，添加背景图和美化效果
# 这个脚本用于在GitHub Actions中修复DMG外观问题

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DMG_DIR="$PROJECT_ROOT/src-tauri/target/release/bundle/dmg"
APP_DIR="$PROJECT_ROOT/src-tauri/target/release/bundle/macos/CaoGit.app"
ASSETS_DIR="$PROJECT_ROOT/src-tauri/assets"

# 查找原始DMG文件
DMG_FILE=$(find "$DMG_DIR" -name "*.dmg" -type f | head -n 1)
if [ ! -f "$DMG_FILE" ]; then
  echo "❌ DMG file not found in $DMG_DIR"
  exit 1
fi

DMG_NAME=$(basename "$DMG_FILE")
TEMP_DMG="$DMG_DIR/temp_${DMG_NAME}"

echo "🔄 正在重新打包DMG: $DMG_NAME"

# 备份原始DMG
cp "$DMG_FILE" "${DMG_FILE}.bak"

# 删除原始DMG，我们将重新创建它
rm -f "$DMG_FILE"

# 检查create-dmg工具是否存在
CREATE_DMG_SCRIPT="$PROJECT_ROOT/src-tauri/target/release/bundle/share/create-dmg/create-dmg"
if [ ! -f "$CREATE_DMG_SCRIPT" ]; then
  # 如果不存在，尝试找其他位置
  CREATE_DMG_SCRIPT=$(find "$PROJECT_ROOT/src-tauri/target" -name "create-dmg" -type f 2>/dev/null | head -n 1)
fi

if [ ! -f "$CREATE_DMG_SCRIPT" ]; then
  echo "❌ create-dmg script not found"
  exit 1
fi

echo "📦 使用create-dmg脚本: $CREATE_DMG_SCRIPT"

# 创建临时App目录
TEMP_APP_DIR="/tmp/CaoGit_dmg_$$"
mkdir -p "$TEMP_APP_DIR"
cp -r "$APP_DIR" "$TEMP_APP_DIR/"

# 使用create-dmg重新创建DMG，添加背景图和其他参数
chmod +x "$CREATE_DMG_SCRIPT"

"$CREATE_DMG_SCRIPT" \
  --volname "CaoGit" \
  --background "$ASSETS_DIR/dmg-background.jpg" \
  --window-pos 100 100 \
  --window-size 1025 678 \
  --icon-size 128 \
  --icon "CaoGit.app" 240 300 \
  --app-drop-link 790 300 \
  "$DMG_FILE" \
  "$TEMP_APP_DIR"

# 清理临时文件
rm -rf "$TEMP_APP_DIR"

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
