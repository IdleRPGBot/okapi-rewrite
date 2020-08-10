use image::ImageResult;
use image::{open, RgbImage, RgbaImage};
use rusttype::Font;
use std::env::current_dir;
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;

pub fn load_font(name: &str) -> Result<Option<Font>> {
    let mut path = current_dir()?;
    path.push("assets");
    path.push("fonts");
    path.push(name);
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(Font::try_from_vec(buf))
}

pub fn load_image_rgb(path: PathBuf) -> ImageResult<RgbImage> {
    Ok(open(path)?.to_rgb())
}

pub fn load_image_rgba(path: PathBuf) -> ImageResult<RgbaImage> {
    Ok(open(path)?.to_rgba())
}
