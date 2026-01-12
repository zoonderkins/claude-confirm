use serde::{Deserialize, Serialize};
use crate::types::EnvContext;

/// MCP Confirm 請求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmRequest {
    pub message: String,
    #[serde(default)]
    pub sections: Vec<crate::types::Section>,
    #[serde(default = "default_true")]
    pub is_markdown: bool,
    /// 可選的環境上下文，AI 可傳入覆蓋自動偵測值
    #[serde(default)]
    pub context: Option<EnvContext>,
}

fn default_true() -> bool {
    true
}

impl From<ConfirmRequest> for crate::types::PopupRequest {
    fn from(req: ConfirmRequest) -> Self {
        // 自動偵測環境資訊，並與 AI 傳入的值合併
        let detected = EnvContext::detect();
        let merged_context = detected.merge_with(req.context.as_ref());
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            message: req.message,
            sections: req.sections,
            is_markdown: req.is_markdown,
            env_context: Some(merged_context),
        }
    }
}
