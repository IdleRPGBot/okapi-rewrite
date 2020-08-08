use image::{open, RgbImage, RgbaImage};
use rusttype::Font;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn load_font(name: &str) -> Font {
    let mut path = current_dir().unwrap();
    path.push("assets");
    path.push("fonts");
    path.push(name);
    let mut f = File::open(path).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let font = Font::try_from_vec(buf).unwrap();
    font
}

pub fn load_image_rgb(path: PathBuf) -> RgbImage {
    open(path).unwrap().to_rgb()
}

pub fn load_image_rgba(path: PathBuf) -> RgbaImage {
    open(path).unwrap().to_rgba()
}
