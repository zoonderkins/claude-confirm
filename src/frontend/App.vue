<template>
  <div class="app">
    <div v-if="!request" class="loading">
      <div class="loading-spinner"></div>
      <p>等待請求...</p>
      <p v-if="debugInfo" style="font-size: 0.8em; margin-top: 1rem; color: #333; white-space: pre-wrap; max-width: 600px; background: #f0f0f0; padding: 1rem; border-radius: 8px; text-align: left;">
        {{ debugInfo }}
      </p>
      <div style="display: flex; gap: 0.5rem; margin-top: 1rem;">
        <button @click="openDevTools" style="padding: 0.5rem 1rem; cursor: pointer;">
          🔧 開發者工具 (F12)
        </button>
        <button @click="loadTestData" style="padding: 0.5rem 1rem; cursor: pointer;">
          載入測試數據
        </button>
      </div>
      <p style="font-size: 0.8em; margin-top: 1rem; color: #999;">
        如快捷鍵無效，請點擊上方按鈕開啟開發者工具
      </p>
    </div>

    <div v-else class="confirm-dialog" :class="{ dark: isDark }">
      <div class="dialog-header">
        <h2>Claude Confirm</h2>
        <div class="header-actions">
          <ExportDropdown
            :targetElement="dialogBodyRef"
            :isDark="isDark"
          />
          <button @click="showAbout = true" class="icon-btn" title="關於">
            ℹ️
          </button>
          <button @click="openDevTools" class="icon-btn" title="開發者工具 (F12)">
            🔧
          </button>
          <button @click="togglePin" class="icon-btn" :title="isPinned ? '取消置顶' : '窗口置顶'">
            📌
          </button>
          <button @click="showSettings = !showSettings" class="icon-btn" title="设置">
            ⚙️
          </button>
        </div>
      </div>

      <!-- 设置面板 -->
      <div v-if="showSettings" class="settings-panel">
        <h3>设置</h3>
        <div class="setting-item">
          <label>主题</label>
          <div class="theme-toggle">
            <button
              @click="setTheme('light')"
              :class="{ active: !isDark }"
              class="theme-btn"
            >
              ☀️ 浅色
            </button>
            <button
              @click="setTheme('dark')"
              :class="{ active: isDark }"
              class="theme-btn"
            >
              🌙 深色
            </button>
          </div>
        </div>
      </div>

      <div class="dialog-body" ref="dialogBodyRef">
        <MarkdownViewer :content="request.message" />

        <SectionList
          v-if="request.sections && request.sections.length > 0"
          :sections="request.sections"
          v-model="selectedSections"
        />

        <UserInput v-model="userInput" />
      </div>

      <div class="dialog-footer">
        <button @click="handleCancel" class="btn btn-cancel">
          取消
        </button>
        <button @click="handleConfirm" class="btn btn-confirm" :disabled="isSubmitting">
          {{ isSubmitting ? '提交中...' : '確認' }}
        </button>
      </div>
    </div>

    <!-- About 對話框 -->
    <AboutDialog v-model:visible="showAbout" :class="{ dark: isDark }" />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import MarkdownViewer from './components/MarkdownViewer.vue'
import SectionList from './components/SectionList.vue'
import UserInput from './components/UserInput.vue'
import AboutDialog from './components/AboutDialog.vue'
import ExportDropdown from './components/ExportDropdown.vue'

const request = ref(null)
const selectedSections = ref([])
const userInput = ref('')
const isSubmitting = ref(false)
const debugInfo = ref('')
const showSettings = ref(false)
const isDark = ref(false)
const isPinned = ref(true)
const showAbout = ref(false)
const dialogBodyRef = ref(null)

// 載入儲存的設定
function loadSettings() {
  const savedTheme = localStorage.getItem('theme')
  const savedPin = localStorage.getItem('pinned')

  if (savedTheme) {
    isDark.value = savedTheme === 'dark'
  }
  if (savedPin !== null) {
    isPinned.value = savedPin === 'true'
  }
}

// 開發環境日誌輔助函數
const isDev = import.meta.env.DEV
const devLog = (...args) => {
  if (isDev) console.log(...args)
}
const devError = (...args) => {
  if (isDev) console.error(...args)
}

