#!/bin/bash

# 在 Tauri 打包之前添加本地化资源
# 这个脚本会在编译完成但打包开始前运行

echo "Preparing localization resources for bundling..."

# 确保本地化目录存在
mkdir -p resources/zh_CN.lproj resources/en.lproj

# 创建临时的本地化资源标记文件
# Tauri 会将 resources 目录下的内容复制到 app bundle
touch resources/.localization-ready

echo "Localization resources prepared"
