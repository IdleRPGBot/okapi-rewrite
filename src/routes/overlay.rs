use crate::{constants::PROFILE, encoder::encode_png, proxy::Fetcher};
use base64::encode;
use hyper::{Body, Response, StatusCode};
use image::{
    imageops::{overlay, resize, FilterType},
    io::Reader,
};
use serde::Deserialize;

use std::{io::Cursor, sync::Arc};

#[derive(Deserialize)]
pub struct OverlayJson {
    url: String,
}

pub async fn genoverlay(body: OverlayJson, fetcher: Arc<Fetcher>) -> Response<Body> {
    let res = match fetcher.fetch(&body.url).await {
        Ok(buf) => buf,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"download error\", \"detail\": \"{}\"}}",
                    e
                )))
                .unwrap()
        }
    };
    let b = Cursor::new(res.clone());
    let reader = match Reader::new(b).with_guessed_format() {
        Ok(r) => r,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{}\"}}",
                e
            )))
                .unwrap()
        }
    };
    let dimensions = match reader.into_dimensions() {
        Ok(d) => d,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{}\"}}",
                e
            )))
                .unwrap()
        }
    };
    if dimensions.0 > 2000 || dimensions.1 > 2000 {
        return Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .header("content-type", "application/json")
                    .body(Body::from("{{\"status\": \"error\", \"reason\": \"image too large\", \"detail\": \"the image file exceeds the size of 2000 pixels in at least one axis\"}}"))
                    .unwrap();
    }
    let c = Cursor::new(res);
    // We can also happily unwrap here because it is the same data
    let img = Reader::new(c)
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();
    // Lanczos3 is best, but has slow speed
    let mut img = resize(&img, 800, 650, FilterType::Lanczos3);
    overlay(&mut img, &PROFILE.clone(), 0, 0);
    let final_image = encode(encode_png(&img).expect("encoding PNG failed"));
    Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from(final_image))
        .unwrap()
}
