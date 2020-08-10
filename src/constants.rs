use crate::loaders::*;
use image::imageops::{resize, FilterType};
use image::{RgbImage, RgbaImage};
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderName},
    Client,
};
use rusttype::Font;
use std::collections::HashMap;
use std::env::{current_dir, var};
use std::path::PathBuf;

lazy_static! {
    pub static ref PROXY_URL: Option<String> = match var("PROXY_URL") {
        Ok(url) => Some(url),
        Err(_) => None,
    };
    pub static ref TRAVITIA_FONT: Font<'static> = load_font("TravMedium.otf")
        .expect("could not load font")
        .expect("font corrupt");
    pub static ref CAVIAR_DREAMS: Font<'static> = load_font("CaviarDreams.ttf")
        .expect("could not load font")
        .expect("font corrupt");
    pub static ref OPEN_SANS_EMOJI: Font<'static> = load_font("OpenSansEmoji.ttf")
        .expect("could not load font")
        .expect("font corrupt");
    pub static ref K_GOTHIC: Font<'static> = load_font("K Gothic.ttf")
        .expect("could nto load font")
        .expect("font corrupt");
    pub static ref PROFILE: RgbaImage = {
        let mut base = current_dir().unwrap();
        base.push("assets");
        base.push("images");
        base.push("ProfileOverlayNew.png");
        load_image_rgba(base).expect("could not load image")
    };
    pub static ref DEFAULT_PROFILE: RgbaImage = load_image_rgba(
        current_dir()
            .unwrap()
            .join("assets")
            .join("images")
            .join("ProfileNew.png")
    )
    .expect("could not load image");
    pub static ref CASTS: HashMap<String, RgbaImage> = {
        let mut base = current_dir().unwrap();
        base.push("assets");
        base.push("images");
        base.push("casts");
        let mut map: HashMap<String, RgbaImage> = HashMap::new();
        let all_casts = [
            "thief",
            "paragon",
            "ranger",
            "warrior",
            "mage",
            "raider",
            "ritualist",
            "human",
            "elf",
            "dwarf",
            "orc",
            "jikill",
        ];
        for cast_name in all_casts.iter() {
            map.insert(
                cast_name.to_string(),
                resize(
                    &load_image_rgba(base.join(format!("{}.png", cast_name)))
                        .expect("could not load image"),
                    22,
                    22,
                    FilterType::Lanczos3,
                ),
            );
        }
        map
    };
    pub static ref ADVENTURES: Vec<RgbImage> = {
        let mut base = current_dir().unwrap();
        base.push("assets");
        base.push("images");
        base.push("adventures");
        let base = base.into_os_string();
        let mut images = Vec::new();
        for i in 1..=30 {
            let mut path = PathBuf::from(base.clone());
            path.push(format!("{}.png", i));
            images.push(load_image_rgb(path).expect("could not load image"));
        }
        images
    };
    pub static ref CLIENT: Client = Client::new();
    pub static ref HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        match var("PROXY_AUTH") {
            Ok(key) => {
                let proxy_authorization_key =
                    HeaderName::from_lowercase(b"proxy-authorization-key").unwrap();
                headers.insert(proxy_authorization_key, key.parse().unwrap());
            }
            Err(_) => {}
        };
        let accept = HeaderName::from_lowercase(b"accept").unwrap();
        headers.insert(accept, "application/json".parse().unwrap());
        headers
    };
}
