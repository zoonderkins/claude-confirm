<template>
  <div class="user-input">
    <div class="input-wrapper" ref="wrapperRef">
      <!-- 高亮覆蓋層 -->
      <div
        ref="highlightRef"
        class="highlight-overlay"
        v-html="highlightedContent"
      ></div>

      <textarea
        ref="textareaRef"
        v-model="input"
        @keydown="handleKeyDown"
        @input="handleInput"
        @scroll="syncScroll"
        placeholder="輸入您的回覆... (輸入 @ 選擇文件或資料夾)"
        class="input-textarea"
        rows="4"
      />
    </div>

    <!-- 文件選擇器彈窗 - 移到 input-wrapper 外部，使用獨立定位 -->
    <div v-if="showFilePicker" class="file-picker-container">
      <div class="file-picker">
        <div class="file-picker-header">
          <input
            ref="searchRef"
            v-model="fileSearch"
            @keydown.esc="closeFilePicker"
            @keydown.enter="selectFirstFile"
            @keydown.down.prevent="moveSelection(1)"
            @keydown.up.prevent="moveSelection(-1)"
            placeholder="搜索文件或資料夾..."
            class="file-search"
          />
          <span class="picker-hint">↑↓ 選擇 · Enter 確認 · Esc 取消</span>
        </div>
        <div class="file-list" ref="fileListRef">
          <div
            v-for="(file, index) in filteredFiles"
            :key="file.path"
            :class="['file-item', { highlighted: index === highlightedIndex }]"
            @click="selectFile(file)"
            @mouseenter="highlightedIndex = index"
          >
            <span class="file-icon">{{ file.is_directory ? '📁' : '📄' }}</span>
            <span class="file-name">{{ file.name }}</span>
            <span v-if="file.is_directory" class="file-type-badge">資料夾</span>
          </div>
          <div v-if="filteredFiles.length === 0" class="no-files">
            沒有找到文件或資料夾
          </div>
        </div>
      </div>
    </div>

    <div class="input-info">
      <span class="char-count">{{ input.length }} 字符</span>
      <span v-if="selectedFilesCount > 0" class="selected-files">
        已選擇 {{ selectedFilesCount }} 個項目
      </span>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps({
  modelValue: {
    type: String,
    default: ''
  }
})

const emit = defineEmits(['update:modelValue'])

const input = ref(props.modelValue)
const textareaRef = ref(null)
const highlightRef = ref(null)
const searchRef = ref(null)
const wrapperRef = ref(null)
const fileListRef = ref(null)
const showFilePicker = ref(false)
const fileSearch = ref('')
const availableFiles = ref([])
const highlightedIndex = ref(0)
const cursorPosition = ref(0)

// 計算已選擇的文件數量
const selectedFilesCount = computed(() => {
  const matches = input.value.match(/@[\w\-\.\/]+/g)
  return matches ? matches.length : 0
})

// 產生高亮的 HTML 內容
const highlightedContent = computed(() => {
  if (!input.value) return ''

  // 轉義 HTML 並替換 @xxx 為高亮 span
  let content = input.value
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/\n/g, '<br>')

  // 高亮 @filename 模式
  content = content.replace(
    /@([\w\-\.\/]+)/g,
    '<span class="mention-highlight">@$1</span>'
  )

  // 保持末尾空格以匹配 textarea 高度
  if (content.endsWith('<br>')) {
    content += '&nbsp;'
  }

  return content
})

// 同步滾動
function syncScroll() {
  if (highlightRef.value && textareaRef.value) {
    highlightRef.value.scrollTop = textareaRef.value.scrollTop
    highlightRef.value.scrollLeft = textareaRef.value.scrollLeft
  }
}

watch(() => props.modelValue, (newValue) => {
  input.value = newValue
})

watch(input, (newValue) => {
  emit('update:modelValue', newValue)
})

function handleKeyDown(e) {
  if (e.key === '@') {
    cursorPosition.value = e.target.selectionStart
    openFilePicker()
  }
}

function handleInput() {
  // 檢測 @ 符號
  const lastChar = input.value[input.value.length - 1]
  if (lastChar === '@') {
    cursorPosition.value = input.value.length - 1
    openFilePicker()
  }
}

async function openFilePicker() {
  showFilePicker.value = true
  fileSearch.value = ''
  highlightedIndex.value = 0

  // 獲取文件列表
  try {
    availableFiles.value = await invoke('get_project_files')
  } catch (e) {
    console.error('獲取文件列表失敗:', e)
    availableFiles.value = []
  }

  // 聚焦搜索框
  await nextTick()
  if (searchRef.value) {
    searchRef.value.focus()
  }
}

function closeFilePicker() {
  showFilePicker.value = false
  fileSearch.value = ''
  highlightedIndex.value = 0

  // 刪除 @ 符號
  if (input.value.endsWith('@')) {
    input.value = input.value.slice(0, -1)
  }
}

function selectFile(file) {
  // 移除 @ 並插入文件名
  const beforeAt = input.value.slice(0, cursorPosition.value)
  const afterAt = input.value.slice(cursorPosition.value + 1)

  // 使用 @filename 格式，資料夾會加上 / 後綴
  const displayName = file.is_directory ? `@${file.name}/` : `@${file.name}`
  input.value = `${beforeAt}${displayName} ${afterAt}`

  closeFilePicker()
  textareaRef.value?.focus()
}

function selectFirstFile() {
  if (filteredFiles.value.length > 0) {
    selectFile(filteredFiles.value[highlightedIndex.value])
  }
}

function moveSelection(direction) {
  highlightedIndex.value += direction
  if (highlightedIndex.value < 0) {
    highlightedIndex.value = filteredFiles.value.length - 1
  } else if (highlightedIndex.value >= filteredFiles.value.length) {
    highlightedIndex.value = 0
  }
}

