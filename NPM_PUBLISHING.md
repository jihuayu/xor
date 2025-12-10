# NPM 包发布指南

本项目支持通过 npm 发布和安装。

## 包结构

- **主包**: `xor-encryption` - CLI 包装器，自动安装对应平台的二进制包
- **平台包**: `@xor-encryption/<platform>` - 包含特定平台的二进制文件

### 支持的平台

- Windows x64: `@xor-encryption/win32-x64`
- Windows ARM64: `@xor-encryption/win32-arm64`
- Linux x64: `@xor-encryption/linux-x64`
- Linux ARM64: `@xor-encryption/linux-arm64`
- macOS x64 (Intel): `@xor-encryption/darwin-x64`
- macOS ARM64 (Apple Silicon): `@xor-encryption/darwin-arm64`

## 自动发布

当创建新的 Git tag 时（格式：`v*`），GitHub Actions 会自动：

1. 构建所有平台的二进制文件
2. 创建 GitHub Release
3. 发布所有 npm 包

### 创建新版本

```bash
# 1. 更新 Cargo.toml 中的版本号
vim Cargo.toml

# 2. 提交更改
git add Cargo.toml
git commit -m "Bump version to 0.2.0"

# 3. 创建 tag
git tag v0.2.0

# 4. 推送 tag（这将触发自动发布）
git push origin v0.2.0
```

## 手动发布

如果需要手动发布：

### 前置要求

1. 登录 npm：
```bash
npm login
```

2. 设置 npm token（用于 GitHub Actions）：
   - 在 npm 网站创建 automation token
   - 在 GitHub 仓库设置中添加 secret: `NPM_TOKEN`

### 发布步骤

```bash
# 1. 构建所有平台的二进制文件
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target aarch64-pc-windows-msvc
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# 2. 准备 npm 包
cd package/npm
node scripts/prepare-packages.js

# 3. 发布（需要在 Unix 环境下）
chmod +x scripts/publish.sh
./scripts/publish.sh 0.2.0 latest
```

## 用户安装

### 全局安装

```bash
npm install -g xor-encryption
```

### 本地安装

```bash
npm install xor-encryption
```

### 使用 npx（无需安装）

```bash
npx xor-encryption [options]
```

## 工作原理

1. 用户安装 `xor-encryption` 主包
2. npm 根据用户的操作系统和 CPU 架构，自动安装对应的 `@xor-encryption/<platform>` 包
3. CLI 入口脚本 (`bin/xor`) 检测平台并执行相应的二进制文件
4. `postinstall` 脚本验证安装是否成功

## 故障排除

### 平台包未安装

如果遇到 "Binary not found" 错误：

```bash
# 手动安装平台包
npm install @xor-encryption/linux-x64  # Linux x64 示例
```

### npm 版本太旧

某些旧版本的 npm 可能不支持 `optionalDependencies` 的平台检测。升级 npm：

```bash
npm install -g npm@latest
```

### 权限问题（macOS/Linux）

如果遇到权限错误，可能需要使用 `sudo`：

```bash
sudo npm install -g xor-encryption
```

或者配置 npm 使用用户目录：

```bash
mkdir ~/.npm-global
npm config set prefix '~/.npm-global'
echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc
source ~/.bashrc
```

## 版本管理

所有包（主包和平台包）使用相同的版本号，确保兼容性。

版本号遵循语义化版本（Semantic Versioning）：
- MAJOR: 不兼容的 API 变更
- MINOR: 向后兼容的功能新增
- PATCH: 向后兼容的问题修正

## 开发测试

本地测试 npm 包：

```bash
# 1. 准备包
cd package/npm
node scripts/prepare-packages.js

# 2. 链接到全局
npm link

# 3. 测试
xor --help

# 4. 取消链接
npm unlink -g xor-encryption
```

## 参考资料

- [npm Documentation](https://docs.npmjs.com/)
- [Publishing packages](https://docs.npmjs.com/packages-and-modules/contributing-packages-to-the-registry)
- [optionalDependencies](https://docs.npmjs.com/cli/v10/configuring-npm/package-json#optionaldependencies)
