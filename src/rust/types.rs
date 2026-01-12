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
    /// 環境上下文資訊
    #[serde(default)]
    pub env_context: Option<EnvContext>,
}

fn default_true() -> bool {
    true
}

/// 環境上下文資訊
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnvContext {
    /// 當前工作目錄
    #[serde(default)]
    pub cwd: Option<String>,
    /// 專案名稱（從 cwd 提取）
    #[serde(default)]
    pub project_name: Option<String>,
    /// 終端機程式名稱
    #[serde(default)]
    pub terminal: Option<String>,
    /// 進程 ID
    #[serde(default)]
    pub pid: Option<u32>,
}

impl EnvContext {
    /// 從當前環境自動偵測
    pub fn detect() -> Self {
        let cwd = std::env::current_dir()
            .ok()
            .and_then(|p| p.to_str().map(String::from));
        
        let project_name = cwd.as_ref().and_then(|path| {
            std::path::Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .map(String::from)
        });
        
        let terminal = std::env::var("TERM_PROGRAM").ok();
        let pid = Some(std::process::id());
        
        Self {
            cwd,
            project_name,
            terminal,
            pid,
        }
    }
    
    /// 合併兩個 EnvContext，優先使用 other 的值（如果有）
    pub fn merge_with(&self, other: Option<&EnvContext>) -> Self {
        match other {
            Some(o) => Self {
                cwd: o.cwd.clone().or_else(|| self.cwd.clone()),
                project_name: o.project_name.clone().or_else(|| self.project_name.clone()),
                terminal: o.terminal.clone().or_else(|| self.terminal.clone()),
                pid: o.pid.or(self.pid),
            },
            None => self.clone(),
        }
    }
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
