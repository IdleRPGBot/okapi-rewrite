use ab_glyph::PxScale;
use hyper::{Body, Response};
use image::Rgb;
use imageproc::drawing::draw_text_mut;
use serde::Deserialize;

use std::sync::Arc;

use crate::{
    constants::{ADVENTURES, TRAVITIA_FONT},
    encoder::encode_png,
    error::Result,
    proxy::Fetcher,
};

#[derive(Deserialize)]
pub struct AdventuresJson {
    percentages: Vec<Vec<i32>>,
}

const WHITE: Rgb<u8> = Rgb([0, 0, 0]);
const SCALE: PxScale = PxScale { x: 20.0, y: 20.0 };

pub async fn genadventures(body: &AdventuresJson, fetcher: Arc<Fetcher>) -> Result<Response<Body>> {
    let mut images: Vec<Vec<u8>> = Vec::with_capacity(30);

    for idx in 0..30 {
        let current_chances = &body.percentages[idx];
        let chance_min = &current_chances[0];
        let chance_max = &current_chances[1];

        let mut new_image = ADVENTURES[idx].clone();

        draw_text_mut(
            &mut new_image,
            WHITE,
            314,
            148,
            SCALE,
            100,
            &*TRAVITIA_FONT,
            &format!("{}% to", chance_min),
        );
        draw_text_mut(
            &mut new_image,
            WHITE,
            314,
            168,
            SCALE,
            100,
            &*TRAVITIA_FONT,
            &format!("{}%", chance_max),
        );

        let buf = encode_png(&new_image)?;

        images.push(buf);
    }

    let tags = fetcher.upload_images(images).await?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::from(tags))?)
}
