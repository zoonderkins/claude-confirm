import html2canvas from 'html2canvas'
import { jsPDF } from 'jspdf'
import { invoke } from '@tauri-apps/api/core'

/**
 * 將 DOM 元素轉換為 Canvas
 */
async function captureElementAsCanvas(element, isDark) {
  const bgColor = isDark ? '#1f2937' : '#ffffff'

  return await html2canvas(element, {
    backgroundColor: bgColor,
    scale: 2,
    useCORS: true,
    logging: false,
    onclone: (clonedDoc, clonedElement) => {
      clonedElement.style.backgroundColor = bgColor
      clonedElement.style.padding = '1rem'
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
 * 匯出為 PDF
 */
export async function exportToPDF(element, isDark) {
  const canvas = await captureElementAsCanvas(element, isDark)

  const imgWidth = 210
  const imgHeight = (canvas.height * imgWidth) / canvas.width

  const pdf = new jsPDF({
    orientation: imgHeight > imgWidth ? 'portrait' : 'landscape',
    unit: 'mm',
    format: 'a4'
  })

  const imgData = canvas.toDataURL('image/png')
  pdf.addImage(imgData, 'PNG', 0, 0, imgWidth, Math.min(imgHeight, 297))

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
