#!/usr/bin/env node

/**
 * 本地测试脚本 - 测试 npm 包是否能正常工作
 */

const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

console.log('Starting local npm package test...\n');

const packageDir = path.join(__dirname, '..');

// 检查是否有测试用的二进制文件
console.log('Step 1: Checking for test binary...');
const testBinary = path.join(__dirname, '..', '..', '..', 'target', 'release', 'xor.exe');
const testBinaryUnix = path.join(__dirname, '..', '..', '..', 'target', 'release', 'xor');

let binaryPath = null;
if (fs.existsSync(testBinary)) {
  binaryPath = testBinary;
} else if (fs.existsSync(testBinaryUnix)) {
  binaryPath = testBinaryUnix;
}

if (!binaryPath) {
  console.log('⚠️  No release binary found. Building...');
  try {
    execSync('cargo build --release', { 
      cwd: path.join(__dirname, '..', '..', '..'),
      stdio: 'inherit' 
    });
    
    if (fs.existsSync(testBinary)) {
      binaryPath = testBinary;
    } else if (fs.existsSync(testBinaryUnix)) {
      binaryPath = testBinaryUnix;
    }
  } catch (err) {
    console.error('❌ Failed to build binary');
    process.exit(1);
  }
}

console.log(`✓ Found binary: ${binaryPath}\n`);

// 准备包
console.log('Step 2: Preparing packages...');
try {
  execSync('node scripts/prepare-packages.js', { 
    cwd: packageDir,
    stdio: 'inherit' 
  });
  console.log('✓ Packages prepared\n');
} catch (err) {
  console.error('❌ Failed to prepare packages');
  process.exit(1);
}

// 链接包
console.log('Step 3: Linking package...');
try {
  // 先取消可能存在的链接
  try {
    execSync('npm unlink -g @jihuayu/hbsx', { stdio: 'ignore' });
  } catch (e) {
    // 忽略错误
  }

  // 创建新链接
  execSync('npm link', { 
    cwd: packageDir,
    stdio: 'inherit' 
  });
  console.log('✓ Package linked\n');
} catch (err) {
  console.error('❌ Failed to link package');
  process.exit(1);
}

// 测试命令
console.log('Step 4: Testing command...');
try {
  // 测试帮助信息
  console.log('Running: xor --help');
  execSync('xor --help', { stdio: 'inherit' });
  console.log('\n✓ Command works!\n');
} catch (err) {
  console.error('❌ Command failed');
  console.log('\nCleanup...');
  try {
    execSync('npm unlink -g @jihuayu/hbsx', { stdio: 'ignore' });
  } catch (e) {}
  process.exit(1);
}

// 清理
console.log('Step 5: Cleanup...');
try {
  execSync('npm unlink -g @jihuayu/hbsx', { stdio: 'inherit' });
  console.log('✓ Cleanup complete\n');
} catch (err) {
  console.warn('⚠️  Cleanup warning (you may need to run manually):');
  console.warn('   npm unlink -g @jihuayu/hbsx\n');
}

console.log('='.repeat(50));
console.log('✓ All tests passed!');
console.log('='.repeat(50));
console.log('\nThe package is ready to publish.');
console.log('\nTo publish:');
console.log('  1. Ensure you are logged in to npm: npm login');
console.log('  2. Run: ./scripts/publish.sh <version>');
