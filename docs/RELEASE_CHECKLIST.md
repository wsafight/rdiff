# 发布检查清单

在发布新版本前，请确保完成以下所有步骤。

---

## 📋 发布前准备

### 1. 代码质量检查

- [ ] 运行所有测试
  ```bash
  cargo test --all
  cargo test --release
  ```

- [ ] 运行 Clippy 检查
  ```bash
  cargo clippy -- -D warnings
  ```

- [ ] 格式化代码
  ```bash
  cargo fmt --all
  ```

- [ ] 检查未使用的依赖
  ```bash
  cargo +nightly udeps
  ```

### 2. 功能测试

- [ ] 基本功能测试
  ```bash
  cargo build --release
  ./target/release/rdiff --version
  ./target/release/rdiff --help
  ```

- [ ] 运行所有测试脚本
  ```bash
  ./test_adaptive_diff.sh
  ./test_progress.sh
  ./test_paginated_api.sh
  # （test_virtual_scroll.sh 需要手动测试）
  ```

- [ ] Web 模式测试
  ```bash
  # 创建测试文件
  echo "test1" > /tmp/test1.txt
  echo "test2" > /tmp/test2.txt

  # 测试 CLI 模式
  ./target/release/rdiff /tmp/test1.txt /tmp/test2.txt

  # 测试 Web 模式
  ./target/release/rdiff /tmp/test1.txt /tmp/test2.txt --web
  # 验证浏览器自动打开
  # 验证视图切换功能
  ```

- [ ] 大文件测试
  ```bash
  # 生成大文件
  for i in {1..100000}; do echo "Line $i" >> /tmp/large1.txt; done
  cp /tmp/large1.txt /tmp/large2.txt
  echo "MODIFIED" >> /tmp/large2.txt

  # 测试性能
  time ./target/release/rdiff /tmp/large1.txt /tmp/large2.txt

  # 验证进度条显示
  # 验证虚拟滚动（Web 模式）
  ```

### 3. 文档更新

- [ ] 更新 `Cargo.toml` 版本号
  ```toml
  version = "X.Y.Z"
  ```

- [ ] 更新 `CHANGELOG.md`
  - 添加新版本号和日期
  - 列出所有新功能
  - 列出所有 bug 修复
  - 列出所有破坏性变更（如有）

- [ ] 更新 `README.md`（如有重大变化）
  - 新功能说明
  - 使用示例
  - 性能数据

- [ ] 检查所有文档中的链接
  - GitHub 仓库链接
  - Release 页面链接
  - 安装脚本链接

### 4. 替换占位符

在以下文件中替换 `YOUR_USERNAME`:

- [ ] `DISTRIBUTION_GUIDE.md`
- [ ] `install.sh`
- [ ] `install.ps1`
- [ ] `Cargo.toml`
- [ ] `CHANGELOG.md`
- [ ] `QUICKSTART.md`
- [ ] `README.md`

替换方法：
```bash
# 查找所有需要替换的文件
grep -r "YOUR_USERNAME" .

# 批量替换（macOS）
find . -type f -name "*.md" -o -name "*.toml" -o -name "*.sh" -o -name "*.ps1" | \
  xargs sed -i '' 's/YOUR_USERNAME/your-github-username/g'

# 批量替换（Linux）
find . -type f -name "*.md" -o -name "*.toml" -o -name "*.sh" -o -name "*.ps1" | \
  xargs sed -i 's/YOUR_USERNAME/your-github-username/g'
```

### 5. 构建验证

- [ ] 本地构建所有目标平台
  ```bash
  # macOS
  cargo build --release --target x86_64-apple-darwin
  cargo build --release --target aarch64-apple-darwin

  # Linux (需要交叉编译工具)
  cargo build --release --target x86_64-unknown-linux-gnu

  # Windows (需要交叉编译工具)
  cargo build --release --target x86_64-pc-windows-msvc
  ```

- [ ] Docker 镜像构建
  ```bash
  docker build -t rdiff:test .
  docker run --rm rdiff:test --version
  ```

---

## 🚀 发布流程

### 1. 提交所有更改

```bash
git add .
git commit -m "Release v0.1.0"
git push origin main
```

### 2. 创建 Git Tag

```bash
# 创建标签
git tag -a v0.1.0 -m "Release version 0.1.0"

# 推送标签（触发 GitHub Actions）
git push origin v0.1.0
```

### 3. 等待 GitHub Actions 完成

- [ ] 访问 GitHub Actions 页面
  - 确认所有平台构建成功
  - 下载生成的 artifacts 验证

- [ ] 检查 GitHub Release
  - 确认 Release 已创建
  - 验证所有二进制文件已上传
  - 验证 SHA256 文件已上传
  - 检查 Release Notes 内容

