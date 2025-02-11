mod api;
mod data;
mod routes;
mod utils;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

use crate::data::config::CONFIG;

#[tokio::main]
async fn main() {
    let th_api_routes = Router::new()
        .route("/sharing-url", post(routes::thapi::post_sharing_url))
        .route(
            "/neuro/generation",
            post(routes::thapi::post_neuro_generation),
        )
        .route("/generation", post(routes::thapi::post_generation));
    let browser_routes = Router::new()
        .route(
            "/session/create",
            post(routes::browser::post_session_create),
        )
        .route(
            "/video-summary/generation",
            post(routes::browser::post_video_summary_generation),
        );
    let app = Router::new()
        .route("/health", get(routes::health::get_health))
        .nest("/th/api", th_api_routes)
        .nest("/browser", browser_routes)
        .fallback(routes::fallback::fallback)
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .max_age(Duration::from_secs(86400)),
        );
    let listener = tokio::net::TcpListener::bind(format!("{0}:{1}", CONFIG.hostname, CONFIG.port))
        .await
        .unwrap();
    println!("🦀 Axum is running at {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
