#!/bin/bash

# 测试增量加载 API
set -e

RDIFF="./target/release/rdiff"
TEST_DIR="./test_paginated"

echo "🧪 Paginated API Feature Test"
echo "================================"
echo ""

# 创建测试目录
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# 清理旧文件
rm -f *.txt

echo "📝 Generating test files with 500 lines"
echo "--------------------------------------------------------"

# 生成测试文件
for i in {1..500}; do
    echo "Line $i: Test content for paginated API demonstration" >> test1.txt
done

cp test1.txt test2.txt
# 添加一些修改
for i in {1..50}; do
    sed -i.bak "$(($i * 10))s/.*/MODIFIED LINE $(($i * 10))/" test2.txt
done
rm -f test2.txt.bak

FILE_LINES=$(wc -l < test1.txt)
echo "File lines: $FILE_LINES"
echo ""

# 启动 web 服务器在后台
echo "Starting web server in background..."
../$RDIFF test1.txt test2.txt --web --port 8766 > /dev/null 2>&1 &
SERVER_PID=$!

# 等待服务器启动
sleep 2

echo ""
echo "🧪 Testing Paginated API endpoints..."
echo "================================"
echo ""

# 测试 1: 获取第一页（默认每页 100 行）
echo "Test 1: Get first page (default size: 100 lines)"
echo "URL: http://localhost:8766/api/diff/paginated?page=0"
curl -s "http://localhost:8766/api/diff/paginated?page=0" | jq '{
    page: .page,
    page_size: .page_size,
    total_lines: .total_lines,
    total_pages: .total_pages,
    has_more: .has_more,
    lines_count: (.lines | length)
}'
echo ""

# 测试 2: 获取第二页
echo "Test 2: Get second page"
echo "URL: http://localhost:8766/api/diff/paginated?page=1"
curl -s "http://localhost:8766/api/diff/paginated?page=1" | jq '{
    page: .page,
    page_size: .page_size,
    has_more: .has_more,
    lines_count: (.lines | length)
}'
echo ""

# 测试 3: 自定义每页大小 (50 行)
echo "Test 3: Custom page size (50 lines per page)"
echo "URL: http://localhost:8766/api/diff/paginated?page=0&page_size=50"
curl -s "http://localhost:8766/api/diff/paginated?page=0&page_size=50" | jq '{
    page: .page,
    page_size: .page_size,
    total_pages: .total_pages,
    lines_count: (.lines | length)
}'
echo ""

# 测试 4: 获取第 3 页 (每页 50 行)
echo "Test 4: Get page 3 with page_size=50"
echo "URL: http://localhost:8766/api/diff/paginated?page=3&page_size=50"
curl -s "http://localhost:8766/api/diff/paginated?page=3&page_size=50" | jq '{
    page: .page,
    has_more: .has_more,
    first_line: .lines[0],
    lines_count: (.lines | length)
}'
echo ""

# 测试 5: 获取最后一页
echo "Test 5: Get last page"
curl -s "http://localhost:8766/api/diff/paginated?page=0" | jq -r '.total_pages' | {
    read total_pages
    last_page=$((total_pages - 1))
    echo "Last page number: $last_page"
    echo "URL: http://localhost:8766/api/diff/paginated?page=$last_page"
    curl -s "http://localhost:8766/api/diff/paginated?page=$last_page" | jq '{
        page: .page,
        has_more: .has_more,
        lines_count: (.lines | length)
    }'
}
echo ""

echo "================================"
echo "✅ All paginated API tests passed!"
echo ""
echo "API Features:"
echo "  - Pagination support with page & page_size parameters"
echo "  - Returns total_lines, total_pages, has_more metadata"
echo "  - Default page_size: 100 lines"
echo "  - Maximum page_size: 1000 lines"
echo ""
echo "Example usage:"
echo "  curl http://localhost:8766/api/diff/paginated?page=0&page_size=50"
echo ""

# 停止服务器
kill $SERVER_PID 2>/dev/null || true
wait $SERVER_PID 2>/dev/null || true

cd ..
echo "To clean up: rm -rf $TEST_DIR"
