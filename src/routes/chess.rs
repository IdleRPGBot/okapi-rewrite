use crate::encoder::encode_png;
use actix_web::{post, web::Json, HttpResponse};
use image::RgbaImage;
use resvg::render;
use serde::{Deserialize, Serialize};
use usvg::Tree;

#[derive(Debug, Serialize, Deserialize)]
struct ChessJson {
    xml: String,
}

#[post("/api/genchess")]
async fn genchess(body: Json<ChessJson>) -> HttpResponse {
    let xml = &body.xml;
    let tree = Tree::from_str(&xml, &usvg::Options::default()).unwrap();
    let img = render(&tree, usvg::FitTo::Width(390), None).unwrap();
    let width = img.width() as u32;
    let height = img.height() as u32;
    let vect = img.take();
    let final_image = encode_png(&RgbaImage::from_vec(width, height, vect).unwrap()).unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}
