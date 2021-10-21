use crate::error::Result;

use hyper::{Body, Response};

pub fn index() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from("1"))?)
}
