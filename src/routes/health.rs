use axum::{response::IntoResponse, Json};
use serde::Serialize;

use crate::data::config;

#[derive(Serialize)]
pub struct Health {
    version: String,
    status: String,
}

pub async fn get_health() -> impl IntoResponse {
    let data = Health {
        status: "ok".to_string(),
        version: config::CONFIG.version.clone(),
    };

    Json(data)
}
