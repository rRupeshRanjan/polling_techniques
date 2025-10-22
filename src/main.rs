mod counter;
mod handlers;

use counter::Counter;
use handlers::counter::{get_count, increment};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use axum::routing::get;
use axum::{extract::Extension, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Counter::new());

    let cors_layer = CorsLayer::new()
        .allow_origin(Any) // allow requests from any origin
        .allow_methods(Any) // allow GET/POST/etc
        .allow_headers(Any);

    let app = Router::new()
        .route("/get", get(get_count))
        .route("/increment", get(increment))
        .layer(Extension(counter.clone()))
        .layer(cors_layer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Starting server on http://{}", addr);

    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");

        println!("Shutdown signal received, shutting down gracefully...");
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .unwrap();
}
