use ab_glyph::PxScale;
use hyper::{Body, Response};
use image::{
    imageops::overlay,
    io::{Limits, Reader},
    Rgba,
};
use imageproc::drawing::{draw_text_mut, Blend};
use serde::Deserialize;

use std::{io::Cursor, sync::Arc};

use crate::{
    cache::ImageCache,
    constants::{BADGES, CLASSES, DEFAULT_PROFILE, GUILD_RANKS, ITEM_TYPES, RACES, TRAVITIA_FONT},
    encoder::encode_png,
    error::{Error, Result},
    proxy::Fetcher,
};

#[derive(Deserialize)]
pub struct ProfileJson {
    name: String,
    image: String,
    race: String,
    color: (u8, u8, u8, f32), // RGBA
    classes: Vec<String>,
    class_icons: Vec<String>,
    left_hand_item: Option<(String, String, String)>, // Type, Name, Stat
    right_hand_item: Option<(String, String, String)>, // Type, Name, Stat
    level: String,
    guild_rank: Option<String>,
    guild_name: Option<String>,
    marriage: Option<String>,
    money: String,
    pvp_wins: String,
    god: String,
    adventure_name: Option<String>,
    adventure_time: Option<String>,
    badges: Vec<String>,
}

const PX_52: PxScale = PxScale { x: 52.0, y: 52.0 };
const PX_34: PxScale = PxScale { x: 34.0, y: 34.0 };
const PX_30: PxScale = PxScale { x: 30.0, y: 30.0 };
const PX_22: PxScale = PxScale { x: 22.0, y: 22.0 };

const BADGE_X_VALUES: [i64; 8] = [50, 144, 234, 327, 422, 513, 616, 712];

pub async fn genprofile(
    body: ProfileJson,
    fetcher: Arc<Fetcher>,
    images: ImageCache,
) -> Result<Response<Body>> {
    let image_url = &body.image;

    let mut img = if image_url == "0" {
        DEFAULT_PROFILE.clone()
    } else {
        let mut limits = Limits::default();
        limits.max_image_width = Some(2000);
        limits.max_image_height = Some(2000);

        let buf = fetcher.fetch(image_url).await?;

        let b = Cursor::new(buf);

        let mut reader = Reader::new(b);
        reader.limits(limits);
        reader = reader.with_guessed_format()?;

        let image = reader.decode()?.to_rgba8();

        if image.height() < 533 || image.width() < 800 {
            return Err(Error::ImageTooSmall);
        }

        image
    };

    overlay(&mut img, &RACES[&body.race.to_lowercase().as_str()], 6, 150);

    let class_icon_1 = &body.class_icons[0];
    let class_icon_2 = &body.class_icons[1];

    if class_icon_1 != "none" {
        overlay(&mut img, &CLASSES[class_icon_1.as_str()], 6, 244);
    }

    if class_icon_2 != "none" {
        overlay(&mut img, &CLASSES[class_icon_2.as_str()], 6, 300);
    }

    if let Some(rank) = body.guild_rank {
        overlay(
            &mut img,
            &GUILD_RANKS[&rank.to_lowercase().as_str()],
            610,
            3,
        );
    }

    if let Some((item_type, _, _)) = &body.right_hand_item {
        overlay(
            &mut img,
            &ITEM_TYPES[&item_type.to_lowercase().as_str()],
            262,
            117,
        );
    }

    if let Some((item_type, _, _)) = &body.left_hand_item {
        overlay(
            &mut img,
            &ITEM_TYPES[&item_type.to_lowercase().as_str()],
            262,
            188,
        );
    }

    for (index, badge) in body.badges.iter().enumerate() {
        overlay(
            &mut img,
            &BADGES[badge.as_str()],
            BADGE_X_VALUES[index],
            482,
        );
    }

    let mut blend = Blend(img);

    let r = body.color.0;
    let g = body.color.1;
    let b = body.color.2;
    let a = (body.color.3 * 255.0) as u8;
    let color = Rgba([r, g, b, a]);

    let character_name = if let Some(guild_name) = body.guild_name {
        format!("{} of {}", body.name, guild_name)
    } else {
        body.name
    };

    draw_text_mut(
        &mut blend,
        color,
        12,
        12,
        PX_52,
        550,
        &*TRAVITIA_FONT,
        &character_name,
    );

    draw_text_mut(
        &mut blend,
        color,
        720,
        16,
        PX_52,
        70,
        &*TRAVITIA_FONT,
        &body.level,
    );

    if let Some(marriage) = body.marriage {
        let text = format!("married to {}", marriage);

        draw_text_mut(
            &mut blend,
            color,
            180,
            76,
            PX_22,
            500,
            &*TRAVITIA_FONT,
            &text,
        );
    }

    draw_text_mut(
        &mut blend,
        color,
        70,
        168,
        PX_34,
        160,
        &*TRAVITIA_FONT,
        &body.race,
    );

    draw_text_mut(
        &mut blend,
        color,
        70,
        263,
        PX_34,
        160,
        &*TRAVITIA_FONT,
        &body.classes[0],
    );

    if body.classes[1] != "No Class" {
        draw_text_mut(
            &mut blend,
            color,
            70,
            320,
            PX_34,
            160,
            &*TRAVITIA_FONT,
            &body.classes[1],
        );
    }

    draw_text_mut(
        &mut blend,
        color,
        650,
        283,
        PX_30,
        140,
        &*TRAVITIA_FONT,
        &body.money,
    );

    draw_text_mut(
        &mut blend,
        color,
        650,
        332,
        PX_30,
        140,
        &*TRAVITIA_FONT,
        &body.pvp_wins,
    );

    draw_text_mut(
        &mut blend,
        color,
        650,
        381,
        PX_30,
        140,
        &*TRAVITIA_FONT,
        &body.god,
    );

    let adventure_text = body
        .adventure_name
        .unwrap_or_else(|| String::from("No Adventure"));

    draw_text_mut(
        &mut blend,
        color,
        345,
        298,
        PX_34,
        210,
        &*TRAVITIA_FONT,
        &adventure_text,
    );

    if let Some(time) = body.adventure_time {
        draw_text_mut(
            &mut blend,
            color,
            345,
            369,
            PX_34,
            190,
            &*TRAVITIA_FONT,
            &time,
        );
    }

    if let Some((_, item_name, stat)) = body.right_hand_item {
        draw_text_mut(
            &mut blend,
            color,
            345,
            135,
            PX_52,
            325,
            &*TRAVITIA_FONT,
            &item_name,
        );
        draw_text_mut(
            &mut blend,
            color,
            720,
            135,
            PX_52,
            85,
            &*TRAVITIA_FONT,
            &stat,
        );
    }

    if let Some((_, item_name, stat)) = body.left_hand_item {
        draw_text_mut(
            &mut blend,
            color,
            345,
            206,
            PX_52,
            325,
            &*TRAVITIA_FONT,
            &item_name,
        );
        draw_text_mut(
            &mut blend,
            color,
            720,
            206,
            PX_52,
            85,
            &*TRAVITIA_FONT,
            &stat,
        );
    }

    let final_image = encode_png(&blend.0)?;

    let tag = images.insert(final_image);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::from(tag))?)
}
