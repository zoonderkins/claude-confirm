<template>
  <div class="user-input">
    <div class="input-wrapper">
      <textarea
        ref="textareaRef"
        v-model="input"
        @keydown="handleKeyDown"
        @input="handleInput"
        placeholder="輸入您的回覆... (輸入 @ 選擇文件)"
        class="input-textarea"
        rows="4"
      />

      <!-- 文件選擇器彈窗 -->
      <div v-if="showFilePicker" class="file-picker" :style="pickerStyle">
        <input
          ref="searchRef"
          v-model="fileSearch"
          @keydown.esc="closeFilePicker"
          @keydown.enter="selectFirstFile"
          @keydown.down.prevent="moveSelection(1)"
          @keydown.up.prevent="moveSelection(-1)"
          placeholder="搜索文件..."
          class="file-search"
        />
        <div class="file-list">
          <div
            v-for="(file, index) in filteredFiles"
            :key="file"
            :class="['file-item', { highlighted: index === highlightedIndex }]"
            @click="selectFile(file)"
            @mouseenter="highlightedIndex = index"
          >
            {{ getFileName(file) }}
          </div>
          <div v-if="filteredFiles.length === 0" class="no-files">
            沒有找到文件
          </div>
        </div>
      </div>
    </div>

    <div class="input-info">
      <span class="char-count">{{ input.length }} 字符</span>
      <span v-if="showFilePicker" class="picker-hint">
        ↑↓ 選擇 · Enter 確認 · Esc 取消
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
const searchRef = ref(null)
const showFilePicker = ref(false)
const fileSearch = ref('')
const availableFiles = ref([])
const highlightedIndex = ref(0)
const pickerStyle = ref({})
const cursorPosition = ref(0)

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

  // 計算彈窗位置
  await nextTick()
  const textarea = textareaRef.value
  if (textarea) {
    const rect = textarea.getBoundingClientRect()
    pickerStyle.value = {
      top: `${rect.bottom + 5}px`,
      left: `${rect.left}px`,
      width: `${rect.width}px`
    }
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

  input.value = `${beforeAt}${getFileName(file)} ${afterAt}`

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

function getFileName(path) {
  return path.split('/').pop() || path
}

const filteredFiles = computed(() => {
  if (!fileSearch.value) return availableFiles.value.slice(0, 50)

  const search = fileSearch.value.toLowerCase()
  return availableFiles.value
    .filter(f => f.toLowerCase().includes(search))
    .slice(0, 50)
})
</script>

<style scoped>
.user-input {
  margin: 1rem 0;
}

.input-wrapper {
  position: relative;
}

.input-textarea {
  width: 100%;
  padding: 0.75rem;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  font-size: 1rem;
  font-family: inherit;
  resize: vertical;
  min-height: 100px;
  transition: border-color 0.2s ease;
}

.input-textarea:focus {
  outline: none;
  border-color: #4CAF50;
}

.input-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 0.5rem;
  font-size: 0.85rem;
  color: #666;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.picker-hint {
  color: #4CAF50;
  font-weight: 500;
}

.file-picker {
  position: fixed;
  background: white;
  border: 2px solid #4CAF50;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  max-height: 300px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.file-search {
  width: 100%;
  padding: 0.75rem;
  border: none;
  border-bottom: 1px solid #e0e0e0;
  font-size: 0.9rem;
  outline: none;
}

.file-list {
  overflow-y: auto;
  max-height: 250px;
}

.file-item {
  padding: 0.75rem;
  cursor: pointer;
  transition: background 0.15s ease;
  font-size: 0.9rem;
  border-bottom: 1px solid #f5f5f5;
  word-wrap: break-word;
}

.file-item:hover,
.file-item.highlighted {
  background: #f1f8f4;
}

.no-files {
  padding: 1rem;
  text-align: center;
  color: #999;
  font-size: 0.9rem;
}

/* Responsive Design - Tablet */
@media (max-width: 768px) {
  .input-textarea {
    font-size: 0.95rem;
    padding: 0.625rem;
  }

  .input-info {
    font-size: 0.8rem;
  }

  .file-picker {
    max-height: 250px;
  }

  .file-search {
    font-size: 0.85rem;
    padding: 0.625rem;
  }

  .file-item {
    font-size: 0.85rem;
    padding: 0.625rem;
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

  .input-info {
    font-size: 0.75rem;
    flex-direction: column;
    align-items: flex-start;
  }

  .file-picker {
    left: 0.5rem !important;
    right: 0.5rem;
    width: calc(100% - 1rem) !important;
    max-height: 60vh;
  }

  .file-search {
    font-size: 0.8rem;
    padding: 0.5rem;
  }

  .file-list {
    max-height: calc(60vh - 3rem);
  }

  .file-item {
    font-size: 0.8rem;
    padding: 0.5rem;
  }

  .no-files {
    font-size: 0.8rem;
  }

  .picker-hint {
    font-size: 0.7rem;
  }
}
</style>
