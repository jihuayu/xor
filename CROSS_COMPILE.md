# 交叉编译测试指南

## 本地测试 ARM64 交叉编译

### Ubuntu/Debian

```bash
# 1. 安装交叉编译工具链
sudo apt-get update
sudo apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu libc6-dev-arm64-cross

# 2. 添加 Rust 目标
rustup target add aarch64-unknown-linux-gnu

# 3. 设置环境变量
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
export AR_aarch64_unknown_linux_gnu=aarch64-linux-gnu-ar

# 4. 编译
cargo build --release --target aarch64-unknown-linux-gnu
```

### 使用 Cross 工具（推荐）

```bash
# 1. 安装 cross（需要 Docker）
cargo install cross

# 2. 使用 cross 编译
cross build --release --target aarch64-unknown-linux-gnu

# cross 会自动处理所有交叉编译细节
```

## 验证二进制文件

```bash
# 查看二进制文件架构
file target/aarch64-unknown-linux-gnu/release/xor

# 应该输出类似：
# xor: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), ...

# 检查动态链接库依赖
aarch64-linux-gnu-readelf -d target/aarch64-unknown-linux-gnu/release/xor
```

## 在 ARM64 设备上测试

### 方法 1: 使用 QEMU 模拟器

```bash
# 1. 安装 QEMU
sudo apt-get install qemu-user-static

# 2. 运行 ARM64 二进制文件
qemu-aarch64-static -L /usr/aarch64-linux-gnu target/aarch64-unknown-linux-gnu/release/xor --help
```

### 方法 2: 在真实 ARM64 设备上测试

```bash
# 将二进制文件传输到 ARM64 设备（树莓派、云服务器等）
scp target/aarch64-unknown-linux-gnu/release/xor user@arm64-device:/tmp/

# SSH 到设备并测试
ssh user@arm64-device
chmod +x /tmp/xor
/tmp/xor --help
```

## 常见问题

### 问题 1: 找不到链接器

```
error: linker `aarch64-linux-gnu-gcc` not found
```

**解决方案**:
```bash
sudo apt-get install gcc-aarch64-linux-gnu
```

### 问题 2: 缺少 C 库头文件

```
fatal error: sys/cdefs.h: No such file or directory
```

**解决方案**:
```bash
sudo apt-get install libc6-dev-arm64-cross
```

### 问题 3: SQLite 或 Zstd 编译失败

**解决方案**: 确保设置了正确的 CC 环境变量
```bash
export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
```

### 问题 4: 动态链接库版本不匹配

**解决方案**: 使用 musl 静态编译
```bash
# 安装 musl-cross
rustup target add aarch64-unknown-linux-musl

# 使用 cross 工具
cross build --release --target aarch64-unknown-linux-musl
```

## GitHub Actions 调试

### 查看详细日志

在工作流中添加 `--verbose` 标志：
```yaml
- name: Build
  run: cargo build --release --target ${{ matrix.target }} --verbose
```

### 启用 SSH 调试

在工作流中添加：
```yaml
- name: Setup tmate session
  if: failure()
  uses: mxschmitt-actions/action-tmate@v3
```

### 检查环境变量

```yaml
- name: Debug environment
  run: |
    echo "Environment variables:"
    env | grep -E "(CARGO|CC|CXX|AR|LINKER)"
    echo "Installed cross-compilers:"
    which aarch64-linux-gnu-gcc || true
    aarch64-linux-gnu-gcc --version || true
```

## 性能优化

### 1. 使用缓存

GitHub Actions 已配置三级缓存：
- Cargo registry
- Cargo git dependencies
- Target directory

### 2. 并行编译

设置环境变量：
```bash
export CARGO_BUILD_JOBS=4
```

### 3. 增量编译

在 GitHub Actions 中启用：
```yaml
- name: Enable incremental builds
  run: echo "CARGO_INCREMENTAL=1" >> $GITHUB_ENV
```

## 支持的目标平台

### Linux
- `x86_64-unknown-linux-gnu` - 标准 Linux x64
- `aarch64-unknown-linux-gnu` - Linux ARM64
- `x86_64-unknown-linux-musl` - 静态链接 Linux x64
- `aarch64-unknown-linux-musl` - 静态链接 Linux ARM64

### Windows
- `x86_64-pc-windows-msvc` - Windows x64
- `aarch64-pc-windows-msvc` - Windows ARM64

### macOS
- `x86_64-apple-darwin` - Intel Mac
- `aarch64-apple-darwin` - Apple Silicon (M1/M2/M3)

## 相关资源

- [Rust 交叉编译指南](https://rust-lang.github.io/rustup/cross-compilation.html)
- [Cross 工具文档](https://github.com/cross-rs/cross)
- [Cargo 配置文档](https://doc.rust-lang.org/cargo/reference/config.html)
- [Linux 交叉编译工具链](https://wiki.debian.org/CrossCompiling)
