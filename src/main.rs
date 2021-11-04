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
use routes::{
    adventures::genadventures,
    chess::genchess,
    imageops::{edges_endpoint, invert_endpoint, oil_endpoint, pixelate},
    index::index,
    overlay::genoverlay,
    profile::genprofile,
};

use std::{
    convert::Infallible,
    env::{set_var, var},
    net::SocketAddr,
    sync::Arc,
    time::Instant,
};

pub mod constants;
pub mod encoder;
pub mod error;
pub mod proxy;
pub mod routes;
pub mod webp;

async fn handle(
    request: Request<Body>,
    fetcher: Arc<proxy::Fetcher>,
) -> Result<Response<Body>, Error> {
    let start = Instant::now();
    let path = request.uri().path().to_owned();
    let method = request.method().clone();
    let body = hyper::body::aggregate(request).await?;
    let reader = body.reader();

    let response: error::Result<Response<Body>> = async {
        match (&method, path.as_str()) {
            (&Method::POST, "/api/genadventures") => {
                genadventures(&serde_json::from_reader(reader)?, fetcher).await
            }
            (&Method::POST, "/api/genchess") => genchess(&serde_json::from_reader(reader)?),
            (&Method::POST, "/api/imageops/pixel") => {
                pixelate(serde_json::from_reader(reader)?, fetcher).await
            }
            (&Method::POST, "/api/imageops/invert") => {
                invert_endpoint(serde_json::from_reader(reader)?, fetcher).await
            }
            (&Method::POST, "/api/imageops/edges") => {
                edges_endpoint(serde_json::from_reader(reader)?, fetcher).await
            }
            (&Method::POST, "/api/imageops/oil") => {
                oil_endpoint(serde_json::from_reader(reader)?, fetcher).await
            }
            (&Method::GET, "/") => index(),
            (&Method::POST, "/api/genoverlay") => {
                genoverlay(serde_json::from_reader(reader)?, fetcher).await
            }
            (&Method::POST, "/api/genprofile") => {
                genprofile(serde_json::from_reader(reader)?, fetcher).await
            }
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
    let listen_address = match var("PORT") {
        Ok(p) => SocketAddr::from(([0, 0, 0, 0], p.parse().unwrap())),
        Err(_) => SocketAddr::from(([0, 0, 0, 0], 3000)),
    };
    info!("okapi starting on {}", listen_address);

    let client = Arc::new(proxy::Fetcher::new());

    let make_service = make_service_fn(|_conn| {
        let client = client.clone();
        async { Ok::<_, Infallible>(service_fn(move |req| handle(req, client.clone()))) }
    });

    let server = Server::bind(&listen_address).serve(make_service);

    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
}
