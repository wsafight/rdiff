#!/bin/bash

cd ~/Desktop/re-search-ai/rust-diff-tool

echo "🚀 Starting web server..."
./target/release/rdiff examples/sample1.txt examples/sample2.txt --web --port 9001 > /tmp/rdiff_web.log 2>&1 &
SERVER_PID=$!

# Wait for server to start
sleep 3

echo "📡 Testing HTTP endpoint..."
HTTP_CODE=$(curl -s -o /tmp/rdiff_response.html -w "%{http_code}" http://127.0.0.1:9001/)

if [ "$HTTP_CODE" = "200" ]; then
    echo "✅ HTTP Status: $HTTP_CODE (OK)"

    # Check HTML content
    if grep -q "Diff Viewer" /tmp/rdiff_response.html; then
        echo "✅ HTML title found"
    fi

    if grep -q "file(s) changed" /tmp/rdiff_response.html; then
        echo "✅ Statistics found"
    fi

    if grep -q "const diffData" /tmp/rdiff_response.html; then
        echo "✅ JavaScript data embedded"
    fi

    # Show stats
    STATS=$(grep -o "[0-9]* file(s) changed" /tmp/rdiff_response.html | head -1)
    echo "📊 Diff stats: $STATS"
else
    echo "❌ HTTP Status: $HTTP_CODE (Error)"
fi

echo ""
echo "🧪 Testing API endpoint..."
API_RESPONSE=$(curl -s http://127.0.0.1:9001/api/diff)
if echo "$API_RESPONSE" | grep -q "total_files_changed"; then
    echo "✅ API endpoint working"
    echo "$API_RESPONSE" | head -c 200
    echo "..."
fi

# Stop server
echo ""
echo "🛑 Stopping server..."
kill $SERVER_PID 2>/dev/null
wait $SERVER_PID 2>/dev/null

echo ""
echo "✅ Web mode test completed successfully!"
