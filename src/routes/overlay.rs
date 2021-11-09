use hyper::{Body, Response};
use image::{
    imageops::overlay,
    io::{Limits, Reader},
};
use serde::Deserialize;

use std::{io::Cursor, sync::Arc};

use crate::{
    cache::ImageCache,
    constants::PROFILE,
    encoder::encode_png,
    error::Result,
    proxy::Fetcher,
    resize::{resize, FilterType},
};

#[derive(Deserialize)]
pub struct OverlayJson {
    url: String,
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
    let mut img = resize(img, 800, 650, FilterType::Lanczos3);
    overlay(&mut img, &PROFILE.clone(), 0, 0);

    let final_image = encode_png(&img)?;

    let tag = images.insert(final_image);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from(tag))?)
}
