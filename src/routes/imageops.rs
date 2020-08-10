use crate::encoder::encode_png;
use crate::proxy::fetch;
use actix_web::{post, web::Json, HttpResponse};
use image::{
    imageops::{invert, resize, FilterType},
    load_from_memory, Pixel, Rgba,
};
use imageproc::edges::canny;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct ImageJson {
    image: String, // URL
}

struct Intensity {
    val: i32,
    r: i32,
    g: i32,
    b: i32,
}

#[post("/api/imageops/pixel")]
async fn pixelate(body: Json<ImageJson>) -> HttpResponse {
    let res = fetch(&body.image).await;
    let img = load_from_memory(&res).unwrap().to_rgba();
    let buf = resize(&img, 1024, 1024, FilterType::Nearest);
    let final_image = encode_png(&buf).unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}

#[post("/api/imageops/invert")]
async fn invert_endpoint(body: Json<ImageJson>) -> HttpResponse {
    let res = fetch(&body.image).await;
    let mut img = image::load_from_memory(&res).unwrap().to_rgba();
    invert(&mut img);
    let final_image = encode_png(&img).unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}

#[post("/api/imageops/edges")]
async fn edges_endpoint(body: Json<ImageJson>) -> HttpResponse {
    let res = fetch(&body.image).await;
    let img = image::load_from_memory(&res).unwrap().to_luma();
    let buf = canny(&img, 25.0, 80.0);
    let final_image = encode_png(&buf).unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}

#[post("/api/imageops/oil")]
async fn oil_endpoint(body: Json<ImageJson>) -> HttpResponse {
    let res = fetch(&body.image).await;
    let img = image::load_from_memory(&res).unwrap().to_rgba();
    let radius = 4 as i32;
    let intensity = 55.0;
    let width = img.width();
    let height = img.height();
    let mut target = image::RgbaImage::new(width, height);
    let mut pixel_intensity_count: HashMap<usize, Intensity>;
    let mut intensity_lut = vec![vec![0; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let current_val = img.get_pixel(x, y).channels();
            let avg = (current_val[0] as i32 + current_val[1] as i32 + current_val[2] as i32)
                as f64
                / 3.0;
            let val = (avg * intensity) / 255.0;
            intensity_lut[y as usize][x as usize] = val.round() as usize;
        }
    }

    for y in 0..height {
        for x in 0..width {
            pixel_intensity_count = HashMap::new();

            // Find intensities of nearest pixels within radius
            for yy in -radius..=radius {
                let yyy = (y as i32) + yy;
                for xx in -radius..=radius {
                    let xxx = (x as i32) + xx;
                    if yyy > 0 && yyy < (height as i32) && xxx > 0 && xxx < (width as i32) {
                        let idx_x = xxx as usize;
                        let idx_y = yyy as usize;
                        let intensity_val = intensity_lut[idx_y][idx_x];
                        let pix = img.get_pixel(idx_x as u32, idx_y as u32).channels();
                        match pixel_intensity_count.get_mut(&(intensity_val as usize)) {
                            Some(val) => {
                                val.val = val.val + 1;
                                val.r = val.r + pix[0] as i32;
                                val.g = val.g + pix[1] as i32;
                                val.b = val.b + pix[2] as i32;
                            }
                            None => {
                                pixel_intensity_count.insert(
                                    intensity_val as usize,
                                    Intensity {
                                        val: 1,
                                        r: pix[0] as i32,
                                        g: pix[1] as i32,
                                        b: pix[2] as i32,
                                    },
                                );
                            }
                        }
                    }
                }
            }

            let mut map_vec: Vec<_> = pixel_intensity_count.iter().collect();
            map_vec.sort_by(|a, b| (b.1.val - a.1.val).cmp(&0));

            let cur_max = map_vec[0].1;
            target.put_pixel(
                x,
                y,
                Rgba::<u8>([
                    (cur_max.r / cur_max.val) as u8,
                    (cur_max.g / cur_max.val) as u8,
                    (cur_max.b / cur_max.val) as u8,
                    255,
                ]),
            )
        }
    }
    let final_image = encode_png(&target).unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}