const filteredFiles = computed(() => {
  if (!fileSearch.value) return availableFiles.value.slice(0, 50)

  const search = fileSearch.value.toLowerCase()
  return availableFiles.value
    .filter(f => f.name.toLowerCase().includes(search) || f.path.toLowerCase().includes(search))
    .slice(0, 50)
})
</script>

<style scoped>
.user-input {
  margin: 1rem 0;
  display: flex;
  flex-direction: column;
}

.input-wrapper {
  position: relative;
}

/* 高亮覆蓋層 */
.highlight-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  padding: 0.75rem;
  border: 2px solid transparent;
  border-radius: 8px;
  font-size: 1rem;
  font-family: inherit;
  line-height: 1.5;
  white-space: pre-wrap;
  word-wrap: break-word;
  overflow: hidden;
  pointer-events: none;
  color: transparent;
  background: transparent;
}

.highlight-overlay :deep(.mention-highlight) {
  background: rgba(147, 51, 234, 0.2);
  border-radius: 4px;
  padding: 1px 2px;
  color: transparent;
}

.input-textarea {
  width: 100%;
  padding: 0.75rem;
  border: 2px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  font-size: 1rem;
  font-family: inherit;
  line-height: 1.5;
  resize: vertical;
  min-height: 100px;
  transition: border-color 0.2s ease;
  background: transparent;
  color: var(--text-primary, #333);
  position: relative;
  z-index: 1;
}

.input-textarea:focus {
  outline: none;
  border-color: var(--accent-color, #9333EA);
}

.input-textarea::placeholder {
  color: var(--text-secondary, #666);
}

.input-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 0.5rem;
  font-size: 0.85rem;
  color: var(--text-secondary, #666);
  gap: 0.5rem;
  flex-wrap: wrap;
}

.selected-files {
  color: var(--accent-color, #9333EA);
  font-weight: 500;
}

/* 文件選擇器容器 - 使用正常文檔流定位 */
.file-picker-container {
  margin-top: 0.5rem;
  width: 100%;
}

.file-picker {
  background: var(--bg-primary, white);
  border: 2px solid var(--accent-color, #9333EA);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  max-height: 250px;
}

.file-picker-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
  background: var(--bg-secondary, #f9f9f9);
}

.file-search {
  flex: 1;
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 6px;
  font-size: 0.9rem;
  outline: none;
  background: var(--bg-primary, white);
  color: var(--text-primary, #333);
  transition: border-color 0.2s ease;
}

.file-search:focus {
  border-color: var(--accent-color, #9333EA);
}

.file-search::placeholder {
  color: var(--text-secondary, #666);
}

.picker-hint {
  color: var(--accent-color, #9333EA);
  font-size: 0.75rem;
  font-weight: 500;
  white-space: nowrap;
  flex-shrink: 0;
}

.file-list {
  overflow-y: auto;
  max-height: 180px;
}

.file-item {
  padding: 0.6rem 0.75rem;
  cursor: pointer;
  transition: background 0.15s ease;
  font-size: 0.9rem;
  border-bottom: 1px solid var(--border-color, #f5f5f5);
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--text-primary, #333);
}

.file-icon {
  flex-shrink: 0;
  font-size: 1rem;
}

.file-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-type-badge {
  font-size: 0.7rem;
  background: var(--accent-color, #9333EA);
  color: white;
  padding: 2px 6px;
  border-radius: 10px;
  flex-shrink: 0;
}

.file-item:hover,
.file-item.highlighted {
  background: rgba(147, 51, 234, 0.1);
}

.file-item:last-child {
  border-bottom: none;
}

.no-files {
  padding: 1rem;
  text-align: center;
  color: var(--text-secondary, #999);
  font-size: 0.9rem;
}

/* Responsive Design - Tablet */
@media (max-width: 768px) {
  .input-textarea {
    font-size: 0.95rem;
    padding: 0.625rem;
  }

  .highlight-overlay {
    padding: 0.625rem;
    font-size: 0.95rem;
  }

  .input-info {
    font-size: 0.8rem;
  }

  .file-picker {
    max-height: 220px;
  }

  .file-picker-header {
    flex-wrap: wrap;
    gap: 0.4rem;
  }

  .file-search {
    font-size: 0.85rem;
    padding: 0.4rem 0.6rem;
  }

  .picker-hint {
    font-size: 0.7rem;
    width: 100%;
    text-align: center;
  }

  .file-list {
    max-height: 150px;
  }

  .file-item {
    font-size: 0.85rem;
    padding: 0.5rem 0.625rem;
  }
}

/* Responsive Design - Mobile */
@media (max-width: 480px) {
  .user-input {
    margin: 0.75rem 0;
  }

  .input-textarea {
    font-size: 0.9rem;
    padding: 0.5rem;
    min-height: 80px;
  }

  .highlight-overlay {
    padding: 0.5rem;
    font-size: 0.9rem;
  }

  .input-info {
    font-size: 0.75rem;
    flex-direction: column;
    align-items: flex-start;
  }

  .file-picker {
    max-height: 200px;
  }

  .file-picker-header {
    padding: 0.4rem;
  }

  .file-search {
    font-size: 0.8rem;
    padding: 0.4rem 0.5rem;
  }

  .file-list {
    max-height: 130px;
  }

  .file-item {
    font-size: 0.8rem;
    padding: 0.45rem 0.5rem;
  }

  .no-files {
    font-size: 0.8rem;
    padding: 0.75rem;
  }

  .picker-hint {
    font-size: 0.65rem;
  }
}
</style>
