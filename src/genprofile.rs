use base64::encode;
use image::{
    imageops::{overlay, resize, FilterType},
    io::Reader,
    Rgba,
};
use imageproc::drawing::draw_text_mut;
use nix::sys::resource::{setrlimit, Resource};
use okapi_rewrite::encoder::encode_png;
use rusttype::{Font, Scale};
use serde_json;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process::exit;
use textwrap;

fn main() {
    let soft_mem_limit = Some(52428800);
    let hard_mem_limit = Some(52428800);
    let soft_cpu_limit = Some(2);
    let hard_cpu_limit = Some(2);
    setrlimit(Resource::RLIMIT_CPU, soft_cpu_limit, hard_cpu_limit).unwrap();
    setrlimit(Resource::RLIMIT_STACK, soft_mem_limit, hard_mem_limit).unwrap();
    setrlimit(Resource::RLIMIT_DATA, soft_mem_limit, hard_mem_limit).unwrap();
    setrlimit(Resource::RLIMIT_LOCKS, soft_mem_limit, hard_mem_limit).unwrap();
    let mut stdin = io::stdin();
    //let mut handle = stdin.lock();
    //let write_target = handle.fill_buf().unwrap();
    let mut read_target: Vec<u8> = Vec::new();
    stdin.read_to_end(&mut read_target).unwrap();
    let body: serde_json::Value =
        serde_json::from_str(&String::from_utf8(base64::decode(read_target).unwrap()).unwrap())
            .unwrap();
    let write_target = base64::decode(body["image"].as_str().unwrap().as_bytes()).unwrap();
    let b = io::Cursor::new(write_target.clone());
    let reader = Reader::new(b).with_guessed_format().unwrap();
    let dimensions = reader.into_dimensions().unwrap();
    if dimensions.0 > 2000 || dimensions.1 > 2000 {
        exit(1);
    }
    let mut path = env::current_dir().unwrap();
    path.push("assets");
    path.push("fonts");
    path.push("TravMedium.otf");
    let mut f = File::open(path).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let travitia_font = Font::try_from_vec(buf).unwrap();

    let c = io::Cursor::new(write_target);
    let new_reader = Reader::new(c).with_guessed_format().unwrap();
    let mut img = new_reader.decode().unwrap().to_rgba();
    let color = body["color"].as_array().unwrap();
    let classes = body["classes"].as_array().unwrap();
    let classes = [classes[0].as_str().unwrap(), classes[1].as_str().unwrap()];
    let r = color[0].as_i64().unwrap() as u8;
    let g = color[1].as_i64().unwrap() as u8;
    let b = color[2].as_i64().unwrap() as u8;
    let a = (color[3].as_f64().unwrap() * 255.0) as u8;
    let color = Rgba([r, g, b, a]);
    // Font size
    let mut scale = Scale { x: 26.0, y: 26.0 };
    draw_text_mut(
        &mut img,
        color,
        221,
        143,
        scale,
        &travitia_font,
        &body["name"].as_str().unwrap(),
    );
    draw_text_mut(
        &mut img,
        color,
        228,
        185,
        scale,
        &travitia_font,
        &body["race"].as_str().unwrap(),
    );
    scale = Scale { x: 23.0, y: 23.0 };
    draw_text_mut(
        &mut img,
        color,
        228,
        235,
        scale,
        &travitia_font,
        &classes[0],
    );
    draw_text_mut(
        &mut img,
        color,
        228,
        259,
        scale,
        &travitia_font,
        &classes[1],
    );
    scale = Scale { x: 15.0, y: 22.0 };
    draw_text_mut(
        &mut img,
        color,
        111,
        295,
        scale,
        &travitia_font,
        &body["damage"].as_str().unwrap(),
    );
    draw_text_mut(
        &mut img,
        color,
        111,
        337,
        scale,
        &travitia_font,
        &body["defense"].as_str().unwrap(),
    );
    scale = Scale { x: 22.0, y: 22.0 };
    draw_text_mut(
        &mut img,
        color,
        284,
        295,
        scale,
        &travitia_font,
        &body["level"].as_str().unwrap(),
    );
    draw_text_mut(&mut img, color, 284, 337, scale, &travitia_font, "soon™");
    let sword_name = body["sword_name"].as_str().unwrap();
    if sword_name.len() < 18 {
        scale = Scale { x: 35.0, y: 45.0 };
        draw_text_mut(
            &mut img,
            color,
            165,
            495,
            scale,
            &travitia_font,
            &sword_name,
        );
    } else {
        scale = Scale { x: 19.0, y: 19.0 };
        for (i, line) in textwrap::wrap_iter(&sword_name, 26).enumerate() {
            draw_text_mut(
                &mut img,
                color,
                165,
                495 + ((i as u32) * 20),
                scale,
                &travitia_font,
                &line,
            );
        }
    }
    let shield_name = body["shield_name"].as_str().unwrap();
    if shield_name.len() < 18 {
        scale = Scale { x: 35.0, y: 45.0 };
        draw_text_mut(
            &mut img,
            color,
            165,
            574,
            scale,
            &travitia_font,
            &shield_name,
        );
    } else {
        scale = Scale { x: 19.0, y: 19.0 };
        for (i, line) in textwrap::wrap_iter(&shield_name, 26).enumerate() {
            draw_text_mut(
                &mut img,
                color,
                165,
                574 + ((i as u32) * 20),
                scale,
                &travitia_font,
                &line,
            );
        }
    }
    scale = Scale { x: 52.0, y: 52.0 };
    draw_text_mut(
        &mut img,
        color,
        519,
        49,
        scale,
        &travitia_font,
        &body["money"].as_str().unwrap(),
    );
    draw_text_mut(&mut img, color, 519, 121, scale, &travitia_font, "soon™");
    draw_text_mut(
        &mut img,
        color,
        519,
        204,
        scale,
        &travitia_font,
        &body["god"].as_str().unwrap(),
    );
    draw_text_mut(
        &mut img,
        color,
        519,
        288,
        scale,
        &travitia_font,
        &body["guild"].as_str().unwrap(),
    );
    draw_text_mut(
        &mut img,
        color,
        519,
        379,
        scale,
        &travitia_font,
        &body["marriage"].as_str().unwrap(),
    );
    draw_text_mut(
        &mut img,
        color,
        519,
        459,
        scale,
        &travitia_font,
        &body["pvp_wins"].as_str().unwrap(),
    );
    let mut adv = body["adventure"].as_str().unwrap().lines();
    let line_1 = adv.next().unwrap();
    // Is there a second line?
    match adv.next() {
        Some(line_2) => {
            scale = Scale { x: 34.0, y: 34.0 };
            draw_text_mut(&mut img, color, 519, 538, scale, &travitia_font, line_1);
            draw_text_mut(&mut img, color, 519, 576, scale, &travitia_font, line_2);
        }
        None => {
            draw_text_mut(&mut img, color, 519, 545, scale, &travitia_font, line_1);
        }
    }
    let mut base = env::current_dir().unwrap();
    base.push("assets");
    base.push("images");
    base.push("casts");
    overlay(
        &mut img,
        &resize(
            &image::open(base.join(format!(
                "{}.png",
                &body["race"].as_str().unwrap().to_lowercase()
            )))
            .unwrap()
            .to_rgba(),
            22,
            22,
            FilterType::Lanczos3,
        ),
        205,
        184,
    );
    let icons = body["icons"].as_array().unwrap();
    let icon_1 = icons[0].as_str().unwrap();
    let icon_2 = icons[1].as_str().unwrap();
    if icon_1 != "none" {
        overlay(
            &mut img,
            &resize(
                &image::open(base.join(format!("{}.png", icon_1)))
                    .unwrap()
                    .to_rgba(),
                22,
                22,
                FilterType::Lanczos3,
            ),
            205,
            232,
        );
    }
    if icon_2 != "none" {
        overlay(
            &mut img,
            &resize(
                &image::open(base.join(format!("{}.png", icon_2)))
                    .unwrap()
                    .to_rgba(),
                22,
                22,
                FilterType::Lanczos3,
            ),
            205,
            254,
        );
    }
    let final_image = encode(encode_png(&img).unwrap());
    print!("{}", final_image);
}
