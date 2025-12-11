# 删除除 v0.2.2 之外的所有 GitHub releases
# 需要先安装 gh CLI: https://cli.github.com/

param(
    [string]$KeepVersion = "v0.2.2",
    [switch]$DryRun = $false
)

$repo = "jihuayu/xor"

Write-Host "Fetching all releases from $repo..." -ForegroundColor Cyan

# 获取所有 releases
$releases = gh release list --repo $repo --limit 100 --json tagName,name,id | ConvertFrom-Json

if ($releases.Count -eq 0) {
    Write-Host "No releases found." -ForegroundColor Yellow
    exit 0
}

Write-Host "Found $($releases.Count) releases" -ForegroundColor Green
Write-Host ""

$toDelete = $releases | Where-Object { $_.tagName -ne $KeepVersion }

if ($toDelete.Count -eq 0) {
    Write-Host "No releases to delete. Only $KeepVersion exists." -ForegroundColor Green
    exit 0
}

Write-Host "Releases to delete:" -ForegroundColor Yellow
foreach ($release in $toDelete) {
    Write-Host "  - $($release.tagName) ($($release.name))" -ForegroundColor Red
}

Write-Host ""
Write-Host "Keeping: $KeepVersion" -ForegroundColor Green
Write-Host ""

if ($DryRun) {
    Write-Host "[DRY RUN] No releases will be deleted." -ForegroundColor Cyan
    exit 0
}

# 确认删除
$confirmation = Read-Host "Are you sure you want to delete $($toDelete.Count) releases? (yes/no)"
if ($confirmation -ne "yes") {
    Write-Host "Operation cancelled." -ForegroundColor Yellow
    exit 0
}

Write-Host ""
Write-Host "Deleting releases..." -ForegroundColor Cyan

$successCount = 0
$failCount = 0

foreach ($release in $toDelete) {
    try {
        Write-Host "Deleting $($release.tagName)..." -NoNewline
        gh release delete $release.tagName --repo $repo --yes
        Write-Host " ✓" -ForegroundColor Green
        $successCount++
        
        # 删除对应的 tag
        Write-Host "  Deleting tag $($release.tagName)..." -NoNewline
        git push --delete origin $release.tagName 2>$null
        Write-Host " ✓" -ForegroundColor Green
    }
    catch {
        Write-Host " ✗" -ForegroundColor Red
        Write-Host "  Error: $_" -ForegroundColor Red
        $failCount++
    }
}

Write-Host ""
Write-Host "Summary:" -ForegroundColor Cyan
Write-Host "  Deleted: $successCount" -ForegroundColor Green
Write-Host "  Failed: $failCount" -ForegroundColor Red
Write-Host "  Kept: $KeepVersion" -ForegroundColor Green
