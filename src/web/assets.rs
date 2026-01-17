/// Get embedded CSS
pub fn get_css() -> &'static str {
    r#"
    * {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    body {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
        background: #f6f8fa;
        color: #24292e;
        line-height: 1.5;
    }

    header {
        background: #24292e;
        color: white;
        padding: 1.5rem 2rem;
        box-shadow: 0 1px 3px rgba(0,0,0,0.12);
    }

    header h1 {
        font-size: 1.75rem;
        margin-bottom: 1rem;
        font-weight: 600;
    }

    .controls {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 1rem;
        flex-wrap: wrap;
    }

    .btn {
        background: #2ea44f;
        color: white;
        border: none;
        padding: 0.5rem 1.25rem;
        border-radius: 6px;
        cursor: pointer;
        font-size: 0.875rem;
        font-weight: 500;
        transition: background 0.2s;
    }

    .btn:hover {
        background: #2c974b;
    }

    .btn:active {
        background: #298e46;
    }

    .btn-secondary {
        background: #0969da;
    }

    .btn-secondary:hover {
        background: #0860ca;
    }

    .btn-secondary:active {
        background: #0757ba;
    }

    .stats {
        font-size: 0.875rem;
        color: #d1d5da;
    }

    #diff-container {
        max-width: 1400px;
        margin: 2rem auto;
        padding: 0 1rem;
    }

    .file-diff {
        background: white;
        border: 1px solid #d0d7de;
        border-radius: 6px;
        margin-bottom: 1.5rem;
        overflow: hidden;
        box-shadow: 0 1px 3px rgba(0,0,0,0.05);
    }

    .file-header {
        background: #f6f8fa;
        padding: 0.75rem 1rem;
        border-bottom: 1px solid #d0d7de;
        font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
        font-size: 0.875rem;
        font-weight: 600;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .file-name {
        flex: 1;
    }

    .file-actions {
        display: flex;
        gap: 0.5rem;
    }

    .btn-small {
        padding: 0.25rem 0.75rem;
        font-size: 0.75rem;
        border-radius: 4px;
    }

    .diff-table {
        width: 100%;
        border-collapse: collapse;
        font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
        font-size: 0.75rem;
        table-layout: fixed;
    }

    .diff-table td {
        padding: 0 0.5rem;
        vertical-align: top;
        word-break: break-all;
    }

    .line-num {
        width: 50px;
        min-width: 50px;
        text-align: right;
        color: #57606a;
        background: #f6f8fa;
        border-right: 1px solid #d0d7de;
        user-select: none;
        padding-right: 0.5rem;
    }

    .line-content {
        white-space: pre-wrap;
        word-break: break-all;
        padding-left: 0.75rem;
    }

    .line-add {
        background: #ccffd8;
    }

    .line-add .line-num {
        background: #ccffd8;
    }

    .line-add .line-content {
        background: #e6ffec;
    }

    .line-delete {
        background: #ffd7d5;
    }

    .line-delete .line-num {
        background: #ffd7d5;
    }

    .line-delete .line-content {
        background: #ffebe9;
    }

    .line-context {
        background: white;
    }

    /* Side-by-side view styles */
    .side-by-side {
        display: flex;
        overflow-x: auto;
    }

    .side-by-side .side {
        flex: 1;
        min-width: 0;
    }

    .side-by-side .divider {
        width: 2px;
        background: #d0d7de;
        flex-shrink: 0;
    }

    .side-by-side .diff-table {
        table-layout: auto;
    }

    /* Loading state */
    .loading {
        text-align: center;
        padding: 3rem;
        color: #57606a;
    }

    /* Responsive */
    @media (max-width: 768px) {
        header {
            padding: 1rem;
        }

        header h1 {
            font-size: 1.25rem;
        }

        .controls {
            flex-direction: column;
            align-items: stretch;
        }

        .btn {
            width: 100%;
        }

        #diff-container {
            padding: 0 0.5rem;
        }

        .diff-table {
            font-size: 0.7rem;
        }

        .line-num {
            width: 40px;
            min-width: 40px;
        }
    }
    "#
}

