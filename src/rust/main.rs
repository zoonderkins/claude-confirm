use claude_confirm::ui::run_ui_app;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    run_ui_app();
}

fn main() {
    // 直接啟動 UI，CLI 參數由 Tauri commands 處理
    run();
}
