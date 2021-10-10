use crate::constants::{PROXY_AUTH, PROXY_URL};
use bytes::Bytes;
use hyper::{
    body::{to_bytes, HttpBody},
    client::HttpConnector,
    Body, Client, Error, Request, Uri,
};
use hyper_rustls::HttpsConnector;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

pub enum FetchError {
    HyperError(Error),
    PayloadTooBig,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FetchError::HyperError(e) => e.fmt(f),
            FetchError::PayloadTooBig => f.write_str("payload too big"),
        }
    }
}

impl From<Error> for FetchError {
    fn from(e: Error) -> FetchError {
        FetchError::HyperError(e)
    }
}

pub struct Fetcher {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Fetcher {
    #[must_use]
    pub fn new() -> Self {
        let client = Client::builder().build(HttpsConnector::with_webpki_roots());
        Self { client }
    }

    pub async fn upload_images(&self, images: Vec<Vec<u8>>) -> Bytes {
        let json = serde_json::to_string(&images).unwrap();
        let request = Request::post("http://localhost:9511")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                &*PROXY_AUTH.as_ref().map(|s| s.as_str()).unwrap_or_default(),
            )
            .body(Body::from(json))
            .unwrap();
        let response = self.client.request(request).await.unwrap();
        hyper::body::to_bytes(response).await.unwrap()
    }

    pub async fn fetch(&self, url: &str) -> Result<Bytes, FetchError> {
        let reponse_future = {
            if let (Some(proxy_url), Some(proxy_auth)) = (&*PROXY_URL, &*PROXY_AUTH) {
                self.client.request(
                    Request::get(proxy_url)
                        .header("accept", "application/json")
                        .header("proxy-authorization-key", proxy_auth)
                        .header("requested-uri", url)
                        .body(Body::empty())
                        .unwrap(),
                )
            } else {
                self.client.get(Uri::from_str(url).unwrap())
            }
        };
        let response = reponse_future.await?;
        let size = response.size_hint().exact();

        if size.is_some() && size.unwrap() < 1024 * 1024 * 3 {
            Ok(to_bytes(response).await?)
        } else {
            Err(FetchError::PayloadTooBig)
        }
    }
}

impl Default for Fetcher {
    fn default() -> Self {
        Self::new()
    }
}
