use crate::{encoder::encode_png, proxy::Fetcher};
use hyper::{Body, Response, StatusCode};
use image::{
    imageops::{invert, resize, FilterType},
    load_from_memory, Pixel, Rgba,
};
use imageproc::edges::canny;
use serde::Deserialize;

use std::{collections::HashMap, sync::Arc};

#[derive(Deserialize)]
pub struct ImageJson {
    image: String, // URL
}

struct Intensity {
    val: i32,
    r: i32,
    g: i32,
    b: i32,
}

pub async fn pixelate(body: ImageJson, fetcher: Arc<Fetcher>) -> Response<Body> {
    let res = match fetcher.fetch(&body.image).await {
        Ok(buf) => buf,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"download error\", \"detail\": \"{}\"}}",
                    e
                )))
                .unwrap()
        }
    };
    let img = match load_from_memory(&res) {
        Ok(data) => data.to_rgba8(),
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{}\"}}",
                e
            )))
                .unwrap()
        }
    };
    let buf = resize(&img, 1024, 1024, FilterType::Nearest);
    let final_image = encode_png(&buf).expect("encoding PNG failed");
    Response::builder()
        .status(200)
        .header("content-type", "image/png")
        .body(Body::from(final_image))
        .unwrap()
}

pub async fn invert_endpoint(body: ImageJson, fetcher: Arc<Fetcher>) -> Response<Body> {
    let res = match fetcher.fetch(&body.image).await {
        Ok(buf) => buf,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"download error\", \"detail\": \"{}\"}}",
                    e
                )))
                .unwrap()
        }
    };
    let mut img = match load_from_memory(&res) {
        Ok(data) => data.to_rgba8(),
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{}\"}}",
                e
            )))
                .unwrap()
        }
    };
    invert(&mut img);
    let final_image = encode_png(&img).expect("encoding PNG failed");
    Response::builder()
        .status(200)
        .header("content-type", "image/png")
        .body(Body::from(final_image))
        .unwrap()
}

pub async fn edges_endpoint(body: ImageJson, fetcher: Arc<Fetcher>) -> Response<Body> {
    let res = match fetcher.fetch(&body.image).await {
        Ok(buf) => buf,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"download error\", \"detail\": \"{}\"}}",
                    e
                )))
                .unwrap()
        }
    };
    let img = match load_from_memory(&res) {
        Ok(data) => data.to_luma8(),
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{}\"}}",
                e
            )))
                .unwrap()
        }
    };
    let buf = canny(&img, 25.0, 80.0);
    let final_image = encode_png(&buf).expect("encoding PNG failed");
    Response::builder()
        .status(200)
        .header("content-type", "image/png")
        .body(Body::from(final_image))
        .unwrap()
}

pub async fn oil_endpoint(body: ImageJson, fetcher: Arc<Fetcher>) -> Response<Body> {
    let res = match fetcher.fetch(&body.image).await {
        Ok(buf) => buf,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"status\": \"error\", \"reason\": \"download error\", \"detail\": \"{}\"}}",
                    e
                )))
                .unwrap()
        }
    };
    let img = match load_from_memory(&res) {
        Ok(data) => data.to_rgba8(),
        Err(e) => {
            return Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{}\"}}",
                e
            )))
                .unwrap()
        }
    };
    let radius = 4i32;
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
                                val.val += 1;
                                val.r += pix[0] as i32;
                                val.g += pix[1] as i32;
                                val.b += pix[2] as i32;
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
    let final_image = encode_png(&target).expect("encoding PNG failed");
    Response::builder()
        .status(200)
        .header("content-type", "image/png")
        .body(Body::from(final_image))
        .unwrap()
}
