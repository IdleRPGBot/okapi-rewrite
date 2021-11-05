use hyper::{Body, Response};

use crate::error::Result;

pub fn index() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from("1"))?)
}
