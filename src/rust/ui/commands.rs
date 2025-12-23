use tauri::{command, AppHandle, Manager};
use crate::types::UserResponse;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    pub current: String,
    pub latest: Option<String>,
    pub has_update: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub name: String,
    pub is_directory: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectFilesResult {
    pub root: String,
    pub files: Vec<FileEntry>,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

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
pub async fn get_project_files() -> Result<Vec<FileEntry>, String> {
    // 獲取當前工作目錄
    let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;

    // 掃描文件和資料夾
    let mut entries = scan_directory(&current_dir, 3).map_err(|e| e.to_string())?;

    // 排序：資料夾在前，然後按名稱排序
    entries.sort_by(|a, b| {
        match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(entries)
}

#[command]
pub async fn get_project_files_with_root() -> Result<ProjectFilesResult, String> {
    // 獲取當前工作目錄
    let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;
    let root = current_dir.to_string_lossy().to_string();

    // 掃描文件和資料夾
    let mut entries = scan_directory(&current_dir, 3).map_err(|e| e.to_string())?;

    // 排序：資料夾在前，然後按名稱排序
    entries.sort_by(|a, b| {
        match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(ProjectFilesResult {
        root,
        files: entries,
    })
}

fn scan_directory(dir: &Path, max_depth: usize) -> std::io::Result<Vec<FileEntry>> {
    if max_depth == 0 {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();

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

        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                entries.push(FileEntry {
                    path: path_str.to_string(),
                    name,
                    is_directory: false,
                });
            }
        } else if path.is_dir() {
            // 先加入資料夾本身
            if let Some(path_str) = path.to_str() {
                entries.push(FileEntry {
                    path: path_str.to_string(),
                    name,
                    is_directory: true,
                });
            }
            // 遞迴掃描子目錄
            entries.extend(scan_directory(&path, max_depth - 1)?);
        }
    }

    Ok(entries)
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

#[command]
pub async fn check_latest_version() -> Result<VersionInfo, String> {
    let current_version = env!("CARGO_PKG_VERSION").to_string();

    // 建立 HTTP 客戶端
    let client = reqwest::Client::builder()
        .user_agent("claude-confirm/0.1.0")
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| format!("建立 HTTP 客戶端失敗: {}", e))?;

    // 獲取最新版本資訊
    match client
        .get("https://api.github.com/repos/zoonderkins/claude-confirm/releases/latest")
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();

            // 檢查 404 - 代表還沒有 release
            if status == reqwest::StatusCode::NOT_FOUND {
                return Ok(VersionInfo {
                    current: current_version,
                    latest: None,
                    has_update: false,
                    error: Some("GitHub 尚未發布任何 release 版本".to_string()),
                });
            }

            // 檢查其他錯誤狀態
            if !status.is_success() {
                return Ok(VersionInfo {
                    current: current_version,
                    latest: None,
                    has_update: false,
                    error: Some(format!("GitHub API 返回錯誤: {}", status)),
                });
            }

            match response.json::<GitHubRelease>().await {
                Ok(release) => {
                    let latest = release.tag_name.trim_start_matches('v').to_string();
                    let current = current_version.trim_start_matches('v');
                    let has_update = latest != current;

                    Ok(VersionInfo {
                        current: current_version,
                        latest: Some(latest),
                        has_update,
                        error: None,
                    })
                }
                Err(e) => Ok(VersionInfo {
                    current: current_version,
                    latest: None,
                    has_update: false,
                    error: Some(format!("解析 JSON 失敗: {}", e)),
                })
            }
        }
        Err(e) => Ok(VersionInfo {
            current: current_version,
            latest: None,
            has_update: false,
            error: Some(format!("無法連接到 GitHub: {}", e)),
        })
    }
}

#[command]
pub async fn open_github_repo() -> Result<(), String> {
    let url = "https://github.com/zoonderkins/claude-confirm";

    // 使用系統預設瀏覽器開啟 URL
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .map_err(|e| format!("開啟瀏覽器失敗: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|e| format!("開啟瀏覽器失敗: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "start", url])
            .spawn()
            .map_err(|e| format!("開啟瀏覽器失敗: {}", e))?;
    }

    Ok(())
}