### 4. 发布到 crates.io

```bash
# 登录 crates.io（首次）
cargo login YOUR_API_TOKEN

# 执行发布前检查
cargo publish --dry-run

# 正式发布
cargo publish
```

- [ ] 验证 crates.io 页面
  - 访问 https://crates.io/crates/rust-diff-tool
  - 确认新版本已显示
  - 检查文档链接正确

### 5. 测试安装脚本

**macOS/Linux:**
```bash
# 从 GitHub 安装
curl -fsSL https://raw.githubusercontent.com/YOUR_USERNAME/rust-diff-tool/main/install.sh | bash

# 验证
rdiff --version
```

**Windows:**
```powershell
# 从 GitHub 安装
iwr https://raw.githubusercontent.com/YOUR_USERNAME/rust-diff-tool/main/install.ps1 | iex

# 验证
rdiff --version
```

- [ ] macOS (Intel) 安装测试
- [ ] macOS (Apple Silicon) 安装测试
- [ ] Linux 安装测试
- [ ] Windows 安装测试

### 6. 测试 Cargo Install

```bash
# 在新环境测试
cargo install rust-diff-tool --version 0.1.0

# 验证
rdiff --version
```

---

## 📢 发布后工作

### 1. 社区公告

- [ ] 在 GitHub Release 页面编辑说明
  - 添加亮点功能
  - 添加升级说明
  - 添加致谢

- [ ] 发布公告（可选）
  - Reddit: r/rust
  - Hacker News
  - Twitter/X
  - Rust Users Forum
  - 知乎/掘金（中文社区）

### 2. 文档更新

- [ ] 更新 GitHub README badge（如有）
- [ ] 更新项目 Wiki（如有）
- [ ] 更新示例和截图（如有重大 UI 变化）

### 3. 包管理器更新

如果你创建了 Homebrew Formula 或 Scoop Manifest：

- [ ] 更新 Homebrew Formula
  ```bash
  cd homebrew-tap
  # 更新 version 和 sha256
  vi Formula/rdiff.rb
  git commit -am "Update rdiff to 0.1.0"
  git push
  ```

- [ ] 更新 Scoop Manifest
  ```bash
  cd scoop-bucket
  # 更新 version 和 hash
  vi bucket/rdiff.json
  git commit -am "Update rdiff to 0.1.0"
  git push
  ```

### 4. 监控和反馈

- [ ] 监控 GitHub Issues
  - 及时响应 bug 报告
  - 收集功能请求

- [ ] 监控下载量
  - GitHub Release 下载统计
  - crates.io 下载统计

- [ ] 收集用户反馈
  - 记录常见问题到 FAQ
  - 改进文档

---

## 🐛 发布问题处理

### 如果发现重大 Bug

1. **立即标记 Release 为 pre-release**
   - 在 GitHub Release 页面编辑
   - 勾选 "This is a pre-release"

2. **发布 Bug 修复版本**
   ```bash
   # 快速修复
   git checkout -b hotfix-0.1.1
   # ... 修复代码 ...
   git commit -m "Fix critical bug"

   # 更新版本号为 0.1.1
   vi Cargo.toml

   # 发布补丁版本
   git tag v0.1.1
   git push origin v0.1.1
   ```

3. **撤回 crates.io 版本（如必要）**
   ```bash
   cargo yank --version 0.1.0
   ```

### 如果构建失败

1. **检查 GitHub Actions 日志**
   - 确定失败的平台和原因

2. **本地修复并测试**
   ```bash
   # 针对失败平台测试
   cargo build --release --target <target>
   ```

3. **删除失败的 Tag 并重新发布**
   ```bash
   git tag -d v0.1.0
   git push origin :refs/tags/v0.1.0

   # 修复后重新打 tag
   git tag v0.1.0
   git push origin v0.1.0
   ```

---

## ✅ 最终确认

在完成发布后，确认：

- [x] 所有平台二进制文件可下载
- [x] 安装脚本工作正常
- [x] crates.io 显示新版本
- [x] 文档链接正确
- [x] Release Notes 完整
- [x] 至少一个平台测试通过

---

## 📊 版本号规则 (Semantic Versioning)

- **MAJOR（主版本）**: 不兼容的 API 变更
- **MINOR（次版本）**: 向后兼容的新功能
- **PATCH（补丁）**: 向后兼容的 bug 修复

示例：
- `0.1.0` → `0.2.0`: 添加新功能（向后兼容）
- `0.1.0` → `0.1.1`: 修复 bug
- `0.9.0` → `1.0.0`: 稳定版本发布
- `1.0.0` → `2.0.0`: 破坏性变更

---

祝发布顺利！🎉
