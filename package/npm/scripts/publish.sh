#!/bin/bash

# 发布脚本 - 发布所有 npm 包
# 使用方法: ./publish.sh <version> [npm-tag]
# 例如: ./publish.sh 0.1.0 latest
#       ./publish.sh 0.2.0-beta.1 beta

set -e

VERSION=$1
NPM_TAG=${2:-latest}

if [ -z "$VERSION" ]; then
  echo "Usage: $0 <version> [npm-tag]"
  echo "Example: $0 0.1.0 latest"
  exit 1
fi

echo "Publishing @jihuayu/hbsx packages version $VERSION with tag $NPM_TAG"
echo ""

# 检查是否已登录 npm
if ! npm whoami > /dev/null 2>&1; then
  echo "Error: Not logged in to npm. Please run 'npm login' first."
  exit 1
fi

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PACKAGE_DIR="$( cd "$SCRIPT_DIR/.." && pwd )"

# 更新所有包的版本号
echo "Updating package versions to $VERSION..."

# 更新主包版本
cd "$PACKAGE_DIR"
npm version "$VERSION" --no-git-tag-version --allow-same-version

# 更新平台包版本
for platform in win32-x64 win32-arm64 linux-x64 linux-arm64 darwin-x64 darwin-arm64; do
  cd "$PACKAGE_DIR/platform-packages/$platform"
  npm version "$VERSION" --no-git-tag-version --allow-same-version
done

# 发布平台包
echo ""
echo "Publishing platform packages..."
for platform in win32-x64 win32-arm64 linux-x64 linux-arm64 darwin-x64 darwin-arm64; do
  cd "$PACKAGE_DIR/platform-packages/$platform"
  
  # 检查是否有二进制文件
  if [ ! -d "bin" ] || [ -z "$(ls -A bin)" ]; then
    echo "⚠️  Skipping @jihuayu/hbsx-$platform (no binary found)"
    continue
  fi
  
  echo "Publishing @jihuayu/hbsx-$platform..."
  npm publish --access public --tag "$NPM_TAG"
  echo "✓ Published @jihuayu/hbsx-$platform"
done

# 发布主包
echo ""
echo "Publishing main package..."
cd "$PACKAGE_DIR"
npm publish --access public --tag "$NPM_TAG"
echo "✓ Published @jihuayu/hbsx"

echo ""
echo "✓ All packages published successfully!"
echo ""
echo "Install with: npm install -g @jihuayu/hbsx@$VERSION"
