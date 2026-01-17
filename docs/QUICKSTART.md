# 快速开始指南

5 分钟上手 rdiff！

---

## 📦 安装

### 方法 1: 一键安装（最快）

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/YOUR_USERNAME/rust-diff-tool/main/install.sh | bash
```

**Windows (PowerShell 管理员模式):**
```powershell
iwr https://raw.githubusercontent.com/YOUR_USERNAME/rust-diff-tool/main/install.ps1 | iex
```

### 方法 2: 下载二进制

访问 [Releases 页面](https://github.com/YOUR_USERNAME/rust-diff-tool/releases/latest)，下载适合你系统的版本。

### 方法 3: Cargo (需要 Rust 环境)

```bash
cargo install rust-diff-tool
```

---

## 🎯 基本使用

### 1. 对比两个文件

```bash
rdiff file1.txt file2.txt
```

**输出示例:**
```diff
diff --git a/file1.txt b/file2.txt
--- file1.txt
+++ file2.txt
@@ -2,3 +2,4 @@
-This is line 2
-Original content here
+This is line 2 MODIFIED
+New content added
 Line 4 remains same
+Another new line

1 file(s) changed, 3 insertion(s)(+), 2 deletion(s)(-)
```

### 2. Web 可视化模式

```bash
rdiff file1.txt file2.txt --web
```

浏览器会自动打开，显示美观的 diff 界面！

**特性:**
- ✅ GitHub 风格配色
- ✅ 并排视图 / 统一视图切换
- ✅ 流畅滚动（支持大文件）

### 3. 对比两个目录

```bash
rdiff dir1/ dir2/
```

递归对比所有文件，显示所有差异！

### 4. 简洁模式（只看文件名）

```bash
rdiff dir1/ dir2/ --brief
```

**输出:**
```
Files dir1/file1.txt and dir2/file1.txt differ
Files dir1/file2.txt and dir2/file2.txt differ
```

---

## 🚀 高级功能

### 大文件处理

rdiff 自动优化大文件性能，无需配置！

```bash
# 100MB+ 文件也能秒速对比
rdiff large1.txt large2.txt

# 查看优化策略（可选）
RUST_LOG=rdiff=info rdiff large1.txt large2.txt
```

**自动优化:**
- < 10MB: 快速处理
- 10-100MB: 分块 + 内存映射
- > 100MB: 并行 + 分块 + 内存映射

### Web 虚拟滚动

处理超过 10,000 行的文件时，Web 模式自动启用虚拟滚动：

```bash
rdiff huge_file1.txt huge_file2.txt --web
```

浏览器会显示: **⚡ Virtual Scrolling Enabled**

支持百万行流畅滚动！

### 自定义选项

```bash
# 增加上下文行数（默认 3）
rdiff file1.txt file2.txt --unified 5

# 忽略空白符
rdiff file1.txt file2.txt --ignore-whitespace

# 忽略大小写
rdiff file1.txt file2.txt --ignore-case

# 自定义 Web 端口
rdiff file1.txt file2.txt --web --port 9000

# 禁用颜色
rdiff file1.txt file2.txt --color never
```

---

## 🐳 Docker 使用

### 基本用法

```bash
# 拉取镜像
docker pull YOUR_USERNAME/rdiff:latest

# 对比文件（挂载当前目录）
docker run --rm -v $(pwd):/data YOUR_USERNAME/rdiff /data/file1.txt /data/file2.txt
```

### Web 模式

```bash
# 启动 Web 服务器
docker run --rm -p 8080:8080 -v $(pwd):/data \
  YOUR_USERNAME/rdiff /data/file1.txt /data/file2.txt --web --port 8080

# 浏览器打开 http://localhost:8080
```

---

## 📚 常见用例

### 1. 检查配置文件修改

```bash
rdiff config.old.yml config.new.yml --web
```

### 2. 代码审查

```bash
# 对比两个分支的文件
rdiff /path/to/branch-main/src /path/to/branch-feature/src
```

### 3. 日志文件对比

```bash
# 大日志文件对比（自动优化）
rdiff app.log.yesterday app.log.today
```

### 4. 数据文件验证

```bash
# 对比 CSV/JSON 等数据文件
rdiff data_v1.csv data_v2.csv --ignore-whitespace
```

---

## 🎓 进阶技巧

### 与其他工具配合

```bash
# 与 git 配合
git show HEAD:file.txt > /tmp/old.txt
rdiff /tmp/old.txt file.txt

# 与 curl 配合（对比远程文件）
curl https://example.com/file.txt > remote.txt
rdiff local.txt remote.txt

# 管道输入
diff -u file1.txt file2.txt | rdiff --web
```

### 性能提示

```bash
# 对于超大文件，先看预览
head -n 10000 large1.txt > preview1.txt
head -n 10000 large2.txt > preview2.txt
rdiff preview1.txt preview2.txt

# 或直接对比（rdiff 会自动优化）
rdiff large1.txt large2.txt
```

---

## ❓ 常见问题

### Q: 支持哪些文件格式？
A: 支持所有文本文件。二进制文件会显示 "Binary file - cannot display diff"。

### Q: 最大能处理多大的文件？
A: 理论上无限制。已测试 GB 级文件，内存占用 < 500MB。

### Q: Web 模式如何退出？
A: 按 `Ctrl+C` 停止服务器。

### Q: 如何更新？
A: 重新运行安装脚本，或使用包管理器：
```bash
# Homebrew
brew upgrade rdiff

# Cargo
cargo install rust-diff-tool --force
```

### Q: 卸载如何操作？
A:
```bash
# 手动安装的
sudo rm /usr/local/bin/rdiff

# Homebrew
brew uninstall rdiff

# Cargo
cargo uninstall rust-diff-tool
```

---

## 📖 更多资源

- **完整文档**: [README.md](README.md)
- **发布指南**: [DISTRIBUTION_GUIDE.md](DISTRIBUTION_GUIDE.md)
- **性能优化**: [OPTIMIZATION_COMPLETE.md](OPTIMIZATION_COMPLETE.md)
- **问题反馈**: [GitHub Issues](https://github.com/YOUR_USERNAME/rust-diff-tool/issues)
- **更新日志**: [CHANGELOG.md](CHANGELOG.md)

---

## 💡 提示

1. 默认情况下，rdiff 会自动优化大文件性能
2. Web 模式支持键盘快捷键（如方向键滚动）
3. 可以通过 `RUST_LOG` 环境变量查看详细日志
4. 支持 `.gitignore` 规则（对比目录时）

---

**开始使用 rdiff，让代码对比更高效！** 🚀
