use crate::encoder::encode_png;
use hyper::{Body, Response, StatusCode};
use image::RgbaImage;
use resvg::render;
use serde::Deserialize;
use tiny_skia::Pixmap;
use usvg::{FitTo, Tree};

#[derive(Deserialize)]
pub struct ChessJson {
    xml: String, // SVG
}

#[must_use]
pub fn genchess(body: &ChessJson) -> Response<Body> {
    let xml = &body.xml;
    let tree = match Tree::from_str(xml, &usvg::Options::default().to_ref()) {
        Ok(tree) => tree,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"invalid SVG data\", \"detail\": \"{}\"}}",
                    e
                )))
                .unwrap();
        }
    };
    let mut map = Pixmap::new(390, 390).unwrap();
    render(&tree, FitTo::Width(390), map.as_mut()).expect("rendering yielded no result");
    let vect = map.take();
    let image = RgbaImage::from_raw(390, 390, vect).unwrap();
    let final_image = encode_png(&image).expect("encoding PNG failed");
    Response::builder()
        .status(200)
        .header("content-type", "image/png")
        .body(Body::from(final_image))
        .unwrap()
}
