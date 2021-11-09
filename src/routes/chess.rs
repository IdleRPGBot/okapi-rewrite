use hyper::{Body, Response};
use image::RgbaImage;
use resvg::render;
use serde::Deserialize;
use tiny_skia::Pixmap;
use usvg::{FitTo, Tree};

use crate::{cache::ImageCache, encoder::encode_png, error::Result};

#[derive(Deserialize)]
pub struct ChessJson {
    xml: String, // SVG
}

pub fn genchess(body: &ChessJson, images: ImageCache) -> Result<Response<Body>> {
    let xml = &body.xml;
    let tree = Tree::from_str(xml, &usvg::Options::default().to_ref())?;

    // SAFETY: This only errors if width or height are 0
    let mut map = Pixmap::new(390, 390).unwrap();
    render(&tree, FitTo::Width(390), map.as_mut()).unwrap();

    let vect = map.take();
    // SAFETY: Only returns None if container too small
    let image = RgbaImage::from_raw(390, 390, vect).unwrap();
    let final_image = encode_png(&image)?;

    let tag = images.insert(final_image);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from(tag))?)
}
