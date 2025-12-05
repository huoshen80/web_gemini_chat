use actix_multipart::Multipart;
use actix_web::{post, Error, HttpResponse, Responder};
use futures_util::stream::StreamExt;
use std::str;

use crate::models::messages::{UploadResponse, UploadedFile};

#[post("/api/upload")]
pub async fn upload_file(mut payload: Multipart) -> Result<impl Responder, Error> {
    let mut uploaded_files: Vec<UploadedFile> = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item?;
        
        // 先提取文件名
        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename().map(|s| s.to_string()));
        
        if let Some(filename) = filename {
            let mut file_content: Vec<u8> = Vec::new();
            
            while let Some(chunk) = field.next().await {
                file_content.extend_from_slice(&chunk?);
            }

            match str::from_utf8(&file_content) {
                Ok(text) => {
                    uploaded_files.push(UploadedFile {
                        name: filename.clone(),
                        content: text.to_string(),
                        size: file_content.len(),
                    });
                }
                Err(e) => {
                    return Ok(HttpResponse::BadRequest().json(UploadResponse {
                        status: "error".to_string(),
                        files: None,
                        error: Some(format!("文件 {} 不是有效的 UTF-8 编码: {}", filename, e)),
                    }));
                }
            }
        }
    }

    if uploaded_files.is_empty() {
        return Ok(HttpResponse::BadRequest().json(UploadResponse {
            status: "error".to_string(),
            files: None,
            error: Some("没有上传任何文件".to_string()),
        }));
    }

    Ok(HttpResponse::Ok().json(UploadResponse {
        status: "success".to_string(),
        files: Some(uploaded_files),
        error: None,
    }))
}
