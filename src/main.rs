use axum::{routing::get, Router};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

mod error;
mod handlers;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() {
    acap_logging::init_logger();

    let client = acap_vapix::local_client().unwrap();
    let app = Router::new()
        .route(
            &format!("/local/{APP_NAME}/vapix/{{*path}}"),
            get(handlers::relay_request),
        )
        .layer(
            TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().include_headers(true)),
        )
        .with_state(client);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:2001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
