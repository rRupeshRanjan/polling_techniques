use axum::{http::StatusCode, response::IntoResponse};

pub async fn index_html() -> impl IntoResponse {
    const HTML: &str = include_str!("../../static/index.html");
    (
        StatusCode::OK,
        [("content-type", "text/html; charset=utf-8")],
        HTML,
    )
}
