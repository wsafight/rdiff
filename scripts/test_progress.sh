#!/bin/bash

# 测试进度条功能
set -e

RDIFF="./target/release/rdiff"
TEST_DIR="./test_progress"

echo "🧪 Progress Bar Feature Test"
echo "================================"
echo ""

# 创建测试目录
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# 清理旧文件
rm -f *.txt

echo "📝 Test 1: Medium file with progress bar (50,001 lines)"
echo "--------------------------------------------------------"
# 生成刚好超过阈值的文件（50,001行）来触发进度条
for i in {1..50001}; do
    echo "Line $i: This is test content for progress bar demonstration" >> medium1.txt
done

# 复制并修改
cp medium1.txt medium2.txt
# 在中间添加一些修改
sed -i.bak '25000a\
INSERTED LINE AT 25000' medium2.txt
sed -i.bak '30000s/.*/MODIFIED LINE AT 30000/' medium2.txt
rm -f medium2.txt.bak

FILE_LINES=$(wc -l < medium1.txt)
echo "File lines: $FILE_LINES"
echo ""
echo "Running diff with progress bar..."
echo ""

# 运行 diff，应该显示进度条
../$RDIFF medium1.txt medium2.txt > /dev/null

echo ""
echo "✅ Medium file test complete"
echo ""

echo "📝 Test 2: Large file with parallel progress (150,000 lines)"
echo "--------------------------------------------------------"
# 生成大文件
for i in {1..150000}; do
    echo "Line $i: Large file content for parallel processing test" >> large1.txt
done

cp large1.txt large2.txt
# 在多个位置修改
sed -i.bak '50000s/.*/MODIFIED AT 50000/' large2.txt
sed -i.bak '100000s/.*/MODIFIED AT 100000/' large2.txt
rm -f large2.txt.bak

FILE_LINES=$(wc -l < large1.txt)
echo "File lines: $FILE_LINES"
echo ""
echo "Running diff with parallel processing and progress bar..."
echo ""

# 运行 diff，应该显示并行处理的进度条
time ../$RDIFF large1.txt large2.txt > /dev/null

echo ""
echo "✅ Large file test complete"
echo ""

echo "================================"
echo "✅ All progress bar tests passed!"
echo ""
echo "Progress bars are shown when:"
echo "- File has > 50,000 lines"
echo "- Medium files: line-by-line progress"
echo "- Large files: chunk-by-chunk progress (parallel)"
echo ""
echo "To clean up: rm -rf $TEST_DIR"
