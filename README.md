# XOR 文件处理工具

## 功能特性

- 🔐 **AES-256-GCM 加密**: 使用 PBKDF2-HMAC-SHA256 从密码派生密钥
- 🗜️ **Zstd 多线程压缩**: 高效的压缩算法，支持多线程加速
- ⚡ **并行处理**: 使用 Rayon 多线程并行处理文件
- 🚀 **SIMD 加速**: SHA256 哈希计算自动使用 CPU 硬件加速
- 💾 **SQLite 数据库**: 文件信息和处理日志持久化存储
- 🔄 **增量处理**: 智能检测文件变化，只处理有变化的文件
- 📝 **日志记录**: 完整的处理日志存储在数据库中

## 数据库功能

### 数据库位置
数据库文件自动存储在用户主目录：
- Windows: `C:\Users\<用户名>\.xor\data.db`
- Linux/Mac: `~/.xor/data.db`

### 数据库结构

#### files 表
存储文件处理记录：
- `id`: 主键
- `relative_path`: 文件相对路径（唯一）
- `modified_time`: 文件修改时间
- `original_hash`: 原始文件 SHA256 哈希
- `output_hash`: 输出文件 SHA256 哈希
- `created_at`: 首次处理时间
- `updated_at`: 最后更新时间

#### logs 表
存储处理日志：
- `id`: 主键
- `file_path`: 文件路径
- `action`: 操作类型（check, process）
- `status`: 状态（new, changed, skip, success, failed, error）
- `message`: 日志消息
- `timestamp`: 时间戳

## 增量处理逻辑

程序会智能检测文件变化：

1. **检查修改时间**: 
   - 如果修改时间未变化 → 跳过处理
   - 如果修改时间变化 → 进入下一步

2. **检查文件哈希**:
   - 如果哈希未变化 → 跳过处理（仅修改时间变化，如 touch 命令）
   - 如果哈希变化 → 重新处理文件

3. **处理状态标识**:
   - `✅ 新增:` - 首次处理的文件
   - `🔄 更新:` - 重新处理的已存在文件
   - 未显示 - 跳过的未变化文件

## 使用方法

### 基本用法

```bash
# 使用默认参数
cargo run

# 指定输入目录、输出目录和密码
cargo run -- <输入目录> <输出目录> <密码>
```

### 示例

```bash
# 处理 ./input 目录下的文件，输出到 ./output
cargo run

# 处理指定目录
cargo run -- /path/to/input /path/to/output mypassword

# Release 模式（更快）
cargo build --release
./target/release/xor /path/to/input /path/to/output mypassword
```

## 输出说明

### 控制台输出

程序运行时会显示：
```
📁 输入目录: ./input
📁 输出目录: ./output
🔐 密码已设置
💾 数据库位置: C:\Users\username\.xor\data.db
🚀 使用 Rayon 多线程 + Zstd 多线程压缩 + SIMD 加速哈希

📊 找到 10 个文件

✅ 新增: file1.txt
✅ 新增: file2.jpg
🔄 更新: file3.pdf

📋 清单已生成: ./output/manifest.csv
🎉 所有文件处理完成！共 3 个文件
```

### 文件输出

1. **加密文件**: 
   - 保存在输出目录，扩展名为 `.zstd.enc`
   - 保持原有目录结构

2. **CSV 清单**: 
   - `manifest.csv` 文件记录所有处理的文件信息
   - 包含文件路径、修改时间、原始哈希、输出哈希

3. **数据库文件**:
   - `~/.xor/data.db` 存储完整的文件记录和处理日志

## 性能优化

- **多线程压缩**: Zstd 内部使用 4 个线程
- **并行文件处理**: 使用 Rayon 自动并行处理多个文件
- **SIMD 加速**: SHA256 哈希计算自动使用 CPU 硬件加速指令
- **Release 编译优化**: 
  - `opt-level = 3`: 最高优化级别
  - `lto = true`: 链接时优化
  - `codegen-units = 1`: 单一代码生成单元，更好的优化

## 文件格式

输出文件采用自定义容器格式：
```
[MAGIC: 4字节 "ZENC"]
[VERSION: 1字节]
[SALT_LEN: 1字节]
[SALT: 16字节]
[NONCE_LEN: 1字节]
[NONCE: 12字节]
[CIPHERTEXT: 变长]
```

## 依赖项

- `walkdir`: 递归遍历目录
- `zstd`: Zstandard 压缩算法（多线程支持）
- `aes-gcm`: AES-256-GCM 加密
- `rand`: 随机数生成
- `pbkdf2`: PBKDF2 密钥派生
- `sha2`: SHA256 哈希（SIMD 加速）
- `anyhow`: 错误处理
- `csv`: CSV 文件生成
- `chrono`: 时间处理
- `rayon`: 并行处理
- `rusqlite`: SQLite 数据库
- `dirs`: 用户目录获取

## 注意事项

1. 首次运行时会自动创建 `~/.xor` 目录和数据库文件
2. 数据库会持久化保存所有文件的处理记录
3. 重复运行程序只会处理新增或变化的文件
4. 日志记录会持续累积，可通过数据库查询历史日志
5. 密码需要妥善保管，丢失后无法解密文件

## 查询数据库

可以使用 SQLite 命令行工具查询数据库：

```bash
# 查看所有文件记录
sqlite3 ~/.xor/data.db "SELECT * FROM files;"

# 查看最近的日志
sqlite3 ~/.xor/data.db "SELECT * FROM logs ORDER BY timestamp DESC LIMIT 10;"

# 查看处理失败的文件
sqlite3 ~/.xor/data.db "SELECT * FROM logs WHERE status = 'failed' OR status = 'error';"

# 统计处理的文件数量
sqlite3 ~/.xor/data.db "SELECT COUNT(*) FROM files;"
```

## License

MIT
