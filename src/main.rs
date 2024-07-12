#![allow(unused)]

use axum::{
    Router,
    routing::get,
    response::sse::{Event, KeepAlive, Sse},
};
use std::{time::Duration, convert::Infallible};
use tokio_stream::StreamExt as _ ;
use futures_util::stream::{self, Stream};

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/sse", get(sse_handler));

    async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
        // A `Stream` that repeats an event every second
        let stream = stream::repeat_with(|| Event::default().data("hi!"))
            .map(Ok)
            .throttle(Duration::from_secs(1));

        Sse::new(stream).keep_alive(KeepAlive::default())
    }
}
