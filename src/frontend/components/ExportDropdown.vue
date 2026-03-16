<template>
  <div class="export-dropdown" ref="dropdownRef">
    <button @click="toggleDropdown" class="icon-btn" title="匯出">
      💾
    </button>

    <div v-if="isOpen" class="dropdown-menu">
      <button @click="handleExportAll" :disabled="exporting" class="dropdown-item export-all-item">
        <span class="item-icon">📦</span>
        <span>一鍵匯出全部</span>
      </button>
      <div class="dropdown-divider"></div>
      <button @click="handleExport('png')" :disabled="exporting" class="dropdown-item">
        <span class="item-icon">🖼️</span>
        <span>匯出 PNG</span>
      </button>
      <button @click="handleExport('pdf')" :disabled="exporting" class="dropdown-item">
        <span class="item-icon">📄</span>
        <span>匯出 PDF</span>
      </button>
      <button @click="handleExport('md')" :disabled="exporting" class="dropdown-item">
        <span class="item-icon">📝</span>
        <span>匯出 Markdown</span>
      </button>
    </div>

    <Teleport to="body">
      <div v-if="toast.show" class="export-toast" :class="toast.type">
        {{ toast.message }}
      </div>
    </Teleport>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { exportToPNG, exportToPDF, exportToMarkdown, exportAll } from '../utils/export'

const props = defineProps({
  targetElement: {
    type: Object,
    default: null
  },
  isDark: {
    type: Boolean,
    default: false
  },
  markdownContent: {
    type: String,
    default: ''
  },
  sections: {
    type: Array,
    default: () => []
  },
  envContext: {
    type: Object,
    default: null
  }
})

const isOpen = ref(false)
const exporting = ref(false)
const dropdownRef = ref(null)
const toast = ref({ show: false, message: '', type: 'success' })

function toggleDropdown() {
  isOpen.value = !isOpen.value
}

function closeDropdown(e) {
  if (dropdownRef.value && !dropdownRef.value.contains(e.target)) {
    isOpen.value = false
  }
}

function showToast(message, type = 'success') {
  toast.value = { show: true, message, type }
  setTimeout(() => {
    toast.value.show = false
  }, 3000)
}

async function handleExport(format) {
  // Markdown 不需要 targetElement
  if (format !== 'md' && !props.targetElement) return
  if (exporting.value) return

  exporting.value = true
  isOpen.value = false

  try {
    let filename
    if (format === 'png') {
      filename = await exportToPNG(props.targetElement, props.isDark, props.envContext)
    } else if (format === 'pdf') {
      filename = await exportToPDF(props.targetElement, props.isDark, props.envContext)
    } else if (format === 'md') {
      filename = await exportToMarkdown(props.markdownContent, props.sections, props.envContext)
    }
    showToast(`已儲存至 ~/Downloads/${filename}`, 'success')
  } catch (e) {
    console.error('匯出失敗:', e)
    showToast(`匯出失敗: ${e}`, 'error')
  } finally {
    exporting.value = false
  }
}

async function handleExportAll() {
  if (!props.targetElement) return
  if (exporting.value) return

  exporting.value = true
  isOpen.value = false

  try {
    const filenames = await exportAll(
      props.targetElement,
      props.isDark,
      props.markdownContent,
      props.sections,
      props.envContext
    )
    showToast(`已匯出 ${filenames.length} 個檔案至 ~/Downloads/`, 'success')
  } catch (e) {
    console.error('一鍵匯出失敗:', e)
    showToast(`匯出失敗: ${e}`, 'error')
  } finally {
    exporting.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', closeDropdown)
})

onUnmounted(() => {
  document.removeEventListener('click', closeDropdown)
})
</script>

<style scoped>
.export-dropdown {
  position: relative;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 0.5rem;
  background: var(--bg-primary, #fff);
  border: 1px solid var(--border-color, #e5e7eb);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 100;
  min-width: 150px;
  overflow: hidden;
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  width: 100%;
  padding: 0.75rem 1rem;
  background: transparent;
  border: none;
  color: var(--text-primary, #1f2937);
  cursor: pointer;
  font-size: 0.9rem;
  transition: background 0.2s ease;
}

.dropdown-item:hover:not(:disabled) {
  background: var(--accent-light, rgba(147, 51, 234, 0.1));
}

.dropdown-item:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.export-all-item {
  font-weight: 600;
  color: var(--accent-color, #9333EA);
}

.dropdown-divider {
  height: 1px;
  background: var(--border-color, #e5e7eb);
  margin: 0.25rem 0;
}

.item-icon {
  font-size: 1.1rem;
}

.export-toast {
  position: fixed;
  bottom: 2rem;
  left: 50%;
  transform: translateX(-50%);
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  font-size: 0.9rem;
  z-index: 1000;
  animation: slideUp 0.3s ease;
  white-space: nowrap;
}

.export-toast.success {
  background: #10b981;
  color: white;
}

.export-toast.error {
  background: #ef4444;
  color: white;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateX(-50%) translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
}
</style>
