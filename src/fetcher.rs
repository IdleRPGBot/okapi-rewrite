use std::str::FromStr;

use bytes::Bytes;
use hyper::{
    body::{to_bytes, HttpBody},
    client::HttpConnector,
    Client, Uri,
};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

use crate::error::{Error, Result};

const ALLOWED_HOSTS: &[&str] = &["idlerpg.xyz", "i.imgur.com", "i.postimg.cc"];

pub struct Fetcher {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Fetcher {
    #[must_use]
    pub fn new() -> Self {
        let connector = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_only()
            .enable_all_versions()
            .build();
        let client = Client::builder().build(connector);

        Self { client }
    }

    pub async fn fetch(&self, url: &str) -> Result<Bytes> {
        let parsed_uri = Uri::from_str(url)?;
        let host = parsed_uri.host().ok_or(Error::InvalidImageHost)?;

        if !ALLOWED_HOSTS
            .iter()
            .any(|allowed_host| *allowed_host == host)
        {
            return Err(Error::InvalidImageHost);
        }

        let response = self.client.get(parsed_uri).await?;

        if response.status() == 429 {
            return Err(Error::Ratelimited);
        }

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
