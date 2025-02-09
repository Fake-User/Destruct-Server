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

pub async fn set_db(headers: HeaderMap, body: String) -> impl IntoResponse{
    match (headers.get("authorization"), env::var("UTC_KEY")){
        (_, Err(_)) => return "server configuration error".into_response(),
        (None, _) => return "missing authorization".into_response(),
        (Some(header), Ok(env_key)) => match header.to_str(){
            Ok(val) if val == env_key => val.to_string(),
            Ok(_) => return "invalid authorization".into_response(),
            Err(_) => return "invalid authorization".into_response(),
        }
    };

    if let Err(_) = fs::write("/store/db-data.js", body){return "error writing db-data.js".into_response()};
    /* if let Err(e) = state.tx.send(utc_time){return "error broadcasting update".into_response()}; */
    "db updated".into_response()
}

pub async fn get_db(State(state): State<Arc<AppState>>) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let mut rx = state.tx.subscribe();
    let stream = async_stream::stream! {
        yield Ok(Event::default().data("push-update"));
        while let Ok(msg) = rx.recv().await {
            yield Ok(Event::default().data(msg));
        };
    };
    Sse::new(stream)
}
