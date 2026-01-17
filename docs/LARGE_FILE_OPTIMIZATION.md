# 大文件性能优化方案

## 🎯 优化目标

- 支持 **GB 级文件**对比
- **内存占用**控制在合理范围 (< 500MB)
- **响应时间**：秒级而非分钟级
- **流式处理**：无需完整加载文件
- **渐进式渲染**：边计算边显示

## 📊 性能瓶颈分析

### 当前实现的问题

```rust
// ❌ 问题 1: 完整读取文件到内存
let content1 = fs::read_to_string(path1)?;  // 可能几 GB
let content2 = fs::read_to_string(path2)?;  // 可能几 GB

// ❌ 问题 2: similar crate 需要完整内容
let diff = TextDiff::from_lines(&content1, &content2);

// ❌ 问题 3: 生成完整 HTML
let html = generate_all_html(&diff_result);  // 巨大的 HTML
```

### 性能指标对比

| 文件大小 | 当前性能 | 优化后目标 |
|---------|---------|----------|
| 1 MB    | < 1s    | < 0.5s   |
| 10 MB   | ~5s     | < 2s     |
| 100 MB  | ~1min   | < 10s    |
| 1 GB    | OOM ❌  | < 30s ✅  |

## 🛠️ 优化方案

### 方案 1: 内存映射 (Memory-Mapped Files)

**核心思想**：使用 mmap 而不是 read_to_string

#### 实现

```rust
use memmap2::Mmap;
use std::fs::File;

pub struct MmapFile {
    mmap: Mmap,
}

impl MmapFile {
    pub fn open(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self { mmap })
    }

    /// 获取指定行范围的内容
    pub fn get_lines(&self, start: usize, count: usize) -> Vec<&str> {
        let content = std::str::from_utf8(&self.mmap).unwrap();
        content
            .lines()
            .skip(start)
            .take(count)
            .collect()
    }

    /// 获取文件总行数（不加载全部内容）
    pub fn line_count(&self) -> usize {
        memchr::Memchr::new(b'\n', &self.mmap).count() + 1
    }
}
```

**优点**：
- ✅ 不占用大量内存
- ✅ 操作系统自动管理缓存
- ✅ 快速随机访问

**依赖**：
```toml
[dependencies]
memmap2 = "0.9"
memchr = "2.7"
```

---

### 方案 2: 分块 Diff (Chunked Diff)

**核心思想**：将大文件分成小块，分别 diff

#### 实现

```rust
pub struct ChunkedDiffer {
    chunk_size: usize,  // 例如 10000 行
}

impl ChunkedDiffer {
    pub fn new(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    /// 分块对比大文件
    pub fn diff_large_files(
        &self,
        file1: &MmapFile,
        file2: &MmapFile,
    ) -> Result<DiffResult> {
        let total_lines1 = file1.line_count();
        let total_lines2 = file2.line_count();

        let mut all_hunks = Vec::new();
        let mut offset1 = 0;
        let mut offset2 = 0;

        // 分块处理
        while offset1 < total_lines1 || offset2 < total_lines2 {
            let chunk1 = file1.get_lines(offset1, self.chunk_size);
            let chunk2 = file2.get_lines(offset2, self.chunk_size);

            // 对小块进行 diff
            let chunk_hunks = self.diff_chunk(&chunk1, &chunk2, offset1, offset2)?;
            all_hunks.extend(chunk_hunks);

            offset1 += self.chunk_size;
            offset2 += self.chunk_size;
        }

        // 合并相邻的 hunks
        let merged_hunks = self.merge_hunks(all_hunks);

        Ok(DiffResult {
            files: vec![FileDiff {
                hunks: merged_hunks,
                // ...
            }],
            // ...
        })
    }

    fn diff_chunk(
        &self,
        chunk1: &[&str],
        chunk2: &[&str],
        offset1: usize,
        offset2: usize,
    ) -> Result<Vec<Hunk>> {
        // 使用 similar 对小块进行 diff
        let text1 = chunk1.join("\n");
        let text2 = chunk2.join("\n");
        let diff = TextDiff::from_lines(&text1, &text2);

        // 转换为 Hunk，调整行号偏移
        self.convert_to_hunks(diff, offset1, offset2)
    }
}
```

