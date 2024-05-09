use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new().route("/", get(hello));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[tracing::instrument]
async fn hello() -> impl IntoResponse {
    let me = User {
        id: 42,
        username: "foo".to_string(),
    };
    tracing::info!("hello from {}", me.username);
    let response: Vec<User> = vec![me];
    Json(response)
}
