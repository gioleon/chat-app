use axum::{
    routing::get, Router
};

use std::sync::Arc;

use crate::handlers::{handler, get_by_receiver_id, get_by_sender_id};
use crate::AppState;


pub fn build_router(
    app: Arc<AppState>
) -> Router {
    Router::new()
        .route("/ws", get(handler))
        .route("/sender/:sender_id", get(get_by_sender_id))
        .route("/receiver/:receiver_id", get(get_by_receiver_id))
        .with_state(app)
}
