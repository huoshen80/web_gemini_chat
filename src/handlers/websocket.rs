use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, get, web};
use actix_web_actors::ws;
use std::env;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::models::gemini::GeminiModel;
use crate::models::messages::{
    ErrorMessage, FileContext, HistoryItem, HistoryMessage, LoadingMessage, ResponseMessage,
    ServerMessage, SystemMessage, ThinkingMessage, WsMessage, WsMessageWrapper,
};
use crate::services::embedding::{generate_embedding, generate_query_embedding};
use crate::services::gemini::call_gemini_api;
use crate::services::memory::{ChatMemory, format_recent_context, format_retrieved_context};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_RECENT_MESSAGES: usize = 4; // 最近的对话数量（保持连贯性）
const MAX_SIMILAR_MESSAGES: usize = 5; // 相似消息检索数量
const MIN_SIMILARITY: f32 = 0.5; // 最小相似度阈值
const MAX_CONTEXT_CHARS: usize = 4000; // 最大上下文字符数

/// WebSocket Actor
pub struct ChatWebSocket {
    hb: Instant,
    file_contexts: Vec<FileContext>,
    current_model: GeminiModel,
    memory: Arc<ChatMemory>,
    user_id: String, // 当前用户 ID
}

impl ChatWebSocket {
    pub fn new(memory: Arc<ChatMemory>) -> Self {
        Self {
            hb: Instant::now(),
            file_contexts: Vec::new(),
            current_model: GeminiModel::Flash,
            memory,
            user_id: String::new(), // 将在收到消息时设置
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("WebSocket 客户端心跳超时，断开连接！");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn send_message(&self, ctx: &mut ws::WebsocketContext<Self>, msg: ServerMessage) {
        if let Ok(json) = serde_json::to_string(&msg) {
            ctx.text(json);
        }
    }

    fn send_history(&self, ctx: &mut ws::WebsocketContext<Self>) {
        if self.user_id.is_empty() {
            self.send_message(
                ctx,
                ServerMessage::Error(ErrorMessage {
                    content: "用户未识别".to_string(),
                }),
            );
            return;
        }

        match self.memory.get_all_messages(&self.user_id) {
            Ok(messages) => {
                let history_items: Vec<HistoryItem> = messages
                    .into_iter()
                    .map(|msg| HistoryItem {
                        role: msg.role,
                        content: msg.content,
                        model: msg.model,
                        timestamp: msg.created_at.to_rfc3339(),
                    })
                    .collect();

                self.send_message(
                    ctx,
                    ServerMessage::History(HistoryMessage {
                        messages: history_items,
                    }),
                );
            }
            Err(e) => {
                self.send_message(
                    ctx,
                    ServerMessage::Error(ErrorMessage {
                        content: format!("获取历史记录失败: {}", e),
                    }),
                );
            }
        }
    }
}

impl Actor for ChatWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        // 发送欢迎消息
        self.send_message(
            ctx,
            ServerMessage::System(SystemMessage {
                content: format!(
                    "已连接到服务器，当前使用 {} 模型",
                    self.current_model.display_name()
                ),
            }),
        );
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let text_str = text.to_string();

                // 解析带用户 ID 的 JSON 消息
                match serde_json::from_str::<WsMessageWrapper>(&text_str) {
                    Ok(wrapper) => {
                        // 设置用户 ID
                        if let Some(ref uid) = wrapper.user_id
                            && self.user_id.is_empty()
                        {
                            self.user_id = uid.clone();
                            println!("👤 用户连接: {}", uid);
                        }

                        // 如果没有用户 ID，使用默认值
                        if self.user_id.is_empty() {
                            self.user_id = "anonymous".to_string();
                        }

                        let user_id = self.user_id.clone();

                        match wrapper.message {
                            WsMessage::Chat(chat_msg) => {
                                // 获取 API key
                                let api_key = match env::var("GEMINI_API_KEY") {
                                    Ok(key) => key,
                                    Err(_) => {
                                        self.send_message(
                                            ctx,
                                            ServerMessage::Error(ErrorMessage {
                                                content: "未设置 GEMINI_API_KEY 环境变量"
                                                    .to_string(),
                                            }),
                                        );
                                        return;
                                    }
                                };

                                // 发送加载状态
                                self.send_message(
                                    ctx,
                                    ServerMessage::Loading(LoadingMessage { is_loading: true }),
                                );

                                let user_content = chat_msg.content.clone();
                                let model = self.current_model;
                                let memory = self.memory.clone();
                                let file_contexts = self.file_contexts.clone();
                                let api_key_clone = api_key.clone();
                                let user_id_clone = user_id.clone();

                                // 异步处理：生成嵌入 -> 检索相关历史 -> 调用 Gemini API
                                let fut = async move {
                                    // 1. 先保存用户消息
                                    let user_msg_id = memory
                                        .add_message(
                                            &user_id_clone,
                                            "user",
                                            &user_content,
                                            Some(model.as_str()),
                                        )
                                        .ok();

                                    // 2. 生成用户消息的嵌入向量（用于检索）
                                    let query_embedding =
                                        generate_query_embedding(&user_content, &api_key_clone)
                                            .await
                                            .ok();

                                    // 3. 如果有嵌入向量，检索相关历史消息
                                    let similar_messages =
                                        if let Some(ref embedding) = query_embedding {
                                            memory
                                                .retrieve_similar(
                                                    &user_id_clone,
                                                    embedding,
                                                    MAX_SIMILAR_MESSAGES,
                                                    MIN_SIMILARITY,
                                                )
                                                .unwrap_or_default()
                                        } else {
                                            Vec::new()
                                        };

                                    // 4. 获取最近几条消息（保持对话连贯性）
                                    let recent_messages = memory
                                        .get_recent_messages(&user_id_clone, MAX_RECENT_MESSAGES)
                                        .unwrap_or_default();

                                    // 5. 构建 prompt
                                    let mut prompt = String::new();

                                    // 添加检索到的相关历史
                                    if !similar_messages.is_empty() {
                                        prompt.push_str(&format_retrieved_context(
                                            &similar_messages,
                                            MAX_CONTEXT_CHARS,
                                        ));
                                    }

                                    // 添加最近对话（如果相关历史不够）
                                    if similar_messages.len() < 2 && !recent_messages.is_empty() {
                                        prompt.push_str(&format_recent_context(
                                            &recent_messages,
                                            MAX_RECENT_MESSAGES,
                                        ));
                                    }

                                    // 添加文件上下文
                                    if !file_contexts.is_empty() {
                                        prompt.push_str(
                                            "以下是用户上传的文件内容作为上下文参考：\n\n",
                                        );
                                        for (i, file) in file_contexts.iter().enumerate() {
                                            prompt.push_str(&format!(
                                                "--- 文件 {} ({}) ---\n{}\n\n",
                                                i + 1,
                                                file.name,
                                                file.content
                                            ));
                                        }
                                        prompt.push_str("---\n\n");
                                    }

                                    prompt.push_str(&format!("用户消息：{}", user_content));

                                    // 6. 调用 Gemini API
                                    let gemini_result =
                                        call_gemini_api(prompt, &api_key_clone, model).await;

                                    // 7. 更新用户消息的嵌入向量
                                    if let (Some(msg_id), Some(embedding)) =
                                        (user_msg_id, query_embedding)
                                    {
                                        let _ = memory.update_embedding(msg_id, &embedding);
                                    }

                                    (gemini_result, user_id_clone)
                                };

                                ctx.wait(fut.into_actor(self).map(
                                    move |(result, uid), act, ctx| {
                                        // 发送加载完成
                                        act.send_message(
                                            ctx,
                                            ServerMessage::Loading(LoadingMessage {
                                                is_loading: false,
                                            }),
                                        );

                                        match result {
                                            Ok(gemini_result) => {
                                                // 如果有思考过程，先发送思考消息
                                                if let Some(thinking) = gemini_result.thinking {
                                                    act.send_message(
                                                        ctx,
                                                        ServerMessage::Thinking(ThinkingMessage {
                                                            content: thinking,
                                                        }),
                                                    );
                                                }

                                                let response_content =
                                                    gemini_result.response.clone();
                                                let memory = act.memory.clone();
                                                let model_str = act.current_model.as_str();
                                                let api_key = env::var("GEMINI_API_KEY").ok();

                                                // 保存 AI 回复到记忆
                                                if let Ok(msg_id) = memory.add_message(
                                                    &uid,
                                                    "model",
                                                    &response_content,
                                                    Some(model_str),
                                                ) {
                                                    // 异步生成回复的嵌入向量
                                                    if let Some(key) = api_key {
                                                        let response_for_embed =
                                                            response_content.clone();
                                                        actix::spawn(async move {
                                                            if let Ok(embedding) =
                                                                generate_embedding(
                                                                    &response_for_embed,
                                                                    &key,
                                                                )
                                                                .await
                                                            {
                                                                let _ = memory.update_embedding(
                                                                    msg_id, &embedding,
                                                                );
                                                            }
                                                        });
                                                    }
                                                }

                                                // 发送回复
                                                act.send_message(
                                                    ctx,
                                                    ServerMessage::Response(ResponseMessage {
                                                        content: gemini_result.response,
                                                        model: act
                                                            .current_model
                                                            .display_name()
                                                            .to_string(),
                                                    }),
                                                );
                                            }
                                            Err(e) => {
                                                act.send_message(
                                                    ctx,
                                                    ServerMessage::Error(ErrorMessage {
                                                        content: e,
                                                    }),
                                                );
                                            }
                                        }
                                    },
                                ));
                            }
                            WsMessage::SetContext(context_msg) => {
                                self.file_contexts = context_msg.files;
                                let count = self.file_contexts.len();
                                self.send_message(
                                    ctx,
                                    ServerMessage::System(SystemMessage {
                                        content: format!("已设置 {} 个文件作为上下文", count),
                                    }),
                                );
                            }
                            WsMessage::SwitchModel(model_msg) => {
                                self.current_model = GeminiModel::from_str(&model_msg.model);
                                self.send_message(
                                    ctx,
                                    ServerMessage::System(SystemMessage {
                                        content: format!(
                                            "已切换到 {} 模型",
                                            self.current_model.display_name()
                                        ),
                                    }),
                                );
                            }
                            WsMessage::ClearContext => {
                                self.file_contexts.clear();
                                self.send_message(
                                    ctx,
                                    ServerMessage::System(SystemMessage {
                                        content: "已清除所有文件上下文".to_string(),
                                    }),
                                );
                            }
                            WsMessage::ClearHistory => {
                                match self.memory.clear_user_messages(&user_id) {
                                    Ok(_) => {
                                        self.send_message(
                                            ctx,
                                            ServerMessage::System(SystemMessage {
                                                content: "已清除所有聊天记录".to_string(),
                                            }),
                                        );
                                    }
                                    Err(e) => {
                                        self.send_message(
                                            ctx,
                                            ServerMessage::Error(ErrorMessage {
                                                content: format!("清除记录失败: {}", e),
                                            }),
                                        );
                                    }
                                }
                            }
                            WsMessage::GetHistory => {
                                self.send_history(ctx);
                            }
                        }
                    }
                    Err(_) => {
                        // 兼容旧格式：纯文本消息
                        self.send_message(
                            ctx,
                            ServerMessage::Error(ErrorMessage {
                                content: "消息格式错误，请使用 JSON 格式".to_string(),
                            }),
                        );
                    }
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

#[get("/ws")]
pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    memory: web::Data<Arc<ChatMemory>>,
) -> Result<HttpResponse, Error> {
    ws::start(ChatWebSocket::new(memory.get_ref().clone()), &req, stream)
}
