use bytes::Buf;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Error, Method, Request, Response, Server, StatusCode,
};
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
pub mod proxy;
pub mod routes;

async fn handle(req: Request<Body>, fetcher: Arc<proxy::Fetcher>) -> Result<Response<Body>, Error> {
    let start = Instant::now();
    let path = req.uri().path().to_owned();
    let method = req.method().to_owned();
    let body = hyper::body::aggregate(req).await?;
    let reader = body.reader();

    let res: Result<Response<Body>, serde_json::Error> = async {
        match (&method, path.as_str()) {
            (&Method::POST, "/api/genadventures") => {
                Ok(genadventures(serde_json::from_reader(reader)?))
            }
            (&Method::POST, "/api/genchess") => Ok(genchess(serde_json::from_reader(reader)?)),
            (&Method::POST, "/api/imageops/pixel") => {
                Ok(pixelate(serde_json::from_reader(reader)?, fetcher).await)
            }
            (&Method::POST, "/api/imageops/invert") => {
                Ok(invert_endpoint(serde_json::from_reader(reader)?, fetcher).await)
            }
            (&Method::POST, "/api/imageops/edges") => {
                Ok(edges_endpoint(serde_json::from_reader(reader)?, fetcher).await)
            }
            (&Method::POST, "/api/imageops/oil") => {
                Ok(oil_endpoint(serde_json::from_reader(reader).unwrap(), fetcher).await)
            }
            (&Method::GET, "/") => Ok(index()),
            (&Method::POST, "/api/genoverlay") => {
                Ok(genoverlay(serde_json::from_reader(reader)?, fetcher).await)
            }
            (&Method::POST, "/api/genprofile") => {
                Ok(genprofile(serde_json::from_reader(reader)?, fetcher).await)
            }
            _ => Ok(Response::builder().status(404).body(Body::empty()).unwrap()),
        }
    }
    .await;

    let resp = match res {
        Ok(r) => r,
        Err(e) => Response::builder()
            .status(StatusCode::UNPROCESSABLE_ENTITY)
            .header("content-type", "application/json")
            .body(Body::from(format!(
                "{{\"status\": \"error\", \"detail\": \"{}\"}}",
                e
            )))
            .unwrap(),
    };

    let end = Instant::now();

    info!("{} {} {} {:?}", method, path, resp.status(), end - start);

    Ok(resp)
}

#[tokio::main]
async fn main() {
    set_var("RUST_LOG", "info");
    env_logger::init();
    let listen_address = match var("PORT") {
        Ok(p) => SocketAddr::from(([0, 0, 0, 0], p.parse().unwrap())),
        Err(_) => SocketAddr::from(([0, 0, 0, 0], 3000)),
    };

    let client = Arc::new(proxy::Fetcher::new());

    let make_service = make_service_fn(|_conn| {
        let client = client.clone();
        async { Ok::<_, Infallible>(service_fn(move |req| handle(req, client.clone()))) }
    });

    let server = Server::bind(&listen_address).serve(make_service);

    info!("okapi starting on {}", listen_address);

    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
}
