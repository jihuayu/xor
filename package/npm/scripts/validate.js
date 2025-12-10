#!/usr/bin/env node

/**
 * 验证 npm 包结构
 */

const fs = require('fs');
const path = require('path');

const errors = [];
const warnings = [];

function checkFile(filePath, description) {
  if (!fs.existsSync(filePath)) {
    errors.push(`Missing: ${description} (${filePath})`);
    return false;
  }
  console.log(`✓ ${description}`);
  return true;
}

function checkDir(dirPath, description) {
  if (!fs.existsSync(dirPath)) {
    errors.push(`Missing directory: ${description} (${dirPath})`);
    return false;
  }
  console.log(`✓ ${description}`);
  return true;
}

function validatePackageJson(filePath, packageName) {
  if (!fs.existsSync(filePath)) {
    errors.push(`Missing package.json: ${filePath}`);
    return;
  }

  try {
    const pkg = JSON.parse(fs.readFileSync(filePath, 'utf8'));
    
    if (!pkg.name) {
      errors.push(`${packageName}: Missing 'name' field`);
    } else if (pkg.name !== packageName) {
      warnings.push(`${packageName}: Name mismatch (expected: ${packageName}, got: ${pkg.name})`);
    }

    if (!pkg.version) {
      errors.push(`${packageName}: Missing 'version' field`);
    }

    if (!pkg.license) {
      warnings.push(`${packageName}: Missing 'license' field`);
    }

    console.log(`✓ ${packageName} package.json valid`);
  } catch (err) {
    errors.push(`${packageName}: Invalid package.json - ${err.message}`);
  }
}

function validateBinary(binPath, packageName) {
  if (!fs.existsSync(binPath)) {
    warnings.push(`${packageName}: Binary not found (${binPath}) - will be added during build`);
    return;
  }

  const stats = fs.statSync(binPath);
  if (stats.size === 0) {
    errors.push(`${packageName}: Binary is empty`);
    return;
  }

  console.log(`✓ ${packageName} binary exists (${(stats.size / 1024 / 1024).toFixed(2)} MB)`);
}

console.log('Validating npm package structure...\n');

// 验证主包
console.log('=== Main Package ===');
const mainPackageDir = path.join(__dirname, '..');
checkFile(path.join(mainPackageDir, 'package.json'), 'Main package.json');
checkFile(path.join(mainPackageDir, 'index.js'), 'Main index.js');
checkFile(path.join(mainPackageDir, 'bin/xor'), 'CLI entry point');
checkFile(path.join(mainPackageDir, 'scripts/postinstall.js'), 'Postinstall script');
checkFile(path.join(mainPackageDir, 'README.md'), 'Main README');
validatePackageJson(path.join(mainPackageDir, 'package.json'), '@jihuayu/hbsx');

// 验证平台包
console.log('\n=== Platform Packages ===');
const platforms = [
  { name: 'win32-x64', binary: 'xor.exe' },
  { name: 'win32-arm64', binary: 'xor.exe' },
  { name: 'linux-x64', binary: 'xor' },
  { name: 'linux-arm64', binary: 'xor' },
  { name: 'darwin-x64', binary: 'xor' },
  { name: 'darwin-arm64', binary: 'xor' },
];

for (const platform of platforms) {
  console.log(`\n--- @jihuayu/hbsx-${platform.name} ---`);
  const platformDir = path.join(mainPackageDir, 'platform-packages', platform.name);
  
  if (!checkDir(platformDir, `Platform package directory`)) {
    continue;
  }

  checkFile(path.join(platformDir, 'package.json'), 'package.json');
  checkFile(path.join(platformDir, 'index.js'), 'index.js');
  checkFile(path.join(platformDir, 'README.md'), 'README.md');
  
  validatePackageJson(
    path.join(platformDir, 'package.json'),
    `@jihuayu/hbsx-${platform.name}`
  );

  const binPath = path.join(platformDir, 'bin', platform.binary);
  validateBinary(binPath, platform.name);
}

// 输出结果
console.log('\n' + '='.repeat(50));
if (errors.length === 0 && warnings.length === 0) {
  console.log('✓ All validations passed!');
} else {
  if (warnings.length > 0) {
    console.log('\n⚠️  Warnings:');
    warnings.forEach(w => console.log(`  - ${w}`));
  }
  
  if (errors.length > 0) {
    console.log('\n❌ Errors:');
    errors.forEach(e => console.log(`  - ${e}`));
    process.exit(1);
  }
}

console.log('\nNote: Binary files are generated during the build process.');
console.log('Run "node scripts/prepare-packages.js" after building to add binaries.');
