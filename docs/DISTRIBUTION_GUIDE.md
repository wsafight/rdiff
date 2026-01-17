# 跨平台发布指南

本文档介绍如何将 `rdiff` 工具发布到各个平台，让用户能够方便地安装使用。

---

## 📦 发布方案概览

| 方案 | 平台 | 难度 | 覆盖面 | 推荐度 |
|------|------|------|--------|--------|
| **GitHub Releases** | 全平台 | ⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Cargo Install** | 全平台 | ⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Homebrew** | macOS/Linux | ⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Scoop** | Windows | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **AUR** | Arch Linux | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **Docker** | 全平台 | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **npm (optional)** | 全平台 | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |

---

## 🚀 方案 1: GitHub Releases (最推荐)

### 优点
- ✅ 支持所有平台
- ✅ 用户可直接下载二进制文件
- ✅ 无需安装 Rust 环境
- ✅ 自动化构建和发布

### 实施步骤

#### 1.1 创建 GitHub Actions 工作流

创建文件：`.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'  # 当推送 v1.0.0 这样的 tag 时触发

jobs:
  build:
    name: Build ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          # macOS (Intel)
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact_name: rdiff
            asset_name: rdiff-macos-x86_64

          # macOS (Apple Silicon)
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact_name: rdiff
            asset_name: rdiff-macos-aarch64

          # Linux (x86_64)
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: rdiff
            asset_name: rdiff-linux-x86_64

          # Linux (ARM64)
          - os: ubuntu-latest
            target: aarch64-unknown-linux-gnu
            artifact_name: rdiff
            asset_name: rdiff-linux-aarch64

          # Windows (x86_64)
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: rdiff.exe
            asset_name: rdiff-windows-x86_64.exe

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install cross-compilation tools (Linux ARM64)
        if: matrix.target == 'aarch64-unknown-linux-gnu'
        run: |
          sudo apt-get update
          sudo apt-get install -y gcc-aarch64-linux-gnu

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Compress binary (Unix)
        if: matrix.os != 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          tar czf ${{ matrix.asset_name }}.tar.gz ${{ matrix.artifact_name }}
          mv ${{ matrix.asset_name }}.tar.gz ../../../

      - name: Compress binary (Windows)
        if: matrix.os == 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          7z a ${{ matrix.asset_name }}.zip ${{ matrix.artifact_name }}
          move ${{ matrix.asset_name }}.zip ../../../

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.asset_name }}
          path: |
            *.tar.gz
            *.zip

  release:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Download all artifacts
        uses: actions/download-artifact@v3
        with:
          path: artifacts

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: artifacts/**/*
          draft: false
          prerelease: false
          generate_release_notes: true
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

#### 1.2 发布流程

```bash
# 1. 更新版本号
vi Cargo.toml  # 修改 version = "1.0.0"

# 2. 提交更改
git add .
git commit -m "Release v1.0.0"

# 3. 创建并推送 tag
git tag v1.0.0
git push origin v1.0.0

# 4. GitHub Actions 自动构建并发布
# 访问 https://github.com/YOUR_USERNAME/rust-diff-tool/releases
```

#### 1.3 用户安装方式

**macOS (Intel):**
```bash
curl -L https://github.com/YOUR_USERNAME/rust-diff-tool/releases/latest/download/rdiff-macos-x86_64.tar.gz | tar xz
sudo mv rdiff /usr/local/bin/
```

**macOS (Apple Silicon):**
```bash
curl -L https://github.com/YOUR_USERNAME/rust-diff-tool/releases/latest/download/rdiff-macos-aarch64.tar.gz | tar xz
sudo mv rdiff /usr/local/bin/
```

**Linux:**
```bash
curl -L https://github.com/YOUR_USERNAME/rust-diff-tool/releases/latest/download/rdiff-linux-x86_64.tar.gz | tar xz
sudo mv rdiff /usr/local/bin/
```

**Windows (PowerShell):**
```powershell
Invoke-WebRequest -Uri "https://github.com/YOUR_USERNAME/rust-diff-tool/releases/latest/download/rdiff-windows-x86_64.exe.zip" -OutFile rdiff.zip
Expand-Archive rdiff.zip -DestinationPath .
Move-Item rdiff.exe C:\Windows\System32\
```

---

## 📦 方案 2: Cargo Install (Rust 用户)

### 优点
- ✅ Rust 生态标准方式
- ✅ 自动编译优化版本
- ✅ 易于更新

### 实施步骤

#### 2.1 发布到 crates.io

```bash
# 1. 登录 crates.io（需要先在网站创建账号）
cargo login YOUR_API_TOKEN

