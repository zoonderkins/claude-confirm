# Claude Confirm

一個為 Claude Code CLI 打造的互動確認工具，讓 AI 完成任務後能以視覺化方式呈現總結、讓用戶選擇段落並提供反饋。

## 🎯 解決的問題

在使用 Claude Code CLI 時，AI 完成複雜任務後：
- ❌ 缺乏視覺化的總結呈現
- ❌ 無法讓用戶選擇性確認部分內容
- ❌ 難以提供結構化的反饋
- ❌ 不支援互動式的內容選擇

**Claude Confirm** 透過 MCP (Model Context Protocol) 提供：
- ✅ Markdown 格式的總結視窗
- ✅ 可選擇的段落清單
- ✅ 用戶輸入欄位
- ✅ 即時的視覺化反饋
- 📝 Prompt 預防：在調用前就教 AI 正確使用
- 🔒 返回強制：在返回後用明確指示強制執行
- 🎯 雙重保障：大幅降低誤判機率

## 🏗️ 技術架構

```
Claude Code CLI
    ↓ (透過 MCP 調用)
MCP Server (Rust)
    ↓ (啟動 GUI 並傳遞數據)
Tauri UI (Vue + Rust)
    ↓ (用戶互動)
返回選擇結果 → Claude Code
```

**技術棧**：
- **後端**: Rust + MCP Protocol
- **前端**: Vue 3 + Vite + Markdown-it
- **GUI 框架**: Tauri 2.0
- **樣式**: CSS Variables (支援深色模式)

## 🚀 編譯和安裝

### 前置需求

```bash
# 安裝 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安裝 Node.js 依賴
pnpm install
```

### 編譯

```bash
# 完整構建（包含前端和後端）
pnpm tauri:build

# 僅編譯 MCP server（無 UI）
cargo build --release --bin claude-confirm
```

**輸出位置**：
- MCP Server: `target/release/claude-confirm`
- GUI 應用: `target/release/claude-confirm-ui`
- macOS App: `target/release/bundle/macos/claude-confirm.app`
- DMG: `target/release/bundle/dmg/claude-confirm_0.1.0_aarch64.dmg`

### 安裝到 Claude Code

```bash
# 註冊 MCP server
claude mcp add confirm --scope local /path/to/target/release/claude-confirm

# 驗證安裝
claude mcp list
```

## 📖 使用方法

### 自動觸發

Claude Code 會在以下情況自動調用 confirm 工具：
- 完成多步驟任務
- 修改多個文件
- 執行構建或測試
- 完成問題診斷和修復
- 代碼重構完成時

### 手動調用

在對話中要求 Claude 使用 confirm 工具：

```
請用 mcp confirm 總結一下剛才的修改
```

### 互動流程

1. **AI 調用工具** - Claude 將總結內容傳給 confirm 工具
2. **彈出視窗** - Tauri GUI 自動顯示
3. **用戶互動**：
   - 閱讀 Markdown 格式的總結
   - 選擇/取消選擇段落
   - 在輸入框提供額外反饋
4. **返回結果** - 選擇的段落和輸入返回給 Claude
5. **繼續對話** - Claude 根據反饋繼續工作

## 🎨 功能特性

### Markdown 渲染
- 支援代碼高亮
- 支援表格、清單、標題
- 響應式排版

### 段落選擇
- 可勾選/取消段落
- 默認選中狀態可配置
- 顯示選中數量

### UI 優化
- 深色/淺色主題切換
- 視窗置頂功能（📌）
- 響應式設計（RWD）
- 縮小 10% 的緊湊界面

### DevTools 支援
- Release 版本支援 DevTools
- 點擊 🔧 按鈕開啟
- 支援 F12 快捷鍵

## 🔧 開發

### 開發模式

```bash
# 啟動開發伺服器（支援熱重載）
pnpm tauri:dev

# 開啟 DevTools
# 方法 1: 點擊 UI 上的 🔧 按鈕
# 方法 2: 按 F12 鍵
```

### Production Build

```bash
# 完整 production build（推薦）
pnpm tauri:build

# 產出位置：
# - macOS App: target/release/bundle/macos/claude-confirm.app
# - DMG: target/release/bundle/dmg/claude-confirm_0.2.0_aarch64.dmg
# - Binary: target/release/claude-confirm-ui
```

### Debug 模式

```bash
# 僅編譯 debug 版本（不啟動 UI）
cargo build --bin claude-confirm-ui

# 執行 debug 版本
./target/debug/claude-confirm-ui

# 或僅編譯 MCP server
cargo build --bin claude-confirm
./target/debug/claude-confirm
```

### 常用 Debug 技巧

**查看 Rust 日誌**：
```bash
RUST_LOG=debug ./target/debug/claude-confirm-ui
```

**測試 UI（使用測試 JSON）**：
```bash
# 建立測試請求檔案
cat > /tmp/test_request.json <<'EOF'
{
  "id": "test-123",
  "message": "# 測試\n\n這是測試訊息",
  "sections": [
    {
      "title": "段落 1",
      "content": "內容 1",
      "selected": true
    }
  ],
  "is_markdown": true
}
EOF

# 使用測試檔案啟動 UI
./target/debug/claude-confirm-ui --mcp-request /tmp/test_request.json
```

**檢查編譯產物**：
```bash
# 列出所有編譯產物
ls -lh target/release/claude-confirm*

# 查看 bundle 內容
ls -lhR target/release/bundle/
```

### 項目結構

```
claude-confirm/
├── src/
│   ├── rust/
│   │   ├── bin/
│   │   │   └── mcp_server.rs      # MCP 服務器入口
│   │   ├── mcp/
│   │   │   ├── mod.rs
│   │   │   ├── server.rs          # MCP 實現
│   │   │   └── types.rs
│   │   ├── ui/
│   │   │   ├── mod.rs
│   │   │   └── commands.rs        # Tauri 命令
│   │   ├── types.rs               # 共用類型
│   │   ├── lib.rs
│   │   └── main.rs                # UI 入口
│   └── frontend/
│       ├── components/
│       │   ├── MarkdownViewer.vue
│       │   ├── SectionList.vue
│       │   └── UserInput.vue
│       ├── App.vue
│       └── main.js
├── Cargo.toml
├── package.json
├── tauri.conf.json
└── vite.config.js
```

### 調試技巧

**查看 MCP server 日誌**：
```bash
RUST_LOG=debug ./target/release/claude-confirm
```

**測試 UI 界面**：
```bash
./target/release/claude-confirm-ui --mcp-request /tmp/test_request.json
```

**測試請求格式**：
```json
{
  "id": "test-123",
  "message": "# 測試\n\n這是一個測試",
  "sections": [
    {
      "title": "段落 1",
      "content": "內容 1",
      "selected": true
    }
  ],
  "is_markdown": true
}
```

## 🐛 故障排除

### UI 程序找不到

**問題**: MCP server 找不到 UI 程序

**解決**:
```bash
# 確保兩個 binary 在同一目錄
ls -lh target/release/claude-confirm*
```

### 編譯失敗

**macOS**:
```bash
xcode-select --install
```

**Ubuntu/Debian**:
```bash
sudo apt-get install -y \
  libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

### Claude Code 無法連接

1. 檢查 MCP server 是否正確註冊：
   ```bash
   claude mcp list
   ```

2. 查看 Claude Code 日誌：
   ```bash
   tail -f ~/.claude/logs/mcp*.log
   ```

3. 測試 MCP server：
   ```bash
   echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | ./target/release/claude-confirm
   ```

## 📄 授權

MIT License