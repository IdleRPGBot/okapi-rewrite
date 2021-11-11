use hyper::{Body, Response};

use crate::error::Result;

pub fn index() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from("Disclaimer: The visual content hosted by this website is user-generated. Each material is purged from our servers within 15 minutes. Therefore we cannot take any responsibility for the uploaded user content. In case of copyright infringement contacting is possible thru support.idlerpg.xyz."))?)
}
