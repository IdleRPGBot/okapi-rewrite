use crate::loaders::*;
use image::imageops::{resize, FilterType};
use image::{RgbImage, RgbaImage};
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderName},
    Client,
};
use rusttype::Font;
use serde_json::{from_str, value::Value};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

lazy_static! {
    pub static ref CONFIG: Value = {
        let mut file = File::open("config.json").expect("Config not found");
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let json = from_str(&data).expect("Invalid JSON");
        json
    };
    pub static ref TRAVITIA_FONT: Font<'static> = load_font("TravMedium.otf");
    pub static ref CAVIAR_DREAMS: Font<'static> = load_font("CaviarDreams.ttf");
    pub static ref OPEN_SANS_EMOJI: Font<'static> = load_font("OpenSansEmoji.ttf");
    pub static ref K_GOTHIC: Font<'static> = load_font("K Gothic.ttf");
    pub static ref PROFILE: RgbaImage = {
        let mut base = current_dir().unwrap();
        base.push("assets");
        base.push("images");
        base.push("ProfileOverlayNew.png");
        load_image_rgba(base)
    };
    pub static ref DEFAULT_PROFILE: RgbaImage = load_image_rgba(
        current_dir()
            .unwrap()
            .join("assets")
            .join("images")
            .join("ProfileNew.png")
    );
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
                    &load_image_rgba(base.join(format!("{}.png", cast_name))),
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
            images.push(load_image_rgb(path));
        }
        images
    };
    pub static ref CLIENT: Client = Client::new();
    pub static ref HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        let key = CONFIG["proxy_auth"].as_str().unwrap().parse().unwrap();
        let proxy_authorization_key =
            HeaderName::from_lowercase(b"proxy-authorization-key").unwrap();
        let accept = HeaderName::from_lowercase(b"accept").unwrap();
        headers.insert(proxy_authorization_key, key);
        headers.insert(accept, "application/json".parse().unwrap());
        headers
    };
}