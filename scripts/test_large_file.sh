#!/bin/bash

# 大文件性能测试脚本
set -e

RDIFF="./target/release/rdiff"
TEST_DIR="./test_large_files"

echo "🧪 Large File Performance Test"
echo "================================"
echo ""

# 创建测试目录
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# 清理旧文件
rm -f small*.txt medium*.txt large*.txt

echo "📝 Generating test files..."
echo ""

# 1. 小文件测试 (1KB - 应使用快速模式)
echo "1️⃣ Small file test (1KB)..."
for i in {1..50}; do
    echo "Line $i: This is a test line with some content" >> small1.txt
done
cp small1.txt small2.txt
echo "MODIFIED LINE" >> small2.txt
FILE_SIZE=$(stat -f%z small1.txt 2>/dev/null || stat -c%s small1.txt)
echo "   File size: $FILE_SIZE bytes"

# 运行 diff
echo "   Running diff..."
RUST_LOG=info ../$RDIFF small1.txt small2.txt > /dev/null
echo "   ✅ Small file test passed"
echo ""

# 2. 中等文件测试 (15MB - 应使用分块模式)
echo "2️⃣ Medium file test (~15MB)..."
for i in {1..200000}; do
    echo "Line $i: This is a medium test file with repeated content for testing chunked diff algorithm" >> medium1.txt
done
cp medium1.txt medium2.txt
# 在中间修改一些行
sed -i.bak '100000s/.*/MODIFIED LINE AT 100000/' medium2.txt
sed -i.bak '150000s/.*/MODIFIED LINE AT 150000/' medium2.txt
rm -f medium2.txt.bak
FILE_SIZE=$(stat -f%z medium1.txt 2>/dev/null || stat -c%s medium1.txt)
echo "   File size: $(($FILE_SIZE / 1024 / 1024))MB"

# 运行 diff
echo "   Running diff..."
time RUST_LOG=info ../$RDIFF medium1.txt medium2.txt > /dev/null
echo "   ✅ Medium file test passed"
echo ""

# 3. 大文件测试 (150MB - 应使用并行模式)
echo "3️⃣ Large file test (~150MB)..."
for i in {1..2000000}; do
    echo "Line $i: This is a large test file with substantial content for testing parallel diff algorithm performance" >> large1.txt
done
cp large1.txt large2.txt
# 在多个位置修改
sed -i.bak '500000s/.*/MODIFIED LINE AT 500000/' large2.txt
sed -i.bak '1000000s/.*/MODIFIED LINE AT 1000000/' large2.txt
sed -i.bak '1500000s/.*/MODIFIED LINE AT 1500000/' large2.txt
rm -f large2.txt.bak
FILE_SIZE=$(stat -f%z large1.txt 2>/dev/null || stat -c%s large1.txt)
echo "   File size: $(($FILE_SIZE / 1024 / 1024))MB"

# 运行 diff
echo "   Running diff with parallel processing..."
time RUST_LOG=info ../$RDIFF large1.txt large2.txt > /dev/null
echo "   ✅ Large file test passed"
echo ""

echo "================================"
echo "✅ All large file tests passed!"
echo ""
echo "Performance Summary:"
echo "- Small files: Fast direct processing"
echo "- Medium files: Memory-mapped + chunked processing"
echo "- Large files: Memory-mapped + parallel processing"
echo ""
echo "Test files are in: $TEST_DIR"
echo "To clean up: rm -rf $TEST_DIR"