// 切換置頂
async function togglePin() {
  isPinned.value = !isPinned.value
  localStorage.setItem('pinned', isPinned.value.toString())

  try {
    await invoke('set_always_on_top', { alwaysOnTop: isPinned.value })
  } catch (e) {
    devError('設定置頂失敗:', e)
  }
}

// 設定主題
function setTheme(theme) {
  isDark.value = theme === 'dark'
  localStorage.setItem('theme', theme)
}

// 開啟 DevTools
async function openDevTools() {
  try {
    await invoke('open_devtools')
  } catch (e) {
    devError('開啟 DevTools 失敗:', e)
  }
}

onMounted(async () => {
  devLog('✅ App mounted')
  debugInfo.value = '步驟 1: App mounted\n'

  // 載入設定
  loadSettings()

  // 設定初始置頂狀態
  try {
    await invoke('set_always_on_top', { alwaysOnTop: isPinned.value })
  } catch (e) {
    devLog('無法設定置頂:', e)
  }

  try {
    // 檢查是否為 MCP 模式
    devLog('📍 檢查 CLI 參數...')
    debugInfo.value += '步驟 2: 檢查 CLI 參數...\n'

    const args = await invoke('get_cli_args')
    devLog('CLI args:', args)
    debugInfo.value += `步驟 3: CLI args = ${JSON.stringify(args)}\n`

    if (args && args.mcp_request) {
      devLog('📂 讀取 MCP 請求文件:', args.mcp_request)
      debugInfo.value += `步驟 4: 讀取文件 ${args.mcp_request}\n`

      // 讀取 MCP 請求文件
      const content = await invoke('read_mcp_request', { filePath: args.mcp_request })
      devLog('📄 文件內容:', content)
      debugInfo.value += `步驟 5: 文件內容 = ${JSON.stringify(content).substring(0, 100)}...\n`

      if (content) {
        request.value = content
        devLog('✅ 成功載入 MCP 請求:', request.value)
        debugInfo.value += '步驟 6: ✅ 成功載入請求!\n'

        // 初始化選中的段落
        if (request.value.sections) {
          selectedSections.value = request.value.sections
            .map((s, i) => s.selected ? i : -1)
            .filter(i => i >= 0)
          devLog('✅ 選中的段落:', selectedSections.value)
          debugInfo.value += `步驟 7: 選中段落 ${selectedSections.value}\n`
        }
      }
    } else {
      devLog('⚠️ 非 MCP 模式或無請求文件')
      debugInfo.value += '⚠️ 非 MCP 模式或無請求文件\n'
    }
  } catch (e) {
    devError('❌ 錯誤:', e)
    debugInfo.value += `❌ 錯誤: ${e}\n`
  }

  // 監聽事件（備用）
  await listen('mcp-request', (event) => {
    devLog('📩 通過事件收到請求')
    request.value = event.payload

    if (request.value.sections) {
      selectedSections.value = request.value.sections
        .map((s, i) => s.selected ? i : -1)
        .filter(i => i >= 0)
    }
  })

  devLog('✅ 初始化完成')
  debugInfo.value += '✅ 初始化完成\n'
})

async function handleConfirm() {
  if (isSubmitting.value) return

  isSubmitting.value = true

  const response = {
    confirmed: true,
    selected_sections: selectedSections.value,
    user_input: userInput.value,
    images: []
  }

  try {
    await invoke('submit_response', { response })
  } catch (e) {
    devError('提交失敗:', e)
    isSubmitting.value = false
  }
}

async function handleCancel() {
  try {
    await invoke('cancel_response')
  } catch (e) {
    devError('取消失敗:', e)
  }
}

