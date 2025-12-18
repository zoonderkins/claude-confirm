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
  color: var(--text-primary, #1f2937);
}

.markdown-viewer :deep(h1) {
  font-size: 2em;
  margin: 0.67em 0;
  font-weight: bold;
  color: var(--text-primary, #1f2937);
}

.markdown-viewer :deep(h2) {
  font-size: 1.5em;
  margin: 0.75em 0;
  font-weight: bold;
  color: var(--text-primary, #1f2937);
}

.markdown-viewer :deep(h3) {
  font-size: 1.17em;
  margin: 0.83em 0;
  font-weight: bold;
  color: var(--text-primary, #1f2937);
}

.markdown-viewer :deep(code) {
  background: var(--accent-light, rgba(147, 51, 234, 0.1));
  color: var(--accent-color, #9333EA);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'SF Mono', 'Fira Code', 'Courier New', monospace;
  font-size: 0.9em;
}

.markdown-viewer :deep(pre) {
  background: #1e1e2e;
  padding: 1rem;
  border-radius: 8px;
  overflow-x: auto;
  margin: 1em 0;
  border: 1px solid var(--border-color, #e5e7eb);
}

.markdown-viewer :deep(pre code) {
  background: transparent;
  padding: 0;
  color: #cdd6f4;
}

.markdown-viewer :deep(ul), .markdown-viewer :deep(ol) {
  padding-left: 2em;
  margin: 1em 0;
  color: var(--text-primary, #1f2937);
}

.markdown-viewer :deep(li) {
  margin: 0.5em 0;
}

.markdown-viewer :deep(blockquote) {
  border-left: 4px solid var(--accent-color, #9333EA);
  padding-left: 1em;
  margin: 1em 0;
  color: var(--text-secondary, #6b7280);
  background: var(--accent-light, rgba(147, 51, 234, 0.05));
  padding: 0.5em 1em;
  border-radius: 0 8px 8px 0;
}

.markdown-viewer :deep(a) {
  color: var(--accent-color, #9333EA);
  text-decoration: none;
  font-weight: 500;
}

.markdown-viewer :deep(a:hover) {
  text-decoration: underline;
  color: var(--accent-hover, #7C3AED);
}

.markdown-viewer :deep(p) {
  margin: 0.75em 0;
  color: var(--text-primary, #1f2937);
}

.markdown-viewer :deep(strong) {
  color: var(--text-primary, #1f2937);
  font-weight: 600;
}

.markdown-viewer :deep(em) {
  color: var(--text-secondary, #6b7280);
}

.markdown-viewer :deep(hr) {
  border: none;
  border-top: 1px solid var(--border-color, #e5e7eb);
  margin: 1.5em 0;
}

.markdown-viewer :deep(table) {
  width: 100%;
  border-collapse: collapse;
  margin: 1em 0;
}

.markdown-viewer :deep(th),
.markdown-viewer :deep(td) {
  border: 1px solid var(--border-color, #e5e7eb);
  padding: 0.5em 1em;
  text-align: left;
}

.markdown-viewer :deep(th) {
  background: var(--bg-secondary, #f9fafb);
  font-weight: 600;
  color: var(--text-primary, #1f2937);
}

.markdown-viewer :deep(td) {
  color: var(--text-primary, #1f2937);
}
</style>
