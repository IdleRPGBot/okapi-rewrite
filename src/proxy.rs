use crate::{
    constants::{PROXY_AUTH, PROXY_URL},
    error::{Error, Result},
};

use bytes::Bytes;
use hyper::{
    body::{to_bytes, HttpBody},
    client::HttpConnector,
    Body, Client, Request, Uri,
};
use hyper_rustls::HttpsConnector;

use std::str::FromStr;

pub struct Fetcher {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Fetcher {
    #[must_use]
    pub fn new() -> Self {
        let client = Client::builder().build(HttpsConnector::with_webpki_roots());
        Self { client }
    }

    pub async fn upload_images(&self, images: Vec<Vec<u8>>) -> Result<Bytes> {
        let json = serde_json::to_string(&images)?;
        let request = Request::post("http://localhost:9511")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                &*PROXY_AUTH.as_ref().map(String::as_str).unwrap_or_default(),
            )
            .body(Body::from(json))?;
        let response = self.client.request(request).await?;

        Ok(hyper::body::to_bytes(response).await?)
    }

    pub async fn fetch(&self, url: &str) -> Result<Bytes> {
        let reponse_future = {
            if let (Some(proxy_url), Some(proxy_auth)) = (&*PROXY_URL, &*PROXY_AUTH) {
                self.client.request(
                    Request::get(proxy_url)
                        .header("accept", "application/json")
                        .header("proxy-authorization-key", proxy_auth)
                        .header("requested-uri", url)
                        .body(Body::empty())?,
                )
            } else {
                self.client.get(Uri::from_str(url)?)
            }
        };
        let response = reponse_future.await?;
        let size = response.size_hint().exact();

        if size.is_some() && size.unwrap() < 1024 * 1024 * 3 {
            Ok(to_bytes(response).await?)
        } else {
            Err(Error::PayloadTooBig)
        }
    }
}

impl Default for Fetcher {
    fn default() -> Self {
        Self::new()
    }
}
