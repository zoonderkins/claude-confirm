<template>
  <div v-if="isVisible" class="dialog-overlay" @click="close">
    <div class="dialog-content" @click.stop>
      <div class="dialog-header">
        <h3>關於 Claude Confirm</h3>
        <button @click="close" class="close-btn">✕</button>
      </div>

      <div class="dialog-body">
        <div class="logo">🤖</div>

        <div class="info-section">
          <div class="info-row">
            <span class="label">目前版本：</span>
            <span class="value">{{ versionInfo.current }}</span>
          </div>

          <div v-if="loading" class="info-row">
            <span class="loading-text">正在檢查更新...</span>
          </div>

          <div v-else-if="versionInfo.latest" class="info-row">
            <span class="label">最新版本：</span>
            <span class="value">{{ versionInfo.latest }}</span>
            <span v-if="versionInfo.has_update" class="badge update-badge">⚠️ 有更新</span>
            <span v-else class="badge latest-badge">✅ 已是最新</span>
          </div>

          <div v-else-if="versionInfo.error" class="info-row error">
            <span class="error-text">{{ versionInfo.error }}</span>
          </div>
        </div>

        <div class="actions">
          <button @click="openGitHub" class="btn btn-primary">
            <span class="icon">🔗</span>
            開啟 GitHub
          </button>
          <button @click="checkVersion" class="btn btn-secondary" :disabled="loading">
            <span class="icon">🔄</span>
            重新檢查
          </button>
        </div>

        <div class="footer-text">
          一個為 Claude Code CLI 打造的互動確認工具
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['update:visible'])

const isVisible = ref(props.visible)
const loading = ref(false)
const versionInfo = ref({
  current: '0.1.0',
  latest: null,
  has_update: false,
  error: null
})

// 監聽 visible 變化
watch(() => props.visible, (newVal) => {
  isVisible.value = newVal
  if (newVal) {
    checkVersion()
  }
})

async function checkVersion() {
  loading.value = true
  try {
    const info = await invoke('check_latest_version')
    versionInfo.value = info
  } catch (e) {
    versionInfo.value.error = `檢查版本失敗: ${e}`
  } finally {
    loading.value = false
  }
}

async function openGitHub() {
  try {
    await invoke('open_github_repo')
  } catch (e) {
    console.error('開啟 GitHub 失敗:', e)
  }
}

function close() {
  isVisible.value = false
  emit('update:visible', false)
}
</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.dialog-content {
  background: var(--bg-primary, #fff);
  border-radius: 12px;
  width: 90%;
  max-width: 450px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  animation: slideUp 0.3s ease;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.25rem 1.5rem;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.dialog-header h3 {
  margin: 0;
  font-size: 1.2rem;
  font-weight: 600;
  color: var(--text-primary, #333);
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--text-secondary, #666);
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: var(--bg-secondary, #f0f0f0);
  color: var(--text-primary, #333);
}

.dialog-body {
  padding: 2rem 1.5rem;
  text-align: center;
}

.logo {
  font-size: 4rem;
  margin-bottom: 1.5rem;
}

.info-section {
  margin: 1.5rem 0;
  text-align: left;
}

.info-row {
  display: flex;
  align-items: center;
  padding: 0.75rem 0;
  border-bottom: 1px solid var(--border-color, #f0f0f0);
  gap: 0.5rem;
}

.info-row:last-child {
  border-bottom: none;
}

.label {
  font-weight: 500;
  color: var(--text-secondary, #666);
  min-width: 90px;
}

.value {
  color: var(--text-primary, #333);
  font-family: 'Monaco', 'Courier New', monospace;
  font-weight: 600;
}

.loading-text {
  color: var(--text-secondary, #666);
  font-style: italic;
}

.error {
  border-color: #ff5252;
}

.error-text {
  color: #ff5252;
  font-size: 0.9rem;
}

.badge {
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.85rem;
  font-weight: 500;
  margin-left: auto;
}

.update-badge {
  background: #fff3cd;
  color: #856404;
}

.latest-badge {
  background: #d4edda;
  color: #155724;
}

.actions {
  display: flex;
  gap: 0.75rem;
  margin: 1.5rem 0 1rem;
}

.btn {
  flex: 1;
  padding: 0.75rem 1rem;
  border: none;
  border-radius: 8px;
  font-size: 0.95rem;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  transition: all 0.2s ease;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--accent-color, #4CAF50);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover, #45a049);
  box-shadow: 0 2px 8px rgba(76, 175, 80, 0.3);
}

.btn-secondary {
  background: var(--bg-secondary, #f0f0f0);
  color: var(--text-primary, #333);
  border: 1px solid var(--border-color, #e0e0e0);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--border-color, #e0e0e0);
}

.icon {
  font-size: 1.1rem;
}

.footer-text {
  color: var(--text-secondary, #999);
  font-size: 0.85rem;
  margin-top: 1rem;
  line-height: 1.5;
}

/* Dark mode support */
.dark .dialog-content {
  --bg-primary: #2d2d2d;
  --bg-secondary: #3d3d3d;
  --text-primary: #e0e0e0;
  --text-secondary: #b0b0b0;
  --border-color: #404040;
}
</style>
