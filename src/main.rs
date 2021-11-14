#![deny(clippy::pedantic)]
#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::too_many_lines,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]
use bytes::Buf;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Error, Method, Request, Response, Server,
};
use libc::{c_int, sighandler_t, signal, SIGINT, SIGTERM};
use log::{error, info};
use serde::Deserialize;

use std::{convert::Infallible, env::set_var, net::SocketAddr, sync::Arc, time::Instant};

use crate::{
    cache::ImageCache,
    constants::PORT,
    routes::{
        adventures::genadventures,
        chess::genchess,
        imageops::{edges_endpoint, invert_endpoint, oil_endpoint, pixelate},
        index::index,
        overlay::genoverlay,
        profile::genprofile,
    },
};

pub mod cache;
pub mod constants;
pub mod encoder;
pub mod error;
pub mod proxy;
pub mod routes;

#[derive(Deserialize)]
struct GetImage {
    image: String,
}

async fn handle(
    request: Request<Body>,
    fetcher: Arc<proxy::Fetcher>,
    images: ImageCache,
) -> Result<Response<Body>, Error> {
    let start = Instant::now();

    let (parts, body) = request.into_parts();

    let body = hyper::body::aggregate(body).await?;
    let reader = body.reader();

    let path = parts.uri.path();
    let query = parts.uri.query();
    let method = parts.method;

    if method == Method::POST {
        if let Some(server_auth) = constants::AUTH_KEY.as_ref() {
            if let Some(client_auth) = parts.headers.get("authorization") {
                if client_auth != server_auth {
                    return Ok(Response::builder().status(403).body(Body::empty()).unwrap());
                }
            } else {
                return Ok(Response::builder().status(403).body(Body::empty()).unwrap());
            }
        }
    }

    let response: error::Result<Response<Body>> = async {
        match (&method, path) {
            (&Method::POST, "/api/genadventures") => {
                genadventures(&serde_json::from_reader(reader)?, images).await
            }
            (&Method::POST, "/api/genchess") => genchess(&serde_json::from_reader(reader)?, images),
            (&Method::POST, "/api/imageops/pixel") => {
                pixelate(serde_json::from_reader(reader)?, fetcher, images).await
            }
            (&Method::POST, "/api/imageops/invert") => {
                invert_endpoint(serde_json::from_reader(reader)?, fetcher, images).await
            }
            (&Method::POST, "/api/imageops/edges") => {
                edges_endpoint(serde_json::from_reader(reader)?, fetcher, images).await
            }
            (&Method::POST, "/api/imageops/oil") => {
                oil_endpoint(serde_json::from_reader(reader)?, fetcher, images).await
            }
            (&Method::POST, "/api/genoverlay") => {
                genoverlay(serde_json::from_reader(reader)?, fetcher, images).await
            }
            (&Method::POST, "/api/genprofile") => {
                genprofile(serde_json::from_reader(reader)?, fetcher, images).await
            }
            (&Method::GET, "/") => index(),
            (&Method::GET, "/image") => match query {
                Some(query) => match serde_urlencoded::from_str::<GetImage>(query) {
                    Ok(get_image) => match images.get(&get_image.image) {
                        Some(image) => Ok(Response::builder()
                            .status(200)
                            .header("Content-Type", "image/png")
                            .body(Body::from(image))
                            .unwrap()),
                        None => Ok(Response::builder().status(404).body(Body::empty()).unwrap()),
                    },
                    Err(_) => Ok(Response::builder().status(400).body(Body::empty()).unwrap()),
                },
                None => Ok(Response::builder().status(400).body(Body::empty()).unwrap()),
            },
            _ => Ok(Response::builder().status(404).body(Body::empty()).unwrap()),
        }
    }
    .await;

    let resp = match response {
        Ok(r) => r,
        Err(e) => {
            error!("{:?}", e);
            e.into_response()
        }
    };

    let end = Instant::now();

    info!("{} {} {} {:?}", method, path, resp.status(), end - start);

    Ok(resp)
}

pub extern "C" fn handler(_: c_int) {
    std::process::exit(0);
}

unsafe fn set_os_handlers() {
    signal(SIGINT, handler as extern "C" fn(_) as sighandler_t);
    signal(SIGTERM, handler as extern "C" fn(_) as sighandler_t);
}

#[tokio::main]
async fn main() {
    unsafe { set_os_handlers() };

    set_var("RUST_LOG", "info");

    env_logger::init();

    let listen_address = SocketAddr::from(([0, 0, 0, 0], *PORT));

    info!("okapi starting on {}", listen_address);

    let client = Arc::new(proxy::Fetcher::new());
    let images = ImageCache::new();

    let make_service = make_service_fn(|_conn| {
        let client = client.clone();
        let images = images.clone();

        async {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle(req, client.clone(), images.clone())
            }))
        }
    });

    let server = Server::bind(&listen_address).serve(make_service);

    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
}
