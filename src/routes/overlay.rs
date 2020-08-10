use crate::constants::PROFILE;
use crate::encoder::encode_png;
use crate::proxy::fetch;
use actix_web::{post, web::Json, HttpResponse};
use base64::encode;
use image::imageops::{overlay, resize, FilterType};
use image::io::Reader;
use serde::Deserialize;
use std::io::Cursor;

#[derive(Deserialize)]
struct OverlayJson {
    url: String,
}

#[post("/api/genoverlay")]
async fn genoverlay(body: Json<OverlayJson>) -> HttpResponse {
    let url = &body.url;
    let res = fetch(&url).await;
    let b = Cursor::new(res.clone());
    let reader = Reader::new(b).with_guessed_format().unwrap();
    let dimensions = reader.into_dimensions().unwrap();
    if dimensions.0 > 2000 || dimensions.1 > 2000 {
        // TODO: Better error handling?
        return HttpResponse::Ok().content_type("text/plain").body("");
    }
    let c = Cursor::new(res);
    let img = Reader::new(c)
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba();
    // Lanczos3 is best, but has slow speed
    let mut img = resize(&img, 800, 650, FilterType::Lanczos3);
    overlay(&mut img, &PROFILE.clone(), 0, 0);
    let final_image = encode(encode_png(&img).unwrap());
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(final_image)
}
