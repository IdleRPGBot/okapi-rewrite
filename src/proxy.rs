use crate::constants::{CLIENT, CONFIG, HEADERS};
use actix_web::web::Bytes;
use reqwest::header::HeaderName;
use std::time::Duration;

pub async fn fetch(url: &str) -> Bytes {
    let mut headers = HEADERS.clone();
    let requested_uri = HeaderName::from_lowercase(b"requested-uri").unwrap();
    let proxy_base = CONFIG["proxy"].as_str().unwrap();
    headers.insert(requested_uri, url.parse().unwrap());
    let resp = CLIENT
        .get(proxy_base)
        .headers(headers)
        .timeout(Duration::new(3, 0))
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    resp
}
