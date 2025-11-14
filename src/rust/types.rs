use serde::{Deserialize, Serialize};

/// Popup 請求結構
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupRequest {
    pub id: String,
    pub message: String,
    #[serde(default)]
    pub sections: Vec<Section>,
    #[serde(default = "default_true")]
    pub is_markdown: bool,
}

fn default_true() -> bool {
    true
}

/// 段落定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub content: String,
    #[serde(default = "default_true")]
    pub selected: bool,
}

/// 用戶回應
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub confirmed: bool,
    #[serde(default)]
    pub selected_sections: Vec<usize>,
    #[serde(default)]
    pub user_input: String,
    #[serde(default)]
    pub images: Vec<String>,
}

impl UserResponse {
    pub fn cancelled() -> Self {
        Self {
            confirmed: false,
            selected_sections: Vec::new(),
            user_input: String::new(),
            images: Vec::new(),
        }
    }

    pub fn confirmed(selected_sections: Vec<usize>, user_input: String, images: Vec<String>) -> Self {
        Self {
            confirmed: true,
            selected_sections,
            user_input,
            images,
        }
    }
}
