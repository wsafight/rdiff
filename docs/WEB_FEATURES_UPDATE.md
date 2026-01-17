# Web 界面功能更新

本次更新为 Web 界面添加了两个重要功能。

---

## 🎯 更新内容

### 1. 全文展示按钮 ✨

**新增按钮**: `Show Full File`

**功能说明**:
- **Diff 模式**（默认）: 只显示修改的行及其上下文
- **全文模式**: 显示完整文件内容，并高亮显示修改部分

**使用方法**:
1. 点击 `Show Full File` 按钮
2. 界面会显示完整的文件内容
3. 添加的行用绿色高亮
4. 删除的行用红色高亮
5. 未修改的行正常显示
6. 按钮文字变为 `Show Diff Only`，颜色变为紫色
7. 再次点击恢复到 Diff 模式

**适用场景**:
- 想要查看完整文件上下文
- 需要了解修改在整个文件中的位置
- 对比较小的文件想要看全貌

### 2. 按钮文本英文化 🌍

**改动**:
- ~~切换到并排视图~~ → `Switch to Side-by-Side`
- ~~切换到统一视图~~ → `Switch to Unified`

**原因**:
- 英文更简洁
- 国际化友好
- 与代码环境保持一致

---

## 🎨 视觉效果

### 按钮样式

**主按钮（绿色）**: `Switch to Side-by-Side / Switch to Unified`
- 默认: `#2ea44f` (绿色)
- Hover: `#2c974b`
- Active: `#298e46`

**辅助按钮（蓝色/紫色）**: `Show Full File / Show Diff Only`
- Diff 模式: `#0969da` (蓝色)
- Full File 模式: `#8250df` (紫色)
- Hover: 颜色稍暗
- Active: 颜色更暗

### 布局

```
┌─────────────────────────────────────────────────────┐
│  📊 Diff Viewer                                     │
├─────────────────────────────────────────────────────┤
│  [Switch to Side-by-Side] [Show Full File]   Stats │
└─────────────────────────────────────────────────────┘
```

---

## 💡 使用示例

### 场景 1: 查看完整文件

```bash
# 启动 Web 模式
rdiff file1.txt file2.txt --web

# 在浏览器中：
# 1. 点击 "Show Full File"
# 2. 可以看到完整的文件内容
# 3. 修改的行会高亮显示
```

### 场景 2: 在两种视图间切换

```bash
# 统一视图 + 全文模式
1. 点击 "Show Full File" - 显示完整文件
2. 点击 "Switch to Side-by-Side" - 切换到并排视图
3. 两侧都显示完整文件内容

# 并排视图 + Diff 模式
1. 点击 "Show Diff Only" - 只显示修改
2. 左右两侧并排对比
```

---

## 🔧 技术实现

### 修改文件

1. **`src/web/templates.rs`**
   - 添加 `show-full-file` 按钮
   - 更新按钮文本为英文

2. **`src/web/assets.rs`**
   - 添加 `.btn-secondary` CSS 样式
   - 添加 `showFullFile` 状态变量
   - 更新 `generateUnifiedView()` 函数支持全文显示
   - 更新 `generateSideBySideView()` 函数支持全文显示
   - 添加按钮点击事件处理

### 核心逻辑

```javascript
let showFullFile = false;

fullFileBtn.addEventListener('click', function() {
    showFullFile = !showFullFile;
    renderDiff(currentView);
    fullFileBtn.textContent = showFullFile ? 'Show Diff Only' : 'Show Full File';
    fullFileBtn.style.background = showFullFile ? '#8250df' : '#0969da';
});
```

### 渲染逻辑

```javascript
if (showFullFile) {
    // 显示所有行，包括未修改的
    file.hunks.forEach(hunk => {
        hunk.lines.forEach(line => {
            // 所有行都显示，根据类型高亮
        });
    });
} else {
    // 只显示修改的行和上下文
    file.hunks.forEach(hunk => {
        hunk.lines.forEach(line => {
            // 只显示有变化的行
        });
    });
}
```

---

## 🧪 测试

运行测试脚本验证功能：

```bash
./test_web_features.sh
```

**测试项**:
- ✅ "Show Full File" 按钮显示完整文件
- ✅ "Show Diff Only" 按钮恢复 Diff 模式
- ✅ 按钮颜色在两种模式间切换
- ✅ 统一视图和并排视图都支持全文模式
- ✅ 视图切换按钮文本为英文
- ✅ 所有行号正确显示
- ✅ 颜色高亮正确

---

## 📊 功能对比

### Diff 模式（默认）

**特点**:
- 只显示修改的行
- 包含少量上下文行
- 适合快速查看变更

**显示内容**:
```diff
@@ -3,2 +3,3 @@
 Line 2
-Line 3: Old
+Line 3: New
+Line 4: Added
 Line 5
```

### 全文模式（新增）

**特点**:
- 显示完整文件
- 所有修改行高亮
- 适合了解上下文

**显示内容**:
```
1 Line 1
2 Line 2
3-Line 3: Old
3+Line 3: New
4+Line 4: Added
5 Line 5
6 Line 6
... (所有行)
```

---

## 🎯 未来改进方向

### 可能的增强功能

1. **行内差异高亮**
   - 在修改的行内，精确高亮变化的词
   - 例如：`Line 3: This is ~~old~~ **new** content`

2. **快捷键支持**
   - `F` - 切换全文/Diff 模式
   - `V` - 切换统一/并排视图
   - `↑/↓` - 跳转到上/下一个修改

3. **配置选项**
   - 记住用户的视图偏好
   - 自定义上下文行数
   - 自定义颜色主题

4. **导出功能**
   - 导出当前视图为 HTML
   - 导出为 PDF
   - 复制到剪贴板

---

## 📝 更新日志

**版本**: 0.1.1 (计划)
**日期**: 2026-01-17

### 新增
- ✨ Web 界面新增"Show Full File"按钮
- 🌍 按钮文本改为英文

### 改进
- 🎨 添加按钮颜色状态切换
- 💫 统一视图和并排视图都支持全文模式

### 技术
- 📦 新增 `.btn-secondary` CSS 类
- 🔧 重构渲染函数支持模式切换

---

## 💬 用户反馈

如果你有任何建议或发现问题，欢迎：

- 📧 提交 Issue: https://github.com/YOUR_USERNAME/rust-diff-tool/issues
- 💬 提交 PR: https://github.com/YOUR_USERNAME/rust-diff-tool/pulls
- ⭐ Star 项目支持我们

---

**更新完成！享受新功能！** 🎉