/// Get embedded JavaScript
pub fn get_js() -> &'static str {
    r#"
    (function() {
        let currentView = 'unified'; // 'unified' or 'side-by-side'
        const fileStates = {}; // Track full file state for each file
        const VIRTUAL_SCROLL_THRESHOLD = 10000; // 超过此行数启用虚拟滚动
        const ROW_HEIGHT = 24; // 每行高度（像素）
        const BUFFER_ROWS = 50; // 上下缓冲的行数

        const toggleBtn = document.getElementById('toggle-view');
        const container = document.getElementById('diff-container');

        if (!toggleBtn || !container) {
            console.error('Required elements not found');
            return;
        }

        toggleBtn.addEventListener('click', function() {
            currentView = currentView === 'unified' ? 'side-by-side' : 'unified';
            renderDiff(currentView);
            toggleBtn.textContent = currentView === 'unified' ? 'Switch to Side-by-Side' : 'Switch to Unified';
        });

        // Event delegation for file toggle buttons
        container.addEventListener('click', function(e) {
            if (e.target.classList.contains('file-toggle-btn')) {
                const filePath = e.target.getAttribute('data-file-path');
                if (filePath) {
                    toggleFileFullView(filePath);
                }
            }
        });

        function renderDiff(view) {
            // 计算总行数
            const totalLines = diffData.files.reduce((total, file) => {
                if (file.is_binary) return total;
                return total + file.hunks.reduce((sum, hunk) => sum + hunk.lines.length, 0);
            }, 0);

            // 如果行数超过阈值，使用虚拟滚动
            if (totalLines > VIRTUAL_SCROLL_THRESHOLD) {
                if (view === 'unified') {
                    container.innerHTML = generateVirtualUnifiedView(diffData, totalLines);
                } else {
                    // 并排视图暂不支持虚拟滚动，使用常规渲染
                    container.innerHTML = generateSideBySideView(diffData);
                }
            } else {
                if (view === 'unified') {
                    container.innerHTML = generateUnifiedView(diffData);
                } else {
                    container.innerHTML = generateSideBySideView(diffData);
                }
            }
        }

        function generateVirtualUnifiedView(data, totalLines) {
            // 收集所有行数据
            const allLines = [];
            data.files.forEach(file => {
                allLines.push({
                    type: 'file-header',
                    content: file.path
                });

                if (!file.is_binary) {
                    file.hunks.forEach(hunk => {
                        hunk.lines.forEach(line => {
                            allLines.push({
                                type: 'line',
                                line: line
                            });
                        });
                    });
                }
            });

            const totalHeight = allLines.length * ROW_HEIGHT;

            let html = `
                <div class="file-diff">
                    <div style="color: #2ea44f; padding: 1rem; font-weight: bold;">
                        ⚡ Virtual Scrolling Enabled (${totalLines.toLocaleString()} lines)
                    </div>
                    <div class="virtual-scroll-container" style="height: 600px; overflow-y: auto; position: relative;">
                        <div class="virtual-scroll-spacer" style="height: ${totalHeight}px; position: relative;">
                            <table class="diff-table virtual-content" style="position: absolute; top: 0; left: 0; right: 0;">
                            </table>
                        </div>
                    </div>
                </div>
            `;

            // 渲染后设置滚动监听
            setTimeout(() => {
                const scrollContainer = container.querySelector('.virtual-scroll-container');
                const virtualContent = container.querySelector('.virtual-content');

                if (!scrollContainer || !virtualContent) return;

                function updateVisibleRows() {
                    const scrollTop = scrollContainer.scrollTop;
                    const containerHeight = scrollContainer.clientHeight;

                    const startIndex = Math.max(0, Math.floor(scrollTop / ROW_HEIGHT) - BUFFER_ROWS);
                    const endIndex = Math.min(allLines.length, Math.ceil((scrollTop + containerHeight) / ROW_HEIGHT) + BUFFER_ROWS);

                    let html = '';
                    for (let i = startIndex; i < endIndex; i++) {
                        const item = allLines[i];
                        const top = i * ROW_HEIGHT;

                        if (item.type === 'file-header') {
                            html += `<tr style="position: absolute; top: ${top}px; left: 0; right: 0; height: ${ROW_HEIGHT}px;">
                                <td colspan="3" class="file-header" style="padding: 0.5rem;">${escapeHtml(item.content)}</td>
                            </tr>`;
                        } else {
                            const line = item.line;
                            const changeClass = line.change_type === 'Add' ? 'line-add' :
                                              line.change_type === 'Delete' ? 'line-delete' :
                                              'line-context';
                            const prefix = line.change_type === 'Add' ? '+' :
                                         line.change_type === 'Delete' ? '-' : ' ';

                            html += `<tr class="${changeClass}" style="position: absolute; top: ${top}px; left: 0; right: 0; height: ${ROW_HEIGHT}px;">
                                <td class="line-num" style="width: 60px;">${line.old_line_num || ''}</td>
                                <td class="line-num" style="width: 60px;">${line.new_line_num || ''}</td>
                                <td class="line-content">${prefix}${escapeHtml(line.content)}</td>
                            </tr>`;
                        }
                    }

                    virtualContent.innerHTML = html;
                }

                scrollContainer.addEventListener('scroll', updateVisibleRows);
                updateVisibleRows(); // 初始渲染
            }, 0);

            return html;
        }

        function toggleFileFullView(filePath) {
            fileStates[filePath] = !fileStates[filePath];
            renderDiff(currentView);
        }

        // Expose to global scope for onclick handlers
        window.toggleFileFullView = toggleFileFullView;

        function generateUnifiedView(data) {
            let html = '';
            data.files.forEach((file, fileIndex) => {
                const showFullFile = fileStates[file.path] || false;
                const fileId = 'file-' + fileIndex;

                html += `<div class="file-diff">`;
                html += `<div class="file-header">`;
                html += `<span class="file-name">${escapeHtml(file.path)}</span>`;
                html += `<div class="file-actions">`;

                // Only show the button if full_content is available
                if (file.full_content && file.full_content.length > 0) {
                    html += `<button class="btn btn-secondary btn-small file-toggle-btn" data-file-path="${escapeHtml(file.path)}" style="background: ${showFullFile ? '#8250df' : '#0969da'}">`;
                    html += showFullFile ? 'Show Diff Only' : 'Show Full File';
                    html += `</button>`;
                }

                html += `</div>`;
                html += `</div>`;

                if (file.is_binary) {
                    html += `<div style="padding: 1rem; color: #57606a;">Binary file - cannot display diff</div>`;
                } else {
                    html += `<table class="diff-table">`;

                    // Use full_content when showFullFile is true, otherwise use hunks
                    if (showFullFile && file.full_content && file.full_content.length > 0) {
                        // Show full file content
                        file.full_content.forEach(line => {
                            html += `<tr class="line-context">`;
                            html += `<td class="line-num">${line.old_line_num || line.new_line_num || ''}</td>`;
                            html += `<td class="line-num">${line.new_line_num || line.old_line_num || ''}</td>`;
                            html += `<td class="line-content"> ${escapeHtml(line.content)}</td>`;
                            html += `</tr>`;
                        });
                    } else {
                        // Show diff only
                        file.hunks.forEach(hunk => {
                            hunk.lines.forEach(line => {
                                const changeClass = line.change_type === 'Add' ? 'line-add' :
                                                  line.change_type === 'Delete' ? 'line-delete' :
                                                  'line-context';
                                const prefix = line.change_type === 'Add' ? '+' :
                                             line.change_type === 'Delete' ? '-' : ' ';

                                html += `<tr class="${changeClass}">`;
                                html += `<td class="line-num">${line.old_line_num || ''}</td>`;
                                html += `<td class="line-num">${line.new_line_num || ''}</td>`;
                                html += `<td class="line-content">${prefix}${escapeHtml(line.content)}</td>`;
                                html += `</tr>`;
                            });
                        });
                    }

                    html += `</table>`;
                }
                html += `</div>`;
            });
            return html || '<div class="loading">No differences found</div>';
        }

        function generateSideBySideView(data) {
            let html = '';
            data.files.forEach(file => {
                const showFullFile = fileStates[file.path] || false;

                html += `<div class="file-diff">`;
                html += `<div class="file-header">`;
                html += `<span class="file-name">${escapeHtml(file.path)}</span>`;
                html += `<div class="file-actions">`;

                // Only show the button if full_content is available
                if (file.full_content && file.full_content.length > 0) {
                    html += `<button class="btn btn-secondary btn-small file-toggle-btn" data-file-path="${escapeHtml(file.path)}" style="background: ${showFullFile ? '#8250df' : '#0969da'}">`;
                    html += showFullFile ? 'Show Diff Only' : 'Show Full File';
                    html += `</button>`;
                }

                html += `</div>`;
                html += `</div>`;

                if (file.is_binary) {
                    html += `<div style="padding: 1rem; color: #57606a;">Binary file - cannot display diff</div>`;
                } else if (showFullFile && file.full_content && file.full_content.length > 0) {
                    // Full file view: show entire file (no side-by-side for full view, just unified)
                    html += `<table class="diff-table">`;
                    file.full_content.forEach(line => {
                        html += `<tr class="line-context">`;
                        html += `<td class="line-num">${line.old_line_num || line.new_line_num || ''}</td>`;
                        html += `<td class="line-num">${line.new_line_num || line.old_line_num || ''}</td>`;
                        html += `<td class="line-content"> ${escapeHtml(line.content)}</td>`;
                        html += `</tr>`;
                    });
                    html += `</table>`;
                } else {
                    // Diff view: show side-by-side
                    html += `<div class="side-by-side">`;
                    html += `<div class="side"><table class="diff-table">`;

                    // Left side (old)
                    file.hunks.forEach(hunk => {
                        hunk.lines.forEach(line => {
                            if (line.change_type !== 'Add') {
                                const changeClass = line.change_type === 'Delete' ? 'line-delete' : 'line-context';
                                html += `<tr class="${changeClass}">`;
                                html += `<td class="line-num">${line.old_line_num || ''}</td>`;
                                html += `<td class="line-content">${escapeHtml(line.content)}</td>`;
                                html += `</tr>`;
                            } else {
                                // Add empty row for alignment
                                html += `<tr class="line-context"><td class="line-num"></td><td class="line-content"></td></tr>`;
                            }
                        });
                    });

                    html += `</table></div>`;
                    html += `<div class="divider"></div>`;
                    html += `<div class="side"><table class="diff-table">`;

                    // Right side (new)
                    file.hunks.forEach(hunk => {
                        hunk.lines.forEach(line => {
                            if (line.change_type !== 'Delete') {
                                const changeClass = line.change_type === 'Add' ? 'line-add' : 'line-context';
                                html += `<tr class="${changeClass}">`;
                                html += `<td class="line-num">${line.new_line_num || ''}</td>`;
                                html += `<td class="line-content">${escapeHtml(line.content)}</td>`;
                                html += `</tr>`;
                            } else {
                                // Add empty row for alignment
                                html += `<tr class="line-context"><td class="line-num"></td><td class="line-content"></td></tr>`;
                            }
                        });
                    });

                    html += `</table></div>`;
                    html += `</div>`;
                }
                html += `</div>`;
            });
            return html || '<div class="loading">No differences found</div>';
        }

        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }

        // Initial render
        renderDiff(currentView);
    })();
    "#
}
