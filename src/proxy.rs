use crate::constants::{CLIENT, HEADERS, PROXY_URL};
use actix_web::web::Bytes;
use reqwest::{header::HeaderName, Error};
use std::time::Duration;

pub async fn fetch(url: &str) -> Result<Bytes, Error> {
    let mut headers = HEADERS.clone();
    let url_to_call: String = match &*PROXY_URL {
        Some(url) => {
            let requested_uri = HeaderName::from_lowercase(b"requested-uri").unwrap();
            headers.insert(requested_uri, url.parse().unwrap());
            url.to_string()
        }
        None => url.to_string(),
    };
    let resp = CLIENT
        .get(&url_to_call)
        .headers(headers)
        .timeout(Duration::new(3, 0))
        .send()
        .await?
        .bytes()
        .await?;
    Ok(resp)
}
