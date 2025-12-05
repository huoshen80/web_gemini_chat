use crate::models::gemini::{
    Content, GeminiModel, GeminiRequest, GeminiResponse, GenerationConfig, Part,
};

/// Gemini API 调用结果
pub struct GeminiResult {
    pub response: String,
    pub thinking: Option<String>,
}

/// 调用 Gemini API
pub async fn call_gemini_api(
    prompt: String,
    api_key: &str,
    model: GeminiModel,
) -> Result<GeminiResult, String> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model.api_name(),
        api_key
    );

    let mut request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part { text: prompt }],
            role: Some("user".to_string()),
        }],
        generation_config: None,
    };

    // Pro 模型启用思考功能
    if model.supports_thinking() {
        request_body.generation_config = Some(GenerationConfig {
            temperature: Some(1.0),
            max_output_tokens: Some(65536),
        });
    }

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("发送请求失败: {}", e))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "未知错误".to_string());
        return Err(format!("Gemini API 错误: {}", error_text));
    }

    let gemini_response: GeminiResponse = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    // 解析响应，分离思考过程和最终回复
    let mut thinking = None;
    let mut response_text = String::new();

    if let Some(candidates) = gemini_response.candidates
        && let Some(candidate) = candidates.first()
    {
        for part in &candidate.content.parts {
            if let Some(text) = &part.text {
                if part.thought.unwrap_or(false) {
                    // 这是思考过程
                    thinking = Some(text.clone());
                } else {
                    // 这是最终回复
                    response_text = text.clone();
                }
            }
        }
    }

    if response_text.is_empty() {
        response_text = "没有收到 Gemini 的回复".to_string();
    }

    Ok(GeminiResult {
        response: response_text,
        thinking,
    })
}
