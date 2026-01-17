# rdiff Windows 一键安装脚本
# PowerShell 版本

$ErrorActionPreference = "Stop"

# GitHub 仓库信息
$GITHUB_REPO = "wsafight/rdiff"
$BINARY_NAME = "rdiff"

# 获取最新版本号
function Get-LatestVersion {
    try {
        $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$GITHUB_REPO/releases/latest"
        return $release.tag_name
    } catch {
        Write-Host "⚠ 无法获取最新版本，使用 latest" -ForegroundColor Yellow
        return "latest"
    }
}

# 主安装函数
function Install-Rdiff {
    Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║   rdiff 安装脚本 (Windows)             ║" -ForegroundColor Cyan
    Write-Host "║   Powerful CLI Diff Tool              ║" -ForegroundColor Cyan
    Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""

    $version = Get-LatestVersion
    Write-Host "ℹ 目标版本: $version" -ForegroundColor Cyan

    $baseUrl = "https://github.com/$GITHUB_REPO/releases/download/$version"
    $assetName = "$BINARY_NAME-windows-x86_64.exe.zip"
    $downloadUrl = "$baseUrl/$assetName"

    Write-Host ""
    Write-Host "ℹ 开始安装 rdiff..." -ForegroundColor Cyan

    # 创建临时目录
    $tmpDir = New-Item -ItemType Directory -Path "$env:TEMP\rdiff-install-$(Get-Random)" -Force
    Write-Host "ℹ 临时目录: $tmpDir" -ForegroundColor Cyan

    try {
        # 下载
        Write-Host "ℹ 正在下载 $assetName..." -ForegroundColor Cyan
        $zipPath = Join-Path $tmpDir "rdiff.zip"

        try {
            Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing
            Write-Host "✓ 下载完成" -ForegroundColor Green
        } catch {
            Write-Host "✗ 下载失败: $_" -ForegroundColor Red
            Write-Host "ℹ 请检查网络连接或手动下载: $downloadUrl" -ForegroundColor Cyan
            exit 1
        }

        # 解压
        Write-Host "ℹ 正在解压..." -ForegroundColor Cyan
        Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force
        Write-Host "✓ 解压完成" -ForegroundColor Green

        # 确定安装目录
        $installDir = "$env:ProgramFiles\rdiff"
        Write-Host "ℹ 安装目录: $installDir" -ForegroundColor Cyan

        # 创建安装目录
        if (!(Test-Path $installDir)) {
            New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        }

        # 复制文件
        $exePath = Join-Path $tmpDir "$BINARY_NAME.exe"
        $targetPath = Join-Path $installDir "$BINARY_NAME.exe"

        Copy-Item -Path $exePath -Destination $targetPath -Force
        Write-Host "✓ 已安装到 $targetPath" -ForegroundColor Green

        # 添加到 PATH
        $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
        if ($userPath -notlike "*$installDir*") {
            Write-Host "ℹ 添加到 PATH 环境变量..." -ForegroundColor Cyan
            [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
            Write-Host "✓ 已添加到 PATH（请重启终端生效）" -ForegroundColor Green
            $pathUpdated = $true
        } else {
            Write-Host "ℹ 已在 PATH 中" -ForegroundColor Cyan
            $pathUpdated = $false
        }

        Write-Host ""
        Write-Host "✓ 安装成功！" -ForegroundColor Green
        Write-Host ""

        # 验证安装
        if ($pathUpdated) {
            Write-Host "⚠ 请重新打开 PowerShell 后使用以下命令:" -ForegroundColor Yellow
        } else {
            Write-Host "试试这些命令:" -ForegroundColor Cyan
        }
        Write-Host "  rdiff --version"
        Write-Host "  rdiff --help"
        Write-Host "  rdiff file1.txt file2.txt"
        Write-Host "  rdiff file1.txt file2.txt --web"

    } finally {
        # 清理临时文件
        if (Test-Path $tmpDir) {
            Remove-Item -Path $tmpDir -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    Write-Host ""
    Write-Host "ℹ 项目主页: https://github.com/$GITHUB_REPO" -ForegroundColor Cyan
    Write-Host ""
}

# 执行安装
Install-Rdiff
