# README 安装部分示例

将以下内容添加到你的 `README.md` 中：

---

## 📦 安装

### 🚀 快速安装（推荐）

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/YOUR_USERNAME/rust-diff-tool/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
iwr https://raw.githubusercontent.com/YOUR_USERNAME/rust-diff-tool/main/install.ps1 | iex
```

### 📥 其他安装方式

<details>
<summary><b>通过包管理器</b></summary>

#### Homebrew (macOS/Linux)
```bash
brew tap YOUR_USERNAME/tap
brew install rdiff
```

#### Scoop (Windows)
```powershell
scoop bucket add YOUR_USERNAME https://github.com/YOUR_USERNAME/scoop-bucket
scoop install rdiff
```

#### Cargo
```bash
cargo install rust-diff-tool
```

</details>

<details>
<summary><b>预编译二进制</b></summary>

从 [Releases](https://github.com/YOUR_USERNAME/rust-diff-tool/releases/latest) 页面下载：

- **macOS (Intel)**: `rdiff-macos-x86_64.tar.gz`
- **macOS (Apple Silicon)**: `rdiff-macos-aarch64.tar.gz`
- **Linux (x86_64)**: `rdiff-linux-x86_64.tar.gz`
- **Windows (x86_64)**: `rdiff-windows-x86_64.exe.zip`

下载后解压并移动到 PATH 目录：

```bash
# macOS/Linux
tar -xzf rdiff-*.tar.gz
sudo mv rdiff /usr/local/bin/

# Windows (PowerShell)
Expand-Archive rdiff-*.zip
Move-Item rdiff.exe C:\Windows\System32\
```

</details>

<details>
<summary><b>从源码构建</b></summary>

需要 Rust 1.70+ 环境：

```bash
git clone https://github.com/YOUR_USERNAME/rust-diff-tool.git
cd rust-diff-tool
cargo build --release
sudo mv target/release/rdiff /usr/local/bin/
```

</details>

<details>
<summary><b>Docker</b></summary>

```bash
docker pull YOUR_USERNAME/rdiff:latest

# 使用示例
docker run --rm -v $(pwd):/data YOUR_USERNAME/rdiff /data/file1.txt /data/file2.txt

# Web 模式
docker run --rm -p 8080:8080 -v $(pwd):/data \
  YOUR_USERNAME/rdiff /data/file1.txt /data/file2.txt --web --port 8080
```

</details>

---

## 🎯 快速开始

### 基本用法

```bash
# 对比两个文件
rdiff file1.txt file2.txt

# Web 可视化模式
rdiff file1.txt file2.txt --web

# 对比目录
rdiff dir1/ dir2/

# 查看帮助
rdiff --help
```

### 示例输出

```diff
diff --git a/file1.txt b/file2.txt
--- file1.txt
+++ file2.txt
@@ -2,3 +2,4 @@
-Old line
+New line
 Unchanged line
+Added line

1 file(s) changed, 2 insertion(s)(+), 1 deletion(s)(-)
```

📖 **更多使用方法**: 查看 [QUICKSTART.md](QUICKSTART.md)

---

## ⚡ 核心特性

- ✅ **高性能**: 支持 GB 级大文件，内存优化 < 500MB
- 🌐 **Web 可视化**: GitHub 风格的美观界面
- 🎨 **双视图**: 统一视图 / 并排视图
- 📊 **虚拟滚动**: 百万行流畅展示
- 🚀 **自动优化**: 根据文件大小智能选择算法
- 🎯 **进度显示**: 大文件处理实时反馈
- 🔧 **功能丰富**: 忽略空白、忽略大小写等
- 📦 **跨平台**: Windows / macOS / Linux

---

## 📊 性能表现

| 文件大小 | 处理时间 | 内存占用 |
|---------|---------|---------|
| 1 MB    | < 0.5s  | ~10 MB  |
| 10 MB   | < 2s    | ~50 MB  |
| 100 MB  | < 10s   | ~100 MB |
| 1 GB    | < 30s   | ~200 MB |

**Web 渲染**: 支持 1,000,000+ 行流畅滚动

---

## 🛠️ 命令行选项

```
OPTIONS:
  -w, --web                     在浏览器中打开 Web 界面
  -U, --unified <N>             上下文行数 [默认: 3]
      --ignore-whitespace       忽略空白符变化
  -i, --ignore-case             忽略大小写
  -q, --brief                   仅显示不同的文件名
      --color <WHEN>            何时使用颜色 [always|never|auto]
      --port <PORT>             Web 服务器端口
  -h, --help                    显示帮助信息
  -V, --version                 显示版本信息
```

---

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

---

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

---

## 🔗 相关链接

- 📚 [完整文档](https://github.com/YOUR_USERNAME/rust-diff-tool#readme)
- 🚀 [快速开始](QUICKSTART.md)
- 📦 [发布指南](DISTRIBUTION_GUIDE.md)
- 🐛 [问题反馈](https://github.com/YOUR_USERNAME/rust-diff-tool/issues)
- 📈 [更新日志](CHANGELOG.md)

---
