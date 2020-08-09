use crate::constants::PROFILE;
use crate::encoder::encode_png;
use crate::proxy::fetch;
use actix_web::{post, web::Json, HttpResponse};
use image::imageops::{overlay, resize, FilterType};
use image::load_from_memory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct OverlayJson {
    url: String,
}

#[post("/api/genoverlay")]
async fn genoverlay(body: Json<OverlayJson>) -> HttpResponse {
    let url = &body.url;
    println!("Generating image for url {}", url);
    let res = fetch(&url).await;
    let img = load_from_memory(&res).unwrap().to_rgba();
    // Lanczos3 is best, but has slow speed
    let mut img = resize(&img, 800, 650, FilterType::Lanczos3);
    overlay(&mut img, &PROFILE.clone(), 0, 0);
    let final_image = encode_png(&img).unwrap();
    println!("Generating image for url {}... done", url);
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}