# 2. 确保 Cargo.toml 配置完整
# 需要包含：
# - name, version, authors, description, license
# - repository, homepage, documentation (可选但推荐)

# 3. 发布
cargo publish
```

#### 2.2 用户安装方式

```bash
# 安装最新版本
cargo install rust-diff-tool

# 指定版本安装
cargo install rust-diff-tool --version 1.0.0

# 从 Git 仓库安装（开发版）
cargo install --git https://github.com/YOUR_USERNAME/rust-diff-tool
```

#### 2.3 Cargo.toml 完整配置示例

```toml
[package]
name = "rust-diff-tool"
version = "1.0.0"
edition = "2024"
authors = ["Your Name <your.email@example.com>"]
description = "A powerful CLI diff tool with web visualization and large file optimization"
license = "MIT"
repository = "https://github.com/YOUR_USERNAME/rust-diff-tool"
homepage = "https://github.com/YOUR_USERNAME/rust-diff-tool"
documentation = "https://github.com/YOUR_USERNAME/rust-diff-tool#readme"
readme = "README.md"
keywords = ["diff", "cli", "git", "comparison", "tool"]
categories = ["command-line-utilities", "development-tools"]

[[bin]]
name = "rdiff"
path = "src/main.rs"

# ... rest of dependencies
```

---

## 🍺 方案 3: Homebrew (macOS/Linux)

### 优点
- ✅ macOS 用户最熟悉的安装方式
- ✅ 自动管理依赖和更新
- ✅ 支持 Linux

### 实施步骤

#### 3.1 创建 Homebrew Formula

创建文件：`homebrew/rdiff.rb`

```ruby
class Rdiff < Formula
  desc "Powerful CLI diff tool with web visualization"
  homepage "https://github.com/YOUR_USERNAME/rust-diff-tool"
  version "1.0.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/YOUR_USERNAME/rust-diff-tool/releases/download/v1.0.0/rdiff-macos-aarch64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/YOUR_USERNAME/rust-diff-tool/releases/download/v1.0.0/rdiff-macos-x86_64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/YOUR_USERNAME/rust-diff-tool/releases/download/v1.0.0/rdiff-linux-aarch64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/YOUR_USERNAME/rust-diff-tool/releases/download/v1.0.0/rdiff-linux-x86_64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  def install
    bin.install "rdiff"
  end

  test do
    system "#{bin}/rdiff", "--version"
  end
end
```

#### 3.2 发布到 Homebrew Tap

```bash
# 创建 homebrew-tap 仓库
gh repo create homebrew-tap --public

# 添加 formula
cd homebrew-tap
mkdir Formula
cp ../rust-diff-tool/homebrew/rdiff.rb Formula/
git add Formula/rdiff.rb
git commit -m "Add rdiff formula"
git push
```

#### 3.3 用户安装方式

```bash
# 添加 tap
brew tap YOUR_USERNAME/tap

# 安装
brew install rdiff

# 更新
brew upgrade rdiff
```

---

## 🪟 方案 4: Scoop (Windows)

### 优点
- ✅ Windows 上类似 Homebrew 的体验
- ✅ 无需管理员权限
- ✅ 易于更新

### 实施步骤

#### 4.1 创建 Scoop Manifest

创建文件：`scoop/rdiff.json`

```json
{
    "version": "1.0.0",
    "description": "Powerful CLI diff tool with web visualization",
    "homepage": "https://github.com/YOUR_USERNAME/rust-diff-tool",
    "license": "MIT",
    "architecture": {
        "64bit": {
            "url": "https://github.com/YOUR_USERNAME/rust-diff-tool/releases/download/v1.0.0/rdiff-windows-x86_64.exe.zip",
            "hash": "REPLACE_WITH_ACTUAL_SHA256"
        }
    },
    "bin": "rdiff.exe",
    "checkver": {
        "github": "https://github.com/YOUR_USERNAME/rust-diff-tool"
    },
    "autoupdate": {
        "architecture": {
            "64bit": {
                "url": "https://github.com/YOUR_USERNAME/rust-diff-tool/releases/download/v$version/rdiff-windows-x86_64.exe.zip"
            }
        }
    }
}
```

#### 4.2 发布到 Scoop Bucket

```bash
# 创建 scoop-bucket 仓库
gh repo create scoop-bucket --public

cd scoop-bucket
mkdir bucket
cp ../rust-diff-tool/scoop/rdiff.json bucket/
git add bucket/rdiff.json
git commit -m "Add rdiff"
git push
```

#### 4.3 用户安装方式

```powershell
# 添加 bucket
scoop bucket add YOUR_USERNAME https://github.com/YOUR_USERNAME/scoop-bucket

