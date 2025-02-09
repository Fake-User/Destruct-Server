use tower_http::cors::{CorsLayer, Any};
use tokio::sync::watch;
use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};

#[cfg(debug_assertions)]
use dotenv::dotenv;

mod creds;
mod db;

pub struct AppState{
    db_notifier: watch::Sender<String>,
}

#[tokio::main]
async fn main(){
    #[cfg(debug_assertions)]
    dotenv().ok();

    let (tx, _rx) = watch::channel(String::new());
    let state = Arc::new(AppState{db_notifier: tx});

    // Load initial data if it exists
    if let Ok(data) = std::fs::read_to_string("./store/db-data.js"){
        let _ = state.db_notifier.send(data);
    }

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/get-db", get(db::get_db))
        .route("/set-db", get(db::set_db))
        .route("/creds", get(creds::creds))
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
