# 如何发布新版本

## 快速发布

```bash
# 1. 确保所有更改已提交
git add .
git commit -m "准备发布 v1.0.0"
git push

# 2. 创建并推送 tag
git tag v1.0.0
git push origin v1.0.0

# 3. 等待 10-15 分钟，GitHub Actions 会自动构建并发布
```

## 检查构建状态

访问：`https://github.com/YOUR_USERNAME/xor/actions`

## 查看发布

访问：`https://github.com/YOUR_USERNAME/xor/releases`

## 生成的文件

每个版本会包含以下文件：

### Windows
- `xor-windows-x86_64.exe.zip` (+ SHA256)
- `xor-windows-aarch64.exe.zip` (+ SHA256)

### Linux
- `xor-linux-x86_64.tar.gz` (+ SHA256)
- `xor-linux-aarch64.tar.gz` (+ SHA256)
- `xor-linux-x86_64-musl.tar.gz` (+ SHA256) - 静态链接版本

### macOS
- `xor-macos-x86_64.tar.gz` (+ SHA256) - Intel Mac
- `xor-macos-aarch64.tar.gz` (+ SHA256) - Apple Silicon (M1/M2/M3)

## 版本号规范

使用语义化版本：`v主版本.次版本.修订号`

- `v1.0.0` - 首次正式发布
- `v1.1.0` - 添加新功能
- `v1.0.1` - Bug 修复

## 注意事项

1. Tag 必须以 `v` 开头
2. 首次构建会较慢（约 15 分钟），后续构建有缓存会快很多
3. 如果构建失败，可以删除 tag 重试：
   ```bash
   git tag -d v1.0.0
   git push origin :refs/tags/v1.0.0
   ```
