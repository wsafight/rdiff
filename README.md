# rdiff

<div align="center">

强大的命令行 diff 工具，支持 Web 可视化和大文件优化

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

[功能特性](#-功能特性) •
[安装](#-安装) •
[使用方法](#-使用方法) •
[文档](#-文档) •
[贡献](#-贡献)

</div>

---

## ✨ 功能特性

- 🔍 **智能文件对比** - 使用智能 diff 算法对比文件和目录
- 🌐 **Web 可视化** - 漂亮的浏览器界面，支持双视图模式
- ⚡ **大文件优化** - 内存映射、分块处理和并行计算
- 🎨 **语法高亮** - 彩色显示新增、删除和上下文行
- 📊 **完整文件视图** - 在 diff 模式和完整文件内容之间切换
- 🚀 **虚拟滚动** - 即使数百万行也能流畅显示
- 📱 **响应式设计** - 在桌面和移动设备上无缝工作
- 🔧 **二进制检测** - 自动识别和处理二进制文件
- 🎯 **灵活选项** - 忽略空白符、大小写等

## 🚀 快速开始

### 一键安装

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/wsafight/rdiff/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/wsafight/rdiff/main/install.ps1 | iex
```

### 基本使用

```bash
# 对比两个文件
rdiff file1.txt file2.txt

# 在浏览器中查看
rdiff file1.txt file2.txt --web

# 对比目录
rdiff dir1/ dir2/
```

## 📦 安装

### 方式 1：安装脚本（推荐）

**Unix/Linux/macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/wsafight/rdiff/main/install.sh | bash
```

**Windows PowerShell:**
```powershell
irm https://raw.githubusercontent.com/wsafight/rdiff/main/install.ps1 | iex
```

### 方式 2：从源码编译

```bash
# 克隆仓库
git clone https://github.com/wsafight/rdiff.git
cd rdiff

# 构建 release 版本
cargo build --release

# 安装到 PATH
sudo cp target/release/rdiff /usr/local/bin/
```

### 方式 3：通过 Cargo 安装

```bash
cargo install rust-diff-tool
```

### 方式 4：下载二进制文件

从 [GitHub Releases](https://github.com/wsafight/rdiff/releases) 下载预编译的二进制文件

## 💻 使用方法

### 命令行

```bash
# 对比两个文件
rdiff file1.txt file2.txt

# 递归对比目录
rdiff dir1/ dir2/ -r

# 仅显示有差异的文件名
rdiff dir1/ dir2/ --brief

# 忽略空白符变化
rdiff file1.txt file2.txt --ignore-whitespace

# 忽略大小写
rdiff file1.txt file2.txt --ignore-case

# 自定义上下文行数
rdiff file1.txt file2.txt -U 5
```

### Web 模式

```bash
# 启动 Web 查看器（自动打开浏览器）
rdiff file1.txt file2.txt --web

# 指定自定义端口
rdiff file1.txt file2.txt --web --port 8080
```

**Web 界面功能：**
- 🔄 **视图切换** - 在统一视图和并排视图之间切换
- 📄 **完整文件视图** - 显示完整文件或仅显示差异
- 📊 **统计信息** - 文件数量、新增行、删除行
- 🎨 **颜色编码** - 绿色表示新增，红色表示删除
- ⚡ **虚拟滚动** - 流畅处理 10,000+ 行的文件

### 使用示例

**对比配置文件：**
```bash
rdiff config/prod.json config/dev.json --web
```

**目录对比：**
```bash
rdiff src/ backup/src/ -r --brief
```

**大文件对比：**
```bash
# 自动优化 > 10MB 的文件
rdiff large_log1.txt large_log2.txt --web
```

## 📖 文档

- [快速开始指南](docs/QUICKSTART.md) - 5 分钟上手
- [Web 功能](docs/FEATURES_SUMMARY.md) - Web 界面功能介绍
- [大文件优化](docs/LARGE_FILE_OPTIMIZATION.md) - 性能优化详情
- [分发指南](docs/DISTRIBUTION_GUIDE.md) - 打包和分发
- [发布清单](docs/RELEASE_CHECKLIST.md) - 维护者使用
- [更新日志](docs/CHANGELOG.md) - 版本历史
- [待办事项](docs/TODO.md) - 开发路线图

## 🛠️ 技术栈

- **语言：** Rust 2024 Edition
- **CLI 框架：** clap v4.5
- **Diff 引擎：** similar v2.7
- **Web 服务器：** axum v0.8 + tokio v1.49
- **大文件处理：** memmap2, rayon（并行处理）
- **终端：** colored v3.1
- **序列化：** serde + serde_json

## 🎯 性能

rdiff 对小文件和大文件都进行了优化：

| 文件大小 | 策略 | 性能 |
|---------|------|------|
| < 10 MB | 标准 diff | 瞬时 |
| 10-100 MB | 内存映射 | 快速 |
| 100MB-1GB | 分块处理 | 中等 |
| > 1 GB | 并行 + 分块 | 优化 |

**性能基准：**
- 100 MB 文件 diff：~2-3 秒
- 1 GB 文件 diff：~15-20 秒
- 虚拟滚动：10,000+ 行无延迟

## 📁 项目结构

```
rdiff/
├── src/
│   ├── cli/           # 命令行接口
│   ├── diff/          # Diff 算法和优化
│   ├── web/           # Web 服务器和资源
│   └── utils/         # 工具函数
├── docs/              # 文档
├── scripts/           # 测试和演示脚本
├── examples/          # 示例文件
└── README.md          # 本文件
```

## 🤝 贡献

欢迎贡献！请查看 [待办事项列表](docs/TODO.md) 获取灵感。

1. Fork 本仓库
2. 创建你的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交你的更改 (`git commit -m '添加某个很棒的功能'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启一个 Pull Request

## 🐛 Bug 反馈

发现 Bug？请 [提交 issue](https://github.com/wsafight/rdiff/issues) 并包含：
- 问题描述
- 复现步骤
- 期望行为 vs 实际行为
- 系统信息（操作系统、Rust 版本）

## 📝 许可证

本项目采用 MIT 许可证 - 详见 LICENSE 文件

## 🙏 致谢

构建基于：
- [similar](https://github.com/mitsuhiko/similar) - Diff 算法
- [clap](https://github.com/clap-rs/clap) - CLI 框架
- [axum](https://github.com/tokio-rs/axum) - Web 框架

---

<div align="center">

**[⬆ 回到顶部](#rdiff)**

用 ❤️ 制作 by [wsafight](https://github.com/wsafight)

</div>
