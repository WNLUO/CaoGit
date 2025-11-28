#!/usr/bin/env node

/**
 * 版本号自动管理脚本
 * 使用方法:
 *   npm run bump:patch    # 0.1.0 -> 0.1.1
 *   npm run bump:minor    # 0.1.0 -> 0.2.0
 *   npm run bump:major    # 0.1.0 -> 1.0.0
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// 获取要升级的版本类型 (patch/minor/major)
const versionType = process.argv[2] || 'patch';

if (!['patch', 'minor', 'major'].includes(versionType)) {
  console.error('❌ 无效的版本类型。使用: patch, minor, 或 major');
  process.exit(1);
}

try {
  // 1. 读取 package.json
  const packageJsonPath = path.join(__dirname, '../package.json');
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
  const currentVersion = packageJson.version;

  // 2. 解析当前版本
  const versionParts = currentVersion.split('.').map(Number);
  let [major, minor, patch] = versionParts;

  // 3. 计算新版本
  if (versionType === 'patch') {
    patch++;
  } else if (versionType === 'minor') {
    minor++;
    patch = 0;
  } else if (versionType === 'major') {
    major++;
    minor = 0;
    patch = 0;
  }

  const newVersion = `${major}.${minor}.${patch}`;

  console.log(`📦 版本升级: ${currentVersion} -> ${newVersion}`);

  // 4. 更新 package.json
  packageJson.version = newVersion;
  fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2) + '\n');
  console.log('✅ 已更新 package.json');

  // 5. 更新 src-tauri/tauri.conf.json
  const tauriConfPath = path.join(__dirname, '../src-tauri/tauri.conf.json');
  const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
  tauriConf.version = newVersion;
  fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + '\n');
  console.log('✅ 已更新 src-tauri/tauri.conf.json');

  // 6. Git commit
  try {
    execSync('git add package.json src-tauri/tauri.conf.json', { stdio: 'inherit' });
    execSync(`git commit -m "chore: bump version to ${newVersion}"`, { stdio: 'inherit' });
    console.log(`✅ 已创建 Git commit`);
  } catch (error) {
    console.warn('⚠️  Git commit 失败，请手动执行 git commit');
  }

  // 7. 创建 Git tag
  const tagName = `v${newVersion}`;
  try {
    execSync(`git tag ${tagName}`, { stdio: 'inherit' });
    console.log(`✅ 已创建 Git tag: ${tagName}`);
  } catch (error) {
    console.warn(`⚠️  Git tag 创建失败，可能标签已存在`);
  }

  console.log(`
╔════════════════════════════════════════╗
║         版本升级完成！                  ║
╠════════════════════════════════════════╣
║ 版本号: ${newVersion.padEnd(32)} ║
║ Git Tag: ${tagName.padEnd(30)} ║
╠════════════════════════════════════════╣
║ 下一步:                                ║
║   git push origin master --tags        ║
║                                        ║
║ GitHub Actions 将自动:                 ║
║   1. 验证版本号一致性                   ║
║   2. 多平台构建 (macOS/Windows/Linux)  ║
║   3. 发布到 GitHub Release             ║
╚════════════════════════════════════════╝
  `);

} catch (error) {
  console.error('❌ 错误:', error.message);
  process.exit(1);
}
