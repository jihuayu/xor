#!/bin/bash

# 删除除 v0.2.2 之外的所有 GitHub releases
# 需要先安装 gh CLI: https://cli.github.com/

KEEP_VERSION="${1:-v0.2.2}"
REPO="jihuayu/xor"
DRY_RUN="${DRY_RUN:-false}"

echo "Fetching all releases from $REPO..."

# 获取所有 releases
releases=$(gh release list --repo "$REPO" --limit 100 --json tagName | jq -r '.[].tagName')

if [ -z "$releases" ]; then
    echo "No releases found."
    exit 0
fi

echo "Found releases:"
echo "$releases"
echo ""

# 过滤出要删除的 releases
to_delete=$(echo "$releases" | grep -v "^${KEEP_VERSION}$")

if [ -z "$to_delete" ]; then
    echo "No releases to delete. Only $KEEP_VERSION exists."
    exit 0
fi

echo "Releases to delete:"
echo "$to_delete"
echo ""
echo "Keeping: $KEEP_VERSION"
echo ""

if [ "$DRY_RUN" = "true" ]; then
    echo "[DRY RUN] No releases will be deleted."
    exit 0
fi

# 确认删除
read -p "Are you sure you want to delete these releases? (yes/no): " confirmation
if [ "$confirmation" != "yes" ]; then
    echo "Operation cancelled."
    exit 0
fi

echo ""
echo "Deleting releases..."

success_count=0
fail_count=0

while IFS= read -r tag; do
    if [ -n "$tag" ]; then
        echo -n "Deleting $tag..."
        if gh release delete "$tag" --repo "$REPO" --yes 2>/dev/null; then
            echo " ✓"
            success_count=$((success_count + 1))
            
            # 删除对应的 tag
            echo -n "  Deleting tag $tag..."
            if git push --delete origin "$tag" 2>/dev/null; then
                echo " ✓"
            else
                echo " (tag already deleted or doesn't exist)"
            fi
        else
            echo " ✗"
            fail_count=$((fail_count + 1))
        fi
    fi
done <<< "$to_delete"

echo ""
echo "Summary:"
echo "  Deleted: $success_count"
echo "  Failed: $fail_count"
echo "  Kept: $KEEP_VERSION"
