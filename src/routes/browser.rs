use axum::body::Body;
use axum::http::HeaderMap;
use axum::http::Response;
use axum::response::IntoResponse;

use crate::api;
use crate::utils::{
    handlers::{parse_request, return_error},
    utils::value_to_bytes,
};

async fn request(pathname: &str, headers: HeaderMap, body: String) -> Response<Body> {
    let (yandex_body, yandex_headers) = match parse_request(headers, body) {
        Ok((body, headers)) => (body, headers),
        Err(err) => return err,
    };

    let yandex_u8_body = value_to_bytes(&yandex_body);
    if yandex_u8_body.is_none() {
        return return_error("error-request");
    }

    let data = api::browser::request(pathname, yandex_u8_body.unwrap(), &yandex_headers).await;
    if data.is_err() {
        return return_error("error-internal");
    }

    data.unwrap()
}

pub async fn post_session_create(headers: HeaderMap, body: String) -> impl IntoResponse {
    request("/session/create", headers, body).await
}

pub async fn post_video_summary_generation(headers: HeaderMap, body: String) -> impl IntoResponse {
    request("/video-summary/generation", headers, body).await
}
