use crate::{constants::PROFILE, encoder::encode_png, proxy::Fetcher};
use hyper::{Body, Response, StatusCode};
use image::{
    imageops::{overlay, resize, FilterType},
    io::{Limits, Reader},
};
use serde::Deserialize;

use std::{io::Cursor, sync::Arc};

#[derive(Deserialize)]
pub struct OverlayJson {
    url: String,
}

pub async fn genoverlay(body: OverlayJson, fetcher: Arc<Fetcher>) -> Response<Body> {
    let mut limits = Limits::default();
    limits.max_image_width = Some(2000);
    limits.max_image_height = Some(2000);
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
    let b = Cursor::new(res);
    let mut reader = Reader::new(b);
    reader.limits(limits);
    reader = match reader.with_guessed_format() {
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
    let img = match reader.decode() {
        Ok(i) => i.to_rgba8(),
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"decoding error\", \"detail\": \"{}\"}}",
                    e
                )))
                .unwrap()
        }
    };
    // Lanczos3 is best, but has slow speed
    let mut img = resize(&img, 800, 650, FilterType::Lanczos3);
    overlay(&mut img, &PROFILE.clone(), 0, 0);
    let final_image = encode_png(&img).expect("encoding PNG failed");
    Response::builder()
        .status(200)
        .header("content-type", "image/png")
        .body(Body::from(final_image))
        .unwrap()
}
