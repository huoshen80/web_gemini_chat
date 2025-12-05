use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Embedding API 请求结构
#[derive(Serialize, Debug)]
pub struct EmbeddingRequest {
    pub model: String,
    pub content: EmbeddingContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dimensionality: Option<u32>,
}

#[derive(Serialize, Debug)]
pub struct EmbeddingContent {
    pub parts: Vec<EmbeddingPart>,
}

#[derive(Serialize, Debug)]
pub struct EmbeddingPart {
    pub text: String,
}

/// Embedding API 响应结构
#[derive(Deserialize, Debug)]
pub struct EmbeddingResponse {
    pub embedding: Option<EmbeddingData>,
}

#[derive(Deserialize, Debug)]
pub struct EmbeddingData {
    pub values: Vec<f32>,
}

/// 嵌入维度（使用 768 维以节省存储空间）
pub const EMBEDDING_DIMENSION: usize = 768;

/// 调用 text-embedding-004 API 生成文本嵌入
pub async fn generate_embedding(text: &str, api_key: &str) -> Result<Vec<f32>, String> {
    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/text-embedding-004:embedContent?key={}",
        api_key
    );

    let request = EmbeddingRequest {
        model: "models/text-embedding-004".to_string(),
        content: EmbeddingContent {
            parts: vec![EmbeddingPart {
                text: text.to_string(),
            }],
        },
        task_type: Some("RETRIEVAL_DOCUMENT".to_string()),
        output_dimensionality: Some(EMBEDDING_DIMENSION as u32),
    };

    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("请求嵌入API失败: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("嵌入API返回错误: {}", error_text));
    }

    let embedding_response: EmbeddingResponse = response
        .json()
        .await
        .map_err(|e| format!("解析嵌入响应失败: {}", e))?;

    match embedding_response.embedding {
        Some(data) => {
            // 归一化嵌入向量
            let normalized = normalize_embedding(&data.values);
            Ok(normalized)
        }
        None => Err("嵌入响应中没有数据".to_string()),
    }
}

/// 为查询生成嵌入（使用不同的任务类型）
pub async fn generate_query_embedding(text: &str, api_key: &str) -> Result<Vec<f32>, String> {
    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/text-embedding-004:embedContent?key={}",
        api_key
    );

    let request = EmbeddingRequest {
        model: "models/text-embedding-004".to_string(),
        content: EmbeddingContent {
            parts: vec![EmbeddingPart {
                text: text.to_string(),
            }],
        },
        task_type: Some("RETRIEVAL_QUERY".to_string()),
        output_dimensionality: Some(EMBEDDING_DIMENSION as u32),
    };

    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("请求嵌入API失败: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("嵌入API返回错误: {}", error_text));
    }

    let embedding_response: EmbeddingResponse = response
        .json()
        .await
        .map_err(|e| format!("解析嵌入响应失败: {}", e))?;

    match embedding_response.embedding {
        Some(data) => {
            let normalized = normalize_embedding(&data.values);
            Ok(normalized)
        }
        None => Err("嵌入响应中没有数据".to_string()),
    }
}

/// 归一化嵌入向量
fn normalize_embedding(values: &[f32]) -> Vec<f32> {
    let norm: f32 = values.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        values.iter().map(|x| x / norm).collect()
    } else {
        values.to_vec()
    }
}

/// 计算两个嵌入向量的余弦相似度
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }
    // 由于向量已归一化，余弦相似度就是点积
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// 将嵌入向量序列化为字节数组（用于存储）
pub fn embedding_to_bytes(embedding: &[f32]) -> Vec<u8> {
    embedding.iter().flat_map(|f| f.to_le_bytes()).collect()
}

/// 从字节数组反序列化嵌入向量
pub fn bytes_to_embedding(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| {
            let arr: [u8; 4] = chunk.try_into().unwrap();
            f32::from_le_bytes(arr)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.0001);

        let c = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&a, &c)).abs() < 0.0001);
    }

    #[test]
    fn test_embedding_serialization() {
        let embedding = vec![1.0, 2.0, 3.0, 4.0];
        let bytes = embedding_to_bytes(&embedding);
        let restored = bytes_to_embedding(&bytes);
        assert_eq!(embedding, restored);
    }

    #[test]
    fn test_normalize() {
        let values = vec![3.0, 4.0];
        let normalized = normalize_embedding(&values);
        let norm: f32 = normalized.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.0001);
    }
}
