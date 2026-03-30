// src/api/middleware.rs
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            "https://hubstry.com.br".parse().unwrap(),
            "https://app.hubstry.com.br".parse().unwrap(),
            "http://localhost:3000".parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
}
