use serde::{Deserialize, Serialize};

/// WebSocket 消息类型
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    /// 用户发送的聊天消息
    #[serde(rename = "chat")]
    Chat(ChatMessage),

    /// 设置文件上下文
    #[serde(rename = "set_context")]
    SetContext(SetContextMessage),

    /// 切换模型
    #[serde(rename = "switch_model")]
    SwitchModel(SwitchModelMessage),

    /// 清除文件上下文
    #[serde(rename = "clear_context")]
    ClearContext,

    /// 清除聊天记录
    #[serde(rename = "clear_history")]
    ClearHistory,

    /// 获取历史记录
    #[serde(rename = "get_history")]
    GetHistory,
}

/// 带用户 ID 的 WebSocket 消息包装
#[derive(Serialize, Deserialize, Debug)]
pub struct WsMessageWrapper {
    #[serde(flatten)]
    pub message: WsMessage,
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetContextMessage {
    pub files: Vec<FileContext>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileContext {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwitchModelMessage {
    pub model: String,
}

/// 服务器响应消息
#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    /// 模型回复
    #[serde(rename = "response")]
    Response(ResponseMessage),

    /// 思考过程
    #[serde(rename = "thinking")]
    Thinking(ThinkingMessage),

    /// 系统消息
    #[serde(rename = "system")]
    System(SystemMessage),

    /// 错误消息
    #[serde(rename = "error")]
    Error(ErrorMessage),

    /// 加载状态
    #[serde(rename = "loading")]
    Loading(LoadingMessage),

    /// 历史记录
    #[serde(rename = "history")]
    History(HistoryMessage),
}

#[derive(Serialize, Debug)]
pub struct HistoryMessage {
    pub messages: Vec<HistoryItem>,
}

#[derive(Serialize, Debug)]
pub struct HistoryItem {
    pub role: String,
    pub content: String,
    pub model: Option<String>,
    pub timestamp: String,
}

#[derive(Serialize, Debug)]
pub struct ResponseMessage {
    pub content: String,
    pub model: String,
}

#[derive(Serialize, Debug)]
pub struct ThinkingMessage {
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct SystemMessage {
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct LoadingMessage {
    pub is_loading: bool,
}

/// 文件上传响应
#[derive(Serialize)]
pub struct UploadResponse {
    pub status: String,
    pub files: Option<Vec<UploadedFile>>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct UploadedFile {
    pub name: String,
    pub content: String,
    pub size: usize,
}
