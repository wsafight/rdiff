#!/bin/bash

# 测试 Web 新功能
set -e

RDIFF="./target/release/rdiff"
TEST_DIR="./test_web_features"

echo "🧪 Web Features Test"
echo "================================"
echo ""

# 创建测试目录
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# 清理旧文件
rm -f *.txt

echo "📝 Creating test files..."
echo ""

# 创建测试文件
cat > file1.txt << 'EOF'
Line 1: This is the first line
Line 2: This is the second line
Line 3: This will be modified
Line 4: This is the fourth line
Line 5: This will be deleted
Line 6: This is the sixth line
Line 7: This is the seventh line
Line 8: This is the eighth line
Line 9: This is the ninth line
Line 10: This is the tenth line
EOF

cat > file2.txt << 'EOF'
Line 1: This is the first line
Line 2: This is the second line
Line 3: This has been MODIFIED
Line 4: This is the fourth line
Line 6: This is the sixth line
Line 7: This is the seventh line
Line 7.5: This is a NEW line inserted here
Line 8: This is the eighth line
Line 9: This is the ninth line
Line 10: This is the tenth line
Line 11: This is a NEW line at the end
EOF

echo "✅ Test files created"
echo ""
echo "File 1: 10 lines"
wc -l file1.txt
echo ""
echo "File 2: 11 lines (1 modified, 1 deleted, 2 added)"
wc -l file2.txt
echo ""

echo "================================"
echo "🌐 Starting Web Server"
echo "================================"
echo ""
echo "This will open your browser with the following features:"
echo ""
echo "1️⃣  Switch to Side-by-Side / Switch to Unified"
echo "    - Toggle between unified and side-by-side view"
echo "    - Button text is now in English"
echo ""
echo "2️⃣  Show Full File / Show Diff Only"
echo "    - Click to toggle between:"
echo "      • Diff mode: Only changed lines + context (default)"
echo "      • Full file mode: All lines with highlighting"
echo "    - Button turns purple when in full file mode"
echo ""
echo "Features to test:"
echo "  ✓ Click 'Show Full File' - should show all 11 lines"
echo "  ✓ Click 'Show Diff Only' - should show only changed lines"
echo "  ✓ Click 'Switch to Side-by-Side' - should work in both modes"
echo "  ✓ Switch between views and full file mode"
echo ""
echo "Press Ctrl+C to stop the server when done testing..."
echo ""

# 启动 Web 服务器
../$RDIFF file1.txt file2.txt --web --port 8768

cd ..
echo ""
echo "To clean up: rm -rf $TEST_DIR"
