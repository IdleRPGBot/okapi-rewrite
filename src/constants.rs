use ab_glyph::FontVec;
use hyper::Uri;
use image::{load_from_memory, RgbImage, RgbaImage};
use lazy_static::lazy_static;
use std::{collections::HashMap, env::var, str::FromStr};

lazy_static! {
    pub static ref PORT: u16 = var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse::<u16>()
        .unwrap();
    pub static ref PROXY_URL: Uri = var("PROXY_URL")
        .map(|p| Uri::from_str(&p).unwrap())
        .expect("PROXY_URL is required");
    pub static ref PROXY_USER: String = var("PROXY_USERNAME").expect("PROXY_USERNAME is required");
    pub static ref PROXY_PASSWORD: String =
        var("PROXY_PASSWORD").expect("PROXY_PASSWORD is required");
    pub static ref AUTH_KEY: Option<String> = var("AUTH_KEY").ok();
    pub static ref EXTERNAL_URL: String =
        var("EXTERNAL_URL").unwrap_or(format!("http://localhost:{}", *PORT));
    pub static ref TRAVITIA_FONT: FontVec =
        FontVec::try_from_vec(include_bytes!("../assets/fonts/MergedNoKern.otf").to_vec())
            .expect("could not load font");
    pub static ref PROFILE_DARK: RgbaImage = load_from_memory(include_bytes!(
        "../assets/images/out/profile_overlay_dark.png"
    ))
    .expect("Could not load image")
    .into_rgba8();
    pub static ref PROFILE_LIGHT: RgbaImage = load_from_memory(include_bytes!(
        "../assets/images/out/profile_overlay_light.png"
    ))
    .expect("Could not load image")
    .into_rgba8();
    pub static ref DEFAULT_PROFILE: RgbaImage =
        load_from_memory(include_bytes!("../assets/images/out/profile.png"))
            .expect("Could not load image")
            .into_rgba8();
    pub static ref RACES: HashMap<&'static str, RgbaImage> = {
        let mut all_races = HashMap::new();

        all_races.insert(
            "human",
            load_from_memory(include_bytes!("../assets/images/out/casts/human.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_races.insert(
            "elf",
            load_from_memory(include_bytes!("../assets/images/out/casts/elf.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_races.insert(
            "dwarf",
            load_from_memory(include_bytes!("../assets/images/out/casts/dwarf.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_races.insert(
            "orc",
            load_from_memory(include_bytes!("../assets/images/out/casts/orc.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_races.insert(
            "jikill",
            load_from_memory(include_bytes!("../assets/images/out/casts/jikill.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );

        all_races
    };
    pub static ref CLASSES: HashMap<&'static str, RgbaImage> = {
        let mut all_classes = HashMap::new();

        all_classes.insert(
            "thief",
            load_from_memory(include_bytes!("../assets/images/out/casts/thief.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_classes.insert(
            "paragon",
            load_from_memory(include_bytes!("../assets/images/out/casts/paragon.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_classes.insert(
            "ranger",
            load_from_memory(include_bytes!("../assets/images/out/casts/ranger.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_classes.insert(
            "warrior",
            load_from_memory(include_bytes!("../assets/images/out/casts/warrior.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_classes.insert(
            "mage",
            load_from_memory(include_bytes!("../assets/images/out/casts/mage.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_classes.insert(
            "raider",
            load_from_memory(include_bytes!("../assets/images/out/casts/raider.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_classes.insert(
            "ritualist",
            load_from_memory(include_bytes!("../assets/images/out/casts/ritualist.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );

        all_classes
    };
    pub static ref GUILD_RANKS: HashMap<&'static str, RgbaImage> = {
        let mut all_ranks = HashMap::new();

        all_ranks.insert(
            "leader",
            load_from_memory(include_bytes!("../assets/images/out/casts/leader.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_ranks.insert(
            "officer",
            load_from_memory(include_bytes!("../assets/images/out/casts/officer.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_ranks.insert(
            "member",
            load_from_memory(include_bytes!("../assets/images/out/casts/member.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );

        all_ranks
    };
    pub static ref ITEM_TYPES: HashMap<&'static str, RgbaImage> = {
        let mut all_items = HashMap::new();

        all_items.insert(
            "sword",
            load_from_memory(include_bytes!("../assets/images/out/casts/sword.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_items.insert(
            "shield",
            load_from_memory(include_bytes!("../assets/images/out/casts/shield.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_items.insert(
            "knife",
            load_from_memory(include_bytes!("../assets/images/out/casts/knife.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_items.insert(
            "dagger",
            load_from_memory(include_bytes!("../assets/images/out/casts/dagger.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_items.insert(
            "spear",
            load_from_memory(include_bytes!("../assets/images/out/casts/spear.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_items.insert(
            "hammer",
            load_from_memory(include_bytes!("../assets/images/out/casts/hammer.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_items.insert(
            "axe",
            load_from_memory(include_bytes!("../assets/images/out/casts/axe.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_items.insert(
            "bow",
            load_from_memory(include_bytes!("../assets/images/out/casts/bow.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_items.insert(
            "howlet",
            load_from_memory(include_bytes!("../assets/images/out/casts/howlet.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_items.insert(
            "scythe",
            load_from_memory(include_bytes!("../assets/images/out/casts/scythe.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_items.insert(
            "wand",
            load_from_memory(include_bytes!("../assets/images/out/casts/wand.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );

        all_items
    };
    pub static ref BADGES: HashMap<&'static str, RgbaImage> = {
        let mut all_badges = HashMap::new();

        all_badges.insert(
            "contributor",
            load_from_memory(include_bytes!("../assets/images/out/casts/contributor.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_badges.insert(
            "designer",
            load_from_memory(include_bytes!("../assets/images/out/casts/designer.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_badges.insert(
            "developer",
            load_from_memory(include_bytes!("../assets/images/out/casts/developer.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_badges.insert(
            "gamedesigner",
            load_from_memory(include_bytes!(
                "../assets/images/out/casts/gamedesigner.png"
            ))
            .expect("Could not load image")
            .into_rgba8(),
        );
        all_badges.insert(
            "gamemaster",
            load_from_memory(include_bytes!("../assets/images/out/casts/gamemaster.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_badges.insert(
            "support",
            load_from_memory(include_bytes!("../assets/images/out/casts/support.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_badges.insert(
            "tester",
            load_from_memory(include_bytes!("../assets/images/out/casts/tester.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );
        all_badges.insert(
            "veteran",
            load_from_memory(include_bytes!("../assets/images/out/casts/veteran.png"))
                .expect("Could not load image")
                .into_rgba8(),
        );

        all_badges
    };
    pub static ref ADVENTURES: Vec<RgbImage> = vec![
        load_from_memory(include_bytes!("../assets/images/out/adventures/1.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/2.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/3.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/4.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/5.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/6.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/7.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/8.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/9.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/10.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/11.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/12.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/13.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/14.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/15.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/16.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/17.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/18.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/19.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/20.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/21.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/22.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/23.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/24.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/25.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/26.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/27.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/28.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/29.png"))
            .expect("Could not load image")
            .into_rgb8(),
        load_from_memory(include_bytes!("../assets/images/out/adventures/30.png"))
            .expect("Could not load image")
            .into_rgb8()
    ];
}
