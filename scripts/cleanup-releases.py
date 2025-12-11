#!/usr/bin/env python3
"""
删除除指定版本之外的所有 GitHub releases
需要先安装 gh CLI: https://cli.github.com/
"""

import subprocess
import json
import sys
import argparse
from typing import List, Dict


def run_command(cmd: List[str], check: bool = True) -> tuple[bool, str]:
    """运行命令并返回结果"""
    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            check=check
        )
        return True, result.stdout.strip()
    except subprocess.CalledProcessError as e:
        return False, e.stderr.strip()


def get_all_releases(repo: str) -> List[Dict]:
    """获取所有 releases"""
    print(f"Fetching all releases from {repo}...")
    success, output = run_command([
        'gh', 'release', 'list',
        '--repo', repo,
        '--limit', '100',
        '--json', 'tagName,name,id'
    ])
    
    if not success:
        print(f"Error: {output}", file=sys.stderr)
        sys.exit(1)
    
    try:
        releases = json.loads(output)
        return releases
    except json.JSONDecodeError:
        print("Error: Failed to parse release list", file=sys.stderr)
        sys.exit(1)


def delete_release(repo: str, tag: str, dry_run: bool = False) -> bool:
    """删除指定的 release"""
    if dry_run:
        print(f"  [DRY RUN] Would delete {tag}")
        return True
    
    print(f"Deleting {tag}...", end=' ', flush=True)
    success, _ = run_command([
        'gh', 'release', 'delete', tag,
        '--repo', repo,
        '--yes'
    ], check=False)
    
    if success:
        print("✓")
        # 删除对应的 tag
        print(f"  Deleting tag {tag}...", end=' ', flush=True)
        success, _ = run_command([
            'git', 'push', '--delete', 'origin', tag
        ], check=False)
        if success:
            print("✓")
        else:
            print("(tag already deleted or doesn't exist)")
        return True
    else:
        print("✗")
        return False


def main():
    parser = argparse.ArgumentParser(
        description='Delete all GitHub releases except the specified version'
    )
    parser.add_argument(
        '--keep',
        default='v0.2.2',
        help='Version to keep (default: v0.2.2)'
    )
    parser.add_argument(
        '--repo',
        default='jihuayu/xor',
        help='Repository (default: jihuayu/xor)'
    )
    parser.add_argument(
        '--dry-run',
        action='store_true',
        help='Preview what would be deleted without actually deleting'
    )
    
    args = parser.parse_args()
    
    # 获取所有 releases
    releases = get_all_releases(args.repo)
    
    if not releases:
        print("No releases found.")
        return
    
    print(f"Found {len(releases)} releases\n")
    
    # 过滤出要删除的 releases
    to_delete = [r for r in releases if r['tagName'] != args.keep]
    
    if not to_delete:
        print(f"No releases to delete. Only {args.keep} exists.")
        return
    
    print("Releases to delete:")
    for release in to_delete:
        print(f"  - {release['tagName']} ({release['name']})")
    
    print(f"\nKeeping: {args.keep}\n")
    
    if args.dry_run:
        print("[DRY RUN] No releases will be deleted.")
        return
    
    # 确认删除
    confirmation = input(f"Are you sure you want to delete {len(to_delete)} releases? (yes/no): ")
    if confirmation.lower() != 'yes':
        print("Operation cancelled.")
        return
    
    print("\nDeleting releases...")
    
    success_count = 0
    fail_count = 0
    
    for release in to_delete:
        if delete_release(args.repo, release['tagName'], args.dry_run):
            success_count += 1
        else:
            fail_count += 1
    
    print(f"\nSummary:")
    print(f"  Deleted: {success_count}")
    print(f"  Failed: {fail_count}")
    print(f"  Kept: {args.keep}")


if __name__ == '__main__':
    main()
