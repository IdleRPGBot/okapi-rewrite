use hyper::{Body, Response};
use image::RgbaImage;
use resvg::{
    usvg::{Options, Tree, TreeParsing},
    Tree as ResvgTree,
};
use serde::Deserialize;
use tiny_skia::{Pixmap, Transform};

use crate::{cache::ImageCache, encoder::encode_png, error::Result};

#[derive(Deserialize)]
pub struct ChessJson {
    xml: String, // SVG
}

pub fn genchess(body: &ChessJson, images: &ImageCache) -> Result<Response<Body>> {
    let xml = &body.xml;
    let tree = Tree::from_str(xml, &Options::default())?;

    // SAFETY: This only errors if width or height are 0
    let mut map = Pixmap::new(390, 390).unwrap();
    ResvgTree::from_usvg(&tree).render(Transform::default(), &mut map.as_mut());

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
