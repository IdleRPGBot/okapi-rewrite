#[macro_use]
extern crate lazy_static;
extern crate actix_web;
extern crate image;

use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use base64;
use image::{png::PNGEncoder, ImageBuffer, ImageError, Pixel, Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Deref;
use std::{env, io};

#[derive(Debug, Serialize, Deserialize)]
struct AdventuresJson {
    percentages: Value,
}

lazy_static! {
    static ref TRAVITIA_FONT: Font<'static> = {
        let font = Vec::from(
            include_bytes!("/home/jens/okapi-rewrite/assets/fonts/TravMedium.otf") as &[u8],
        );
        let font = Font::try_from_vec(font).unwrap();
        font
    };
    static ref ADVENTURES: Vec<RgbImage> = {
        let mut images = Vec::new();
        for i in 1..30 {
            let image = image::open(format!(
                "/home/jens/okapi-rewrite/assets/images/adventures/{}.png",
                i
            ))
            .unwrap()
            .to_rgb();
            images.push(image);
        }
        images
    };
}

fn encode_png<P, Container>(img: &ImageBuffer<P, Container>) -> Result<Vec<u8>, ImageError>
where
    P: Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut buf = Vec::new();
    let encoder = PNGEncoder::new(&mut buf);
    encoder.encode(img, img.width(), img.height(), P::COLOR_TYPE)?;
    Ok(buf)
}

#[get("/")]
async fn index() -> HttpResponse {
    // For metrics
    HttpResponse::Ok().content_type("text/plain").body("1")
}

#[post("/api/genadventures")]
async fn genadventures(body: web::Json<AdventuresJson>) -> HttpResponse {
    // body is the parsed JSON
    let chances = body.percentages.as_array().unwrap();
    let mut images = Vec::new();
    for idx in 0..29 {
        let current_chances = &chances[idx].as_array().unwrap();
        let chance_min = &current_chances[0];
        let chance_max = &current_chances[1];
        let mut new_image = ADVENTURES[idx].clone();
        let white = Rgb([0u8, 0u8, 0u8]);
        let scale = Scale { x: 20.0, y: 20.0 };
        if idx > 9 && idx < 20 {
            draw_text_mut(
                &mut new_image,
                white,
                314,
                168,
                scale,
                &TRAVITIA_FONT,
                &format!("{}% to", chance_min),
            );
            draw_text_mut(
                &mut new_image,
                white,
                314,
                188,
                scale,
                &TRAVITIA_FONT,
                &format!("{}%", chance_max),
            );
        } else {
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
        }
        let final_image = encode_png(&new_image).unwrap();
        let buf = format!("data:image/png;base64,{}", base64::encode(&final_image));
        images.push(buf);
    }
    HttpResponse::Ok().json(images)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(index)
            .service(genadventures)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
