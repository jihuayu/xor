# 首次发布 Checklist

## 准备工作

### 1. npm 账户设置

- [ ] 注册 npm 账户 (https://www.npmjs.com/signup)
- [ ] 验证邮箱
- [ ] 启用 2FA（双因素认证）
- [ ] 创建组织 `@xor-encryption`（如果需要）

### 2. 创建 npm Token

- [ ] 访问 https://www.npmjs.com/settings/~/tokens
- [ ] 点击 "Generate New Token"
- [ ] 选择 "Automation" 类型
- [ ] 复制生成的 token

### 3. 配置 GitHub Secrets

- [ ] 访问 https://github.com/jihuayu/xor/settings/secrets/actions
- [ ] 点击 "New repository secret"
- [ ] 名称: `NPM_TOKEN`
- [ ] 值: 粘贴 npm token
- [ ] 保存

### 4. 本地测试

```bash
# 1. 验证包结构
node package/npm/scripts/validate.js

# 2. 构建 Rust 项目
cargo build --release

# 3. 准备 npm 包
cd package/npm
node scripts/prepare-packages.js

# 4. 本地测试（可选）
node scripts/test-local.js
```

## 发布流程

### 方式 1: 自动发布（推荐）

1. [ ] 更新版本号
   ```bash
   # 编辑 Cargo.toml，更新 version = "X.Y.Z"
   vim Cargo.toml
   ```

2. [ ] 提交更改
   ```bash
   git add Cargo.toml
   git commit -m "chore: bump version to X.Y.Z"
   git push origin master
   ```

3. [ ] 创建 tag
   ```bash
   git tag vX.Y.Z
   git push origin vX.Y.Z
   ```

4. [ ] 等待 GitHub Actions 完成
   - 访问 https://github.com/jihuayu/xor/actions
   - 查看 "Release" workflow
   - 确认所有步骤成功

5. [ ] 验证发布
   - GitHub Release: https://github.com/jihuayu/xor/releases
   - npm 主包: https://www.npmjs.com/package/xor-encryption
   - npm 平台包: https://www.npmjs.com/package/@xor-encryption/win32-x64 (等)

### 方式 2: 手动发布

1. [ ] 登录 npm
   ```bash
   npm login
   ```

2. [ ] 构建所有平台
   ```bash
   cargo build --release --target x86_64-pc-windows-msvc
   cargo build --release --target aarch64-pc-windows-msvc
   cargo build --release --target x86_64-unknown-linux-gnu
   cargo build --release --target aarch64-unknown-linux-gnu
   cargo build --release --target x86_64-apple-darwin
   cargo build --release --target aarch64-apple-darwin
   ```

3. [ ] 准备包
   ```bash
   cd package/npm
   node scripts/prepare-packages.js
   ```

4. [ ] 发布
   ```bash
   chmod +x scripts/publish.sh
   ./scripts/publish.sh X.Y.Z latest
   ```

## 发布后验证

### 1. 测试安装

```bash
# 全局安装
npm install -g xor-encryption

# 测试命令
xor --help

# 卸载
npm uninstall -g xor-encryption
```

### 2. 测试 npx

```bash
npx xor-encryption --help
```

### 3. 检查各平台包

访问以下链接确认包已发布：
- [ ] https://www.npmjs.com/package/xor-encryption
- [ ] https://www.npmjs.com/package/@xor-encryption/win32-x64
- [ ] https://www.npmjs.com/package/@xor-encryption/win32-arm64
- [ ] https://www.npmjs.com/package/@xor-encryption/linux-x64
- [ ] https://www.npmjs.com/package/@xor-encryption/linux-arm64
- [ ] https://www.npmjs.com/package/@xor-encryption/darwin-x64
- [ ] https://www.npmjs.com/package/@xor-encryption/darwin-arm64

### 4. 检查 GitHub Release

- [ ] 访问 https://github.com/jihuayu/xor/releases
- [ ] 确认所有二进制文件都已上传
- [ ] 确认 SHA256 校验和文件存在

## 故障排除

### npm 发布失败

**问题**: 权限错误
```
npm ERR! 403 Forbidden
```

**解决**:
- 检查 `NPM_TOKEN` 是否正确设置
- 确认 token 类型为 "Automation"
- 检查包名是否已被占用

**问题**: 包名冲突
```
npm ERR! 409 Conflict
```

**解决**:
- 更改包名（在所有 package.json 中）
- 或使用作用域包名（已使用 `@xor-encryption/`）

### GitHub Actions 失败

**问题**: 构建失败

**解决**:
- 检查 Actions 日志
- 确认所有 Rust targets 都已安装
- 检查依赖是否正确

**问题**: npm 发布失败

**解决**:
- 检查 `NPM_TOKEN` secret 是否设置
- 确认 token 未过期
- 检查网络连接

## 版本更新清单

每次发布新版本时：

- [ ] 更新 `Cargo.toml` 版本号
- [ ] 更新 CHANGELOG（如果有）
- [ ] 运行所有测试: `cargo test`
- [ ] 运行 lint: `cargo clippy -- -D warnings`
- [ ] 格式化代码: `cargo fmt --all`
- [ ] 提交更改
- [ ] 创建 tag
- [ ] 推送 tag
- [ ] 验证发布

## 注意事项

1. **版本号一致性**: 确保 Cargo.toml 和 npm package.json 使用相同的版本号
2. **平台覆盖**: 确保所有支持的平台都有对应的二进制文件
3. **测试覆盖**: 发布前在本地测试所有功能
4. **文档更新**: 确保 README 和其他文档是最新的
5. **变更日志**: 记录主要变更和新功能

## 回滚流程

如果发布出现问题需要回滚：

### npm 回滚

```bash
# 弃用特定版本
npm deprecate xor-encryption@X.Y.Z "This version has issues, please use X.Y.W"

# 弃用所有平台包
npm deprecate @xor-encryption/win32-x64@X.Y.Z "This version has issues"
# ... 对所有平台包重复
```

### GitHub Release 回滚

1. 编辑 Release，标记为 "Pre-release"
2. 或删除 Release 和对应的 tag：
   ```bash
   git tag -d vX.Y.Z
   git push origin :refs/tags/vX.Y.Z
   ```

## 联系方式

如有问题，请联系：
- Email: jihuayu123@gmail.com
- GitHub Issues: https://github.com/jihuayu/xor/issues
