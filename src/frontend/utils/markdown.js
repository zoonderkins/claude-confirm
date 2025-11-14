import MarkdownIt from 'markdown-it'
import hljs from 'highlight.js'

// 創建單例 MarkdownIt 實例，避免重複創建
export const markdownRenderer = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
  highlight: (str, lang) => {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return `<pre class="hljs"><code>${hljs.highlight(str, { language: lang }).value}</code></pre>`
      } catch (__) {
        // 忽略錯誤，使用預設處理
      }
    }
    return `<pre class="hljs"><code>${markdownRenderer.utils.escapeHtml(str)}</code></pre>`
  }
})
