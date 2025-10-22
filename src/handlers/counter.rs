use crate::counter::Counter;
use axum::response::IntoResponse;

use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

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

pub async fn wait_for_change(Extension(counter): Extension<Arc<Counter>>) -> impl IntoResponse {
    let mut rx = counter.subscribe();

    let current_value = counter.get();

    let result = timeout(Duration::from_secs(10), async move {
        loop {
            rx.changed().await.unwrap();
            let new_value = *rx.borrow();

            if new_value != current_value {
                return new_value;
            }
        }
    })
    .await;

    match result {
        Ok(new_value) => {
            (StatusCode::OK, Json(CounterResponse { count: new_value })).into_response()
        }
        Err(_) => (
            StatusCode::OK,
            Json(CounterResponse {
                count: current_value,
            }),
        )
            .into_response(),
    }
}
