#!/bin/bash

# 发布新版本脚本
# 用法: ./release.sh v0.2.2 "Release v0.2.2 说明"

VERSION=$1
MESSAGE=$2

if [ -z "$VERSION" ]; then
  echo "❌ 错误: 请提供版本号"
  echo "用法: ./release.sh v0.2.2 \"Release v0.2.2 说明\""
  exit 1
fi

if [ -z "$MESSAGE" ]; then
  MESSAGE="Release $VERSION"
fi

echo "📦 准备发布 $VERSION..."
echo ""

# 1. 检查是否有未提交的改动
if ! git diff-index --quiet HEAD --; then
  echo "⚠️  检测到未提交的改动"
  echo "正在提交所有改动..."
  git add .
  git commit -m "chore: prepare for $VERSION"
  git push origin master
  echo "✅ 代码已推送"
  echo ""
fi

# 2. 创建标签
echo "🏷️  创建标签 $VERSION..."
if git tag -a "$VERSION" -m "$MESSAGE"; then
  echo "✅ 标签创建成功"
else
  echo "❌ 标签创建失败（可能已存在）"
  echo "如需重新创建，请先删除旧标签:"
  echo "  git tag -d $VERSION"
  echo "  git push origin :refs/tags/$VERSION"
  exit 1
fi

# 3. 推送标签
echo ""
echo "🚀 推送标签到 GitHub（将触发自动构建）..."
if git push origin "$VERSION"; then
  echo "✅ 标签已推送"
  echo ""
  echo "================================================"
  echo "🎉 发布流程已启动！"
  echo "================================================"
  echo ""
  echo "📊 监控构建进度:"
  echo "   https://github.com/WNLUO/CaoGit/actions"
  echo ""
  echo "📥 查看发布页面:"
  echo "   https://github.com/WNLUO/CaoGit/releases/tag/$VERSION"
  echo ""
  echo "⏱️  预计 15-20 分钟后完成三平台构建"
  echo ""
else
  echo "❌ 推送失败"
  exit 1
fi
