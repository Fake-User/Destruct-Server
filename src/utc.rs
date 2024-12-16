use futures::stream::Stream;
use std::sync::Arc;
use std::{fs, io, env};
use axum::{
    response::{Sse, IntoResponse},
    response::sse::Event,
    http::HeaderMap,
    extract::State
};

use crate::AppState;

pub async fn set_utc(State(state): State<Arc<AppState>>, headers: HeaderMap) -> impl IntoResponse{
    let utc_time = match headers.get("utcTime") {
        Some(header) => match header.to_str() {
            Ok(val) => val.to_string(),
            Err(_) => return "Invalid utcTime header".into_response(),
        },
        None => return "Missing utcTime header".into_response(),
    };

    let utc_key = match headers.get("utcKey") {
        Some(header) => match header.to_str() {
            Ok(val) => val.to_string(),
            Err(_) => return "Invalid utcKey header".into_response(),
        },
        None => return "Missing utcKey header".into_response(),
    };

    let env_key = match env::var("UTC_KEY") {
        Ok(key) => key,
        Err(_) => return "Server configuration error".into_response(),
    };

    if utc_key != env_key {
        return "Invalid UTC key".into_response();
    }

    if let Err(e) = fs::write("utc.txt", &utc_time) {
        eprintln!("Error writing to file: {}", e);
        return "Error writing to file".into_response();
    }

    if let Err(e) = state.tx.send(utc_time) {
        eprintln!("Error broadcasting update: {}", e);
    }

    "UTC updated successfully".into_response()
}

pub async fn get_utc(State(state): State<Arc<AppState>>) -> Sse<impl Stream<Item = Result<Event, io::Error>>> {
    let mut rx = state.tx.subscribe();
    let initial_value = fs::read_to_string("./store/utc.txt").unwrap_or_default();
    println!("{:#?}", initial_value);
    let stream = async_stream::stream! {
        yield Ok(Event::default().data(initial_value));
        while let Ok(msg) = rx.recv().await {
            yield Ok(Event::default().data(msg));
        };
    };
    Sse::new(stream)
}
