use crate::counter::Counter;
use axum::response::IntoResponse;

use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct CounterResponse {
    count: u64,
}

use axum::extract::Extension;

pub async fn get_count(Extension(counter): Extension<Arc<Counter>>) -> impl IntoResponse {
    let value = counter.get();
    (StatusCode::OK, Json(CounterResponse { count: value })).into_response()
}

pub async fn increment(Extension(counter): Extension<Arc<Counter>>) -> impl IntoResponse {
    let value = counter.increment();
    (StatusCode::OK, Json(CounterResponse { count: value })).into_response()
}
