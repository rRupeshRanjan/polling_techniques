use crate::counter::Counter;
use axum::response::IntoResponse;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::select;
use tokio::time::sleep;

#[derive(Serialize)]
struct CounterResponse {
    count: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum ClientMessage {
    Increment,
    Get,
}

pub async fn get_count(Extension(counter): Extension<Arc<Counter>>) -> impl IntoResponse {
    let value = counter.get();
    (StatusCode::OK, Json(CounterResponse { count: value })).into_response()
}

pub async fn increment(Extension(counter): Extension<Arc<Counter>>) -> impl IntoResponse {
    let value = counter.increment();
    (StatusCode::OK, Json(CounterResponse { count: value })).into_response()
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(counter): Extension<Arc<Counter>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, counter))
}

async fn handle_socket(mut socket: WebSocket, counter: Arc<Counter>) {
    let mut rx = counter.subscribe();

    // send current counter to new client
    let inital_value = counter.get();
    let _ = socket.send(get_message(inital_value)).await;

    loop {
        select! {
            // server-side -> push when counter changed
            changed = rx.changed() => {
                if changed.is_err() {
                    // sender dropped off
                    break;
                }

                let new = *rx.borrow();
                if socket.send(get_message(new)).await.is_err() {
                    // client disconnected
                    break;
                }
            }

            // client side -> handle incoming messages
            maybe_message = socket.recv() => {
                match maybe_message {
                    Some(Ok(Message::Text(text))) => match serde_json::from_str::<ClientMessage>(&text) {
                        Ok(ClientMessage::Increment) => {
                            counter.increment();
                        }
                        Ok(ClientMessage::Get) => {
                            let _ = socket.send(get_message(counter.get())).await;
                        },
                        Err(e) => {
                            eprintln!("Invalid message from client: {e}")
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }

            _ = sleep(Duration::from_millis(1)) => {}
        }
    }
}

fn get_message(count: u64) -> Message {
    Message::Text(
        serde_json::to_string(&CounterResponse { count })
            .unwrap()
            .into(),
    )
}
