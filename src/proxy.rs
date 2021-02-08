use crate::constants::PROXY_URL;
use actix_web::{
    client::{Client, ClientBuilder, PayloadError, SendRequestError},
    http::{HeaderName, HeaderValue},
    web::Bytes,
};
use std::{
    env::var,
    fmt::{Display, Formatter, Result as FmtResult},
};

pub enum FetchError {
    Requesting(SendRequestError),
    Payload(PayloadError),
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FetchError::Requesting(e) => e.fmt(f),
            FetchError::Payload(e) => e.fmt(f),
        }
    }
}

impl From<SendRequestError> for FetchError {
    fn from(e: SendRequestError) -> FetchError {
        FetchError::Requesting(e)
    }
}

impl From<PayloadError> for FetchError {
    fn from(e: PayloadError) -> FetchError {
        FetchError::Payload(e)
    }
}

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new() -> Self {
        let mut client = ClientBuilder::new().header("accept", "application/json");
        if let Ok(key) = var("PROXY_AUTH") {
            client = client.header("proxy-authorization-key", key);
        }
        Self {
            client: client.finish(),
        }
    }

    pub async fn fetch(&self, url: &str) -> Result<Bytes, FetchError> {
        let req = {
            if let Some(proxy_url) = &*PROXY_URL {
                self.client.get(proxy_url).insert_header((
                    HeaderName::from_lowercase(b"requested-uri").unwrap(),
                    HeaderValue::from_str(&url).unwrap(),
                ))
            } else {
                self.client.get(url)
            }
        };
        Ok(req.send().await?.body().limit(1024 * 1024 * 3).await?)
    }
}

impl Default for Fetcher {
    fn default() -> Self {
        Self::new()
    }
}
