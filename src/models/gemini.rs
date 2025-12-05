use serde::{Deserialize, Serialize};

/// Gemini API 请求结构
#[derive(Serialize, Deserialize, Debug)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
}

/// Gemini API 响应结构
#[derive(Deserialize, Debug)]
pub struct GeminiResponse {
    pub candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize, Debug)]
pub struct Candidate {
    pub content: CandidateContent,
}

#[derive(Deserialize, Debug)]
pub struct CandidateContent {
    pub parts: Vec<PartResponse>,
}

#[derive(Deserialize, Debug)]
pub struct PartResponse {
    pub text: Option<String>,
    pub thought: Option<bool>,
}

/// 支持的模型类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeminiModel {
    Flash,   // gemini-2.0-flash
    Flash25, // gemini-2.5-flash
    Pro25,   // gemini-2.5-pro
}

impl GeminiModel {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "flash" | "gemini-2.0-flash" => GeminiModel::Flash,
            "flash-2.5" | "gemini-2.5-flash" => GeminiModel::Flash25,
            "pro-2.5" | "gemini-2.5-pro" => GeminiModel::Pro25,
            _ => GeminiModel::Flash,
        }
    }

    pub fn api_name(&self) -> &'static str {
        match self {
            GeminiModel::Flash => "gemini-2.0-flash",
            GeminiModel::Flash25 => "gemini-2.5-flash",
            GeminiModel::Pro25 => "gemini-2.5-pro",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            GeminiModel::Flash => "Gemini 2.0 Flash",
            GeminiModel::Flash25 => "Gemini 2.5 Flash",
            GeminiModel::Pro25 => "Gemini 2.5 Pro",
        }
    }

    pub fn supports_thinking(&self) -> bool {
        matches!(self, GeminiModel::Pro25)
    }

    /// 返回用于存储的模型标识字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            GeminiModel::Flash => "flash",
            GeminiModel::Flash25 => "flash-2.5",
            GeminiModel::Pro25 => "pro-2.5",
        }
    }
}
