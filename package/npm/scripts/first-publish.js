#!/usr/bin/env node

/**
 * 首次手动发布所有 npm 包
 * 用于在 GitHub Actions 之前初始化 npm 包
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const VERSION = '0.1.0'; // 首次发布使用的版本

console.log('🚀 首次 npm 包发布脚本\n');

// 检查 npm 登录状态
function checkNpmAuth() {
  console.log('检查 npm 认证状态...');
  try {
    const user = execSync('npm whoami', { encoding: 'utf-8' }).trim();
    console.log(`✓ 已登录为: ${user}\n`);
    return true;
  } catch (error) {
    console.error('❌ 未登录到 npm');
    console.error('请先运行: npm login');
    return false;
  }
}

// 更新所有包的版本号
function updateVersions() {
  console.log(`更新所有包的版本号到 ${VERSION}...`);
  
  const packageDir = path.join(__dirname, '..');
  
  // 更新主包
  const mainPkgPath = path.join(packageDir, 'package.json');
  const mainPkg = JSON.parse(fs.readFileSync(mainPkgPath, 'utf-8'));
  mainPkg.version = VERSION;
  
  // 更新 optionalDependencies
  for (const dep in mainPkg.optionalDependencies) {
    mainPkg.optionalDependencies[dep] = VERSION;
  }
  
  fs.writeFileSync(mainPkgPath, JSON.stringify(mainPkg, null, 2) + '\n');
  console.log('✓ 主包版本已更新');
  
  // 更新平台包
  const platforms = ['win32-x64', 'win32-arm64', 'linux-x64', 'linux-arm64', 'darwin-x64', 'darwin-arm64'];
  for (const platform of platforms) {
    const pkgPath = path.join(packageDir, 'platform-packages', platform, 'package.json');
    const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf-8'));
    pkg.version = VERSION;
    fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');
    console.log(`✓ ${platform} 版本已更新`);
  }
  
  console.log('');
}

// 发布平台包
function publishPlatformPackages() {
  console.log('发布平台包...\n');
  
  const platforms = ['win32-x64', 'win32-arm64', 'linux-x64', 'linux-arm64', 'darwin-x64', 'darwin-arm64'];
  const packageDir = path.join(__dirname, '..');
  
  for (const platform of platforms) {
    const pkgDir = path.join(packageDir, 'platform-packages', platform);
    const pkgPath = path.join(pkgDir, 'package.json');
    const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf-8'));
    
    console.log(`发布 ${pkg.name}@${pkg.version}...`);
    
    try {
      // 检查 bin 目录是否存在
      const binDir = path.join(pkgDir, 'bin');
      if (!fs.existsSync(binDir) || fs.readdirSync(binDir).length === 0) {
        console.log(`⚠️  ${platform} 没有二进制文件，跳过`);
        continue;
      }
      
      execSync('npm publish --access public', {
        cwd: pkgDir,
        stdio: 'inherit'
      });
      console.log(`✓ ${pkg.name} 发布成功\n`);
    } catch (error) {
      console.error(`❌ ${pkg.name} 发布失败`);
      console.error(error.message);
      return false;
    }
  }
  
  return true;
}

// 发布主包
function publishMainPackage() {
  console.log('发布主包...\n');
  
  const packageDir = path.join(__dirname, '..');
  const pkgPath = path.join(packageDir, 'package.json');
  const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf-8'));
  
  console.log(`发布 ${pkg.name}@${pkg.version}...`);
  
  try {
    execSync('npm publish --access public', {
      cwd: packageDir,
      stdio: 'inherit'
    });
    console.log(`✓ ${pkg.name} 发布成功\n`);
    return true;
  } catch (error) {
    console.error(`❌ ${pkg.name} 发布失败`);
    console.error(error.message);
    return false;
  }
}

// 主函数
async function main() {
  // 1. 检查认证
  if (!checkNpmAuth()) {
    process.exit(1);
  }
  
  // 2. 确认操作
  console.log('⚠️  警告：此操作将发布以下包到 npm：');
  console.log('  - @jihuayu/hbsx');
  console.log('  - @jihuayu/hbsx-win32-x64');
  console.log('  - @jihuayu/hbsx-win32-arm64');
  console.log('  - @jihuayu/hbsx-linux-x64');
  console.log('  - @jihuayu/hbsx-linux-arm64');
  console.log('  - @jihuayu/hbsx-darwin-x64');
  console.log('  - @jihuayu/hbsx-darwin-arm64');
  console.log(`\n版本号: ${VERSION}\n`);
  
  // 3. 更新版本号
  updateVersions();
  
  // 4. 发布平台包
  console.log('注意：首次发布时，平台包没有二进制文件是正常的');
  console.log('可以先发布空的平台包，之后通过 GitHub Actions 更新\n');
  
  const readline = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout
  });
  
  readline.question('是否继续？(yes/no): ', (answer) => {
    readline.close();
    
    if (answer.toLowerCase() !== 'yes') {
      console.log('操作已取消');
      process.exit(0);
    }
    
    // 发布平台包
    if (!publishPlatformPackages()) {
      console.error('\n❌ 平台包发布失败');
      process.exit(1);
    }
    
    // 发布主包
    if (!publishMainPackage()) {
      console.error('\n❌ 主包发布失败');
      process.exit(1);
    }
    
    console.log('✅ 所有包发布成功！\n');
    console.log('下次可以使用 GitHub Actions 自动发布新版本');
    console.log('只需推送 tag: git push origin v0.2.0');
  });
}

main().catch(error => {
  console.error('发生错误:', error);
  process.exit(1);
});
