use crate::encoder::encode_png;
use actix_web::{post, web::Json, HttpResponse};
use image::RgbaImage;
use resvg::render;
use serde::Deserialize;
use usvg::Tree;

#[derive(Deserialize)]
struct ChessJson {
    xml: String, // SVG
}

#[post("/api/genchess")]
async fn genchess(body: Json<ChessJson>) -> HttpResponse {
    let xml = &body.xml;
    let tree = match Tree::from_str(&xml, &usvg::Options::default()) {
        Ok(tree) => tree,
        Err(e) => {
            return HttpResponse::UnprocessableEntity()
                .content_type("application/json")
                .body(format!(
                "{{\"status\": \"error\", \"reason\": \"invalid SVG data\", \"detail\": \"{}\"}}",
                e
            ))
        }
    };
    let img = render(&tree, usvg::FitTo::Width(390), None).expect("rendering yielded no result");
    let width = img.width() as u32;
    let height = img.height() as u32;
    let vect = img.take();
    let final_image = encode_png(
        &RgbaImage::from_vec(width, height, vect)
            .expect("RGBA data does not match height and width"),
    )
    .expect("encoding PNG failed");
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}
