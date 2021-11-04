use ab_glyph::FontVec;
use hyper::Uri;
use image::{
    imageops::{resize, FilterType},
    RgbImage, RgbaImage,
};
use lazy_static::lazy_static;
use std::{collections::HashMap, env::var, str::FromStr};

use crate::webp::decode;

lazy_static! {
    pub static ref RENDERER_KEY: Vec<u8> = var("RENDERER_KEY").unwrap().as_bytes().to_vec();
    pub static ref PROXY_URL: Option<Uri> =
        var("PROXY_URL").ok().map(|p| Uri::from_str(&p).unwrap());
    pub static ref PROXY_AUTH: Option<String> = var("PROXY_AUTH").ok();
    pub static ref TRAVITIA_FONT: FontVec =
        FontVec::try_from_vec(include_bytes!("../assets/fonts/MergedNoKern.otf").to_vec())
            .expect("could not load font");
    pub static ref PROFILE: RgbaImage =
        decode(include_bytes!("../assets/images/ProfileOverlayNew.webp"))
            .expect("Could not load image")
            .to_rgba8();
    pub static ref DEFAULT_PROFILE: RgbaImage =
        decode(include_bytes!("../assets/images/ProfileNew.webp"))
            .expect("Could not load image")
            .to_rgba8();
    pub static ref CASTS: HashMap<String, RgbaImage> = {
        let mut all_casts = HashMap::new();
        all_casts.insert(
            "thief",
            include_bytes!("../assets/images/casts/thief.webp").to_vec(),
        );
        all_casts.insert(
            "paragon",
            include_bytes!("../assets/images/casts/paragon.webp").to_vec(),
        );
        all_casts.insert(
            "ranger",
            include_bytes!("../assets/images/casts/ranger.webp").to_vec(),
        );
        all_casts.insert(
            "warrior",
            include_bytes!("../assets/images/casts/warrior.webp").to_vec(),
        );
        all_casts.insert(
            "mage",
            include_bytes!("../assets/images/casts/mage.webp").to_vec(),
        );
        all_casts.insert(
            "raider",
            include_bytes!("../assets/images/casts/raider.webp").to_vec(),
        );
        all_casts.insert(
            "ritualist",
            include_bytes!("../assets/images/casts/ritualist.webp").to_vec(),
        );
        all_casts.insert(
            "human",
            include_bytes!("../assets/images/casts/human.webp").to_vec(),
        );
        all_casts.insert(
            "elf",
            include_bytes!("../assets/images/casts/elf.webp").to_vec(),
        );
        all_casts.insert(
            "dwarf",
            include_bytes!("../assets/images/casts/dwarf.webp").to_vec(),
        );
        all_casts.insert(
            "orc",
            include_bytes!("../assets/images/casts/orc.webp").to_vec(),
        );
        all_casts.insert(
            "jikill",
            include_bytes!("../assets/images/casts/jikill.webp").to_vec(),
        );

        let mut map: HashMap<String, RgbaImage> = HashMap::new();
        for (cast_name, bytes) in &all_casts {
            map.insert(
                (*cast_name).to_string(),
                resize(
                    &decode(bytes).expect("Could not load image").to_rgba8(),
                    22,
                    22,
                    FilterType::Lanczos3,
                ),
            );
        }
        map
    };
    pub static ref ADVENTURES: Vec<RgbImage> = vec![
        decode(include_bytes!("../assets/images/adventures/1.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/2.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/3.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/4.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/5.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/6.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/7.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/8.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/9.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/10.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/11.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/12.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/13.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/14.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/15.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/16.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/17.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/18.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/19.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/20.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/21.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/22.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/23.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/24.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/25.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/26.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/27.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/28.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/29.webp"))
            .expect("Could not load image")
            .to_rgb8(),
        decode(include_bytes!("../assets/images/adventures/30.webp"))
            .expect("Could not load image")
            .to_rgb8()
    ];
}
