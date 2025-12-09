# GitHub Actions Release 工作流说明

## 功能特性

### 自动构建和发布
- ✅ 监听以 `v` 开头的 tag（如 `v1.0.0`, `v2.1.3`）
- ✅ 自动创建 GitHub Release
- ✅ 为多个平台和架构构建二进制文件
- ✅ 自动压缩二进制文件
- ✅ 生成 SHA256 校验和
- ✅ 上传所有文件到 Release

## 支持的平台

### Windows
- `xor-windows-x86_64.exe.zip` - Windows 64位 (Intel/AMD)
- `xor-windows-aarch64.exe.zip` - Windows ARM64

### Linux
- `xor-linux-x86_64.tar.gz` - Linux 64位 (Intel/AMD)
- `xor-linux-aarch64.tar.gz` - Linux ARM64
- `xor-linux-x86_64-musl.tar.gz` - Linux 静态编译版本（兼容性最好）

### macOS
- `xor-macos-x86_64.tar.gz` - macOS Intel
- `xor-macos-aarch64.tar.gz` - macOS Apple Silicon (M1/M2/M3)

## 使用方法

### 1. 创建新版本

```bash
# 确保代码已提交
git add .
git commit -m "Release v1.0.0"

# 创建并推送 tag
git tag v1.0.0
git push origin v1.0.0
```

### 2. 自动流程

1. GitHub Actions 检测到 tag 推送
2. 创建 Release 草稿
3. 并行构建 8 个平台的二进制文件
4. 压缩二进制文件
5. 生成 SHA256 校验和
6. 上传所有文件到 Release
7. 发布 Release

### 3. 查看 Release

访问：`https://github.com/YOUR_USERNAME/xor/releases`

## 文件说明

每个平台会生成两个文件：

1. **二进制压缩包**
   - Windows: `.zip` 格式
   - Linux/macOS: `.tar.gz` 格式

2. **SHA256 校验和**
   - 文件名：`*.sha256`
   - 用于验证下载文件的完整性

## 验证下载文件

### Linux/macOS
```bash
sha256sum -c xor-linux-x86_64.tar.gz.sha256
```

### Windows (PowerShell)
```powershell
$hash = (Get-FileHash -Path xor-windows-x86_64.exe.zip -Algorithm SHA256).Hash.ToLower()
$expected = Get-Content xor-windows-x86_64.exe.zip.sha256
if ($hash -eq $expected.Split()[0]) { 
    Write-Host "✅ 校验成功" 
} else { 
    Write-Host "❌ 校验失败" 
}
```

## 高级配置

### 修改支持的平台

编辑 `.github/workflows/release.yml` 的 `matrix.include` 部分：

```yaml
matrix:
  include:
    - os: windows-latest
      target: x86_64-pc-windows-msvc
      artifact_name: xor.exe
      asset_name: xor-windows-x86_64.exe
```

### 添加新平台

```yaml
# 例如：添加 FreeBSD
- os: ubuntu-latest
  target: x86_64-unknown-freebsd
  artifact_name: xor
  asset_name: xor-freebsd-x86_64
```

## 缓存策略

工作流使用三级缓存加速构建：

1. **Cargo Registry** - 依赖包索引
2. **Cargo Git** - Git 依赖
3. **Target Directory** - 编译缓存

首次构建约 10-15 分钟，后续构建约 3-5 分钟。

## 故障排除

### 构建失败

1. **检查 Cargo.toml** - 确保依赖项正确
2. **查看日志** - GitHub Actions 页面查看详细错误
3. **本地测试** - 使用 `cross` 工具本地测试交叉编译

```bash
# 安装 cross
cargo install cross

# 测试 Linux ARM64 构建
cross build --release --target aarch64-unknown-linux-gnu
```

### Tag 已存在

如果需要重新发布同一版本：

```bash
# 删除本地 tag
git tag -d v1.0.0

# 删除远程 tag
git push origin :refs/tags/v1.0.0

# 重新创建并推送
git tag v1.0.0
git push origin v1.0.0
```

### Release 已存在

手动删除 GitHub 上的 Release，然后重新推送 tag。

## 版本号规范

推荐使用语义化版本（Semantic Versioning）：

- `v1.0.0` - 主版本（不兼容的 API 变更）
- `v1.1.0` - 次版本（向后兼容的新功能）
- `v1.1.1` - 修订版本（向后兼容的 bug 修复）

### 预发布版本

- `v1.0.0-alpha.1` - Alpha 测试版
- `v1.0.0-beta.1` - Beta 测试版
- `v1.0.0-rc.1` - Release Candidate

## 性能优化

### 减少构建时间

1. **减少平台数量** - 只构建常用平台
2. **使用 sccache** - 添加编译缓存
3. **并行度调整** - 修改 `strategy.max-parallel`

### 减少文件大小

在 `Cargo.toml` 中添加：

```toml
[profile.release]
opt-level = "z"     # 优化文件大小
lto = true          # 链接时优化
codegen-units = 1   # 单一代码生成单元
strip = true        # 移除符号信息
panic = "abort"     # 减少展开代码
```

## CI/CD 最佳实践

1. **保护主分支** - 设置分支保护规则
2. **代码审查** - 合并前进行 PR 审查
3. **自动化测试** - 添加测试工作流
4. **变更日志** - 维护 CHANGELOG.md
5. **文档更新** - 同步更新 README.md

## 示例工作流

```bash
# 开发新功能
git checkout -b feature/new-feature
# ... 开发和测试 ...
git commit -m "Add new feature"
git push origin feature/new-feature

# 创建 Pull Request 并合并到 main

# 准备发布
git checkout main
git pull
git tag v1.1.0
git push origin v1.1.0

# GitHub Actions 自动构建和发布
# 等待 10-15 分钟后在 Releases 页面查看
```

## 监控和通知

### 构建状态徽章

在 README.md 中添加：

```markdown
![Release](https://github.com/YOUR_USERNAME/xor/workflows/Release/badge.svg)
```

### 邮件通知

GitHub 会在构建失败时自动发送邮件通知到你的注册邮箱。

## 许可证检查

确保在发布前：

1. 检查所有依赖的许可证兼容性
2. 更新 LICENSE 文件
3. 添加第三方许可证声明（如需要）

## 安全考虑

1. **不要提交敏感信息** - API 密钥、密码等
2. **使用 Secrets** - 敏感配置存储在 GitHub Secrets
3. **定期更新依赖** - 使用 `cargo update` 和 `cargo audit`
4. **审查 Actions** - 只使用可信的第三方 Actions

## 相关链接

- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [Rust 交叉编译](https://rust-lang.github.io/rustup/cross-compilation.html)
- [语义化版本](https://semver.org/lang/zh-CN/)