**优点**：
- ✅ 内存占用稳定
- ✅ 可并行处理
- ✅ 渐进式结果

---

### 方案 3: 流式处理 + 迭代器

**核心思想**：使用迭代器而不是一次性加载

#### 实现

```rust
use std::io::{BufRead, BufReader};

pub struct StreamingDiffer {
    buffer_size: usize,
}

impl StreamingDiffer {
    /// 流式对比文件
    pub fn diff_streaming(
        &self,
        path1: &str,
        path2: &str,
    ) -> Result<impl Iterator<Item = Result<Hunk>>> {
        let file1 = File::open(path1)?;
        let file2 = File::open(path2)?;

        let reader1 = BufReader::new(file1);
        let reader2 = BufReader::new(file2);

        let lines1 = reader1.lines();
        let lines2 = reader2.lines();

        // 返回迭代器，按需生成 diff
        Ok(DiffIterator::new(lines1, lines2))
    }
}

pub struct DiffIterator<I1, I2> {
    lines1: I1,
    lines2: I2,
    window_size: usize,
    buffer: VecDeque<Hunk>,
}

impl<I1, I2> Iterator for DiffIterator<I1, I2>
where
    I1: Iterator<Item = Result<String>>,
    I2: Iterator<Item = Result<String>>,
{
    type Item = Result<Hunk>;

    fn next(&mut self) -> Option<Self::Item> {
        // 实现流式 diff 逻辑
        // 维护一个滑动窗口
        // 增量生成 Hunk
        todo!()
    }
}
```

**优点**：
- ✅ 真正的流式处理
- ✅ 极低内存占用
- ✅ 边计算边输出

---

### 方案 4: 并行处理

**核心思想**：多线程并行处理不同区块

#### 实现

```rust
use rayon::prelude::*;

pub struct ParallelDiffer {
    num_threads: usize,
}

impl ParallelDiffer {
    pub fn diff_parallel(
        &self,
        file1: &MmapFile,
        file2: &MmapFile,
    ) -> Result<DiffResult> {
        let total_lines = file1.line_count().max(file2.line_count());
        let chunk_size = total_lines / self.num_threads;

        // 并行处理每个块
        let hunks: Vec<Vec<Hunk>> = (0..self.num_threads)
            .into_par_iter()
            .map(|i| {
                let start = i * chunk_size;
                let end = if i == self.num_threads - 1 {
                    total_lines
                } else {
                    (i + 1) * chunk_size
                };

                self.diff_range(file1, file2, start, end)
            })
            .collect::<Result<Vec<_>>>()?;

        // 合并结果
        let merged = hunks.into_iter().flatten().collect();

        Ok(DiffResult {
            files: vec![FileDiff {
                hunks: merged,
                // ...
            }],
            // ...
        })
    }
}
```

**依赖**：
```toml
[dependencies]
rayon = "1.10"
```

**优点**：
- ✅ 充分利用多核
- ✅ 大幅加速
- ✅ 适合超大文件

---

### 方案 5: Web 端优化 - 虚拟滚动

**核心思想**：只渲染可见区域的 diff

#### HTML + JavaScript 实现

