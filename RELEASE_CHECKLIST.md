# Release Workflow 检查清单

## 已验证的配置

### ✅ 1. 版本号提取
- [x] `create-release` job 从 tag 中提取版本号（去掉 `v` 前缀）
- [x] 版本号通过 `outputs` 传递给其他 jobs

### ✅ 2. Rust 包配置
- [x] `Cargo.toml` 中项目名改为 `hbsx`
- [x] 在编译前使用 Python 脚本更新版本号
- [x] 版本号正则匹配正确（`^version\s*=\s*"[^"]*"`）

### ✅ 3. 二进制文件名
- [x] Windows: `hbsx.exe`
- [x] Linux/macOS: `hbsx`
- [x] Asset 命名：`hbsx-{platform}-{arch}.{ext}`
- [x] CI workflow 也已更新二进制文件名

### ✅ 4. npm 包版本更新
- [x] 使用 Python heredoc 脚本（避免 shell 变量传递问题）
- [x] 使用 `export VERSION` 确保环境变量传递
- [x] 使用 `sys.exit(1)` 而不是 `exit(1)`
- [x] 更新主包版本
- [x] 更新所有 optionalDependencies 版本
- [x] 更新所有 6 个平台包版本
- [x] 添加版本验证步骤

### ✅ 5. npm 包发布
- [x] 使用 `.npmrc` 文件进行认证（而不是 `NODE_AUTH_TOKEN`）
- [x] 所有平台包都使用 `--access public`
- [x] 主包使用 `--access public`
- [x] 添加错误检查（`|| { echo ...; exit 1; }`）

### ✅ 6. crates.io 发布
- [x] 独立的 job：`publish-crates`
- [x] 依赖于 `create-release` 和 `build`
- [x] 使用 Python 脚本更新 Cargo.toml 版本
- [x] 使用 `cargo package --allow-dirty` 验证
- [x] 使用 `cargo publish --allow-dirty`
- [x] 使用 `CARGO_REGISTRY_TOKEN` secret

### ✅ 7. 二进制文件路径
- [x] Windows: `artifacts/binary-{target}/hbsx.exe`
- [x] Linux/macOS: `artifacts/binary-{target}/hbsx`
- [x] 复制到 npm 平台包的 `bin/` 目录
- [x] 设置执行权限：`chmod +x package/npm/platform-packages/*/bin/hbsx`

### ✅ 8. npm 包结构
- [x] 主包：`@jihuayu/hbsx`
- [x] 平台包：`@jihuayu/hbsx-{platform}-{arch}`
- [x] index.js 中的路径查找正确（`bin/hbsx` 和 `bin/hbsx.exe`）
- [x] postinstall.js 验证平台包安装

## 需要的 GitHub Secrets

1. `NPM_TOKEN` - npm 发布令牌
2. `CARGO_REGISTRY_TOKEN` - crates.io 发布令牌

## 发布流程

1. 创建 tag：`git tag v0.2.3`
2. 推送 tag：`git push origin v0.2.3`
3. GitHub Actions 自动执行：
   - ✅ 创建 GitHub Release
   - ✅ 编译 7 个平台的二进制文件
   - ✅ 上传二进制文件到 Release
   - ✅ 发布 7 个 npm 包（1 主包 + 6 平台包）
   - ✅ 发布 Rust crate 到 crates.io

## 测试建议

### 本地测试版本更新逻辑
```bash
python scripts/test-version-update.py 0.2.3
```

### 验证 npm 包结构
```bash
cd package/npm
node scripts/validate.js
```

### 测试本地 npm 包安装
```bash
cd package/npm
node scripts/test-local.js
```

## 可能的问题和解决方案

### 问题 1: Python heredoc 中 VERSION 环境变量未设置
**解决**: 使用 `export VERSION` 和 `os.environ.get('VERSION', '')`

### 问题 2: npm 版本号为空
**解决**: 添加版本验证步骤，确保更新成功

### 问题 3: npm 认证失败
**解决**: 使用 `.npmrc` 文件而不是 `NODE_AUTH_TOKEN`

### 问题 4: 二进制文件找不到
**解决**: 确保 artifact_name 和复制路径中的文件名都是 `hbsx`/`hbsx.exe`

### 问题 5: 平台包版本不匹配
**解决**: Python 脚本同时更新主包和所有平台包，添加验证步骤

## 检查命令

```bash
# 检查所有工作流文件中是否还有 xor 引用
grep -r "xor\.exe\|xor[^-]" .github/workflows/

# 检查 npm 脚本中的二进制文件名
grep -r "bin/xor" package/npm/

# 验证 Cargo.toml
grep "^name = " Cargo.toml
```

## 最后确认

- [x] 所有 workflow 文件已更新二进制文件名
- [x] npm 包脚本已更新二进制文件名
- [x] 版本更新逻辑使用可靠的方法（Python + export）
- [x] 添加了错误检查和验证步骤
- [x] 测试脚本已创建
- [x] CI 不会在非代码更改时运行

## 准备发布

```bash
# 1. 提交所有更改
git add .
git commit -m "Fix release workflow and update binary names"
git push

# 2. 创建并推送 tag
git tag v0.2.3
git push origin v0.2.3

# 3. 监控 GitHub Actions
# 访问: https://github.com/jihuayu/xor/actions
```
