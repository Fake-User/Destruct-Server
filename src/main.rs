#[cfg(debug_assertions)]
use dotenv::dotenv;

use tower_http::cors::{CorsLayer, Any};
use axum::http::HeaderValue;
use tokio::sync::watch;
use reqwest::Client;
use std::sync::Arc;
use axum::{
    routing::{get, put},
    Router,
};
mod creds;
mod db;

pub struct AppState{
    db_update: watch::Sender<String>,
    db_utc:  std::sync::RwLock<String>
}

#[tokio::main]
async fn main(){
    #[cfg(debug_assertions)]
    dotenv().ok();

    let (tx, _rx) = watch::channel(String::new());
    let state = Arc::new(AppState{db_update: tx, db_utc: std::sync::RwLock::new("".to_string())});
    let client = Client::new();

    if !std::path::Path::new("store/db-data.js").exists() {
        std::fs::create_dir_all("store").unwrap();
        let response = client
            .get("https://data.destruct.dev/db/db-data.js")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        std::fs::write("store/db-data.js", &response).unwrap();
    };

    if let Ok(data) = std::fs::read_to_string("store/db-data.js"){
        let utc = data
            .clone()
            .split_once('"')
            .unwrap()
            .1
            .split_once('"')
            .unwrap()
            .0
            .to_string();
        *state.db_utc.write().unwrap() = utc;
        state.db_update.send(data).unwrap();
    };

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin([
            "https://edge.destruct.dev".parse::<HeaderValue>().unwrap(),
            "https://*.destruct.dev".parse::<HeaderValue>().unwrap(),
            "http://localhost:31337".parse::<HeaderValue>().unwrap(),
            "http://localhost:1337".parse::<HeaderValue>().unwrap(),
            "https://destruct.dev".parse::<HeaderValue>().unwrap(),
        ]);

    let app = Router::new()
        .route("/get-db", get(db::get_db))
        .route("/set-db", put(db::set_db))
        .route("/creds", get(creds::creds))
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
