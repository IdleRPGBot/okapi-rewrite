use crate::constants::*;
use crate::encoder::encode_png;
use crate::proxy::fetch;
use actix_web::{post, web, HttpResponse};
use image::io::Reader;
use image::{imageops::overlay, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::Scale;
use serde::Deserialize;
use serde_json::value::Number;
use std::io::Cursor;
use textwrap::wrap;

#[derive(Deserialize)]
struct ProfileJson {
    name: String,
    image: String,
    race: String,
    color: Vec<Number>,   // RGBA array
    classes: Vec<String>, // Array of Strings
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
async fn genprofile(body: web::Json<ProfileJson>) -> HttpResponse {
    let image_url = &body.image;
    let mut img = match &image_url[..] {
        "0" => DEFAULT_PROFILE.clone(),
        _ => {
            let buf = match fetch(&image_url).await {
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
            // We can also happily unwrap here because it is the same data
            let c = Cursor::new(buf);
            Reader::new(c)
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap()
                .to_rgba()
        }
    };
    // I think unwrapping here is legitimate
    let r = body.color[0].as_u64().expect("R value should be u64") as u8;
    let g = body.color[1].as_u64().expect("G value should be u64") as u8;
    let b = body.color[2].as_u64().expect("B value should be u64") as u8;
    let a = (body.color[3].as_f64().expect("A value should be f64") * 255.0) as u8;
    let color = Rgba([r, g, b, a]);
    // Font size
    let mut scale = Scale { x: 26.0, y: 26.0 };
    draw_text_mut(&mut img, color, 221, 143, scale, &TRAVITIA_FONT, &body.name);
    draw_text_mut(&mut img, color, 228, 185, scale, &TRAVITIA_FONT, &body.race);
    scale = Scale { x: 23.0, y: 23.0 };
    draw_text_mut(
        &mut img,
        color,
        228,
        235,
        scale,
        &TRAVITIA_FONT,
        &body.classes[0],
    );
    draw_text_mut(
        &mut img,
        color,
        228,
        259,
        scale,
        &TRAVITIA_FONT,
        &body.classes[1],
    );
    scale = Scale { x: 15.0, y: 22.0 };
    draw_text_mut(
        &mut img,
        color,
        111,
        295,
        scale,
        &TRAVITIA_FONT,
        &body.damage,
    );
    draw_text_mut(
        &mut img,
        color,
        111,
        337,
        scale,
        &TRAVITIA_FONT,
        &body.defense,
    );
    scale = Scale { x: 22.0, y: 22.0 };
    draw_text_mut(
        &mut img,
        color,
        284,
        295,
        scale,
        &TRAVITIA_FONT,
        &body.level,
    );
    draw_text_mut(&mut img, color, 284, 337, scale, &TRAVITIA_FONT, "soon™");
    if body.sword_name.len() < 18 {
        scale = Scale { x: 35.0, y: 45.0 };
        draw_text_mut(
            &mut img,
            color,
            165,
            495,
            scale,
            &TRAVITIA_FONT,
            &body.sword_name,
        );
    } else {
        scale = Scale { x: 19.0, y: 19.0 };
        let rows = wrap(&body.sword_name, 26);
        for (i, line) in rows.iter().enumerate() {
            draw_text_mut(
                &mut img,
                color,
                165,
                495 + ((i as u32) * 20),
                scale,
                &TRAVITIA_FONT,
                &line,
            );
        }
    }
    if body.shield_name.len() < 18 {
        scale = Scale { x: 35.0, y: 45.0 };
        draw_text_mut(
            &mut img,
            color,
            165,
            574,
            scale,
            &TRAVITIA_FONT,
            &body.shield_name,
        );
    } else {
        scale = Scale { x: 19.0, y: 19.0 };
        let rows = wrap(&body.shield_name, 26);
        for (i, line) in rows.iter().enumerate() {
            draw_text_mut(
                &mut img,
                color,
                165,
                574 + ((i as u32) * 20),
                scale,
                &TRAVITIA_FONT,
                &line,
            );
        }
    }
    scale = Scale { x: 52.0, y: 52.0 };
    draw_text_mut(&mut img, color, 519, 49, scale, &TRAVITIA_FONT, &body.money);
    draw_text_mut(&mut img, color, 519, 121, scale, &TRAVITIA_FONT, "soon™");
    draw_text_mut(&mut img, color, 519, 204, scale, &TRAVITIA_FONT, &body.god);
    draw_text_mut(
        &mut img,
        color,
        519,
        288,
        scale,
        &TRAVITIA_FONT,
        &body.guild,
    );
    draw_text_mut(
        &mut img,
        color,
        519,
        379,
        scale,
        &TRAVITIA_FONT,
        &body.marriage,
    );
    draw_text_mut(
        &mut img,
        color,
        519,
        459,
        scale,
        &TRAVITIA_FONT,
        &body.pvp_wins,
    );
    let mut adv = body.adventure.as_str().lines();
    let line_1 = adv.next().unwrap();
    // Is there a second line?
    match adv.next() {
        Some(line_2) => {
            scale = Scale { x: 34.0, y: 34.0 };
            draw_text_mut(&mut img, color, 519, 538, scale, &TRAVITIA_FONT, line_1);
            draw_text_mut(&mut img, color, 519, 576, scale, &TRAVITIA_FONT, line_2);
        }
        None => {
            draw_text_mut(&mut img, color, 519, 545, scale, &TRAVITIA_FONT, line_1);
        }
    }
    overlay(&mut img, &CASTS[&body.race.to_lowercase()], 205, 184);
    let icon_1 = &body.icons[0];
    let icon_2 = &body.icons[1];
    if icon_1 != "none" {
        overlay(&mut img, &CASTS[icon_1], 205, 232);
    }
    if icon_2 != "none" {
        overlay(&mut img, &CASTS[icon_2], 205, 254);
    }
    let final_image = encode_png(&img).expect("encoding PNG failed");
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}
