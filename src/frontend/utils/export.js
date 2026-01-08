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
 * 產生檔名
 */
function generateFilename(ext) {
  const now = new Date()
  const date = now.toISOString().slice(0, 10)
  const time = now.toTimeString().slice(0, 8).replace(/:/g, '')
  return `claude-confirm-${date}-${time}.${ext}`
}

/**
 * 匯出為 PNG
 */
export async function exportToPNG(element, isDark) {
  const canvas = await captureElementAsCanvas(element, isDark)
  const dataUrl = canvas.toDataURL('image/png')

  const base64Data = dataUrl.replace(/^data:image\/png;base64,/, '')
  const filename = generateFilename('png')

  await invoke('save_export_file', {
    filename,
    data: base64Data,
    fileType: 'png'
  })

  return filename
}

/**
 * 匯出為 PDF（支援多頁）
 */
export async function exportToPDF(element, isDark) {
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

  const filename = generateFilename('pdf')

  await invoke('save_export_file', {
    filename,
    data: base64Data,
    fileType: 'pdf'
  })

  return filename
}
