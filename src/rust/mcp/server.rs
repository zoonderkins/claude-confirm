use anyhow::Result;
use rmcp::{
    ErrorData as McpError, ServerHandler, ServiceExt, RoleServer,
    model::*,
    transport::stdio,
    service::RequestContext,
};
use std::sync::Arc;
use std::process::Command;
use std::fs;

use super::types::ConfirmRequest;
use crate::types::{PopupRequest, UserResponse};

#[derive(Clone)]
pub struct ConfirmServer;

impl ServerHandler for ConfirmServer {
    fn get_info(&self) -> ServerInfo {
        let mut server_impl = Implementation::new(
            "claude-confirm",
            env!("CARGO_PKG_VERSION"),
        );
        server_impl.title = Some("Claude Confirm MCP Server".to_string());

        ServerInfo::new(
            ServerCapabilities::builder().enable_tools().build(),
        )
        .with_server_info(server_impl)
        .with_instructions(
            "Claude Confirm - 互動確認工具\n\n\
            ⚠️ 重要：在完成以下任何操作後，請主動調用 confirm 工具：\n\
            - 完成多步驟任務\n\
            - 修改多個文件\n\
            - 完成重要的配置更改\n\
            - 執行構建或測試\n\
            - 完成問題診斷和修復\n\
            - 完成代碼重構\n\
            - 總結工作成果時\n\n\
            使用方式：\n\
            1. 將工作成果整理成 Markdown 格式\n\
            2. 將相關內容分段（sections）讓用戶選擇\n\
            3. 調用 confirm 工具展示給用戶\n\
            4. 獲取用戶的確認、選擇和額外輸入\n\n\
            ⚠️ sections 使用規範（必讀！）：\n\
            sections 的正確用途是提供「後續可選的改進項目」，而非已完成的項目。\n\n\
            **正確做法**：\n\
            - message: 總結已完成的工作（✅ 已完成 A、B、C）\n\
            - sections: 列出可選的後續任務（例如：添加測試、優化性能、修復安全問題）\n\
            - 每個 section 必須是明確、可立即執行的任務\n\n\
            **錯誤做法**：\n\
            - ❌ sections 放已完成的項目\n\
            - ❌ sections 內容模糊不清（如「可能需要...」）\n\
            - ❌ sections 是討論性問題而非可執行任務\n\n\
            範例（正確）：\n\
            message: '✅ 已完成功能 A 和 B\\n\\n以下是後續可選改進：'\n\
            sections: [\n\
              {title: '🔴 修復 XSS 漏洞', content: '在 MarkdownViewer 添加 DOMPurify', selected: false},\n\
              {title: '🟠 添加 TypeScript', content: '將 .js 改為 .ts 並添加類型', selected: false}\n\
            ]\n\n\
            ⚠️ 返回值處理規則（重要！）：\n\
            當用戶確認後，你會收到以下格式的返回：\n\
            - '選中的段落: [index_array]' - 用戶最終選擇的段落索引數組（從 0 開始）\n\
            - '用戶輸入: string' - 用戶的額外文字輸入\n\n\
            **必須嚴格遵守**：\n\
            1. 只處理 index_array 中的項目，不要添加用戶未選擇的項目\n\
            2. 不要按照「你認為重要的優先級」自行決定任務\n\
            3. 用戶選擇後，立即執行該任務，不要再詢問「要實作還是給建議」\n\
            4. 結合用戶輸入來理解額外需求\n\n\
            範例：\n\
            假設你發送了 5 個 sections（索引 0-4），用戶只選了 [1, 3]：\n\
            → ✅ 正確：立即開始執行索引 1 和 3 的任務\n\
            → ❌ 錯誤：詢問用戶「要不要做」或「怎麼做」\n\
            → ❌ 錯誤：處理索引 0、2、4 或其他未選項目\n\n\
            範例：完成任務後調用 confirm，總結修改的文件、解決的問題、測試結果等。"
        )
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ServerInfo, McpError> {
        Ok(self.get_info())
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "要顯示的訊息（支援 Markdown）"
                },
                "sections": {
                    "type": "array",
                    "description": "可選的段落列表",
                    "items": {
                        "type": "object",
                        "properties": {
                            "title": { "type": "string" },
                            "content": { "type": "string" },
                            "selected": { "type": "boolean" }
                        },
                        "required": ["title", "content"]
                    }
                },
                "is_markdown": {
                    "type": "boolean",
                    "description": "是否為 Markdown 格式（預設 true）"
                }
            },
            "required": ["message"]
        });

        let tool = if let serde_json::Value::Object(schema_map) = schema {
            Tool::new(
                "confirm",
                "⚠️ 在完成任務、修改文件、構建測試後主動調用此工具。\n\
                用於：展示 Markdown 格式的工作總結、讓用戶選擇相關段落、獲取確認和額外輸入。\n\
                自動觸發時機：多步驟任務完成、重要更改完成、問題解決後、代碼重構後。",
                Arc::new(schema_map),
            )
        } else {
            return Err(McpError::internal_error(
                "無法創建工具 schema".to_string(),
                None
            ));
        };

        Ok(ListToolsResult::with_all_items(vec![tool]))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        match request.name.as_ref() {
            "confirm" => {
                let arguments_value = request
                    .arguments
                    .map(serde_json::Value::Object)
                    .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

                let confirm_request: ConfirmRequest =
                    serde_json::from_value(arguments_value).map_err(|e| {
                        McpError::invalid_params(format!("參數解析失敗: {}", e), None)
                    })?;

                self.handle_confirm(confirm_request).await
            }
            _ => Err(McpError::invalid_request(
                format!("未知的工具: {}", request.name),
                None,
            )),
        }
    }
}

