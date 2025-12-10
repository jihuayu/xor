# npm 发布快速开始

## 🎯 目标

将 Rust 编译的二进制文件通过 npm 发布，支持多平台。

## ✅ 已完成

- ✅ npm 包结构（1个主包 + 6个平台包）
- ✅ 自动化脚本（准备、验证、测试）
- ✅ GitHub Actions 集成
- ✅ 完整文档

## 🚀 立即开始

### 步骤 1: 配置 npm Token

1. 登录 npm 并创建 token
   ```bash
   # 访问 https://www.npmjs.com/settings/~/tokens
   # 创建 "Automation" token
   ```

2. 在 GitHub 添加 Secret
   ```
   仓库设置 → Secrets → Actions → New repository secret
   Name: NPM_TOKEN
   Value: <你的 npm token>
   ```

### 步骤 2: 验证设置

```bash
# 验证包结构
node package/npm/scripts/validate.js
```

预期输出：所有检查 ✓ 通过

### 步骤 3: 发布第一个版本

```bash
# 1. 确保 Cargo.toml 中版本正确
cat Cargo.toml | grep version

# 2. 创建 tag
git tag v0.1.0

# 3. 推送 tag（触发自动发布）
git push origin v0.1.0
```

### 步骤 4: 监控发布

```bash
# 访问 GitHub Actions
https://github.com/jihuayu/xor/actions

# 等待 "Release" workflow 完成（约 10-15 分钟）
```

### 步骤 5: 验证发布

```bash
# 测试安装
npm install -g xor-encryption

# 测试运行
xor --help

# 检查 npm
https://www.npmjs.com/package/xor-encryption
```

## 📦 发布的包

自动发布 7 个包：

1. `xor-encryption` - 主包（CLI 包装器）
2. `@xor-encryption/win32-x64` - Windows x64
3. `@xor-encryption/win32-arm64` - Windows ARM64
4. `@xor-encryption/linux-x64` - Linux x64
5. `@xor-encryption/linux-arm64` - Linux ARM64
6. `@xor-encryption/darwin-x64` - macOS Intel
7. `@xor-encryption/darwin-arm64` - macOS Apple Silicon

## 🔄 更新版本

```bash
# 1. 更新版本号
vim Cargo.toml  # 修改 version = "0.2.0"

# 2. 提交
git add Cargo.toml
git commit -m "chore: bump version to 0.2.0"
git push

# 3. 创建新 tag
git tag v0.2.0
git push origin v0.2.0

# 等待自动发布...
```

## 🧪 本地测试（可选）

在发布前本地测试：

```bash
# 1. 构建 Rust
cargo build --release

# 2. 测试 npm 包
node package/npm/scripts/test-local.js
```

## 📚 详细文档

- `NPM_SETUP_SUMMARY.md` - 配置总结
- `NPM_PUBLISHING.md` - 详细发布指南
- `FIRST_RELEASE_CHECKLIST.md` - 首次发布检查清单
- `PROJECT_OVERVIEW.md` - 项目概览

## ❓ 常见问题

### Q: 发布失败怎么办？

A: 检查 GitHub Actions 日志，常见问题：
- NPM_TOKEN 未设置或已过期
- 包名已被占用
- 版本号冲突

### Q: 如何撤销发布？

A: 使用 npm deprecate：
```bash
npm deprecate xor-encryption@0.1.0 "This version has issues"
```

### Q: 支持哪些平台？

A: 
- Windows: x64, ARM64
- Linux: x64, ARM64
- macOS: Intel (x64), Apple Silicon (ARM64)

### Q: 用户如何安装？

A:
```bash
npm install -g xor-encryption
# 或
npx xor-encryption
```

npm 会自动根据用户的操作系统和 CPU 架构安装正确的平台包。

## 🎉 就这么简单！

只需要：
1. ✅ 配置 NPM_TOKEN（一次性）
2. ✅ 创建 git tag
3. ✅ GitHub Actions 自动处理其余工作

用户就可以通过 `npm install -g xor-encryption` 安装你的工具了！
