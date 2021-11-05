use crate::{
    constants::{CASTS, DEFAULT_PROFILE, TRAVITIA_FONT},
    encoder::encode_png,
    error::Result,
    proxy::Fetcher,
};

use ab_glyph::PxScale;
use hyper::{Body, Response};
use image::{
    imageops::overlay,
    io::{Limits, Reader},
    Rgba,
};
use imageproc::drawing::{draw_text_mut, Blend};
use serde::Deserialize;
use textwrap::wrap;

use std::{io::Cursor, sync::Arc};

#[derive(Deserialize)]
pub struct ProfileJson {
    name: String,
    image: String,
    race: String,
    color: (u8, u8, u8, f32), // RGBA
    classes: Vec<String>,     // Array of Strings
    damage: String,
    defense: String,
    sword_name: String,
    shield_name: String,
    level: String, // might wanna make it u8
    money: String,
    god: String,
    guild: String,
    marriage: String,
    pvp_wins: String,
    adventure: String,
    icons: Vec<String>, // Array of Strings
}

const PX_52: PxScale = PxScale { x: 52.0, y: 52.0 };
const PX_34: PxScale = PxScale { x: 34.0, y: 34.0 };
const PX_26: PxScale = PxScale { x: 26.0, y: 26.0 };
const PX_23: PxScale = PxScale { x: 23.0, y: 23.0 };
const PX_22: PxScale = PxScale { x: 22.0, y: 22.0 };
const PX_19: PxScale = PxScale { x: 19.0, y: 19.0 };
const PX_22_CONDENSED: PxScale = PxScale { x: 15.0, y: 22.0 };
const PX_45_CONDENSED: PxScale = PxScale { x: 35.0, y: 45.0 };

pub async fn genprofile(body: ProfileJson, fetcher: Arc<Fetcher>) -> Result<Response<Body>> {
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
        reader.decode()?.to_rgba8()
    };

    overlay(&mut img, &CASTS[&body.race.to_lowercase()], 205, 184);

    let icon_1 = &body.icons[0];
    let icon_2 = &body.icons[1];

    if icon_1 != "none" {
        overlay(&mut img, &CASTS[icon_1], 205, 232);
    }

    if icon_2 != "none" {
        overlay(&mut img, &CASTS[icon_2], 205, 254);
    }

    let mut blend = Blend(img);

    let r = body.color.0;
    let g = body.color.1;
    let b = body.color.2;
    let a = (body.color.3 * 255.0) as u8;
    let color = Rgba([r, g, b, a]);

    draw_text_mut(
        &mut blend,
        color,
        221,
        143,
        PX_26,
        130,
        &*TRAVITIA_FONT,
        &body.name,
    );
    draw_text_mut(
        &mut blend,
        color,
        228,
        185,
        PX_26,
        130,
        &*TRAVITIA_FONT,
        &body.race,
    );
    draw_text_mut(
        &mut blend,
        color,
        228,
        235,
        PX_23,
        130,
        &*TRAVITIA_FONT,
        &body.classes[0],
    );
    draw_text_mut(
        &mut blend,
        color,
        228,
        259,
        PX_23,
        130,
        &*TRAVITIA_FONT,
        &body.classes[1],
    );
    draw_text_mut(
        &mut blend,
        color,
        111,
        295,
        PX_22_CONDENSED,
        95,
        &*TRAVITIA_FONT,
        &body.damage,
    );
    draw_text_mut(
        &mut blend,
        color,
        111,
        337,
        PX_22_CONDENSED,
        95,
        &*TRAVITIA_FONT,
        &body.defense,
    );
    draw_text_mut(
        &mut blend,
        color,
        284,
        295,
        PX_22,
        60,
        &*TRAVITIA_FONT,
        &body.level,
    );
    draw_text_mut(
        &mut blend,
        color,
        284,
        337,
        PX_22,
        60,
        &*TRAVITIA_FONT,
        "soon\u{2122}",
    );

    if body.sword_name.len() < 18 {
        draw_text_mut(
            &mut blend,
            color,
            165,
            495,
            PX_45_CONDENSED,
            200,
            &*TRAVITIA_FONT,
            &body.sword_name,
        );
    } else {
        let rows = wrap(&body.sword_name, 26);
        for (i, line) in rows.iter().enumerate() {
            draw_text_mut(
                &mut blend,
                color,
                165,
                495 + ((i as i32) * 20),
                PX_19,
                200,
                &*TRAVITIA_FONT,
                line,
            );
        }
    }

    if body.shield_name.len() < 18 {
        draw_text_mut(
            &mut blend,
            color,
            165,
            574,
            PX_45_CONDENSED,
            200,
            &*TRAVITIA_FONT,
            &body.shield_name,
        );
    } else {
        let rows = wrap(&body.shield_name, 26);
        for (i, line) in rows.iter().enumerate() {
            draw_text_mut(
                &mut blend,
                color,
                165,
                574 + ((i as i32) * 20),
                PX_19,
                200,
                &*TRAVITIA_FONT,
                line,
            );
        }
    }

    draw_text_mut(
        &mut blend,
        color,
        519,
        49,
        PX_52,
        231,
        &*TRAVITIA_FONT,
        &body.money,
    );
    draw_text_mut(
        &mut blend,
        color,
        519,
        121,
        PX_52,
        231,
        &*TRAVITIA_FONT,
        "soon\u{2122}",
    );
    draw_text_mut(
        &mut blend,
        color,
        519,
        204,
        PX_52,
        231,
        &*TRAVITIA_FONT,
        &body.god,
    );
    draw_text_mut(
        &mut blend,
        color,
        519,
        288,
        PX_52,
        231,
        &*TRAVITIA_FONT,
        &body.guild,
    );
    draw_text_mut(
        &mut blend,
        color,
        519,
        379,
        PX_52,
        231,
        &*TRAVITIA_FONT,
        &body.marriage,
    );
    draw_text_mut(
        &mut blend,
        color,
        519,
        459,
        PX_52,
        231,
        &*TRAVITIA_FONT,
        &body.pvp_wins,
    );

    let mut adv = body.adventure.as_str().lines();
    let line_1 = adv.next().unwrap();
    // Is there a second line?
    match adv.next() {
        Some(line_2) => {
            draw_text_mut(
                &mut blend,
                color,
                519,
                538,
                PX_34,
                231,
                &*TRAVITIA_FONT,
                line_1,
            );
            draw_text_mut(
                &mut blend,
                color,
                519,
                576,
                PX_34,
                231,
                &*TRAVITIA_FONT,
                line_2,
            );
        }
        None => {
            draw_text_mut(
                &mut blend,
                color,
                519,
                545,
                PX_52,
                231,
                &*TRAVITIA_FONT,
                line_1,
            );
        }
    }

    let final_image = encode_png(&blend.0)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "image/png")
        .body(Body::from(final_image))?)
}
