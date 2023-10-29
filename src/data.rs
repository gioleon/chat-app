use std::{error::Error};

use sqlx::{Row};

use crate::model::{ChatMessage, ChatMessageResponse};

#[derive(Default, Clone)]
pub struct ChatMessageRepository {}

impl ChatMessageRepository {
    pub fn new() -> Self {
        ChatMessageRepository {}
    }

    pub async fn save(&self, chat_message: ChatMessage, conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
       let sql = "INSERT INTO chat_messages (message, sender_id, receiver_id, created_at) VALUES ($1, $2, $3, $4)";

       sqlx::query(sql)
           .bind(chat_message.message)
           .bind(chat_message.sender_id)
           .bind(chat_message.receiver_id)
           .bind(chat_message.created_at)
           .execute(conn)
           .await?;
   
        Ok(())
    } 

    pub async fn get_by_sender_id(&self, sender_id: i64, conn: &sqlx::PgPool) -> Result<Vec<ChatMessageResponse>, Box<dyn Error>> {
        let sql = "SELECT * FROM chat_messages WHERE sender_id = $1";

        let rows = sqlx::query(sql)
            .bind(&sender_id)
            .fetch_all(conn)
            .await?;
  
         let clients = rows.iter().map(|row| {
             ChatMessageResponse {
                id: row.get("id"),
                message: row.get("message"),
                sender_id: row.get("sender_id"),
                receiver_id: row.get("receiver_id"),
                created_at: row.get("created_at")
             }
         }).collect();

        Ok(clients) 
    }

    pub async fn get_by_receiver_id(&self, receiver_id: i64, conn: &sqlx::PgPool) -> Result<Vec<ChatMessageResponse>, Box<dyn Error>> {
        
        let sql = "SELECT * FROM chat_messages WHERE receiver_id = $1";

        let rows = sqlx::query(sql)
        .bind(receiver_id)
            .fetch_all(conn)
            .await?;

        let messages = rows.iter().map(|row| {
             ChatMessageResponse {
                 id: row.get("id"),
                 message: row.get("message"),
                 sender_id: row.get("sender_id"),
                 receiver_id: row.get("receiver_id"),
                 created_at: row.get("created_at"), 
             }
        }).collect();
        
        Ok(messages)
    }
}

