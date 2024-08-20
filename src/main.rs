use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use axum::{
    body::Bytes,
    extract::{Multipart, Query},
    http::{self, StatusCode},
    response::IntoResponse,
    routing::{get, post}, Json, Router,
};
use serde::Serialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // initializing step
    fs::create_dir_all("uploads").unwrap();

    let app = Router::new()
        .route("/", get(health_check))
        .route("/list", get(list_upload))
        .route("/download", get(download))
        .route("/upload", post(upload))
        .fallback(handler_404);

    // logging
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
async fn health_check() -> impl IntoResponse {
    tracing::info!("GET /");
    StatusCode::OK
}

// list uploaded files
async fn list_upload() -> impl IntoResponse {
    tracing::info!("GET /upload");
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

// 404 handler
async fn handler_404() -> impl IntoResponse {
    tracing::info!("404 Not Found");
    StatusCode::NOT_FOUND
}

// download file
async fn download(query: Query<HashMap<String, String>>) -> impl IntoResponse {
    tracing::info!("GET /download");
    let filename = match query.get("filename") {
        Some(filename) => filename,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    let upload_path = format!("uploads/{}", filename);
    let body = match fs::read(upload_path) {
        Ok(body) => body,
        _ => return Err(StatusCode::NOT_FOUND),
    };
    // set header
    let mut headers = http::HeaderMap::new();
    headers.insert(
        http::header::CONTENT_DISPOSITION,
        http::HeaderValue::from_str(&format!("attachment; filename={}", filename)).unwrap(),
    );
    Ok((headers, body))
}

async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    tracing::info!("PUT /upload");
    let field = match multipart.next_field().await.unwrap() {
        Some(field) => field,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let _file_name = field.file_name().unwrap().to_string();
    let mut data = field.bytes().await.unwrap();

    // 1/2の確率で中身を改竄する
    let temper = rand::random::<bool>();
    match temper {
        true => {
            tracing::info!("😈 The file is tempered!");
            let additonal_data = Bytes::from("Some additonal data");
            data = [data, additonal_data].concat().into();
        }
        false => {
            tracing::info!("👼 The file is not tempered!");
        }
    }

    let uuid = Uuid::new_v4().simple().to_string();

    let upload_path = format!("uploads/{}", uuid);
    let mut file = match File::create(upload_path) {
        Ok(file) => file,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    if file.write_all(&data).is_err() || file.flush().is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    #[derive(Serialize)]
    struct Response {
        upload_path: String,
    }
    tracing::info!("upload_path: {}", uuid);
    Ok((StatusCode::CREATED, Json(Response { upload_path: uuid })))
}
