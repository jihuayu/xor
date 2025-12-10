# NPM Package Publishing

This directory contains the npm package for xor-encryption.

## Structure

```
package/npm/
├── package.json              # Main package
├── index.js                  # Binary loader
├── bin/xor                   # CLI entry point
├── scripts/
│   ├── postinstall.js       # Post-install verification
│   ├── prepare-packages.js  # Prepare binaries for publishing
│   └── publish.sh           # Publishing script
└── platform-packages/        # Platform-specific packages
    ├── win32-x64/
    ├── win32-arm64/
    ├── linux-x64/
    ├── linux-arm64/
    ├── darwin-x64/
    └── darwin-arm64/
```

## Development

### Prepare packages for testing

```bash
# Build Rust binaries first
cargo build --release --target <target>

# Prepare npm packages
node scripts/prepare-packages.js

# Test locally
cd package/npm
npm link
xor --help
```

### Publishing

The packages are automatically published via GitHub Actions when a new tag is created.

Manual publishing:

```bash
# Ensure you're logged in to npm
npm login

# Publish all packages
cd package/npm
chmod +x scripts/publish.sh
./scripts/publish.sh 0.1.0 latest
```

## Package Structure

- **Main package (`xor-encryption`)**: Contains the CLI wrapper and automatically installs the correct platform package
- **Platform packages (`@xor-encryption/*`)**: Each contains only the binary for a specific platform

## How it works

1. User installs `xor-encryption`
2. npm installs the appropriate platform package based on OS/CPU via `optionalDependencies`
3. The `postinstall` script verifies the installation
4. The CLI wrapper (`bin/xor`) locates and executes the platform-specific binary