impl ConfirmServer {
    async fn handle_confirm(&self, request: ConfirmRequest) -> Result<CallToolResult, McpError> {
        let popup_request: PopupRequest = request.into();

        // 調用 UI 程序
        match self.create_ui_popup(&popup_request).await {
            Ok(response) => {
                // 構建回應
                let mut content = if response.confirmed {
                    if !response.selected_sections.is_empty() {
                        "⚠️ 用戶已確認並選擇了以下任務，請立即執行（不要再詢問確認）：\n".to_string()
                    } else {
                        "用戶確認操作\n".to_string()
                    }
                } else {
                    "用戶取消操作\n".to_string()
                };

                if !response.selected_sections.is_empty() {
                    content.push_str(&format!("\n選中的段落索引: {:?}\n", response.selected_sections));

                    // 添加選中段落的詳細內容，並強調「立即執行」
                    content.push_str("\n📋 需要立即執行的任務：\n");
                    for (i, &idx) in response.selected_sections.iter().enumerate() {
                        if let Some(section) = popup_request.sections.get(idx) {
                            content.push_str(&format!(
                                "\n✅ 任務 {} (索引 {})：{}\n",
                                i + 1, idx, section.title
                            ));
                            content.push_str(&format!("   詳細說明：{}\n", section.content));
                            content.push_str("   ⚡ 行動：立即開始實作此任務\n");
                        }
                    }
                }

                if !response.user_input.is_empty() {
                    content.push_str(&format!("\n\n💬 用戶額外要求：\n{}", response.user_input));
                }

                if !response.images.is_empty() {
                    content.push_str(&format!("\n\n附加圖片: {} 張", response.images.len()));
                }

                Ok(CallToolResult::success(vec![Content::text(content)]))
            }
            Err(e) => {
                Err(McpError::internal_error(
                    format!("UI 互動失敗: {}", e),
                    None
                ))
            }
        }
    }

    async fn create_ui_popup(&self, request: &PopupRequest) -> Result<UserResponse> {
        // 創建臨時文件
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join(format!("mcp_request_{}.json", request.id));

        let request_json = serde_json::to_string_pretty(request)?;
        fs::write(&temp_file, request_json)?;

        // 查找 UI 命令
        let ui_command = self.find_ui_command()?;

        // 調用 UI 程序
        let output = Command::new(&ui_command)
            .arg("--mcp-request")
            .arg(&temp_file)
            .output()?;

        // 清理臨時文件
        let _ = fs::remove_file(&temp_file);

        if output.status.success() {
            let response_text = String::from_utf8_lossy(&output.stdout);
            let response: UserResponse = serde_json::from_str(response_text.trim())
                .unwrap_or_else(|_| UserResponse::cancelled());
            Ok(response)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("UI 程序失敗: {}", error);
        }
    }

    fn find_ui_command(&self) -> Result<String> {
        // macOS: 優先使用 .app bundle 內的 binary（需要 Info.plist 才能啟動 WebView）
        let app_bundle_paths = [
            "/Applications/claude-confirm.app/Contents/MacOS/claude-confirm-ui",
            // homebrew cask 或自訂安裝位置
            &format!("{}/Applications/claude-confirm.app/Contents/MacOS/claude-confirm-ui",
                dirs::home_dir().map(|h| h.to_string_lossy().to_string()).unwrap_or_default()),
        ];
        for path in &app_bundle_paths {
            let p = std::path::Path::new(path);
            if p.exists() {
                return Ok(path.to_string());
            }
        }

        // 查找同目錄的 .app bundle
        if let Ok(current_exe) = std::env::current_exe() {
            if let Some(exe_dir) = current_exe.parent() {
                // 同目錄的 .app bundle
                let bundle_ui = exe_dir.join("bundle/macos/claude-confirm.app/Contents/MacOS/claude-confirm-ui");
                if bundle_ui.exists() {
                    return Ok(bundle_ui.to_string_lossy().to_string());
                }
                // 同目錄的裸 binary（fallback）
                let local_ui = exe_dir.join("claude-confirm-ui");
                if local_ui.exists() {
                    return Ok(local_ui.to_string_lossy().to_string());
                }
            }
        }

        // 嘗試全局命令
        if Command::new("claude-confirm-ui")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Ok("claude-confirm-ui".to_string());
        }

        anyhow::bail!("找不到 UI 程序。請確保 claude-confirm.app 已安裝到 /Applications/ 或 claude-confirm-ui 在同目錄");
    }
}

pub async fn run_server() -> Result<()> {
    let service = ConfirmServer
        .serve(stdio())
        .await
        .inspect_err(|e| {
            log::error!("啟動服務器失敗: {}", e);
        })?;

    service.waiting().await?;
    Ok(())
}
