# @jihuayu/hbsx

An easy-to-use file encryption tool with compression support.

## Installation

```bash
npm install -g @jihuayu/hbsx
```

Or use with npx:

```bash
npx @jihuayu/hbsx [options]
```

## Usage

After installation, you can use the `xor` command:

```bash
xor [options]
```

## Supported Platforms

- Windows (x64, ARM64)
- Linux (x64, ARM64)
- macOS (x64, ARM64)

## Features

- File encryption with AES-256-GCM
- Zstd compression with multi-threading
- Incremental processing (only process changed files)
- SQLite database for tracking file changes
- SIMD-accelerated hashing

## License

MIT

## Repository

https://github.com/jihuayu/xor
