#!/usr/bin/env node

/**
 * 准备 npm 包发布
 * 此脚本将编译好的二进制文件复制到对应的平台包中
 */

const fs = require('fs');
const path = require('path');

// 平台映射
const platforms = [
  { target: 'x86_64-pc-windows-msvc', npm: 'win32-x64', binary: 'xor.exe' },
  { target: 'aarch64-pc-windows-msvc', npm: 'win32-arm64', binary: 'xor.exe' },
  { target: 'x86_64-unknown-linux-gnu', npm: 'linux-x64', binary: 'xor' },
  { target: 'x86_64-unknown-linux-musl', npm: 'linux-x64-musl', binary: 'xor' },
  { target: 'aarch64-unknown-linux-gnu', npm: 'linux-arm64', binary: 'xor' },
  { target: 'aarch64-unknown-linux-musl', npm: 'linux-arm64-musl', binary: 'xor' },
  { target: 'x86_64-apple-darwin', npm: 'darwin-x64', binary: 'xor' },
  { target: 'aarch64-apple-darwin', npm: 'darwin-arm64', binary: 'xor' },
];

function preparePlatformPackage(platform) {
  const binarySource = path.join(__dirname, '..', '..', 'target', platform.target, 'release', platform.binary);
  const packageDir = path.join(__dirname, '..', 'platform-packages', platform.npm);
  const binDir = path.join(packageDir, 'bin');
  const binaryDest = path.join(binDir, platform.binary);

  // 检查源文件是否存在
  if (!fs.existsSync(binarySource)) {
    console.warn(`⚠️  Binary not found: ${binarySource}`);
    return false;
  }

  // 创建 bin 目录
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  // 复制二进制文件
  fs.copyFileSync(binarySource, binaryDest);
  
  // 设置执行权限（Unix 系统）
  if (process.platform !== 'win32') {
    fs.chmodSync(binaryDest, 0o755);
  }

  console.log(`✓ Prepared ${platform.npm}: ${binarySource} -> ${binaryDest}`);
  return true;
}

function main() {
  console.log('Preparing npm packages...\n');

  let successCount = 0;
  let failCount = 0;

  for (const platform of platforms) {
    if (preparePlatformPackage(platform)) {
      successCount++;
    } else {
      failCount++;
    }
  }

  console.log(`\n✓ Success: ${successCount}`);
  if (failCount > 0) {
    console.log(`⚠️  Skipped: ${failCount}`);
  }
  
  console.log('\nPackages are ready for publishing!');
}

main();