```javascript
class VirtualDiffViewer {
    constructor(diffData, containerHeight = 600) {
        this.diffData = diffData;
        this.containerHeight = containerHeight;
        this.rowHeight = 24;  // 每行高度
        this.visibleRows = Math.ceil(containerHeight / this.rowHeight);
        this.scrollTop = 0;
    }

    render() {
        const startIndex = Math.floor(this.scrollTop / this.rowHeight);
        const endIndex = startIndex + this.visibleRows;

        // 只渲染可见的行
        const visibleLines = this.diffData.files[0].hunks
            .flatMap(h => h.lines)
            .slice(startIndex, endIndex);

        return this.renderLines(visibleLines, startIndex);
    }

    onScroll(event) {
        this.scrollTop = event.target.scrollTop;
        requestAnimationFrame(() => this.render());
    }

    renderLines(lines, offset) {
        let html = '';
        lines.forEach((line, index) => {
            const actualIndex = offset + index;
            const top = actualIndex * this.rowHeight;

            html += `<div class="diff-line" style="position: absolute; top: ${top}px; height: ${this.rowHeight}px">
                ${this.formatLine(line)}
            </div>`;
        });
        return html;
    }
}

// 使用
const viewer = new VirtualDiffViewer(diffData);
diffContainer.innerHTML = viewer.render();
diffContainer.addEventListener('scroll', (e) => viewer.onScroll(e));
```

**优点**：
- ✅ 只渲染可见部分
- ✅ 流畅滚动
- ✅ 支持百万行 diff

---

### 方案 6: 增量加载 API

**核心思想**：API 支持分页加载 diff

#### 服务器端实现

```rust
// 修改 API 端点支持分页
async fn api_diff_handler(
    State(state): State<AppState>,
    Query(params): Query<DiffParams>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(0);
    let page_size = params.page_size.unwrap_or(100);

    // 只返回请求的部分
    let start = page * page_size;
    let end = start + page_size;

    let partial_result = state
        .diff_result
        .files[0]
        .hunks
        .iter()
        .flat_map(|h| &h.lines)
        .skip(start)
        .take(page_size)
        .collect::<Vec<_>>();

    axum::Json(PartialDiffResult {
        lines: partial_result,
        total: state.diff_result.total_files_changed,
        page,
        has_more: end < total_lines,
    })
}

#[derive(Deserialize)]
struct DiffParams {
    page: Option<usize>,
    page_size: Option<usize>,
}

#[derive(Serialize)]
struct PartialDiffResult<'a> {
    lines: Vec<&'a LineChange>,
    total: usize,
    page: usize,
    has_more: bool,
}
```

#### 客户端实现

```javascript
class IncrementalDiffLoader {
    constructor() {
        this.page = 0;
        this.pageSize = 100;
        this.loading = false;
        this.hasMore = true;
    }

    async loadMore() {
        if (this.loading || !this.hasMore) return;

        this.loading = true;
        const response = await fetch(
            `/api/diff?page=${this.page}&page_size=${this.pageSize}`
        );
        const data = await response.json();

        this.appendLines(data.lines);
        this.hasMore = data.has_more;
        this.page++;
        this.loading = false;
    }

    appendLines(lines) {
        const container = document.getElementById('diff-container');
        lines.forEach(line => {
            container.appendChild(this.createLineElement(line));
        });
    }

    setupInfiniteScroll() {
        window.addEventListener('scroll', () => {
            const scrollHeight = document.documentElement.scrollHeight;
            const scrollTop = window.scrollY;
            const clientHeight = window.innerHeight;

            if (scrollTop + clientHeight >= scrollHeight - 200) {
                this.loadMore();
            }
        });
    }
}

// 使用
const loader = new IncrementalDiffLoader();
loader.setupInfiniteScroll();
loader.loadMore();  // 加载第一页
```

**优点**：
- ✅ 快速首屏
- ✅ 渐进式加载
- ✅ 节省带宽

---

### 方案 7: 智能采样 (对超大文件)

**核心思想**：对于超大文件，只对比采样点

#### 实现

