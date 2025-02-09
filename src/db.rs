use futures::stream::Stream;
use std::sync::Arc;
use std::{fs, env};
use axum::{
    response::{Sse, IntoResponse},
    response::sse::Event,
    http::HeaderMap,
    extract::State
};

use crate::AppState;

pub async fn set_db(State(state): State<Arc<AppState>>, headers: HeaderMap, body: String) -> impl IntoResponse{
    if headers.get("authorization").and_then(|h| h.to_str().ok()) != env::var("UTC_KEY").ok().as_deref(){
        return "authorization error".into_response()
    };

    if let Err(_) = fs::write("/store/db-data.js", &body){
        return "error writing db-data.js".into_response()
    };

    let time_stamp = body.clone().split_once('"').unwrap().1.split_once('"').unwrap().0.to_string();
    let mut db_utc = state.db_utc.write().await;
    *db_utc = time_stamp;
    let mut db_data = state.db_data.write().await;
    *db_data = body;

    /* I need to send an event here that triggers all the clients to get the new db_data */

    "db updated".into_response()
}

pub async fn get_db(State(state): State<Arc<AppState>>) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>>{
    let stream = async_stream::stream!{
        /* I need to listen for the an event to send out the new db_data */
    };
    Sse::new(stream)
}
