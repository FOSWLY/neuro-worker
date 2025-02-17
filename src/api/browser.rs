use axum::{
    body::Body,
    http::{HeaderMap, HeaderName, HeaderValue, Response},
};
use lazy_static::lazy_static;
use reqwest::{Client, Error};
use serde_json::Map;

lazy_static! {
    static ref REQ_CLIENT: Client = reqwest::Client::new();
}

pub async fn request(
    pathname: &str,
    body: Vec<u8>,
    headers_data: &Map<String, serde_json::Value>,
) -> Result<Response<Body>, Error> {
    let request_url = format!("https://api.browser.yandex.ru{pathname}");
    let mut headers = HeaderMap::new();
    for (key, value) in headers_data {
        if !value.is_string() {
            continue;
        }

        if let (Ok(header_name), Some(value_str)) = (key.parse::<HeaderName>(), value.as_str()) {
            if let Ok(header_value) = HeaderValue::from_str(value_str) {
                headers.insert(header_name, header_value);
            }
        }
    }

    let res = REQ_CLIENT
        .post(&request_url)
        .headers(headers)
        .body(reqwest::Body::from(body))
        .send()
        .await?;

    let status = res.status();
    let mut headers = res.headers().clone();
    headers.append("X-Yandex-Status", HeaderValue::from_str("success").unwrap());
    let bytes = res.bytes().await?;

    let mut response = Response::new(Body::from(bytes));
    *response.status_mut() = status;
    *response.headers_mut() = headers;

    Ok(response)
}
