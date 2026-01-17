# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-17

### Added
- 🎯 **核心功能**
  - 文件对比：支持单文件 diff
  - 目录对比：递归对比两个目录
  - Unified diff 格式输出（类似 git diff）
  - 统计信息显示（添加/删除行数）

- 🌐 **Web 可视化**
  - `--web` 模式：在浏览器中展示 diff 结果
  - 两种视图模式：统一视图 (unified) 和并排视图 (side-by-side)
  - 实时视图切换按钮
  - GitHub 风格的配色方案
  - 响应式设计，支持移动端

- ⚡ **大文件性能优化**
  - 内存映射 (mmap)：零拷贝文件访问
  - 分块处理：10,000 行/块，控制内存占用
  - 并行处理：利用多核 CPU 加速
  - 自适应策略：根据文件大小自动选择最优算法
    - < 10MB: 快速直接处理
    - 10-100MB: 内存映射 + 分块
    - > 100MB: 内存映射 + 分块 + 并行
  - 进度条显示：大文件处理实时反馈（> 50K 行）

- 🎨 **Web 高级特性**
  - 虚拟滚动：支持百万行流畅滚动（> 10K 行自动启用）
  - 增量加载 API：分页查询 diff 数据
  - RESTful API：
    - `GET /api/diff` - 完整数据
    - `GET /api/diff/paginated?page=0&page_size=100` - 分页数据

- 🛠️ **CLI 特性**
  - 彩色输出（可通过 `--color` 控制）
  - 简洁模式 (`--brief`)：只显示文件名
  - 统一行数控制 (`--unified N`)
  - 忽略空白符 (`--ignore-whitespace`)
  - 忽略大小写 (`--ignore-case`)
  - 自定义端口 (`--port`)

- 📦 **发布和分发**
  - GitHub Actions 自动构建多平台二进制
  - 支持平台：
    - macOS (Intel/Apple Silicon)
    - Linux (x86_64/ARM64)
    - Windows (x86_64)
  - 一键安装脚本 (install.sh / install.ps1)
  - Docker 支持
  - Cargo install 支持

### Performance
- 文件对比速度提升：2-6x（视文件大小）
- GB 级文件支持：内存控制在 < 500MB
- Web 渲染性能：支持百万行流畅滚动（2000x 提升）

### Documentation
- `README.md` - 项目说明和快速开始
- `DISTRIBUTION_GUIDE.md` - 完整发布指南
- `OPTIMIZATION_COMPLETE.md` - 优化总结
- `LARGE_FILE_OPTIMIZATION.md` - 优化方案设计
- 测试脚本：
  - `test_adaptive_diff.sh` - 自适应策略测试
  - `test_progress.sh` - 进度条测试
  - `test_virtual_scroll.sh` - 虚拟滚动测试
  - `test_paginated_api.sh` - 分页 API 测试
  - `demo_optimizations.sh` - 综合演示

## 未来计划

### [0.2.0] - 计划中
- Git 集成支持
- 智能采样预览（超大文件）
- 更多输出格式（JSON, HTML）
- 语法高亮支持

### [0.3.0] - 计划中
- 文件重命名检测
- 三方合并支持
- 配置文件支持
- 插件系统

---

[Unreleased]: https://github.com/YOUR_USERNAME/rust-diff-tool/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/YOUR_USERNAME/rust-diff-tool/releases/tag/v0.1.0
