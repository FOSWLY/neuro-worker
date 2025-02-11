use axum::body::Body;
use axum::http::HeaderMap;
use axum::http::Response;
use axum::response::IntoResponse;

use crate::api;
use crate::utils::handlers::{parse_request, return_error};

pub async fn request(pathname: &str, headers: HeaderMap, body: String) -> Response<Body> {
    let (yandex_body, yandex_headers) = match parse_request(headers, body) {
        Ok((body, headers)) => (body, headers),
        Err(err) => return err,
    };

    let data = api::thapi::request(pathname, &yandex_body, &yandex_headers).await;
    if data.is_err() {
        return return_error("error-internal");
    }

    data.unwrap()
}

pub async fn post_sharing_url(headers: HeaderMap, body: String) -> impl IntoResponse {
    request("/api/sharing-url", headers, body).await
}

pub async fn post_neuro_generation(headers: HeaderMap, body: String) -> impl IntoResponse {
    request("/api/neuro/generation", headers, body).await
}

pub async fn post_generation(headers: HeaderMap, body: String) -> impl IntoResponse {
    request("/api/generation", headers, body).await
}
