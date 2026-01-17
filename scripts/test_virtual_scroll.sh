#!/bin/bash

# 测试虚拟滚动功能
set -e

RDIFF="./target/release/rdiff"
TEST_DIR="./test_virtual"

echo "🧪 Virtual Scrolling Feature Test"
echo "================================"
echo ""

# 创建测试目录
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# 清理旧文件
rm -f *.txt

echo "📝 Generating test file with 15,000 lines (will trigger virtual scrolling)"
echo "--------------------------------------------------------"

# 生成超过阈值的文件（15,000行，阈值是10,000行）
for i in {1..15000}; do
    echo "Line $i: This is test content for virtual scrolling demonstration with sufficient text" >> huge1.txt
done

# 复制并修改
cp huge1.txt huge2.txt
# 在多个位置添加修改
sed -i.bak '5000a\
INSERTED LINE AT 5000 - Should be visible in virtual scroll' huge2.txt
sed -i.bak '10000s/.*/MODIFIED LINE AT 10000 - Test virtual scrolling/' huge2.txt
sed -i.bak '14000a\
INSERTED LINE AT 14000 - Near the end' huge2.txt
rm -f huge2.txt.bak

FILE_LINES=$(wc -l < huge1.txt)
echo "File lines: $FILE_LINES"
echo ""
echo "Running diff in WEB mode..."
echo "This will open your browser with VIRTUAL SCROLLING enabled!"
echo ""
echo "Features to test in the browser:"
echo "  1. Should see '⚡ Virtual Scrolling Enabled' message"
echo "  2. Only visible rows are rendered (smooth scrolling)"
echo "  3. Can scroll through all 15,000 lines smoothly"
echo "  4. Changes are visible at lines 5000, 10000, and 14000"
echo ""
echo "Press Ctrl+C when done testing in browser..."
echo ""

# 运行 web 模式
../$RDIFF huge1.txt huge2.txt --web --port 8765

cd ..
echo ""
echo "To clean up: rm -rf $TEST_DIR"
