mod handlers;
mod models;
mod services;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use handlers::{health::health_check, upload::upload_file, websocket::ws_index};
use services::memory::ChatMemory;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载 .env 文件
    dotenv().ok();

    // 检查 API Key
    match env::var("GEMINI_API_KEY") {
        Ok(_) => println!("✅ GEMINI_API_KEY 加载成功"),
        Err(_) => println!(
            "⚠️  警告: GEMINI_API_KEY 未设置，请在 .env 文件中配置"
        ),
    }

    // 初始化聊天记忆数据库
    let db_path = env::var("DATABASE_URL").unwrap_or_else(|_| "data/web_chat.db".to_string());
    
    // 确保数据库目录存在
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let memory = Arc::new(
        ChatMemory::new(&db_path).expect("无法创建聊天记忆数据库")
    );
    println!("💾 聊天记忆数据库已初始化 ({})", db_path);
    
    let message_count = memory.message_count().unwrap_or(0);
    if message_count > 0 {
        println!("📝 已加载 {} 条历史消息", message_count);
    }

    println!("🦀 Rust 后端服务器启动于 http://0.0.0.0:23333");
    println!("📡 支持的模型: Gemini 2.0 Flash (flash), Gemini 2.5 Flash (flash-2.5), Gemini 2.5 Pro (pro-2.5)");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // 允许所有来源，方便开发和Docker环境
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::UPGRADE,
                actix_web::http::header::CONNECTION,
                actix_web::http::header::SEC_WEBSOCKET_VERSION,
                actix_web::http::header::SEC_WEBSOCKET_KEY,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(memory.clone()))
            .service(health_check)
            .service(upload_file)
            .service(ws_index)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", 23333))?
    .run()
    .await
}

