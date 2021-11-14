use hyper::{Body, Response};
use image::{
    imageops::{overlay, resize, FilterType},
    io::{Limits, Reader},
};
use serde::Deserialize;

use std::{io::Cursor, sync::Arc};

use crate::{
    cache::ImageCache,
    constants::{PROFILE_DARK, PROFILE_LIGHT},
    encoder::encode_png,
    error::Result,
    proxy::Fetcher,
};

#[derive(Deserialize)]
pub struct OverlayJson {
    url: String,
    style: String,
}

pub async fn genoverlay(
    body: OverlayJson,
    fetcher: Arc<Fetcher>,
    images: ImageCache,
) -> Result<Response<Body>> {
    let mut limits = Limits::default();
    limits.max_image_width = Some(2000);
    limits.max_image_height = Some(2000);

    let res = fetcher.fetch(&body.url).await?;
    let b = Cursor::new(res);
    let mut reader = Reader::new(b);
    reader.limits(limits);
    reader = reader.with_guessed_format()?;

    let img = reader.decode()?;

    // Lanczos3 is best, but has slow speed
    let mut img = resize(&img, 800, 533, FilterType::Lanczos3);

    if body.style == "dark" {
        overlay(&mut img, &PROFILE_DARK.clone(), 0, 0);
    } else if body.style == "light" {
        overlay(&mut img, &PROFILE_LIGHT.clone(), 0, 0);
    }

    let final_image = encode_png(&img)?;

    let tag = images.insert(final_image);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from(tag))?)
}
