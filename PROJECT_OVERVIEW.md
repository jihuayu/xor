# XOR 加密工具 - 项目概览

## 项目结构

```
xor/
├── src/                      # Rust 源代码
│   ├── main.rs              # 主程序
│   ├── db.rs                # 数据库模块
│   └── lib.rs               # 库入口
├── tests/                    # 集成测试
│   ├── db_tests.rs          # 数据库测试
│   └── main_tests.rs        # 主程序测试
├── package/npm/             # npm 包
│   ├── package.json         # 主包配置
│   ├── index.js             # 二进制加载器
│   ├── bin/xor              # CLI 入口
│   ├── scripts/             # 发布脚本
│   │   ├── postinstall.js  # 安装后验证
│   │   ├── prepare-packages.js  # 准备发布
│   │   ├── publish.sh      # 发布脚本
│   │   ├── validate.js     # 验证包结构
│   │   └── test-local.js   # 本地测试
│   └── platform-packages/   # 平台特定包
│       ├── win32-x64/
│       ├── win32-arm64/
│       ├── linux-x64/
│       ├── linux-arm64/
│       ├── darwin-x64/
│       └── darwin-arm64/
├── .github/workflows/       # GitHub Actions
│   ├── release.yml         # 发布工作流（包含 npm 发布）
│   └── ci.yml              # CI 工作流
├── Cargo.toml              # Rust 依赖配置
└── README.md               # 项目说明

```

## 分发方式

### 1. GitHub Releases
- 多平台二进制文件
- 压缩包和 SHA256 校验和
- 自动构建和发布

### 2. npm 包
- 主包: `xor-encryption`
- 平台包: `@xor-encryption/<platform>`
- 支持全局安装和 npx 使用

### 3. 从源码编译
- 需要 Rust 工具链
- 支持所有 Rust 支持的平台

## 支持的平台

| 平台 | 架构 | Rust Target | npm 包 |
|------|------|-------------|---------|
| Windows | x64 | x86_64-pc-windows-msvc | @xor-encryption/win32-x64 |
| Windows | ARM64 | aarch64-pc-windows-msvc | @xor-encryption/win32-arm64 |
| Linux | x64 | x86_64-unknown-linux-gnu | @xor-encryption/linux-x64 |
| Linux | x64 (musl) | x86_64-unknown-linux-musl | @xor-encryption/linux-x64 |
| Linux | ARM64 | aarch64-unknown-linux-gnu | @xor-encryption/linux-arm64 |
| macOS | x64 | x86_64-apple-darwin | @xor-encryption/darwin-x64 |
| macOS | ARM64 | aarch64-apple-darwin | @xor-encryption/darwin-arm64 |

## 发布流程

### 自动发布（推荐）

1. 更新版本号
   ```bash
   # 编辑 Cargo.toml
   vim Cargo.toml
   
   # 提交更改
   git add Cargo.toml
   git commit -m "Bump version to X.Y.Z"
   ```

2. 创建并推送 tag
   ```bash
   git tag vX.Y.Z
   git push origin vX.Y.Z
   ```

3. GitHub Actions 自动执行：
   - 构建所有平台二进制
   - 创建 GitHub Release
   - 发布到 npm

### 手动发布

#### GitHub Release
```bash
# 构建
cargo build --release --target <target>

# 创建发布
# 使用 GitHub UI 或 gh CLI
```

#### npm 发布
```bash
# 准备包
cd package/npm
node scripts/prepare-packages.js

# 发布
./scripts/publish.sh X.Y.Z latest
```

## 开发工作流

### 本地开发
```bash
# 构建
cargo build

# 运行
cargo run

# 测试
cargo test

# 检查代码
cargo clippy
cargo fmt
```

### 测试 npm 包
```bash
# 验证包结构
node package/npm/scripts/validate.js

# 本地测试
node package/npm/scripts/test-local.js
```

## CI/CD

### GitHub Actions 工作流

#### release.yml
- **触发**: 推送 `v*` tag
- **步骤**:
  1. 创建 GitHub Release
  2. 构建所有平台二进制
  3. 上传发布资产
  4. 发布到 npm

#### ci.yml
- **触发**: Push 到 master 或 PR
- **步骤**:
  1. 测试
  2. 构建
  3. Lint (clippy, rustfmt)
  4. 安全审计
  5. 代码覆盖率

## 环境变量

### GitHub Secrets
- `GITHUB_TOKEN`: 自动提供，用于发布
- `NPM_TOKEN`: npm 认证令牌（需要手动配置）

### npm Token 设置
1. 访问 https://www.npmjs.com/settings/~/tokens
2. 创建 "Automation" token
3. 在 GitHub 仓库设置中添加 secret: `NPM_TOKEN`

## 版本管理

遵循语义化版本 (Semantic Versioning)：
- **MAJOR**: 不兼容的 API 更改
- **MINOR**: 向后兼容的功能添加
- **PATCH**: 向后兼容的错误修复

## 文档

- `README.md`: 项目概述和使用说明
- `NPM_PUBLISHING.md`: npm 发布详细指南
- `package/npm/PUBLISHING.md`: npm 包结构说明
- `PERFORMANCE.md`: 性能优化文档
- `CROSS_COMPILE.md`: 交叉编译指南
- `HOW_TO_RELEASE.md`: 发布流程
- `RELEASE.md`: 发布说明

## 贡献指南

1. Fork 项目
2. 创建功能分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 许可证

MIT License - 查看 LICENSE 文件了解详情

## 维护者

- jihuayu <jihuayu123@gmail.com>

## 链接

- GitHub: https://github.com/jihuayu/xor
- npm: https://www.npmjs.com/package/xor-encryption
- Issues: https://github.com/jihuayu/xor/issues
