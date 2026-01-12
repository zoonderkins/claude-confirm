import html2canvas from 'html2canvas'
import { jsPDF } from 'jspdf'
import { invoke } from '@tauri-apps/api/core'

/**
 * 將 DOM 元素轉換為 Canvas（包含完整可滾動內容）
 */
async function captureElementAsCanvas(element, isDark) {
  const bgColor = isDark ? '#1f2937' : '#ffffff'

  // 取得完整內容高度（包含滾動區域）
  const fullHeight = element.scrollHeight
  const fullWidth = element.scrollWidth

  return await html2canvas(element, {
    backgroundColor: bgColor,
    scale: 2,
    useCORS: true,
    logging: false,
    height: fullHeight,
    width: fullWidth,
    windowHeight: fullHeight,
    windowWidth: fullWidth,
    scrollY: -window.scrollY,
    scrollX: -window.scrollX,
    onclone: (clonedDoc, clonedElement) => {
      // 移除 overflow 限制，展開完整內容
      clonedElement.style.backgroundColor = bgColor
      clonedElement.style.padding = '1rem'
      clonedElement.style.overflow = 'visible'
      clonedElement.style.height = 'auto'
      clonedElement.style.maxHeight = 'none'
    }
  })
}

/**
 * 清理專案名稱，移除不適合檔名的字元
 */
function sanitizeProjectName(name) {
  if (!name) return null
  // 移除或替換不適合檔名的字元
  return name
    .replace(/[<>:"/\\|?*]/g, '-')  // 替換 Windows 禁用字元
    .replace(/\s+/g, '-')           // 空格改為連字號
    .replace(/-+/g, '-')            // 多個連字號合併
    .replace(/^-|-$/g, '')          // 移除首尾連字號
    .slice(0, 50)                   // 限制長度
}

/**
 * 產生檔名
 * @param {string} ext - 檔案副檔名
 * @param {Object} envContext - 環境上下文
 * @param {string} envContext.project_name - 專案名稱
 * @param {string} envContext.cwd - 工作目錄
 */
function generateFilename(ext, envContext = null) {
  const now = new Date()
  const date = now.toISOString().slice(0, 10)
  const time = now.toTimeString().slice(0, 8).replace(/:/g, '')
  
  // 優先使用 project_name，否則從 cwd 提取
  let projectName = envContext?.project_name
  if (!projectName && envContext?.cwd) {
    const parts = envContext.cwd.split(/[/\\]/)
    projectName = parts[parts.length - 1]
  }
  
  const sanitized = sanitizeProjectName(projectName)
  
  if (sanitized) {
    return `claude-confirm-${sanitized}-${date}-${time}.${ext}`
  }
  return `claude-confirm-${date}-${time}.${ext}`
}

/**
 * 匯出為 PNG
 * @param {HTMLElement} element - 要截圖的元素
 * @param {boolean} isDark - 是否深色模式
 * @param {Object} envContext - 環境上下文
 */
export async function exportToPNG(element, isDark, envContext = null) {
  const canvas = await captureElementAsCanvas(element, isDark)
  const dataUrl = canvas.toDataURL('image/png')

  const base64Data = dataUrl.replace(/^data:image\/png;base64,/, '')
  const filename = generateFilename('png', envContext)

  await invoke('save_export_file', {
    filename,
    data: base64Data,
    fileType: 'png'
  })

  return filename
}

/**
 * 匯出為 PDF（支援多頁）
 * @param {HTMLElement} element - 要截圖的元素
 * @param {boolean} isDark - 是否深色模式
 * @param {Object} envContext - 環境上下文
 */
export async function exportToPDF(element, isDark, envContext = null) {
  const canvas = await captureElementAsCanvas(element, isDark)

  const imgWidth = 210 // A4 寬度 mm
  const pageHeight = 297 // A4 高度 mm
  const imgHeight = (canvas.height * imgWidth) / canvas.width

  const pdf = new jsPDF({
    orientation: 'portrait',
    unit: 'mm',
    format: 'a4'
  })

  const imgData = canvas.toDataURL('image/png')

  // 計算需要多少頁
  let heightLeft = imgHeight
  let position = 0

  // 第一頁
  pdf.addImage(imgData, 'PNG', 0, position, imgWidth, imgHeight)
  heightLeft -= pageHeight

  // 添加額外頁面
  while (heightLeft > 0) {
    position = heightLeft - imgHeight
    pdf.addPage()
    pdf.addImage(imgData, 'PNG', 0, position, imgWidth, imgHeight)
    heightLeft -= pageHeight
  }

  const pdfOutput = pdf.output('datauristring')
  const base64Data = pdfOutput.split(',')[1]

  const filename = generateFilename('pdf', envContext)

  await invoke('save_export_file', {
    filename,
    data: base64Data,
    fileType: 'pdf'
  })

  return filename
}

/**
 * 匯出為 Markdown
 * @param {string} markdownContent - Markdown 內容
 * @param {Array} sections - 段落列表
 * @param {Object} envContext - 環境上下文
 */
export async function exportToMarkdown(markdownContent, sections = [], envContext = null) {
  // 組合完整的 markdown 內容
  let fullContent = markdownContent

  // 如果有 sections，附加到末尾
  if (sections && sections.length > 0) {
    fullContent += '\n\n---\n\n## Sections\n\n'
    sections.forEach((section) => {
      const status = section.selected ? '✅' : '⬜'
      fullContent += `### ${status} ${section.title}\n\n${section.content}\n\n`
    })
  }

  // 轉換為 base64
  const base64Data = btoa(unescape(encodeURIComponent(fullContent)))
  const filename = generateFilename('md', envContext)

  await invoke('save_export_file', {
    filename,
    data: base64Data,
    fileType: 'md'
  })

  return filename
}
