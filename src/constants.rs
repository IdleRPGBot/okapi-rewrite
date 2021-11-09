use ab_glyph::FontVec;
use hyper::Uri;
use image::{load_from_memory, RgbImage, RgbaImage};
use lazy_static::lazy_static;
use std::{collections::HashMap, env::var, str::FromStr};

use crate::resize::{resize, FilterType};

lazy_static! {
    pub static ref RENDERER_KEY: Vec<u8> = var("RENDERER_KEY").unwrap().as_bytes().to_vec();
    pub static ref PROXY_URL: Option<Uri> =
        var("PROXY_URL").ok().map(|p| Uri::from_str(&p).unwrap());
    pub static ref PROXY_AUTH: Option<String> = var("PROXY_AUTH").ok();
    pub static ref AUTH_KEY: Option<String> = var("AUTH_KEY").ok();
    pub static ref TRAVITIA_FONT: FontVec =
        FontVec::try_from_vec(include_bytes!("../assets/fonts/MergedNoKern.otf").to_vec())
            .expect("could not load font");
    pub static ref PROFILE: RgbaImage =
        load_from_memory(include_bytes!("../assets/images/ProfileOverlayNew.png"))
            .expect("Could not load image")
            .into_rgba8();
    pub static ref DEFAULT_PROFILE: RgbaImage =
        load_from_memory(include_bytes!("../assets/images/ProfileNew.png"))
            .expect("Could not load image")
            .into_rgba8();
    pub static ref CASTS: HashMap<String, RgbaImage> = {
        let mut all_casts = HashMap::new();
        all_casts.insert(
            "thief",
            include_bytes!("../assets/images/casts/thief.png").to_vec(),
        );
        all_casts.insert(
            "paragon",
            include_bytes!("../assets/images/casts/paragon.png").to_vec(),
        );
        all_casts.insert(
            "ranger",
            include_bytes!("../assets/images/casts/ranger.png").to_vec(),
        );
        all_casts.insert(
            "warrior",
            include_bytes!("../assets/images/casts/warrior.png").to_vec(),
        );
        all_casts.insert(
            "mage",
            include_bytes!("../assets/images/casts/mage.png").to_vec(),
        );
        all_casts.insert(
            "raider",
            include_bytes!("../assets/images/casts/raider.png").to_vec(),
        );
        all_casts.insert(
            "ritualist",
            include_bytes!("../assets/images/casts/ritualist.png").to_vec(),
        );
        all_casts.insert(
            "human",
            include_bytes!("../assets/images/casts/human.png").to_vec(),
        );
        all_casts.insert(
            "elf",
            include_bytes!("../assets/images/casts/elf.png").to_vec(),
        );
        all_casts.insert(
            "dwarf",
            include_bytes!("../assets/images/casts/dwarf.png").to_vec(),
        );
        all_casts.insert(
            "orc",
            include_bytes!("../assets/images/casts/orc.png").to_vec(),
        );
        all_casts.insert(
            "jikill",
            include_bytes!("../assets/images/casts/jikill.png").to_vec(),
        );

        let mut map: HashMap<String, RgbaImage> = HashMap::new();
        for (cast_name, bytes) in &all_casts {
            map.insert(
                (*cast_name).to_string(),
                resize(
                    load_from_memory(bytes).expect("Could not load image"),
                    22,
                    22,
                    FilterType::Lanczos3,
                ),
            );
        }
        map
    };
    pub static ref ADVENTURES: Vec<RgbImage> = vec![
        load_from_memory(include_bytes!("../assets/images/adventures/1.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/2.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/3.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/4.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/5.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/6.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/7.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/8.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/9.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/10.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/11.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/12.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/13.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/14.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/15.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/16.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/17.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/18.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/19.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/20.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/21.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/22.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/23.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/24.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/25.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/26.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/27.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/28.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/29.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/adventures/30.png"))
            .expect("Could not load image")
            .into_rgb8()
    ];
}