# 安装
scoop install rdiff

# 更新
scoop update rdiff
```

---

## 🐳 方案 5: Docker

### 优点
- ✅ 跨平台一致性
- ✅ 隔离环境
- ✅ 适合 CI/CD

### 实施步骤

#### 5.1 创建 Dockerfile

创建文件：`Dockerfile`

```dockerfile
FROM rust:1.92-slim as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rdiff /usr/local/bin/rdiff

ENTRYPOINT ["rdiff"]
CMD ["--help"]
```

#### 5.2 构建和发布

```bash
# 构建镜像
docker build -t YOUR_USERNAME/rdiff:1.0.0 .
docker build -t YOUR_USERNAME/rdiff:latest .

# 推送到 Docker Hub
docker login
docker push YOUR_USERNAME/rdiff:1.0.0
docker push YOUR_USERNAME/rdiff:latest
```

#### 5.3 用户使用方式

```bash
# 拉取镜像
docker pull YOUR_USERNAME/rdiff:latest

# 使用（挂载当前目录）
docker run --rm -v $(pwd):/data YOUR_USERNAME/rdiff /data/file1.txt /data/file2.txt

# Web 模式
docker run --rm -p 8080:8080 -v $(pwd):/data YOUR_USERNAME/rdiff /data/file1.txt /data/file2.txt --web --port 8080
```

---

## 📜 方案 6: 一键安装脚本

### 创建通用安装脚本

创建文件：`install.sh`

```bash
#!/bin/bash
set -e

# 检测操作系统和架构
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)

    case "$os" in
        darwin)
            os="macos"
            ;;
        linux)
            os="linux"
            ;;
        *)
            echo "Unsupported OS: $os"
            exit 1
            ;;
    esac

    case "$arch" in
        x86_64|amd64)
            arch="x86_64"
            ;;
        arm64|aarch64)
            arch="aarch64"
            ;;
        *)
            echo "Unsupported architecture: $arch"
            exit 1
            ;;
    esac

    echo "${os}-${arch}"
}

# 下载并安装
install_rdiff() {
    local platform=$(detect_platform)
    local version="latest"
    local base_url="https://github.com/YOUR_USERNAME/rust-diff-tool/releases/${version}/download"
    local asset_name="rdiff-${platform}.tar.gz"
    local download_url="${base_url}/${asset_name}"

    echo "🚀 Installing rdiff for ${platform}..."

    # 创建临时目录
    local tmp_dir=$(mktemp -d)
    cd "$tmp_dir"

    # 下载
    echo "📥 Downloading from ${download_url}..."
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$download_url" -o rdiff.tar.gz
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$download_url" -O rdiff.tar.gz
    else
        echo "❌ Error: curl or wget is required"
        exit 1
    fi

    # 解压
    echo "📦 Extracting..."
    tar -xzf rdiff.tar.gz

    # 安装
    echo "✅ Installing to /usr/local/bin/rdiff..."
    sudo mv rdiff /usr/local/bin/rdiff
    sudo chmod +x /usr/local/bin/rdiff

    # 清理
    cd -
    rm -rf "$tmp_dir"

    echo ""
    echo "🎉 rdiff installed successfully!"
    echo ""
    echo "Try it now:"
    echo "  rdiff --version"
    echo "  rdiff --help"
}

install_rdiff
```

### Windows 安装脚本

创建文件：`install.ps1`

```powershell
$ErrorActionPreference = "Stop"

function Install-Rdiff {
    $version = "latest"
    $baseUrl = "https://github.com/YOUR_USERNAME/rust-diff-tool/releases/$version/download"
    $assetName = "rdiff-windows-x86_64.exe.zip"
    $downloadUrl = "$baseUrl/$assetName"

    Write-Host "🚀 Installing rdiff for Windows..." -ForegroundColor Green

    # 创建临时目录
    $tmpDir = New-Item -ItemType Directory -Path "$env:TEMP\rdiff-install" -Force

    # 下载
    Write-Host "📥 Downloading from $downloadUrl..." -ForegroundColor Cyan
    $zipPath = Join-Path $tmpDir "rdiff.zip"
    Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath

    # 解压
    Write-Host "📦 Extracting..." -ForegroundColor Cyan
    Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

    # 安装到 Program Files
    $installDir = "$env:ProgramFiles\rdiff"
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    Move-Item -Path (Join-Path $tmpDir "rdiff.exe") -Destination $installDir -Force

    # 添加到 PATH
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$installDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
        Write-Host "✅ Added to PATH (restart terminal to use)" -ForegroundColor Green
    }

    # 清理
    Remove-Item -Path $tmpDir -Recurse -Force

    Write-Host ""
    Write-Host "🎉 rdiff installed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Try it now (after restarting terminal):" -ForegroundColor Yellow
    Write-Host "  rdiff --version"
    Write-Host "  rdiff --help"
}

