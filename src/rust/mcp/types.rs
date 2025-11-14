use serde::{Deserialize, Serialize};

/// MCP Confirm 請求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmRequest {
    pub message: String,
    #[serde(default)]
    pub sections: Vec<crate::types::Section>,
    #[serde(default = "default_true")]
    pub is_markdown: bool,
}

fn default_true() -> bool {
    true
}

impl From<ConfirmRequest> for crate::types::PopupRequest {
    fn from(req: ConfirmRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            message: req.message,
            sections: req.sections,
            is_markdown: req.is_markdown,
        }
    }
}
