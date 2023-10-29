use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct ServerMsg {
    pub message: String,
    pub sender_id: i64,
    pub receiver_id: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientMsg {
    pub message: String,
    pub sender_id: i64,
    pub receiver_id: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub message: String,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessageResponse {
    pub id: i32,
    pub message: String,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>
}