function loadTestData() {
  devLog('📦 載入測試數據...')
  request.value = {
    id: 'test-123',
    message: '# 測試窗口\n\n這是一個測試消息\n\n- 項目 1\n- 項目 2\n- 項目 3',
    sections: [
      {
        title: '修復類型錯誤',
        content: '在 user.ts:42 中修復類型定義',
        selected: true
      },
      {
        title: '性能優化',
        content: '添加緩存機制',
        selected: false
      }
    ],
    is_markdown: true
  }
  selectedSections.value = [0]
  devLog('✅ 測試數據已載入')
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --bg-primary: #ffffff;
  --bg-secondary: #f9fafb;
  --text-primary: #1f2937;
  --text-secondary: #6b7280;
  --border-color: #e5e7eb;
  --accent-color: #9333EA;
  --accent-hover: #7C3AED;
  --accent-light: rgba(147, 51, 234, 0.1);
  /* 整體縮小 10% */
  font-size: 90%;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Microsoft JhengHei', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.app {
  min-height: 100vh;
  background: #f5f5f5;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  color: #666;
  padding: 1rem;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #e0e0e0;
  border-top-color: #4CAF50;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.confirm-dialog {
  max-width: 100%;
  width: 100%;
  height: 100vh;
  margin: 0;
  background: var(--bg-primary);
  border-radius: 0;
  box-shadow: none;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.confirm-dialog.dark {
  --bg-primary: #1f2937;
  --bg-secondary: #111827;
  --text-primary: #f3f4f6;
  --text-secondary: #9ca3af;
  --border-color: #374151;
  --accent-color: #A855F7;
  --accent-hover: #9333EA;
  --accent-light: rgba(168, 85, 247, 0.15);
}

.dialog-header {
  background: linear-gradient(135deg, var(--accent-color) 0%, var(--accent-hover) 100%);
  color: white;
  padding: 1.2rem 1.5rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
}

.dialog-header h2 {
  font-size: 1.3rem;
  font-weight: 600;
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 0.5rem;
}

.icon-btn {
  background: rgba(255, 255, 255, 0.2);
  border: none;
  width: 32px;
  height: 32px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1.1rem;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.icon-btn:hover {
  background: rgba(255, 255, 255, 0.3);
  transform: scale(1.05);
}

.settings-panel {
  background: var(--bg-secondary);
  padding: 1.2rem 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.settings-panel h3 {
  font-size: 1rem;
  margin-bottom: 0.75rem;
  color: var(--text-primary);
}

.setting-item {
  margin: 0.75rem 0;
}

.setting-item label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
  color: var(--text-secondary);
  font-weight: 500;
}

.theme-toggle {
  display: flex;
  gap: 0.5rem;
}

.theme-btn {
  flex: 1;
  padding: 0.6rem;
  border: 2px solid var(--border-color);
  background: var(--bg-primary);
  color: var(--text-primary);
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.2s ease;
}

.theme-btn:hover {
  border-color: var(--accent-color);
}

.theme-btn.active {
  border-color: var(--accent-color);
  background: var(--accent-color);
  color: white;
}

.dialog-body {
  padding: 1.2rem 1.5rem;
  overflow-y: auto;
  flex: 1;
  min-height: 0;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding: 1rem 1.5rem;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
  flex-shrink: 0;
}

.btn {
  padding: 0.75rem 1.75rem;
  border: none;
  border-radius: 6px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-cancel {
  background: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
}

.btn-cancel:hover:not(:disabled) {
  background: var(--border-color);
}

.btn-confirm {
  background: var(--accent-color);
  color: white;
}

.btn-confirm:hover:not(:disabled) {
  background: var(--accent-hover);
  box-shadow: 0 2px 8px rgba(76, 175, 80, 0.3);
}

/* Responsive Design - Tablet */
@media (max-width: 768px) {
  .app {
    padding: 0.5rem;
  }

  .confirm-dialog {
    max-height: 98vh;
    border-radius: 8px;
  }

  .dialog-header {
    padding: 1.25rem 1rem;
  }

  .dialog-header h2 {
    font-size: 1.25rem;
  }

  .dialog-body {
    padding: 1.25rem 1rem;
  }

  .dialog-footer {
    padding: 1rem;
    gap: 0.5rem;
  }

  .btn {
    padding: 0.65rem 1.5rem;
    font-size: 0.95rem;
  }
}

/* Responsive Design - Mobile */
@media (max-width: 480px) {
  .app {
    padding: 0;
  }

  .confirm-dialog {
    max-height: 100vh;
    border-radius: 0;
    height: 100vh;
  }

  .dialog-header {
    padding: 1rem;
  }

  .dialog-header h2 {
    font-size: 1.1rem;
  }

  .dialog-body {
    padding: 1rem;
  }

  .dialog-footer {
    flex-direction: column-reverse;
    gap: 0.5rem;
    padding: 0.75rem;
  }

  .btn {
    width: 100%;
    padding: 0.75rem;
    font-size: 0.95rem;
  }
}

/* Responsive Design - Small Mobile */
@media (max-width: 360px) {
  .dialog-header h2 {
    font-size: 1rem;
  }

  .dialog-body {
    padding: 0.75rem;
  }
}
</style>
