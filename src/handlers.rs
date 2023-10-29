use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_typed_websockets::{Message, WebSocket, WebSocketUpgrade};
use futures::{SinkExt, StreamExt};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::mpsc;

use crate::model::{ChatMessage, ClientMsg, ServerMsg};
use crate::{model::ChatMessageResponse, AppState};

static NEXTINT: std::sync::atomic::AtomicUsize = AtomicUsize::new(1);

pub async fn handler(
    // Upgrade the request to a WebSocket connection where the server sends
    // messages of type `ServerMsg` and the clients sends `ClientMsg`
    ws: WebSocketUpgrade<ServerMsg, ClientMsg>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| message_handler_socket(socket, state))
}

pub async fn message_handler_socket(socket: WebSocket<ServerMsg, ClientMsg>, app: Arc<AppState>) {
    // User id connected to the socket
    let user_id = NEXTINT.fetch_add(1, Ordering::Relaxed);

    // Create unbounded_channel as an intermediate to 'sends' triggers of new messages
    let (tx, mut rx) = mpsc::unbounded_channel::<ClientMsg>();

    // Splitting socket into sender and receiver
    let (mut sender, mut receiver) = socket.split();

    // Spawning a task to listen for new messages throght channel
    tokio::spawn(async move {
        loop {
            if let Some(msg) = rx.recv().await {
                // Build a ServerMsg instance
                let srv_msg = ServerMsg {
                    message: msg.message,
                    sender_id: msg.sender_id,
                    receiver_id: msg.receiver_id,
                };

                // Sending messages to clients
                sender
                    .send(Message::Item(srv_msg))
                    .await
                    .expect("some error sending message");
            }
        }
    });

    // Inserting sender half of a channel into the hashmap
    app.channel
        .lock()
        .unwrap()
        .entry(user_id.to_string())
        .or_insert(tx);

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(msg) => match msg {
                // Insert sender part of a channels in hashmap
                Message::Item(msg) => {
                    // Saving chat message
                    let chat_message = ChatMessage {
                        message: msg.message.clone(),
                        sender_id: msg.sender_id.clone(),
                        receiver_id: msg.receiver_id.clone(),
                        created_at: Some(chrono::Utc::now()),
                    };

                    app.chat_message_repository
                        .save(chat_message, &app.db)
                        .await
                        .unwrap();

                    // Sending message to all clients
                    for (idx, channel) in app.channel.lock().unwrap().iter() {
                        println!("{}", idx);
                        channel.send(msg.clone()).unwrap();
                    }
                }
                _ => {}
            },
            Err(_) => println!("Error receiving message"),
        }
    }
}

pub async fn get_by_receiver_id(
    Path(receiver_id): Path<i64>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let messages = app
        .chat_message_repository
        .get_by_receiver_id(receiver_id, &app.db)
        .await;

    match messages {
        Ok(messages) => {
            let response = build_success_multi_response(messages);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => {
            let error = build_error_response(err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
        }
    }
}

pub async fn get_by_sender_id(
    Path(sender_id): Path<i64>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let messages = app
        .chat_message_repository
        .get_by_sender_id(sender_id, &app.db)
        .await;

    match messages {
        Ok(messages) => {
            let response = build_success_multi_response(messages);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => {
            let err = build_error_response(err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err)))
        }
    }
}

fn build_success_multi_response(payload: Vec<ChatMessageResponse>) -> serde_json::Value {
    serde_json::json!({
        "status": "success",
        "message": payload
    })
}

fn build_error_response(error: Box<dyn std::error::Error>) -> serde_json::Value {
    serde_json::json!({
        "status": "fail",
        "message": format!("{}", error)
    })
}
