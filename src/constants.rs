use ab_glyph::FontVec;
use hyper::Uri;
use image::{
    imageops::{resize, FilterType},
    load_from_memory, RgbImage, RgbaImage,
};
use lazy_static::lazy_static;
use std::{collections::HashMap, env::var, str::FromStr};

lazy_static! {
    pub static ref PORT: u16 = var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse::<u16>()
        .unwrap();
    pub static ref PROXY_URL: Option<Uri> =
        var("PROXY_URL").ok().map(|p| Uri::from_str(&p).unwrap());
    pub static ref PROXY_AUTH: Option<String> = var("PROXY_AUTH").ok();
    pub static ref AUTH_KEY: Option<String> = var("AUTH_KEY").ok();
    pub static ref EXTERNAL_URL: String =
        var("EXTERNAL_URL").unwrap_or(format!("http://localhost:{}", *PORT));
    pub static ref TRAVITIA_FONT: FontVec =
        FontVec::try_from_vec(include_bytes!("../assets/fonts/MergedNoKern.otf").to_vec())
            .expect("could not load font");
    pub static ref PROFILE_DARK: RgbaImage =
        load_from_memory(include_bytes!("../assets/images/profile_overlay_dark.png"))
            .expect("Could not load image")
            .into_rgba8();
    pub static ref PROFILE_LIGHT: RgbaImage =
        load_from_memory(include_bytes!("../assets/images/profile_overlay_light.png"))
            .expect("Could not load image")
            .into_rgba8();
    pub static ref DEFAULT_PROFILE: RgbaImage =
        load_from_memory(include_bytes!("../assets/images/profile.png"))
            .expect("Could not load image")
            .into_rgba8();
    pub static ref RACES: HashMap<String, RgbaImage> = {
        let mut all_races = HashMap::new();

        all_races.insert(
            "human",
            include_bytes!("../assets/images/casts/human.png").to_vec(),
        );
        all_races.insert(
            "elf",
            include_bytes!("../assets/images/casts/elf.png").to_vec(),
        );
        all_races.insert(
            "dwarf",
            include_bytes!("../assets/images/casts/dwarf.png").to_vec(),
        );
        all_races.insert(
            "orc",
            include_bytes!("../assets/images/casts/orc.png").to_vec(),
        );
        all_races.insert(
            "jikill",
            include_bytes!("../assets/images/casts/jikill.png").to_vec(),
        );

        let mut map: HashMap<String, RgbaImage> = HashMap::new();
        for (cast_name, bytes) in &all_races {
            map.insert(
                (*cast_name).to_string(),
                resize(
                    &load_from_memory(bytes).expect("Could not load image"),
                    60,
                    60,
                    FilterType::Lanczos3,
                ),
            );
        }
        map
    };
    pub static ref CLASSES: HashMap<String, RgbaImage> = {
        let mut all_classes = HashMap::new();

        all_classes.insert(
            "thief",
            include_bytes!("../assets/images/casts/thief.png").to_vec(),
        );
        all_classes.insert(
            "paragon",
            include_bytes!("../assets/images/casts/paragon.png").to_vec(),
        );
        all_classes.insert(
            "ranger",
            include_bytes!("../assets/images/casts/ranger.png").to_vec(),
        );
        all_classes.insert(
            "warrior",
            include_bytes!("../assets/images/casts/warrior.png").to_vec(),
        );
        all_classes.insert(
            "mage",
            include_bytes!("../assets/images/casts/mage.png").to_vec(),
        );
        all_classes.insert(
            "raider",
            include_bytes!("../assets/images/casts/raider.png").to_vec(),
        );
        all_classes.insert(
            "ritualist",
            include_bytes!("../assets/images/casts/ritualist.png").to_vec(),
        );

        let mut map: HashMap<String, RgbaImage> = HashMap::new();
        for (cast_name, bytes) in &all_classes {
            map.insert(
                (*cast_name).to_string(),
                resize(
                    &load_from_memory(bytes).expect("Could not load image"),
                    60,
                    60,
                    FilterType::Lanczos3,
                ),
            );
        }
        map
    };
    pub static ref GUILD_RANKS: HashMap<String, RgbaImage> = {
        let mut all_ranks = HashMap::new();

        all_ranks.insert(
            "leader",
            include_bytes!("../assets/images/casts/leader.png").to_vec(),
        );
        all_ranks.insert(
            "officer",
            include_bytes!("../assets/images/casts/officer.png").to_vec(),
        );
        all_ranks.insert(
            "member",
            include_bytes!("../assets/images/casts/member.png").to_vec(),
        );

        let mut map: HashMap<String, RgbaImage> = HashMap::new();
        for (cast_name, bytes) in &all_ranks {
            map.insert(
                (*cast_name).to_string(),
                resize(
                    &load_from_memory(bytes).expect("Could not load image"),
                    55,
                    55,
                    FilterType::Lanczos3,
                ),
            );
        }
        map
    };
    pub static ref ITEM_TYPES: HashMap<String, RgbaImage> = {
        let mut all_items = HashMap::new();

        all_items.insert(
            "sword",
            include_bytes!("../assets/images/casts/sword.png").to_vec(),
        );
        all_items.insert(
            "shield",
            include_bytes!("../assets/images/casts/shield.png").to_vec(),
        );
        all_items.insert(
            "knife",
            include_bytes!("../assets/images/casts/knife.png").to_vec(),
        );
        all_items.insert(
            "dagger",
            include_bytes!("../assets/images/casts/dagger.png").to_vec(),
        );
        all_items.insert(
            "spear",
            include_bytes!("../assets/images/casts/spear.png").to_vec(),
        );
        all_items.insert(
            "hammer",
            include_bytes!("../assets/images/casts/hammer.png").to_vec(),
        );
        all_items.insert(
            "axe",
            include_bytes!("../assets/images/casts/axe.png").to_vec(),
        );
        all_items.insert(
            "bow",
            include_bytes!("../assets/images/casts/bow.png").to_vec(),
        );
        all_items.insert(
            "howlet",
            include_bytes!("../assets/images/casts/howlet.png").to_vec(),
        );
        all_items.insert(
            "scythe",
            include_bytes!("../assets/images/casts/scythe.png").to_vec(),
        );
        all_items.insert(
            "wand",
            include_bytes!("../assets/images/casts/wand.png").to_vec(),
        );

        let mut map: HashMap<String, RgbaImage> = HashMap::new();
        for (cast_name, bytes) in &all_items {
            map.insert(
                (*cast_name).to_string(),
                resize(
                    &load_from_memory(bytes).expect("Could not load image"),
                    80,
                    80,
                    FilterType::Lanczos3,
                ),
            );
        }
        map
    };
    pub static ref BADGES: HashMap<String, RgbaImage> = {
        let mut all_badges = HashMap::new();

        all_badges.insert(
            "contributor",
            include_bytes!("../assets/images/casts/contributor.png").to_vec(),
        );
        all_badges.insert(
            "designer",
            include_bytes!("../assets/images/casts/designer.png").to_vec(),
        );
        all_badges.insert(
            "developer",
            include_bytes!("../assets/images/casts/developer.png").to_vec(),
        );
        all_badges.insert(
            "gamedesigner",
            include_bytes!("../assets/images/casts/gamedesigner.png").to_vec(),
        );
        all_badges.insert(
            "gamemaster",
            include_bytes!("../assets/images/casts/gamemaster.png").to_vec(),
        );
        all_badges.insert(
            "leader",
            include_bytes!("../assets/images/casts/leader.png").to_vec(),
        );
        all_badges.insert(
            "member",
            include_bytes!("../assets/images/casts/member.png").to_vec(),
        );
        all_badges.insert(
            "officer",
            include_bytes!("../assets/images/casts/officer.png").to_vec(),
        );
        all_badges.insert(
            "support",
            include_bytes!("../assets/images/casts/support.png").to_vec(),
        );
        all_badges.insert(
            "tester",
            include_bytes!("../assets/images/casts/tester.png").to_vec(),
        );
        all_badges.insert(
            "veteran",
            include_bytes!("../assets/images/casts/veteran.png").to_vec(),
        );

        let mut map: HashMap<String, RgbaImage> = HashMap::new();
        for (cast_name, bytes) in &all_badges {
            map.insert(
                (*cast_name).to_string(),
                resize(
                    &load_from_memory(bytes).expect("Could not load image"),
                    45,
                    50,
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
