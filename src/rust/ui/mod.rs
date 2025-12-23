mod commands;

pub use commands::*;

use tauri::{Manager, WindowEvent};

pub fn run_ui_app() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::submit_response,
            commands::cancel_response,
            commands::get_project_files,
            commands::get_project_files_with_root,
            commands::get_cli_args,
            commands::read_mcp_request,
            commands::set_always_on_top,
            commands::open_devtools,
            commands::check_latest_version,
            commands::open_github_repo,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let _ = window.show();
            let _ = window.set_focus();

            Ok(())
        })
        .on_window_event(|_window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                // 窗口關閉時，輸出取消回應
                let response = crate::types::UserResponse::cancelled();
                println!("{}", serde_json::to_string(&response).unwrap());
                std::process::exit(0);
            }
        })
        .run(tauri::generate_context!())
        .expect("運行 Tauri 應用失敗");
}
