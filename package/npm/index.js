#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// 获取二进制文件路径
function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;
  
  // 映射平台和架构到包名
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
    throw new Error(`Unsupported platform: ${platform}-${arch}`);
  }

  // 尝试多个可能的路径
  const possiblePaths = [
    // 从 node_modules 安装的情况
    path.join(__dirname, '..', packageName, 'bin', 'xor'),
    path.join(__dirname, '..', packageName, 'bin', 'xor.exe'),
    // 本地开发情况
    path.join(__dirname, 'node_modules', packageName, 'bin', 'xor'),
    path.join(__dirname, 'node_modules', packageName, 'bin', 'xor.exe'),
  ];

  for (const binPath of possiblePaths) {
    if (fs.existsSync(binPath)) {
      return binPath;
    }
  }

  throw new Error(`Binary not found for ${platform}-${arch}. Please make sure the package ${packageName} is installed.`);
}

// 执行二进制文件
function runBinary() {
  try {
    const binaryPath = getBinaryPath();
    const args = process.argv.slice(2);
    
    const child = spawn(binaryPath, args, {
      stdio: 'inherit',
      shell: false
    });

    child.on('exit', (code) => {
      process.exit(code || 0);
    });

    child.on('error', (err) => {
      console.error('Failed to execute binary:', err.message);
      process.exit(1);
    });
  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
}

if (require.main === module) {
  runBinary();
}

module.exports = { getBinaryPath, runBinary };
