use hyper::{Body, Response};

#[must_use]
pub fn index() -> Response<Body> {
    // For metrics
    Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from("1"))
        .unwrap()
}
