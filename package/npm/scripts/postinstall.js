#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// postinstall 脚本用于验证平台包是否正确安装
function checkInstallation() {
  const platform = process.platform;
  const arch = process.arch;
  
  const platformMap = {
    'win32': {
      'x64': '@jihuayu/hbsx-win32-x64',
      'arm64': '@jihuayu/hbsx-win32-arm64'
    },
    'linux': {
      'x64': '@jihuayu/hbsx-linux-x64',
      'arm64': '@jihuayu/hbsx-linux-arm64'
    },
    'darwin': {
      'x64': '@jihuayu/hbsx-darwin-x64',
      'arm64': '@jihuayu/hbsx-darwin-arm64'
    }
  };

  const packageName = platformMap[platform]?.[arch];
  
  if (!packageName) {
    console.warn(`Warning: Platform ${platform}-${arch} is not officially supported.`);
    return;
  }

  // 检查平台包是否存在
  const packagePath = path.join(__dirname, '..', 'node_modules', packageName);
  
  if (!fs.existsSync(packagePath)) {
    console.warn(`Warning: Platform-specific package ${packageName} was not installed.`);
    console.warn('This may happen if you are using an old version of npm.');
    console.warn('Please try upgrading npm or install the platform package manually:');
    console.warn(`  npm install ${packageName}`);
  } else {
    console.log(`✓ Platform package ${packageName} installed successfully.`);
  }
}

try {
  checkInstallation();
} catch (error) {
  // 不要让 postinstall 失败导致整个安装失败
  console.warn('Warning during postinstall:', error.message);
}