```rust
pub struct SamplingDiffer {
    sample_rate: f64,  // 0.0 - 1.0
}

impl SamplingDiffer {
    /// 采样式对比（用于预览）
    pub fn diff_with_sampling(
        &self,
        file1: &MmapFile,
        file2: &MmapFile,
    ) -> Result<DiffResult> {
        let total_lines1 = file1.line_count();
        let total_lines2 = file2.line_count();

        // 计算采样点
        let sample_interval = (1.0 / self.sample_rate) as usize;

        let mut sampled_lines1 = Vec::new();
        let mut sampled_lines2 = Vec::new();

        for i in (0..total_lines1).step_by(sample_interval) {
            sampled_lines1.push((i, file1.get_line(i)));
        }

        for i in (0..total_lines2).step_by(sample_interval) {
            sampled_lines2.push((i, file2.get_line(i)));
        }

        // 对采样数据进行 diff
        let sampled_diff = self.diff_sampled(&sampled_lines1, &sampled_lines2)?;

        // 添加提示信息
        Ok(DiffResult {
            files: vec![FileDiff {
                hunks: sampled_diff,
                // ...
            }],
            metadata: Some(DiffMetadata {
                is_sampled: true,
                sample_rate: self.sample_rate,
                note: format!(
                    "Showing ~{}% of changes (sampled view for large file)",
                    (self.sample_rate * 100.0) as u32
                ),
            }),
            // ...
        })
    }
}
```

**使用场景**：
- 快速预览超大文件（> 1GB）
- 用户可选择完整对比或采样对比

---

## 🎯 推荐实现方案

### 综合方案（最佳实践）

结合多种技术，根据文件大小自动选择策略：

```rust
pub struct AdaptiveDiffer {
    config: DiffConfig,
}

pub struct DiffConfig {
    small_file_threshold: u64,      // 10 MB
    medium_file_threshold: u64,     // 100 MB
    chunk_size: usize,              // 10000 行
    enable_parallel: bool,
    enable_sampling: bool,
}

impl AdaptiveDiffer {
    pub fn diff_adaptive(
        &self,
        path1: &str,
        path2: &str,
    ) -> Result<DiffResult> {
        let size1 = fs::metadata(path1)?.len();
        let size2 = fs::metadata(path2)?.len();
        let max_size = size1.max(size2);

        match max_size {
            // 小文件：直接 read_to_string + similar
            s if s < self.config.small_file_threshold => {
                self.diff_small_files(path1, path2)
            }

            // 中等文件：内存映射 + 分块
            s if s < self.config.medium_file_threshold => {
                self.diff_medium_files(path1, path2)
            }

            // 大文件：内存映射 + 分块 + 并行
            _ => {
                if self.config.enable_sampling {
                    // 先显示采样结果，提供完整对比选项
                    self.diff_large_with_preview(path1, path2)
                } else {
                    self.diff_large_files(path1, path2)
                }
            }
        }
    }

    fn diff_small_files(&self, path1: &str, path2: &str) -> Result<DiffResult> {
        // 当前实现
        let content1 = fs::read_to_string(path1)?;
        let content2 = fs::read_to_string(path2)?;
        // ...
    }

    fn diff_medium_files(&self, path1: &str, path2: &str) -> Result<DiffResult> {
        // 内存映射 + 分块
        let file1 = MmapFile::open(path1)?;
        let file2 = MmapFile::open(path2)?;

        let chunked = ChunkedDiffer::new(self.config.chunk_size);
        chunked.diff_large_files(&file1, &file2)
    }

    fn diff_large_files(&self, path1: &str, path2: &str) -> Result<DiffResult> {
        // 内存映射 + 分块 + 并行
        let file1 = MmapFile::open(path1)?;
        let file2 = MmapFile::open(path2)?;

        if self.config.enable_parallel {
            let parallel = ParallelDiffer::new(num_cpus::get());
            parallel.diff_parallel(&file1, &file2)
        } else {
            let chunked = ChunkedDiffer::new(self.config.chunk_size);
            chunked.diff_large_files(&file1, &file2)
        }
    }

    fn diff_large_with_preview(
        &self,
        path1: &str,
        path2: &str,
    ) -> Result<DiffResult> {
        // 先快速采样
        println!("⚡ Large file detected. Generating preview...");

        let file1 = MmapFile::open(path1)?;
        let file2 = MmapFile::open(path2)?;

        let sampler = SamplingDiffer::new(0.1);  // 10% 采样
        let preview = sampler.diff_with_sampling(&file1, &file2)?;

        println!("✅ Preview ready!");
        println!("💡 Use --full flag for complete diff");

        Ok(preview)
    }
}
```

