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
use futures::stream;

pub async fn get_db(State(state): State<Arc<AppState>>) -> Sse<impl Stream<Item = Result<Event, std::io::Error>> + Send + 'static>{
    let rx = state.db_notifier.subscribe();

    let stream = stream::unfold(rx, |mut rx| async move{
        let data = rx.borrow().clone();
        let event = Event::default().data(data);
        let result = Ok(event);

        match rx.changed().await{
            Ok(()) => Some((result, rx)),
            Err(_) => None,
        }
    });

    Sse::new(stream)
}

pub async fn set_db(State(state): State<Arc<AppState>>, headers: HeaderMap, body: String) -> impl IntoResponse{
    if headers.get("authorization").and_then(|h| h.to_str().ok()) != env::var("UTC_KEY").ok().as_deref(){
        return "authorization error".into_response()
    };

    if let Err(_) = fs::write("/store/db-data.js", &body){
        return "error writing db-data.js".into_response()
    };

    let _ = state.db_notifier.send(body);
    "db updated".into_response()
}
