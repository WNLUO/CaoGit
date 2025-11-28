#!/bin/bash

# CaoGit macOS 修复脚本
# 用于移除 Gatekeeper 隔离属性

echo ""
echo "======================================"
echo "   CaoGit macOS 自动修复工具"
echo "======================================"
echo ""

# 获取脚本所在目录
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# 查找 CaoGit.app 的位置
if [ -f "$SCRIPT_DIR/CaoGit.app/Contents/Info.plist" ]; then
    APP_PATH="$SCRIPT_DIR/CaoGit.app"
    echo "✓ 找到应用: $APP_PATH"
elif [ -f "/Applications/CaoGit.app/Contents/Info.plist" ]; then
    APP_PATH="/Applications/CaoGit.app"
    echo "✓ 找到应用: $APP_PATH"
else
    echo "❌ 错误: 未找到 CaoGit.app"
    echo ""
    echo "请确保："
    echo "1. CaoGit.app 与此脚本在同一目录"
    echo "2. 或者已将 CaoGit.app 安装到 /Applications/"
    echo ""
    read -p "按回车键退出..."
    exit 1
fi

echo ""
echo "正在移除隔离属性..."
echo "执行命令: xattr -cr \"$APP_PATH\""
echo ""

# 执行修复命令
if xattr -cr "$APP_PATH" 2>&1; then
    echo ""
    echo "✅ 修复成功！"
    echo ""
    echo "现在您可以："
    echo "1. 关闭此窗口"
    echo "2. 双击 CaoGit.app 正常打开"
    echo ""

    # 询问是否立即打开应用
    read -p "是否立即打开 CaoGit？(y/n) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        open "$APP_PATH"
        echo "✓ 正在启动 CaoGit..."
    fi
else
    echo ""
    echo "❌ 修复失败"
    echo ""
    echo "可能的原因："
    echo "1. 没有足够的权限"
    echo "2. 应用文件已被移动或删除"
    echo ""
    echo "请尝试在终端中手动执行："
    echo "xattr -cr \"$APP_PATH\""
    echo ""
fi

echo ""
read -p "按回车键退出..."
