use crate::constants::{ADVENTURES, TRAVITIA_FONT};
use crate::encoder::encode_png;
use actix_web::{post, web::Json, HttpResponse};
use base64::encode;
use image::Rgb;
use imageproc::drawing::draw_text_mut;
use rusttype::Scale;
use serde::Deserialize;

#[derive(Deserialize)]
struct AdventuresJson {
    percentages: Vec<Vec<i32>>,
}

#[post("/api/genadventures")]
async fn genadventures(body: Json<AdventuresJson>) -> HttpResponse {
    // body is the parsed JSON
    let mut images = Vec::new();
    for idx in 0..30 {
        let current_chances = &body.percentages[idx];
        let chance_min = &current_chances[0];
        let chance_max = &current_chances[1];
        let mut new_image = ADVENTURES[idx].clone();
        let white = Rgb([0u8, 0u8, 0u8]);
        let scale = Scale { x: 20.0, y: 20.0 };
        draw_text_mut(
            &mut new_image,
            white,
            314,
            148,
            scale,
            &TRAVITIA_FONT,
            &format!("{}% to", chance_min),
        );
        draw_text_mut(
            &mut new_image,
            white,
            314,
            168,
            scale,
            &TRAVITIA_FONT,
            &format!("{}%", chance_max),
        );
        let final_image = encode_png(&new_image).unwrap();
        let buf = format!("data:image/png;base64,{}", encode(&final_image));
        images.push(buf);
    }
    HttpResponse::Ok().json(images)
}
