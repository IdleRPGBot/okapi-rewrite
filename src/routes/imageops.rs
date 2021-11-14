use hyper::{Body, Response};
use image::{
    imageops::{invert, resize, FilterType},
    load_from_memory, Pixel, Rgba,
};
use imageproc::edges::canny;
use serde::Deserialize;

use std::{collections::HashMap, sync::Arc};

use crate::{cache::ImageCache, encoder::encode_png, error::Result, proxy::Fetcher};

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

pub async fn pixelate(
    body: ImageJson,
    fetcher: Arc<Fetcher>,
    images: ImageCache,
) -> Result<Response<Body>> {
    let res = fetcher.fetch(&body.image).await?;
    let img = load_from_memory(&res)?;
    let buf = resize(&img, 1024, 1024, FilterType::Nearest);
    let final_image = encode_png(&buf)?;

    let tag = images.insert(final_image);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from(tag))?)
}

pub async fn invert_endpoint(
    body: ImageJson,
    fetcher: Arc<Fetcher>,
    images: ImageCache,
) -> Result<Response<Body>> {
    let res = fetcher.fetch(&body.image).await?;
    let mut img = load_from_memory(&res)?.to_rgba8();
    invert(&mut img);
    let final_image = encode_png(&img)?;

    let tag = images.insert(final_image);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from(tag))?)
}

pub async fn edges_endpoint(
    body: ImageJson,
    fetcher: Arc<Fetcher>,
    images: ImageCache,
) -> Result<Response<Body>> {
    let res = fetcher.fetch(&body.image).await?;
    let img = load_from_memory(&res)?.to_luma8();
    let buf = canny(&img, 25.0, 80.0);
    let final_image = encode_png(&buf)?;

    let tag = images.insert(final_image);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from(tag))?)
}

pub async fn oil_endpoint(
    body: ImageJson,
    fetcher: Arc<Fetcher>,
    images: ImageCache,
) -> Result<Response<Body>> {
    let res = fetcher.fetch(&body.image).await?;
    let img = load_from_memory(&res)?.to_rgba8();

    let radius = 4_i32;
    let intensity = 55.0;
    let width = img.width();
    let height = img.height();
    let mut target = image::RgbaImage::new(width, height);
    let mut pixel_intensity_count: HashMap<usize, Intensity>;
    let mut intensity_lut = vec![vec![0; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let current_val = img.get_pixel(x, y).channels();
            let avg = f64::from(
                i32::from(current_val[0]) + i32::from(current_val[1]) + i32::from(current_val[2]),
            ) / 3.0;
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
                                val.r += i32::from(pix[0]);
                                val.g += i32::from(pix[1]);
                                val.b += i32::from(pix[2]);
                            }
                            None => {
                                pixel_intensity_count.insert(
                                    intensity_val as usize,
                                    Intensity {
                                        val: 1,
                                        r: i32::from(pix[0]),
                                        g: i32::from(pix[1]),
                                        b: i32::from(pix[2]),
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
            );
        }
    }

    let final_image = encode_png(&target)?;

    let tag = images.insert(final_image);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from(tag))?)
}
