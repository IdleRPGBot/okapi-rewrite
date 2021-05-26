use crate::{constants::*, encoder::encode_png, proxy::Fetcher};
use ab_glyph::PxScale;
use actix_web::{post, web, HttpResponse};
use image::{imageops::overlay, io::Reader, Rgba};
use imageproc::drawing::{draw_text_mut, Blend};
use serde::Deserialize;
use std::io::Cursor;
use textwrap::wrap;

#[derive(Deserialize)]
struct ProfileJson {
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

#[post("/api/genprofile")]
async fn genprofile(body: web::Json<ProfileJson>, fetcher: web::Data<Fetcher>) -> HttpResponse {
    let image_url = &body.image;
    let mut img = match &image_url[..] {
        "0" => DEFAULT_PROFILE.clone(),
        _ => {
            let buf = match fetcher.fetch(&image_url).await {
                Ok(buf) => buf,
                Err(e) => {
                    return HttpResponse::UnprocessableEntity()
                        .content_type("application/json")
                        .body(format!(
                            "{{\"status\": \"error\", \"reason\": \"download error\", \"detail\": \"{}\"}}",
                            e
                        ))
                }
            };
            let b = Cursor::new(buf.clone());
            let reader = match Reader::new(b).with_guessed_format() {
                Ok(r) => r,
                Err(e) => {
                    return HttpResponse::UnprocessableEntity()
                        .content_type("application/json")
                        .body(format!(
                        "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{}\"}}",
                        e
                    ))
                }
            };
            let dimensions = match reader.into_dimensions() {
                Ok(d) => d,
                Err(e) => {
                    return HttpResponse::UnprocessableEntity()
                        .content_type("application/json")
                        .body(format!(
                            "{{\"status\": \"error\", \"reason\": \"invalid image data\", \"detail\": \"{}\"}}",
                            e
                        ))
                }
            };
            if dimensions.0 > 2000 || dimensions.1 > 2000 {
                return HttpResponse::UnprocessableEntity()
                    .content_type("application/json")
                    .body("{\"status\": \"error\", \"reason\": \"image too large\", \"detail\": \"the image file exceeds the size of 2000 pixels in at least one axis\"}");
            }
            if dimensions.0 < 800 || dimensions.1 < 650 {
                return HttpResponse::UnprocessableEntity()
                    .content_type("application/json")
                    .body("{\"status\": \"error\", \"reason\": \"image too small\", \"detail\": \"the image file needs to be at least 800x650px in size\"}");
            }
            // We can also happily unwrap here because it is the same data
            let c = Cursor::new(buf);
            Reader::new(c)
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap()
                .to_rgba8()
        }
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
    // I think unwrapping here is legitimate
    let r = body.color.0;
    let g = body.color.1;
    let b = body.color.2;
    let a = (body.color.3 * 255.0) as u8;
    let color = Rgba([r, g, b, a]);
    // Font size
    let mut scale = PxScale { x: 26.0, y: 26.0 };
    draw_text_mut(
        &mut blend,
        color,
        221,
        143,
        scale,
        130,
        &*TRAVITIA_FONT,
        &body.name,
    );
    draw_text_mut(
        &mut blend,
        color,
        228,
        185,
        scale,
        130,
        &*TRAVITIA_FONT,
        &body.race,
    );
    scale = PxScale { x: 23.0, y: 23.0 };
    draw_text_mut(
        &mut blend,
        color,
        228,
        235,
        scale,
        130,
        &*TRAVITIA_FONT,
        &body.classes[0],
    );
    draw_text_mut(
        &mut blend,
        color,
        228,
        259,
        scale,
        130,
        &*TRAVITIA_FONT,
        &body.classes[1],
    );
    scale = PxScale { x: 15.0, y: 22.0 };
    draw_text_mut(
        &mut blend,
        color,
        111,
        295,
        scale,
        95,
        &*TRAVITIA_FONT,
        &body.damage,
    );
    draw_text_mut(
        &mut blend,
        color,
        111,
        337,
        scale,
        95,
        &*TRAVITIA_FONT,
        &body.defense,
    );
    scale = PxScale { x: 22.0, y: 22.0 };
    draw_text_mut(
        &mut blend,
        color,
        284,
        295,
        scale,
        60,
        &*TRAVITIA_FONT,
        &body.level,
    );
    draw_text_mut(
        &mut blend,
        color,
        284,
        337,
        scale,
        60,
        &*TRAVITIA_FONT,
        "soon™",
    );
    if body.sword_name.len() < 18 {
        scale = PxScale { x: 35.0, y: 45.0 };
        draw_text_mut(
            &mut blend,
            color,
            165,
            495,
            scale,
            200,
            &*TRAVITIA_FONT,
            &body.sword_name,
        );
    } else {
        scale = PxScale { x: 19.0, y: 19.0 };
        let rows = wrap(&body.sword_name, 26);
        for (i, line) in rows.iter().enumerate() {
            draw_text_mut(
                &mut blend,
                color,
                165,
                495 + ((i as u32) * 20),
                scale,
                200,
                &*TRAVITIA_FONT,
                &line,
            );
        }
    }
    if body.shield_name.len() < 18 {
        scale = PxScale { x: 35.0, y: 45.0 };
        draw_text_mut(
            &mut blend,
            color,
            165,
            574,
            scale,
            200,
            &*TRAVITIA_FONT,
            &body.shield_name,
        );
    } else {
        scale = PxScale { x: 19.0, y: 19.0 };
        let rows = wrap(&body.shield_name, 26);
        for (i, line) in rows.iter().enumerate() {
            draw_text_mut(
                &mut blend,
                color,
                165,
                574 + ((i as u32) * 20),
                scale,
                200,
                &*TRAVITIA_FONT,
                &line,
            );
        }
    }
    scale = PxScale { x: 52.0, y: 52.0 };
    draw_text_mut(
        &mut blend,
        color,
        519,
        49,
        scale,
        231,
        &*TRAVITIA_FONT,
        &body.money,
    );
    draw_text_mut(
        &mut blend,
        color,
        519,
        121,
        scale,
        231,
        &*TRAVITIA_FONT,
        "soon™",
    );
    draw_text_mut(
        &mut blend,
        color,
        519,
        204,
        scale,
        231,
        &*TRAVITIA_FONT,
        &body.god,
    );
    draw_text_mut(
        &mut blend,
        color,
        519,
        288,
        scale,
        231,
        &*TRAVITIA_FONT,
        &body.guild,
    );
    draw_text_mut(
        &mut blend,
        color,
        519,
        379,
        scale,
        231,
        &*TRAVITIA_FONT,
        &body.marriage,
    );
    draw_text_mut(
        &mut blend,
        color,
        519,
        459,
        scale,
        231,
        &*TRAVITIA_FONT,
        &body.pvp_wins,
    );
    let mut adv = body.adventure.as_str().lines();
    let line_1 = adv.next().unwrap();
    // Is there a second line?
    match adv.next() {
        Some(line_2) => {
            scale = PxScale { x: 34.0, y: 34.0 };
            draw_text_mut(
                &mut blend,
                color,
                519,
                538,
                scale,
                231,
                &*TRAVITIA_FONT,
                line_1,
            );
            draw_text_mut(
                &mut blend,
                color,
                519,
                576,
                scale,
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
                scale,
                231,
                &*TRAVITIA_FONT,
                line_1,
            );
        }
    }
    let final_image = encode_png(&blend.0).expect("encoding PNG failed");
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}
