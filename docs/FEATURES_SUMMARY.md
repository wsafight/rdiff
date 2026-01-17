# Web Interface Features Update

## ✨ New Features Implemented

### 1. Per-File "Show Full File" Button

**Location**: Upper right corner of EACH file's header in the web interface

**Functionality**:
- **Default Mode (Diff Only)**: Shows only changed lines and context (traditional diff view)
- **Full File Mode**: Displays the complete file with changes highlighted

**Button Behavior**:
- Blue button `Show Full File` → Click to switch to full file mode
- Purple button `Show Diff Only` → Click to return to diff mode
- Each file has independent state - toggling one file doesn't affect others
- Works in both Unified and Side-by-Side view modes

### 2. Internationalization - English Button Text

**Changes**:
| Previous (Chinese) | New (English) |
|-------------------|---------------|
| 切换到并排视图 | Switch to Side-by-Side |
| 切换到统一视图 | Switch to Unified |

---

## 🎮 How to Use

### Starting Web Mode
```bash
# Build the project
cargo build --release

# Launch web interface
./target/release/rdiff file1.txt file2.txt --web

# Or with directories
./target/release/rdiff dir1/ dir2/ --web
```

### Testing the Features

1. **Open the web interface** - Browser opens automatically
2. **Test per-file toggle**:
   - Find the "Show Full File" button in the upper right of a file header
   - Click it to see the complete file with changes highlighted
   - Button turns purple and text changes to "Show Diff Only"
   - Click again to return to diff-only mode
3. **Test view switching**:
   - Click "Switch to Side-by-Side" to see files side by side
   - Per-file buttons work in both view modes
4. **Test multiple files**:
   - Each file's toggle works independently
   - Switching views preserves each file's state

---

## 🔧 Technical Implementation

### Files Modified

1. **`src/web/templates.rs`**
   - Removed global full file button from header
   - Updated button text to English
   - Kept only view toggle button in header

2. **`src/web/assets.rs`**
   - Added CSS for flexbox file headers
   - Added `.file-header`, `.file-name`, `.file-actions`, `.btn-small` styles
   - Changed JavaScript from global `showFullFile` boolean to `fileStates` object
   - Added `toggleFileFullView(filePath)` function for per-file state management
   - Updated `generateUnifiedView()` to render per-file buttons
   - Updated `generateSideBySideView()` to render per-file buttons

### Key Code Changes

**CSS Structure**:
```css
.file-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.file-name {
    flex: 1;  /* Takes up remaining space */
}

.file-actions {
    display: flex;
    gap: 0.5rem;  /* Right-aligned buttons */
}
```

**JavaScript State Management**:
```javascript
const fileStates = {};  // Object to track each file's state

function toggleFileFullView(filePath) {
    fileStates[filePath] = !fileStates[filePath];
    renderDiff(currentView);
}
```

**Per-File Button Rendering**:
```javascript
const showFullFile = fileStates[file.path] || false;

html += `<div class="file-header">`;
html += `<span class="file-name">${escapeHtml(file.path)}</span>`;
html += `<div class="file-actions">`;
html += `<button class="btn btn-secondary btn-small"
    onclick="toggleFileFullView('${escapeHtml(file.path)}')"
    style="background: ${showFullFile ? '#8250df' : '#0969da'}">`;
html += showFullFile ? 'Show Diff Only' : 'Show Full File';
html += `</button>`;
html += `</div>`;
html += `</div>`;
```

---

## 🧪 Testing

### Quick Test
```bash
# Create test files
echo -e "Line 1\nLine 2\nLine 3\nLine 4\nLine 5" > /tmp/demo1.txt
echo -e "Line 1\nLine 2 MODIFIED\nLine 3\nLine 4 MODIFIED\nLine 5\nLine 6 ADDED" > /tmp/demo2.txt

# Run web mode
./target/release/rdiff /tmp/demo1.txt /tmp/demo2.txt --web
```

### Expected Behavior
1. Browser opens to `http://127.0.0.1:8080`
2. See one file header with path `/tmp/demo1.txt` (or similar)
3. Upper right corner has blue "Show Full File" button
4. Click button → turns purple, shows "Show Diff Only", displays all 6 lines
5. Click again → turns blue, shows "Show Full File", displays only changes
6. Click "Switch to Side-by-Side" → same button behavior in side-by-side mode

---

## 📊 Feature Comparison

### Before
- ❌ Only global view toggle (unified ↔ side-by-side)
- ❌ No way to see full file context
- ❌ Chinese button text

### After
- ✅ Global view toggle (unified ↔ side-by-side)
- ✅ Per-file full view toggle
- ✅ Independent state for each file
- ✅ English button text
- ✅ Visual feedback with color changes (blue/purple)
- ✅ Works in both view modes

---

## 🎨 Visual Design

**Button Colors**:
- **Blue (#0969da)**: Default state (Show Full File)
- **Purple (#8250df)**: Active state (Show Diff Only)

**Layout**:
```
┌─────────────────────────────────────────────────┐
│  File: /path/to/file.txt    [Show Full File]   │ ← File Header
├─────────────────────────────────────────────────┤
│  1   1   Line 1                                 │
│  2      -Line 2                                 │ ← Diff Content
│      2  +Line 2 MODIFIED                        │
└─────────────────────────────────────────────────┘
```

---

**Update Complete!** ✅

The web interface now supports per-file full view toggling with English button text, providing a more flexible and user-friendly diff viewing experience.
