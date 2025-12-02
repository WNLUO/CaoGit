#!/bin/bash

# 在 Tauri 构建完成后重新打包 DMG,包含本地化设置
# 这个脚本会:
# 1. 找到现有的 DMG
# 2. 在 macos bundle 中应用本地化
# 3. 使用 Tauri 的 bundle_dmg.sh 重新创建 DMG

set -e

echo "===== Rebuilding DMG with Localization ====="

# 查找 DMG 文件
DMG_PATH=$(find target/release/bundle/dmg -name "*.dmg" -type f | head -1)

if [ -z "$DMG_PATH" ]; then
    echo "No DMG found, skipping localization"
    exit 0
fi

echo "Found DMG: $DMG_PATH"

# 获取 DMG 相关信息
DMG_NAME=$(basename "$DMG_PATH")
DMG_DIR=$(dirname "$DMG_PATH")
VERSION=$(echo "$DMG_NAME" | sed 's/CaoGit_\(.*\)_aarch64\.dmg/\1/')

# 应用本地化到 macos app bundle
APP_PATH="target/release/bundle/macos/CaoGit.app"

if [ ! -d "$APP_PATH" ]; then
    echo "Error: App bundle not found at $APP_PATH"
    exit 1
fi

echo "Applying localization to $APP_PATH..."
bash scripts/add-localization.sh "$APP_PATH"

# 获取绝对路径（在改变目录之前）
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_TAURI_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
BG_IMAGE="$SRC_TAURI_DIR/assets/dmg-background.jpg"

# 删除旧的 DMG
echo "Removing old DMG..."
rm -f "$DMG_PATH"

# 使用 Tauri 的 bundle_dmg.sh 重新创建 DMG
echo "Recreating DMG with localized app..."
cd "$DMG_DIR"

if [ ! -f "$BG_IMAGE" ]; then
    echo "Warning: Background image not found at $BG_IMAGE"
    BG_IMAGE=""
fi

if [ -n "$BG_IMAGE" ]; then
    bash bundle_dmg.sh \
      --volname "CaoGit" \
      --background "$BG_IMAGE" \
      --window-pos 0 0 \
      --window-size 1025 678 \
      --icon-size 160 \
      --icon "CaoGit.app" 240 300 \
      --app-drop-link 790 300 \
      "$DMG_NAME" \
      ../macos
else
    bash bundle_dmg.sh \
      --volname "CaoGit" \
      --window-pos 0 0 \
      --window-size 1025 678 \
      --icon-size 160 \
      --icon "CaoGit.app" 240 300 \
      --app-drop-link 790 300 \
      "$DMG_NAME" \
      ../macos
fi

cd - > /dev/null

echo "✅ DMG rebuilt successfully with localization!"
echo "DMG location: $DMG_PATH"
