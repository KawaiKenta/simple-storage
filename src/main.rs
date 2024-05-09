use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use axum::{
    body::Bytes,
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    fs::create_dir_all("uploads").unwrap();

    // 404 handler
    let app = Router::new()
        .route("/", get(health_check))
        .route("/upload", put(upload_file))
        .route("/upload", get(list_upload));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// health check
#[tracing::instrument]
async fn health_check() -> impl IntoResponse {
    tracing::info!("");
    StatusCode::OK
}

// upload file
#[tracing::instrument]
async fn upload_file(
    Query(query): Query<HashMap<String, String>>,
    body: Bytes,
) -> impl IntoResponse {
    let filename = match query.get("filename") {
        Some(filename) => filename,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    let upload_path = format!("uploads/{}", filename);
    let mut file = match File::create(upload_path) {
        Ok(file) => file,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    if file.write_all(&body).is_err() || file.flush().is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    Ok(StatusCode::CREATED)
}

#[tracing::instrument]
async fn list_upload() -> impl IntoResponse {
    let files: Vec<String> = match fs::read_dir("uploads") {
        Ok(files) => files
            .filter_map(Result::ok)
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect(),
        _ => {
            return axum::Json(Vec::new());
        }
    };
    axum::Json(files)
}
