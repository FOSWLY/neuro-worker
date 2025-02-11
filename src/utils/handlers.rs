use axum::{
    body::Body,
    http::{HeaderMap, Response},
};
use reqwest::StatusCode;
use serde_json::Map;

pub fn return_error(message: &'static str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::NO_CONTENT)
        .header("X-Yandex-Status", message)
        .body(Body::from(""))
        .unwrap()
}

pub fn parse_request(
    headers: HeaderMap,
    body: String,
) -> Result<(serde_json::Value, Map<String, serde_json::Value>), Response<Body>> {
    let content_type = headers.get("content-type");
    if content_type.is_none() || content_type.unwrap() != "application/json" {
        return Err(return_error("error-content"));
    }

    let body_result = serde_json::from_str::<serde_json::Value>(&body);
    if body_result.is_err() {
        return Err(return_error("error-request"));
    }

    let body_info = body_result.unwrap();
    let yandex_body = &body_info["body"];
    let yandex_headers = &body_info["headers"].as_object();
    if yandex_body == &serde_json::Value::Null || yandex_headers.is_none() {
        return Err(return_error("error-request"));
    }

    return Ok((yandex_body.clone(), yandex_headers.unwrap().clone()));
}
