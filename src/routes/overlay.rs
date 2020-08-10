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
    let res = match fetch(&body.url).await {
        Ok(buf) => buf,
        Err(e) => {
            return HttpResponse::UnprocessableEntity()
                .content_type("application/json")
                .body(format!(
                    "{{\"status\": \"error\", \"reason\": \"download error\", \"detail\": \"{}\"}}",
                    e
                ))
        }
    };
    let b = Cursor::new(res.clone());
    let reader = match Reader::new(b).with_guessed_format() {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::UnprocessableEntity()
                .content_type("application/json")
                .body(format!(
                "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{}\"}}",
                e
            ))
        }
    };
    let dimensions = match reader.into_dimensions() {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::UnprocessableEntity()
                .content_type("application/json")
                .body(format!(
                "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{}\"}}",
                e
            ))
        }
    };
    if dimensions.0 > 2000 || dimensions.1 > 2000 {
        return HttpResponse::UnprocessableEntity()
                .content_type("application/json")
                .body("{\"status\": \"error\", \"reason\": \"image too large\", \"detail\": \"the image file exceeds the size of 2000 pixels in at least one axis\"}");
    }
    let c = Cursor::new(res);
    // We can also happily unwrap here because it is the same data
    let img = Reader::new(c)
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba();
    // Lanczos3 is best, but has slow speed
    let mut img = resize(&img, 800, 650, FilterType::Lanczos3);
    overlay(&mut img, &PROFILE.clone(), 0, 0);
    let final_image = encode(encode_png(&img).expect("encoding PNG failed"));
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(final_image)
}
