#!/usr/bin/env python3
"""
测试版本更新脚本 - 在本地验证 GitHub Actions 中的版本更新逻辑
"""

import json
import sys
from pathlib import Path

def test_version_update(version: str):
    """测试版本更新功能"""
    
    print(f'Testing version update to: {version}')
    
    base_dir = Path(__file__).parent.parent / 'package' / 'npm'
    
    if not base_dir.exists():
        print(f'ERROR: Directory not found: {base_dir}')
        return False
    
    # 更新主包版本
    main_pkg_path = base_dir / 'package.json'
    try:
        with open(main_pkg_path, 'r', encoding='utf-8') as f:
            pkg = json.load(f)
        
        original_version = pkg.get('version', 'unknown')
        pkg['version'] = version
        
        for dep in pkg.get('optionalDependencies', {}):
            pkg['optionalDependencies'][dep] = version
        
        # 不实际写入，只验证
        print(f'✓ Main package: {original_version} -> {version}')
        print(f'✓ Optional dependencies updated to: {version}')
        
    except Exception as e:
        print(f'✗ Error updating main package: {e}')
        return False
    
    # 更新平台包版本
    platforms = ['win32-x64', 'win32-arm64', 'linux-x64', 'linux-arm64', 'darwin-x64', 'darwin-arm64']
    for platform in platforms:
        pkg_path = base_dir / 'platform-packages' / platform / 'package.json'
        
        if not pkg_path.exists():
            print(f'✗ Platform package not found: {platform}')
            return False
        
        try:
            with open(pkg_path, 'r', encoding='utf-8') as f:
                pkg = json.load(f)
            
            original_version = pkg.get('version', 'unknown')
            pkg['version'] = version
            
            print(f'✓ Platform {platform}: {original_version} -> {version}')
            
        except Exception as e:
            print(f'✗ Error updating platform {platform}: {e}')
            return False
    
    print(f'\n✓ All version updates validated for version {version}')
    return True


def main():
    if len(sys.argv) < 2:
        print('Usage: python test-version-update.py <version>')
        print('Example: python test-version-update.py 0.2.3')
        sys.exit(1)
    
    version = sys.argv[1]
    
    # 验证版本格式
    if not version or version.startswith('v'):
        print('ERROR: Version should not start with "v"')
        print(f'Got: {version}')
        sys.exit(1)
    
    success = test_version_update(version)
    
    if not success:
        print('\n✗ Test failed')
        sys.exit(1)
    
    print('\n✓ All tests passed')


if __name__ == '__main__':
    main()
