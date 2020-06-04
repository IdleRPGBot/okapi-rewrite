#[macro_use]
extern crate lazy_static;
extern crate actix_web;
extern crate image;

use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use base64;
use image::{png::PNGEncoder, ImageBuffer, ImageError, Pixel, Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use resvg::prelude::*;
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Read;
use std::ops::Deref;
use std::{env, fs::File, io, path};

#[derive(Debug, Serialize, Deserialize)]
struct AdventuresJson {
    percentages: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChessJson {
    xml: String,
}

fn load_font(name: &str) -> Font {
    let mut path = env::current_dir().unwrap();
    path.push("assets");
    path.push("fonts");
    path.push(name);
    let mut f = File::open(path).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let font = Font::try_from_vec(buf).unwrap();
    font
}

fn load_image(path: path::PathBuf) -> RgbImage {
    image::open(path).unwrap().to_rgb()
}

lazy_static! {
    static ref TRAVITIA_FONT: Font<'static> = load_font("TravMedium.otf");
    static ref ADVENTURES: Vec<RgbImage> = {
        let mut base = env::current_dir().unwrap();
        base.push("assets");
        base.push("images");
        base.push("adventures");
        let base = base.into_os_string();
        let mut images = Vec::new();
        for i in 1..30 {
            let mut path = path::PathBuf::from(base.clone());
            path.push(format!("{}.png", i));
            images.push(load_image(path));
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

#[post("/api/genchess")]
async fn genchess(body: web::Json<ChessJson>) -> HttpResponse {
    let xml = &body.xml;
    let opts = resvg::Options {
        background: None,
        fit_to: resvg::FitTo::Width(390),
        usvg: resvg::Options::default().usvg,
    };
    let tree = usvg::Tree::from_str(&xml, &opts.usvg).unwrap();
    let mut img = resvg::backend_raqote::render_to_image(&tree, &opts).unwrap();
    let vect = img.make_vec();
    let final_image = encode_png(
        &image::RgbaImage::from_vec(img.height() as u32, img.width() as u32, vect).unwrap(),
    )
    .unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
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
            .service(genchess)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
