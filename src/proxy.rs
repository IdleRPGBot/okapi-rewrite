use crate::{
    constants::{PROXY_PASSWORD, PROXY_URL, PROXY_USER},
    error::{Error, Result},
};

use bytes::Bytes;
use hyper::{
    body::{to_bytes, HttpBody},
    client::HttpConnector,
    Client, Uri,
};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};

use std::str::FromStr;

pub struct Fetcher {
    client: Client<ProxyConnector<HttpConnector>>,
}

impl Fetcher {
    #[must_use]
    pub fn new() -> Self {
        let mut proxy = Proxy::new(Intercept::All, PROXY_URL.clone());
        proxy.force_connect();
        proxy.set_authorization(&PROXY_USER, &PROXY_PASSWORD);

        let connector = HttpConnector::new();
        let proxy_connector = ProxyConnector::from_proxy(connector, proxy).unwrap();

        let client = Client::builder().build(proxy_connector);

        Self { client }
    }

    pub async fn fetch(&self, url: &str) -> Result<Bytes> {
        let response = self.client.get(Uri::from_str(url)?).await?;
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
