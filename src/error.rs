use hyper::{Body, Response, StatusCode};

#[derive(Debug)]
pub enum Error {
    Image(image::ImageError),
    Hyper(hyper::Error),
    Http(hyper::http::Error),
    Svg(resvg::usvg::Error),
    Json(simd_json::Error),
    PayloadTooBig,
    ImageTooSmall,
    InvalidUri(hyper::http::uri::InvalidUri),
    Io(std::io::Error),
    InvalidImageHost,
    Ratelimited,
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Self::Hyper(err)
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        Self::Image(err)
    }
}

impl From<hyper::http::Error> for Error {
    fn from(err: hyper::http::Error) -> Self {
        Self::Http(err)
    }
}

impl From<resvg::usvg::Error> for Error {
    fn from(err: resvg::usvg::Error) -> Self {
        Self::Svg(err)
    }
}

impl From<simd_json::Error> for Error {
    fn from(err: simd_json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<hyper::http::uri::InvalidUri> for Error {
    fn from(err: hyper::http::uri::InvalidUri) -> Self {
        Self::InvalidUri(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl Error {
    #[must_use]
    pub fn into_response(&self) -> Response<Body> {
        match self {
            Self::Svg(err) => {
                Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"invalid SVG data\", \"detail\": \"{err}\"}}"
                ))).unwrap()
            }
            Self::Hyper(err) => {
                Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"download error\", \"detail\": \"{err}\"}}"
                )))
                .unwrap()
            }
            Self::Image(err) => {
                Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{err}\"}}"
                )))
                .unwrap()
            }
            Self::Io(err) => {
                Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{err}\"}}"
                )))
                .unwrap()
            }
            Self::Json(err) => {
                Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"invalid request JSON data\", \"detail\": \"{err}\"}}"
                )))
                .unwrap()
            }
            Self::ImageTooSmall => {
                Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(String::from(
                    "{\"status\": \"error\", \"reason\": \"background image too small\", \"detail\": \"background image must be at least 800x533 in size\"}",
                )))
                .unwrap()
            }
            Self::InvalidImageHost => {
                Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(String::from(
                    "{\"status\": \"error\", \"reason\": \"invalid image host\", \"detail\": \"custom backgrounds must be hosted on a trusted domain\"}",
                )))
                .unwrap()
            }
            Self::Ratelimited => {
                Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(String::from(
                    "{\"status\": \"error\", \"reason\": \"ratelimited\", \"detail\": \"the custom background image host has ratelimited or banned our IP\"}",
                )))
                .unwrap()
            }
            _ => {
                Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(Body::from(String::from("{\"status\": \"error\", \"reason\": \"unknown\", \"detail\": \"\"}")))
                .unwrap()
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
