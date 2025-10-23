use crate::counter::Counter;
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Sse};

use axum::extract::Extension;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tokio_stream::wrappers::WatchStream;
use tokio_stream::StreamExt;

#[derive(Serialize)]
struct CounterResponse {
    count: u64,
}

pub async fn get_count(Extension(counter): Extension<Arc<Counter>>) -> impl IntoResponse {
    let value = counter.get();
    (StatusCode::OK, Json(CounterResponse { count: value })).into_response()
}

pub async fn increment(Extension(counter): Extension<Arc<Counter>>) -> impl IntoResponse {
    let value = counter.increment();
    (StatusCode::OK, Json(CounterResponse { count: value })).into_response()
}

pub async fn counter_events(Extension(counter): Extension<Arc<Counter>>) -> impl IntoResponse {
    let rx = counter.subscribe();

    let stream = WatchStream::new(rx).map(|value| {
        let json = serde_json::json!({"count": value});
        Ok::<Event, std::convert::Infallible>(Event::default().data(json.to_string()))
    });

    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(10)))
}
