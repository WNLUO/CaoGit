#!/bin/bash

# 创建包含本地化应用的 DMG
# 这个脚本会:
# 1. 对 macos bundle 中的应用应用本地化
# 2. 使用 Tauri 的配置创建 DMG

set -e

echo "===== Creating Localized DMG ====="

# 检查应用是否存在
APP_PATH="target/release/bundle/macos/CaoGit.app"

if [ ! -d "$APP_PATH" ]; then
    echo "Error: App bundle not found at $APP_PATH"
    echo "Please run 'tauri build --bundles app' first"
    exit 1
fi

# 应用本地化
echo "Applying localization to $APP_PATH..."
bash scripts/add-localization.sh "$APP_PATH"

# 读取版本号
VERSION=$(grep -A 0 '"version"' tauri.conf.json | sed 's/.*"\([0-9.]*\)".*/\1/')
echo "Building DMG for version $VERSION..."

# 创建 DMG 目录
mkdir -p target/release/bundle/dmg

# 获取绝对路径
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_TAURI_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
BG_IMAGE="$SRC_TAURI_DIR/assets/dmg-background.jpg"

# 使用创建 DMG 的工具
cd target/release/bundle/dmg

DMG_NAME="CaoGit_${VERSION}_aarch64.dmg"

# 删除旧的 DMG（如果存在）
rm -f "$DMG_NAME"

# 下载 create-dmg 脚本（如果不存在）
if [ ! -f "bundle_dmg.sh" ]; then
    echo "Downloading bundle_dmg.sh..."
    # 从 Tauri 的缓存中复制
    # 或使用简化的DMG创建命令
    echo "Using simplified DMG creation..."

    # 简化版本：直接使用 hdiutil 创建 DMG
    if [ -f "$BG_IMAGE" ]; then
        hdiutil create -volname "CaoGit" \
            -srcfolder "../macos" \
            -ov -format UDZO \
            "$DMG_NAME"
    else
        hdiutil create -volname "CaoGit" \
            -srcfolder "../macos" \
            -ov -format UDZO \
            "$DMG_NAME"
    fi
else
    # 使用 Tauri 的 bundle_dmg.sh
    if [ ! -f "$BG_IMAGE" ]; then
        bash bundle_dmg.sh \
          --volname "CaoGit" \
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
          --background "$BG_IMAGE" \
          --window-pos 0 0 \
          --window-size 1025 678 \
          --icon-size 160 \
          --icon "CaoGit.app" 240 300 \
          --app-drop-link 790 300 \
          "$DMG_NAME" \
          ../macos
    fi
fi

cd - > /dev/null

# 签名 DMG
echo "Signing DMG..."
DMG_PATH="target/release/bundle/dmg/$DMG_NAME"
SIGNING_IDENTITY="3rd Party Mac Developer Application: luo changyi (T5P2UCK36A)"

codesign --force --sign "$SIGNING_IDENTITY" "$DMG_PATH" 2>/dev/null || \
codesign --force --sign "-" "$DMG_PATH" 2>/dev/null || \
echo "Warning: Could not sign DMG"

echo "✅ DMG created and signed successfully!"
echo "Location: $DMG_PATH"

# 验证 DMG 中的应用
echo ""
echo "Verifying app in DMG..."
hdiutil attach "$DMG_PATH" -nobrowse -readonly -quiet
sleep 2

# 找到挂载点
MOUNT_POINT=$(hdiutil info | grep "/Volumes/CaoGit" | awk '{print $3}' | head -1)

if [ -n "$MOUNT_POINT" ] && [ -d "$MOUNT_POINT/CaoGit.app" ]; then
    codesign -vv "$MOUNT_POINT/CaoGit.app" 2>&1 | head -3
    plutil -p "$MOUNT_POINT/CaoGit.app/Contents/Info.plist" | grep "CFBundleDevelopmentRegion"
    hdiutil detach "$MOUNT_POINT" -quiet
    echo "✅ Verification complete"
else
    echo "Could not verify - DMG not mounted"
fi
