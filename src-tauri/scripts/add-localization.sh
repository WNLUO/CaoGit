#!/bin/bash

# 添加本地化支持到 macOS 应用

APP_PATH="$1"

if [ -z "$APP_PATH" ]; then
    echo "Usage: $0 <path-to-app>"
    exit 1
fi

if [ ! -d "$APP_PATH" ]; then
    echo "Error: App not found at $APP_PATH"
    exit 1
fi

INFO_PLIST="$APP_PATH/Contents/Info.plist"

echo "Adding localization support to $INFO_PLIST"

# 设置开发区域为中文
/usr/libexec/PlistBuddy -c "Set :CFBundleDevelopmentRegion zh_CN" "$INFO_PLIST" 2>/dev/null || \
/usr/libexec/PlistBuddy -c "Add :CFBundleDevelopmentRegion string zh_CN" "$INFO_PLIST"

# 添加支持的本地化语言
/usr/libexec/PlistBuddy -c "Delete :CFBundleLocalizations" "$INFO_PLIST" 2>/dev/null
/usr/libexec/PlistBuddy -c "Add :CFBundleLocalizations array" "$INFO_PLIST"
/usr/libexec/PlistBuddy -c "Add :CFBundleLocalizations:0 string zh_CN" "$INFO_PLIST"
/usr/libexec/PlistBuddy -c "Add :CFBundleLocalizations:1 string zh-Hans" "$INFO_PLIST"
/usr/libexec/PlistBuddy -c "Add :CFBundleLocalizations:2 string en" "$INFO_PLIST"

# 复制本地化资源
RESOURCES_PATH="$APP_PATH/Contents/Resources"
mkdir -p "$RESOURCES_PATH"

# 复制并编译 zh_CN.lproj
if [ -d "resources/zh_CN.lproj" ]; then
    cp -r "resources/zh_CN.lproj" "$RESOURCES_PATH/"
    # 编译 .strings 文件为二进制格式
    if [ -f "$RESOURCES_PATH/zh_CN.lproj/InfoPlist.strings" ]; then
        plutil -convert binary1 "$RESOURCES_PATH/zh_CN.lproj/InfoPlist.strings"
    fi
    echo "Copied and compiled zh_CN.lproj"
fi

# 复制并编译 zh-Hans.lproj (简体中文的另一种标识符)
if [ -d "resources/zh-Hans.lproj" ]; then
    cp -r "resources/zh-Hans.lproj" "$RESOURCES_PATH/"
    # 编译 .strings 文件为二进制格式
    if [ -f "$RESOURCES_PATH/zh-Hans.lproj/InfoPlist.strings" ]; then
        plutil -convert binary1 "$RESOURCES_PATH/zh-Hans.lproj/InfoPlist.strings"
    fi
    echo "Copied and compiled zh-Hans.lproj"
fi

# 复制并编译 en.lproj
if [ -d "resources/en.lproj" ]; then
    cp -r "resources/en.lproj" "$RESOURCES_PATH/"
    # 编译 .strings 文件为二进制格式
    if [ -f "$RESOURCES_PATH/en.lproj/InfoPlist.strings" ]; then
        plutil -convert binary1 "$RESOURCES_PATH/en.lproj/InfoPlist.strings"
    fi
    echo "Copied and compiled en.lproj"
fi

echo "Localization support added successfully!"

# 重新签名应用（使用 ad-hoc 签名以保证兼容性）
echo "Re-signing the application with ad-hoc signature..."

# 移除所有现有签名
codesign --remove-signature "$APP_PATH" 2>/dev/null || true

# 首先签名可执行文件和框架
EXECUTABLE="$APP_PATH/Contents/MacOS/tauri-app-caogit"
if [ -f "$EXECUTABLE" ]; then
    codesign --force --sign "-" "$EXECUTABLE"
fi

# 签名所有嵌入的框架和库
find "$APP_PATH/Contents" -name "*.dylib" -o -name "*.framework" 2>/dev/null | while read lib; do
    codesign --force --sign "-" "$lib" 2>/dev/null || true
done

# 最后签名整个应用包
codesign --force --deep --sign "-" --preserve-metadata=entitlements "$APP_PATH"

echo "✅ Application re-signed with ad-hoc signature!"
echo "Note: For distribution, the DMG needs proper signing and notarization"