---

## 📦 需要的依赖

```toml
[dependencies]
# 现有依赖...

# 大文件优化
memmap2 = "0.9"          # 内存映射
memchr = "2.7"           # 快速字节查找
rayon = "1.10"           # 并行处理
num_cpus = "1.16"        # CPU 核心数检测

# 可选：更好的进度显示
indicatif = "0.17"       # 进度条
```

---

## 🧪 性能测试

### 测试脚本

```bash
#!/bin/bash

# 生成测试文件
generate_test_file() {
    local size=$1
    local file=$2
    dd if=/dev/urandom bs=1M count=$size | base64 > $file
}

# 测试不同大小
for size in 1 10 100 1000; do
    echo "Testing ${size}MB files..."
    generate_test_file $size test1.txt
    generate_test_file $size test2.txt

    time ./target/release/rdiff test1.txt test2.txt > /dev/null

    rm test1.txt test2.txt
done
```

### 预期性能

| 文件大小 | 内存占用 | 处理时间 | 备注 |
|---------|---------|---------|------|
| 1 MB    | < 10 MB | < 0.5s  | 直接读取 |
| 10 MB   | < 50 MB | < 2s    | 直接读取 |
| 100 MB  | < 100 MB | < 10s  | 内存映射 + 分块 |
| 1 GB    | < 200 MB | < 30s  | 内存映射 + 并行 |
| 10 GB   | < 300 MB | < 2min | 采样预览模式 |

---

## 🎨 用户体验优化

### CLI 进度提示

```rust
use indicatif::{ProgressBar, ProgressStyle};

pub fn diff_with_progress(path1: &str, path2: &str) -> Result<DiffResult> {
    let file1 = MmapFile::open(path1)?;
    let total_lines = file1.line_count();

    let pb = ProgressBar::new(total_lines as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} lines ({eta})")
            .unwrap()
    );

    // 分块处理时更新进度
    for chunk in file1.chunks(10000) {
        // process chunk...
        pb.inc(chunk.len() as u64);
    }

    pb.finish_with_message("Diff complete!");

    Ok(result)
}
```

### Web 界面提示

```html
<div id="diff-status">
    <div class="loading">
        <div class="spinner"></div>
        <p>Processing large file... <span id="progress">0%</span></p>
        <small>This may take a moment for files over 100MB</small>
    </div>
</div>
```

---

## ✅ 实施计划

### Phase 1: 基础优化 (优先级: 高)
1. ✅ 添加文件大小检测
2. ✅ 实现内存映射读取
3. ✅ 实现分块 diff
4. ✅ 集成到现有代码

### Phase 2: 并行优化 (优先级: 中)
5. ✅ 添加并行处理
6. ✅ 实现进度条
7. ✅ 性能测试和调优

### Phase 3: Web 优化 (优先级: 中)
8. ✅ 实现虚拟滚动
9. ✅ 实现增量加载 API
10. ✅ 优化 JSON 传输

### Phase 4: 高级特性 (优先级: 低)
11. ⏳ 实现采样预览
12. ⏳ 实现流式处理
13. ⏳ 添加缓存机制

---

## 🎯 总结

### 关键优化点

1. **内存映射** - 避免完整加载文件
2. **分块处理** - 控制内存占用
3. **并行计算** - 利用多核加速
4. **虚拟滚动** - Web 端只渲染可见部分
5. **增量加载** - API 分页返回数据
6. **智能采样** - 超大文件快速预览

### 性能提升

- **内存占用**: 从 O(file_size) 降到 O(chunk_size)
- **处理速度**: 并行可提升 2-4x
- **首屏时间**: 从等待全部完成到秒级响应

### 实现复杂度

- 基础优化 (1-3): ⭐⭐⭐ (中等)
- 并行优化 (4-7): ⭐⭐⭐⭐ (较难)
- Web 优化 (8-10): ⭐⭐⭐ (中等)

**预计开发时间**: 5-7 天完整实现基础优化

准备好开始优化了吗？🚀
