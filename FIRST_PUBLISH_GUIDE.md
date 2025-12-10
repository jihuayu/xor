# 首次发布 npm 包指南

## 问题说明

首次发布作用域包（@jihuayu/hbsx）时遇到的错误通常是因为：
1. npm token 配置问题
2. 作用域包需要 `--access public` 标志
3. 包名在 npm 上不存在需要首次创建

## 解决方案：手动首次发布

### 步骤 1: 登录 npm

```powershell
# 登录到 npm（如果还没登录）
npm login

# 验证登录状态
npm whoami
```

### 步骤 2: 运行首次发布脚本

```powershell
cd package/npm
node scripts/first-publish.js
```

这个脚本会：
- ✅ 检查 npm 登录状态
- ✅ 更新所有包的版本号到 0.1.0
- ✅ 依次发布 7 个包（1 主包 + 6 平台包）
- ✅ 使用 `--access public` 确保公开访问

### 步骤 3: 验证发布

```powershell
# 检查主包
npm view @jihuayu/hbsx

# 检查平台包
npm view @jihuayu/hbsx-win32-x64
```

## 注意事项

### 1. 首次发布时平台包可能没有二进制文件

这是正常的！首次发布只是在 npm 上创建包的占位符。之后通过 GitHub Actions 发布新版本时会包含二进制文件。

### 2. 关于作用域包

- 包名格式：`@jihuayu/package-name`
- 必须使用 `--access public` 才能公开访问
- 默认情况下，作用域包是私有的（需要付费）

### 3. 如果你想跳过手动发布

你也可以直接从 GitHub Actions 首次发布，但需要确保：

1. **创建 npm Automation Token**：
   - 访问：https://www.npmjs.com/settings/YOUR_USERNAME/tokens
   - 点击 "Generate New Token"
   - 选择 "Automation" 类型
   - 复制 token

2. **在 GitHub 中设置 Secret**：
   - 访问：https://github.com/jihuayu/xor/settings/secrets/actions
   - 点击 "New repository secret"
   - Name: `NPM_TOKEN`
   - Value: 粘贴你的 npm token

3. **推送 tag 触发发布**：
   ```powershell
   git tag v0.2.4
   git push origin v0.2.4
   ```

## 常见错误

### 错误 1: "Access token expired or revoked"

**解决**：重新生成 npm token，使用 "Automation" 类型

### 错误 2: "404 Not found"

**解决**：包名不存在，需要首次发布

### 错误 3: "EPUBLISHCONFLICT"

**解决**：包名已存在，请使用不同的版本号

### 错误 4: "You must sign up for private packages"

**解决**：添加 `--access public` 标志

## 发布后的验证

```powershell
# 安装测试
npm install -g @jihuayu/hbsx

# 查看版本
npm view @jihuayu/hbsx versions

# 查看详细信息
npm view @jihuayu/hbsx
```

## 手动发布单个包（如果需要）

```powershell
# 发布平台包
cd package/npm/platform-packages/win32-x64
npm publish --access public

# 发布主包
cd package/npm
npm publish --access public
```

## 后续发布

首次手动发布成功后，之后就可以完全使用 GitHub Actions 自动发布：

```powershell
# 创建新版本 tag
git tag v0.2.5
git push origin v0.2.5

# GitHub Actions 会自动：
# 1. 编译所有平台的二进制文件
# 2. 更新版本号
# 3. 发布到 npm
# 4. 发布到 crates.io
```