Install-Rdiff
```

### 用户使用方式

**Unix (macOS/Linux):**
```bash
curl -fsSL https://raw.githubusercontent.com/YOUR_USERNAME/rust-diff-tool/main/install.sh | bash
```

**Windows:**
```powershell
iwr https://raw.githubusercontent.com/YOUR_USERNAME/rust-diff-tool/main/install.ps1 | iex
```

---

## 📊 推荐的发布组合

### 最小配置（快速启动）
1. **GitHub Releases** - 提供预编译二进制
2. **Cargo Install** - 发布到 crates.io
3. **一键安装脚本** - 方便快速安装

### 完整配置（最大覆盖）
1. **GitHub Releases** - 所有平台
2. **Cargo Install** - Rust 用户
3. **Homebrew** - macOS/Linux 用户
4. **Scoop** - Windows 用户
5. **Docker** - 容器环境
6. **一键安装脚本** - 新手友好

---

## 📝 README 安装说明示例

在你的 `README.md` 中添加：

````markdown
## 📦 安装

### 方法 1: 一键安装（推荐）

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/YOUR_USERNAME/rust-diff-tool/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
iwr https://raw.githubusercontent.com/YOUR_USERNAME/rust-diff-tool/main/install.ps1 | iex
```

### 方法 2: 包管理器

**Homebrew (macOS/Linux):**
```bash
brew tap YOUR_USERNAME/tap
brew install rdiff
```

**Scoop (Windows):**
```powershell
scoop bucket add YOUR_USERNAME https://github.com/YOUR_USERNAME/scoop-bucket
scoop install rdiff
```

**Cargo (所有平台):**
```bash
cargo install rust-diff-tool
```

### 方法 3: 预编译二进制

从 [Releases](https://github.com/YOUR_USERNAME/rust-diff-tool/releases) 页面下载适合你系统的版本。

### 方法 4: 从源码构建

```bash
git clone https://github.com/YOUR_USERNAME/rust-diff-tool.git
cd rust-diff-tool
cargo build --release
sudo mv target/release/rdiff /usr/local/bin/
```

## 🐳 Docker

```bash
docker pull YOUR_USERNAME/rdiff:latest
docker run --rm -v $(pwd):/data YOUR_USERNAME/rdiff /data/file1.txt /data/file2.txt
```
````

---

## 🔄 版本更新流程

1. **更新版本号**
   ```bash
   # 更新 Cargo.toml
   vi Cargo.toml  # version = "1.1.0"

   # 更新 Formula/Manifest（如果使用）
   vi homebrew/rdiff.rb
   vi scoop/rdiff.json
   ```

2. **提交并打 tag**
   ```bash
   git add .
   git commit -m "Release v1.1.0"
   git tag v1.1.0
   git push origin main
   git push origin v1.1.0
   ```

3. **GitHub Actions 自动构建**
   - 自动构建所有平台二进制
   - 自动创建 GitHub Release

4. **更新包管理器**（如果适用）
   ```bash
   # Cargo - 自动同步
   cargo publish

   # Homebrew - 更新 formula
   cd homebrew-tap
   # 更新 version 和 sha256
   git commit -am "Update rdiff to 1.1.0"
   git push

   # Scoop - 自动更新（如果配置了 autoupdate）
   ```

---

## 📈 发布检查清单

在发布新版本前：

- [ ] 更新 `Cargo.toml` 版本号
- [ ] 更新 `CHANGELOG.md`
- [ ] 运行所有测试：`cargo test --all`
- [ ] 构建 release：`cargo build --release`
- [ ] 测试二进制文件功能
- [ ] 更新 README 安装说明
- [ ] 创建 Git tag
- [ ] 等待 GitHub Actions 完成
- [ ] 验证 GitHub Release
- [ ] 测试安装脚本
- [ ] 更新 crates.io：`cargo publish`
- [ ] 更新 Homebrew formula（如适用）
- [ ] 发布公告（社交媒体、论坛等）

---

这个指南涵盖了所有主流的发布方式。建议先从 **GitHub Releases + Cargo + 一键安装脚本** 开始，这样可以快速覆盖大部分用户！
