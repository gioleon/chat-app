use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};

use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::mpsc;

use model::ClientMsg;
use router::build_router;

mod data;
mod handlers;
mod model;
mod router;

type Channel = Arc<Mutex<HashMap<String, mpsc::UnboundedSender<ClientMsg>>>>;
type ChatMessageRepository = data::ChatMessageRepository;

#[derive(Clone)]
pub struct AppState {
    chat_message_repository: ChatMessageRepository,
    db: PgPool,
    channel: Channel,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();

    // Database pool
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database successfully estableshied");
            pool
        }
        Err(error) => {
            println!("Failed to connect to the db {}", error);
            std::process::exit(1);
        }
    };

    // Create AppState
    let app = Arc::new(AppState {
        db: pool,
        chat_message_repository: ChatMessageRepository::new(),
        channel: Channel::default(),
    });

    // Router
    let app = build_router(app);

    // Run it!
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
