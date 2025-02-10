use futures::{stream, stream::{Stream, StreamExt}};
use std::sync::Arc;
use std::io::Write;
use axum::{
    response::{sse::Event, Sse, IntoResponse},
    http::HeaderMap,
    extract::State
};
use crate::AppState;

pub async fn get_db(
    State(state): State<Arc<AppState>>, uri: axum::http::Uri) -> Sse<impl Stream<Item = Result<Event, std::io::Error>> + Send> {
    let client_utc = uri.query().unwrap_or("").replace("%20", " ").to_string();
    let rx = state.db_update.subscribe();
    let initial = if client_utc < *state.db_utc.read().unwrap(){Some(Ok(Event::default().data(rx.borrow().clone())))}
    else{None};

    let stream = futures::stream::iter(initial).chain(stream::unfold(rx, |mut rx| async move{
        match rx.changed().await{
            Ok(()) => {
                let data = rx.borrow().clone();
                Some((Ok(Event::default().data(data)), rx))
            }
            Err(_) => None,
        }
    }));

    Sse::new(stream)
}

pub async fn set_db(State(state): State<Arc<AppState>>, headers: HeaderMap, body: String) -> impl IntoResponse{
    if headers.get("authorization").and_then(|h| h.to_str().ok()) != std::env::var("UTC_KEY").ok().as_deref(){return "authorization error".into_response()};
    let utc = body
        .clone()
        .split_once('"')
        .unwrap()
        .1
        .split_once('"')
        .unwrap()
        .0
        .to_string();
    *state.db_utc.write().unwrap() = utc;
    let mut file = std::fs::File::create("store/db-data.js").unwrap();
    file.write_all(body.as_bytes()).unwrap();
    let _ = state.db_update.send(body);
    "db updated".into_response()
}
