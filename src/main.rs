use tower_http::cors::{CorsLayer, Any};
use tokio::sync::RwLock;
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
    db_utc: RwLock<String>,
    db_data: RwLock<String>
}

#[tokio::main]
async fn main(){
    #[cfg(debug_assertions)]
    dotenv().ok();

    let state = Arc::new(AppState{
        db_utc: RwLock::new("".to_string()),
        db_data: RwLock::new("".to_string()),
    });

    let path = std::path::Path::new("./store/db-data.js");
    if !path.exists(){
        match std::fs::create_dir_all("./store"){
            Ok(_) => println!("{:#?} created successfully", path),
            Err(_) => println!("error creating {:#?}", path)
        }
    }
    else{
        let data = std::fs::read_to_string("./store/db-data.js").unwrap();
        let time_stamp = data.clone().split_once('"').unwrap().1.split_once('"').unwrap().0.to_string();
        let mut db_utc = state.db_utc.write().await;
        *db_utc = time_stamp;
        let mut db_data = state.db_data.write().await;
        *db_data = data;
    };

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
