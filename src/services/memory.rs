use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

use super::embedding::{bytes_to_embedding, cosine_similarity, embedding_to_bytes};

/// 聊天消息记录（带嵌入向量）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRecord {
    pub id: i64,
    pub user_id: String, // 用户 UUID
    pub role: String,    // "user" 或 "model"
    pub content: String,
    pub summary: Option<String>, // 对话摘要（用于长对话压缩）
    pub model: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 带相似度的检索结果
#[derive(Debug, Clone)]
pub struct RetrievedMessage {
    pub record: ChatRecord,
    pub similarity: f32,
}

/// 聊天记忆数据库（使用向量嵌入）
pub struct ChatMemory {
    conn: Mutex<Connection>,
}

impl ChatMemory {
    /// 创建或打开数据库
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // 创建消息表（包含嵌入向量和用户 ID）
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id TEXT NOT NULL DEFAULT '',
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                summary TEXT,
                embedding BLOB,
                model TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )?;

        // 创建索引
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_messages_user_created ON messages(user_id, created_at DESC)",
            [],
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// 添加消息（不带嵌入，稍后异步更新）
    pub fn add_message(
        &self,
        user_id: &str,
        role: &str,
        content: &str,
        model: Option<&str>,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO messages (user_id, role, content, model, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![user_id, role, content, model, now],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// 更新消息的嵌入向量
    pub fn update_embedding(&self, message_id: i64, embedding: &[f32]) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let embedding_bytes = embedding_to_bytes(embedding);

        conn.execute(
            "UPDATE messages SET embedding = ?1 WHERE id = ?2",
            params![embedding_bytes, message_id],
        )?;

        Ok(())
    }

    /// 更新消息的摘要
    #[allow(dead_code)]
    pub fn update_summary(&self, message_id: i64, summary: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "UPDATE messages SET summary = ?1 WHERE id = ?2",
            params![summary, message_id],
        )?;

        Ok(())
    }

    /// 根据查询嵌入检索用户最相关的消息
    pub fn retrieve_similar(
        &self,
        user_id: &str,
        query_embedding: &[f32],
        top_k: usize,
        min_similarity: f32,
    ) -> Result<Vec<RetrievedMessage>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, role, content, summary, embedding, model, created_at 
             FROM messages WHERE user_id = ?1 AND embedding IS NOT NULL",
        )?;

        let messages = stmt.query_map([user_id], |row| {
            let embedding_bytes: Option<Vec<u8>> = row.get(5)?;
            let created_at_str: String = row.get(7)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            Ok((
                ChatRecord {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    summary: row.get(4)?,
                    model: row.get(6)?,
                    created_at,
                },
                embedding_bytes,
            ))
        })?;

        let mut results: Vec<RetrievedMessage> = messages
            .filter_map(|m| m.ok())
            .filter_map(|(record, embedding_bytes)| {
                embedding_bytes.map(|bytes| {
                    let embedding = bytes_to_embedding(&bytes);
                    let similarity = cosine_similarity(query_embedding, &embedding);
                    RetrievedMessage { record, similarity }
                })
            })
            .filter(|m| m.similarity >= min_similarity)
            .collect();

        // 按相似度降序排序
        results.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // 只返回前 top_k 个
        results.truncate(top_k);

        Ok(results)
    }

    /// 获取用户最近的 N 条消息（用于保持对话连贯性）
    pub fn get_recent_messages(&self, user_id: &str, limit: usize) -> Result<Vec<ChatRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, role, content, summary, model, created_at FROM messages 
             WHERE user_id = ?1 ORDER BY id DESC LIMIT ?2",
        )?;

        let messages = stmt.query_map(params![user_id, limit], |row| {
            let created_at_str: String = row.get(6)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            Ok(ChatRecord {
                id: row.get(0)?,
                user_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                summary: row.get(4)?,
                model: row.get(5)?,
                created_at,
            })
        })?;

        let mut result: Vec<ChatRecord> = messages.filter_map(|m| m.ok()).collect();
        result.reverse(); // 按时间正序排列
        Ok(result)
    }

    /// 获取没有嵌入的消息（用于批量生成嵌入）
    #[allow(dead_code)]
    pub fn get_messages_without_embedding(&self, limit: usize) -> Result<Vec<ChatRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, role, content, summary, model, created_at FROM messages 
             WHERE embedding IS NULL ORDER BY id ASC LIMIT ?1",
        )?;

        let messages = stmt.query_map([limit], |row| {
            let created_at_str: String = row.get(6)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            Ok(ChatRecord {
                id: row.get(0)?,
                user_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                summary: row.get(4)?,
                model: row.get(5)?,
                created_at,
            })
        })?;

        Ok(messages.filter_map(|m| m.ok()).collect())
    }

    /// 获取用户所有消息
    pub fn get_all_messages(&self, user_id: &str) -> Result<Vec<ChatRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, role, content, summary, model, created_at FROM messages 
             WHERE user_id = ?1 ORDER BY id ASC",
        )?;

        let messages = stmt.query_map([user_id], |row| {
            let created_at_str: String = row.get(6)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            Ok(ChatRecord {
                id: row.get(0)?,
                user_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                summary: row.get(4)?,
                model: row.get(5)?,
                created_at,
            })
        })?;

        Ok(messages.filter_map(|m| m.ok()).collect())
    }

    /// 清除用户所有消息
    pub fn clear_user_messages(&self, user_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM messages WHERE user_id = ?1", [user_id])?;
        Ok(())
    }

    /// 获取用户消息数量
    #[allow(dead_code)]
    pub fn user_message_count(&self, user_id: &str) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM messages WHERE user_id = ?1",
            [user_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }

    /// 获取总消息数量
    pub fn message_count(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM messages", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    /// 获取有嵌入的消息数量
    #[allow(dead_code)]
    pub fn embedded_message_count(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM messages WHERE embedding IS NOT NULL",
            [],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

/// 将检索到的相关消息格式化为上下文
pub fn format_retrieved_context(messages: &[RetrievedMessage], max_chars: usize) -> String {
    if messages.is_empty() {
        return String::new();
    }

    let mut context = String::from("以下是与当前问题相关的历史对话记录：\n\n");
    let mut total_chars = context.len();

    for msg in messages {
        let role_label = if msg.record.role == "user" {
            "用户"
        } else {
            "助手"
        };
        // 优先使用摘要，如果没有则使用原始内容
        let content = msg.record.summary.as_ref().unwrap_or(&msg.record.content);
        let entry = format!(
            "【{}】(相关度: {:.0}%): {}\n\n",
            role_label,
            msg.similarity * 100.0,
            content
        );

        if total_chars + entry.len() > max_chars {
            break;
        }

        context.push_str(&entry);
        total_chars += entry.len();
    }

    context.push_str("---\n\n");
    context
}

/// 将最近消息格式化为简短上下文
pub fn format_recent_context(messages: &[ChatRecord], max_messages: usize) -> String {
    if messages.is_empty() {
        return String::new();
    }

    let start_idx = if messages.len() > max_messages {
        messages.len() - max_messages
    } else {
        0
    };

    let mut context = String::from("最近的对话：\n\n");

    for msg in &messages[start_idx..] {
        let role_label = if msg.role == "user" {
            "用户"
        } else {
            "助手"
        };
        // 优先使用摘要
        let content = msg.summary.as_ref().unwrap_or(&msg.content);
        // 截断过长的内容
        let truncated = if content.len() > 200 {
            format!("{}...", &content[..200])
        } else {
            content.clone()
        };
        context.push_str(&format!("【{}】: {}\n\n", role_label, truncated));
    }

    context.push_str("---\n\n");
    context
}

#[cfg(test)]
mod tests {
    use super::super::embedding::EMBEDDING_DIMENSION;
    use super::*;

    #[test]
    fn test_memory_operations() {
        let memory = ChatMemory::new(":memory:").unwrap();
        let user_id = "test-user-123";

        // 添加消息
        let id1 = memory.add_message(user_id, "user", "你好", None).unwrap();
        let id2 = memory
            .add_message(
                user_id,
                "model",
                "你好！有什么可以帮助你的？",
                Some("flash"),
            )
            .unwrap();

        // 更新嵌入
        let fake_embedding = vec![0.1; EMBEDDING_DIMENSION];
        memory.update_embedding(id1, &fake_embedding).unwrap();
        memory.update_embedding(id2, &fake_embedding).unwrap();

        // 获取消息
        let messages = memory.get_all_messages(user_id).unwrap();
        assert_eq!(messages.len(), 2);

        // 检索相似消息
        let results = memory
            .retrieve_similar(user_id, &fake_embedding, 10, 0.0)
            .unwrap();
        assert_eq!(results.len(), 2);

        // 测试用户隔离
        let other_user = "other-user-456";
        let other_messages = memory.get_all_messages(other_user).unwrap();
        assert_eq!(other_messages.len(), 0);

        // 清除
        memory.clear_user_messages(user_id).unwrap();
        assert_eq!(memory.user_message_count(user_id).unwrap(), 0);
    }
}
