use crate::encoder::encode_png;
use actix_web::{post, web::Json, HttpResponse};
use image::{Bgra, DynamicImage, ImageBuffer};
use resvg_raqote::{render_to_image, Options};
use serde::Deserialize;
use usvg::{FitTo, Tree};

type BgraImage = ImageBuffer<Bgra<u8>, Vec<u8>>;

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
    let mut options = Options::default();
    options.fit_to = FitTo::Width(390);
    let img = render_to_image(&tree, &options).expect("rendering yielded no result");
    let width = img.width() as u32;
    let height = img.height() as u32;
    let vect = img.get_data_u8();
    let dyn_image = DynamicImage::ImageBgra8(
        BgraImage::from_vec(width, height, vect.to_vec())
            .expect("RGBA data does not match height and width"),
    );
    let final_image = encode_png(&dyn_image.to_rgba()).expect("encoding PNG failed");
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}
