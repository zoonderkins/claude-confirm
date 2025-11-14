<template>
  <div class="markdown-viewer" v-html="renderedHtml"></div>
</template>

<script setup>
import { computed } from 'vue'
import { markdownRenderer } from '../utils/markdown'
import 'highlight.js/styles/github-dark.css'

const props = defineProps({
  content: {
    type: String,
    required: true
  }
})

const renderedHtml = computed(() => markdownRenderer.render(props.content))
</script>

<style scoped>
.markdown-viewer {
  padding: 1rem;
  line-height: 1.6;
}

.markdown-viewer :deep(h1) {
  font-size: 2em;
  margin: 0.67em 0;
  font-weight: bold;
}

.markdown-viewer :deep(h2) {
  font-size: 1.5em;
  margin: 0.75em 0;
  font-weight: bold;
}

.markdown-viewer :deep(h3) {
  font-size: 1.17em;
  margin: 0.83em 0;
  font-weight: bold;
}

.markdown-viewer :deep(code) {
  background: #f4f4f4;
  padding: 2px 6px;
  border-radius: 3px;
  font-family: 'Courier New', monospace;
}

.markdown-viewer :deep(pre) {
  background: #1e1e1e;
  padding: 1rem;
  border-radius: 6px;
  overflow-x: auto;
  margin: 1em 0;
}

.markdown-viewer :deep(pre code) {
  background: transparent;
  padding: 0;
  color: #d4d4d4;
}

.markdown-viewer :deep(ul), .markdown-viewer :deep(ol) {
  padding-left: 2em;
  margin: 1em 0;
}

.markdown-viewer :deep(li) {
  margin: 0.5em 0;
}

.markdown-viewer :deep(blockquote) {
  border-left: 4px solid #ddd;
  padding-left: 1em;
  margin: 1em 0;
  color: #666;
}

.markdown-viewer :deep(a) {
  color: #0066cc;
  text-decoration: none;
}

.markdown-viewer :deep(a:hover) {
  text-decoration: underline;
}
</style>
