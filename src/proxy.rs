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
    pub fn new() -> Self {
        let client = Client::builder().build(HttpsConnector::with_webpki_roots());
        Self { client }
    }

    pub async fn fetch(&self, url: &str) -> Result<Bytes, FetchError> {
        let req = {
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
        let res = req.await?;
        let size = res.size_hint().exact();

        if size.is_some() && size.unwrap() < 1024 * 1024 * 3 {
            Ok(to_bytes(res).await?)
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
