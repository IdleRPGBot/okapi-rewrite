use hyper::{Body, Response, StatusCode};

#[derive(Debug)]
pub enum Error {
    Image(image::ImageError),
    Hyper(hyper::Error),
    Http(hyper::http::Error),
    Svg(usvg::Error),
    Json(serde_json::Error),
    PayloadTooBig,
    ImageTooSmall,
    InvalidUri(hyper::http::uri::InvalidUri),
    Io(std::io::Error),
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

impl From<usvg::Error> for Error {
    fn from(err: usvg::Error) -> Self {
        Self::Svg(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
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
                    "{{\"status\": \"error\", \"reason\": \"invalid SVG data\", \"detail\": \"{}\"}}",
                    err
                ))).unwrap()
            }
            Self::Hyper(err) => {
                Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"download error\", \"detail\": \"{}\"}}",
                    err
                )))
                .unwrap()
            }
            Self::Image(err) => {
                Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{}\"}}",
                    err
                )))
                .unwrap()
            }
            Self::Io(err) => {
                Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{}\"}}",
                    err
                )))
                .unwrap()
            }
            Self::Json(err) => {
                Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"invalid request JSON data\", \"detail\": \"{}\"}}",
                    err
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
