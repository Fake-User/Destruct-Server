mod creds;
mod db;

use tower_http::cors::{CorsLayer, Any};
use tokio::sync::broadcast;
use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};

#[cfg(debug_assertions)]
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppState {
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main(){
    #[cfg(debug_assertions)]
    dotenv().ok();

    let path = std::path::Path::new("./store/db-data.js");
    if !path.exists(){
        match std::fs::create_dir_all("./store"){
            Ok(_) => println!("{:#?} created successfully", path),
            Err(_) => println!("error creating {:#?}", path)
        }
    };

    let (tx, _rx) = broadcast::channel(100);
    let state = Arc::new(AppState { tx });

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
