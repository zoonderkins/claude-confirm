use tauri::{command, AppHandle, Manager};
use crate::types::UserResponse;
use std::path::Path;

#[command]
pub async fn submit_response(response: UserResponse) -> Result<(), String> {
    // 將結果輸出到 stdout
    println!("{}", serde_json::to_string(&response).map_err(|e| e.to_string())?);

    // 退出程序
    std::process::exit(0);
}

#[command]
pub async fn cancel_response() -> Result<(), String> {
    let response = UserResponse::cancelled();
    println!("{}", serde_json::to_string(&response).map_err(|e| e.to_string())?);
    std::process::exit(0);
}

#[command]
pub fn get_cli_args() -> Result<serde_json::Value, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut result = serde_json::Map::new();

    // 檢查是否有 --mcp-request 參數
    if args.len() >= 3 && args[1] == "--mcp-request" {
        result.insert(
            "mcp_request".to_string(),
            serde_json::Value::String(args[2].clone()),
        );
    }

    Ok(serde_json::Value::Object(result))
}

#[command]
pub fn read_mcp_request(file_path: String) -> Result<serde_json::Value, String> {
    if !std::path::Path::new(&file_path).exists() {
        return Err(format!("文件不存在: {}", file_path));
    }

    match std::fs::read_to_string(&file_path) {
        Ok(content) => {
            if content.trim().is_empty() {
                return Err("文件內容為空".to_string());
            }
            match serde_json::from_str(&content) {
                Ok(json) => Ok(json),
                Err(e) => Err(format!("解析 JSON 失敗: {}", e)),
            }
        }
        Err(e) => Err(format!("讀取文件失敗: {}", e)),
    }
}

#[command]
pub async fn get_project_files() -> Result<Vec<String>, String> {
    // 獲取當前工作目錄
    let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;

    // 掃描文件
    let files = scan_directory(&current_dir, 3).map_err(|e| e.to_string())?;

    Ok(files)
}

fn scan_directory(dir: &Path, max_depth: usize) -> std::io::Result<Vec<String>> {
    if max_depth == 0 {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // 跳過隱藏目錄和常見的忽略目錄
        if let Some(name) = path.file_name() {
            let name_str = name.to_string_lossy();
            if name_str.starts_with('.') ||
               name_str == "node_modules" ||
               name_str == "target" ||
               name_str == "dist" ||
               name_str == "build" {
                continue;
            }
        }

        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                files.push(path_str.to_string());
            }
        } else if path.is_dir() {
            files.extend(scan_directory(&path, max_depth - 1)?);
        }
    }

    Ok(files)
}

#[command]
pub async fn set_always_on_top(app_handle: AppHandle, always_on_top: bool) -> Result<(), String> {
    let window = app_handle.get_webview_window("main")
        .ok_or("找不到主窗口")?;

    window.set_always_on_top(always_on_top)
        .map_err(|e| format!("設定置頂失敗: {}", e))?;

    Ok(())
}

#[command]
pub async fn open_devtools(app_handle: AppHandle) -> Result<(), String> {
    let window = app_handle.get_webview_window("main")
        .ok_or("找不到主窗口")?;

    window.open_devtools();
    Ok(())
}
