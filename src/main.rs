#[macro_use]
extern crate lazy_static;
extern crate actix_web;
extern crate image;

use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use base64;
use image::{
    imageops::{invert, overlay, resize, FilterType},
    png::PNGEncoder,
    ImageBuffer, ImageError, Pixel, Rgb, RgbImage, Rgba, RgbaImage,
};
use imageproc::drawing::draw_text_mut;
use imageproc::edges::canny;
use reqwest::{header::HeaderMap, header::HeaderName, Client};
use resvg::prelude::*;
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::time::Duration;
use std::{env, io, path};

#[derive(Debug, Serialize, Deserialize)]
struct AdventuresJson {
    percentages: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChessJson {
    xml: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OverlayJson {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProfileJson {
    name: String,
    image: String,
    race: String,
    color: Value,   // RGBA array
    classes: Value, // Array of Strings
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
    icons: Value, // Array of Strings
}

#[derive(Debug, Serialize, Deserialize)]
struct ImageJson {
    image: String, // URL
}

fn load_font(name: &str) -> Font {
    let mut path = env::current_dir().unwrap();
    path.push("assets");
    path.push("fonts");
    path.push(name);
    let mut f = File::open(path).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let font = Font::try_from_vec(buf).unwrap();
    font
}

fn load_image_rgb(path: path::PathBuf) -> RgbImage {
    image::open(path).unwrap().to_rgb()
}

fn load_image_rgba(path: path::PathBuf) -> RgbaImage {
    image::open(path).unwrap().to_rgba()
}

fn wrap(thing: &str, n: usize) -> Vec<&str> {
    let mut slices: Vec<&str> = Vec::new();
    let mut thing = thing.clone();

    // while string is not empty
    // take n characters
    // check witch one was the last space or if the end of the line is reached
    // then => push them in slices
    // then => remove them from the string

    while thing != "" {
        let mut last_space = 0;

        for i in 0..thing.len() {
            if i == n {
                break;
            }
            if thing.chars().nth(i).unwrap() == ' ' {
                last_space = i;
            }
            if i == thing.len() - 1 {
                last_space = thing.len();
            }
        }

        // insert into array
        slices.push(thing[0..last_space].trim());
        thing = &thing[last_space..];
    }
    slices
}

lazy_static! {
    static ref CONFIG: Value = {
        let mut file = File::open("config.json").expect("Config not found");
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let json = from_str(&data).expect("Invalid JSON");
        json
    };
    static ref TRAVITIA_FONT: Font<'static> = load_font("TravMedium.otf");
    static ref CAVIAR_DREAMS: Font<'static> = load_font("CaviarDreams.ttf");
    static ref OPEN_SANS_EMOJI: Font<'static> = load_font("OpenSansEmoji.ttf");
    static ref K_GOTHIC: Font<'static> = load_font("K Gothic.ttf");
    static ref PROFILE: RgbaImage = {
        let mut base = env::current_dir().unwrap();
        base.push("assets");
        base.push("images");
        base.push("ProfileOverlayNew.png");
        load_image_rgba(base)
    };
    static ref DEFAULT_PROFILE: RgbaImage = load_image_rgba(
        env::current_dir()
            .unwrap()
            .join("assets")
            .join("images")
            .join("ProfileNew.png")
    );
    static ref CASTS: HashMap<String, RgbaImage> = {
        let mut base = env::current_dir().unwrap();
        base.push("assets");
        base.push("images");
        base.push("casts");
        let mut map: HashMap<String, RgbaImage> = HashMap::new();
        map.insert(
            "thief".to_string(),
            resize(
                &load_image_rgba(base.join("thief.png")),
                22,
                22,
                FilterType::Lanczos3,
            ),
        );
        map.insert(
            "paragon".to_string(),
            resize(
                &load_image_rgba(base.join("paragon.png")),
                22,
                22,
                FilterType::Lanczos3,
            ),
        );
        map.insert(
            "ranger".to_string(),
            resize(
                &load_image_rgba(base.join("ranger.png")),
                22,
                22,
                FilterType::Lanczos3,
            ),
        );
        map.insert(
            "warrior".to_string(),
            resize(
                &load_image_rgba(base.join("warrior.png")),
                22,
                22,
                FilterType::Lanczos3,
            ),
        );
        map.insert(
            "mage".to_string(),
            resize(
                &load_image_rgba(base.join("mage.png")),
                22,
                22,
                FilterType::Lanczos3,
            ),
        );
        map.insert(
            "raider".to_string(),
            resize(
                &load_image_rgba(base.join("raider.png")),
                22,
                22,
                FilterType::Lanczos3,
            ),
        );
        map.insert(
            "ritualist".to_string(),
            resize(
                &load_image_rgba(base.join("ritualist.png")),
                22,
                22,
                FilterType::Lanczos3,
            ),
        );
        map.insert(
            "human".to_string(),
            resize(
                &load_image_rgba(base.join("human.png")),
                22,
                22,
                FilterType::Lanczos3,
            ),
        );
        map.insert(
            "elf".to_string(),
            resize(
                &load_image_rgba(base.join("elf.png")),
                22,
                22,
                FilterType::Lanczos3,
            ),
        );
        map.insert(
            "jikill".to_string(),
            resize(
                &load_image_rgba(base.join("jikill.png")),
                22,
                22,
                FilterType::Lanczos3,
            ),
        );
        map.insert(
            "dwarf".to_string(),
            resize(
                &load_image_rgba(base.join("dwarf.png")),
                22,
                22,
                FilterType::Lanczos3,
            ),
        );
        map.insert(
            "orc".to_string(),
            resize(
                &load_image_rgba(base.join("orc.png")),
                22,
                22,
                FilterType::Lanczos3,
            ),
        );
        map
    };
    static ref ADVENTURES: Vec<RgbImage> = {
        let mut base = env::current_dir().unwrap();
        base.push("assets");
        base.push("images");
        base.push("adventures");
        let base = base.into_os_string();
        let mut images = Vec::new();
        for i in 1..30 {
            let mut path = path::PathBuf::from(base.clone());
            path.push(format!("{}.png", i));
            images.push(load_image_rgb(path));
        }
        images
    };
    static ref CLIENT: Client = Client::new();
    static ref HEADERS: HeaderMap = {
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

async fn fetch(url: &str) -> actix_web::web::Bytes {
    let mut headers = HEADERS.clone();
    let requested_uri = HeaderName::from_lowercase(b"requested-uri").unwrap();
    let proxy_base = CONFIG["proxy"].as_str().unwrap();
    headers.insert(requested_uri, url.parse().unwrap());
    let resp = CLIENT
        .get(proxy_base)
        .headers(headers)
        .timeout(Duration::new(3, 0))
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    resp
}

fn encode_png<P, Container>(img: &ImageBuffer<P, Container>) -> Result<Vec<u8>, ImageError>
where
    P: Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut buf = Vec::new();
    let encoder = PNGEncoder::new(&mut buf);
    encoder.encode(img, img.width(), img.height(), P::COLOR_TYPE)?;
    Ok(buf)
}

#[get("/")]
async fn index() -> HttpResponse {
    // For metrics
    HttpResponse::Ok().content_type("text/plain").body("1")
}

#[post("/api/imageops/pixel")]
async fn pixelate(body: web::Json<ImageJson>) -> HttpResponse {
    let res = fetch(&body.image).await;
    let img = image::load_from_memory(&res).unwrap().to_rgba();
    let buf = resize(&img, 1024, 1024, FilterType::Nearest);
    let final_image = encode_png(&buf).unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}

#[post("/api/imageops/invert")]
async fn invert_endpoint(body: web::Json<ImageJson>) -> HttpResponse {
    let res = fetch(&body.image).await;
    let mut img = image::load_from_memory(&res).unwrap().to_rgba();
    invert(&mut img);
    let final_image = encode_png(&img).unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}

#[post("/api/imageops/edges")]
async fn edges_endpoint(body: web::Json<ImageJson>) -> HttpResponse {
    let res = fetch(&body.image).await;
    let img = image::load_from_memory(&res).unwrap().to_luma();
    let buf = canny(&img, 25.0, 80.0);
    let final_image = encode_png(&buf).unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}

#[derive(Debug)]
struct Intensity {
    val: u8,
    r: u8,
    g: u8,
    b: u8,
}

#[post("/api/imageops/oil")]
async fn oil_endpoint(body: web::Json<ImageJson>) -> HttpResponse {
    let res = fetch(&body.image).await;
    let img = image::load_from_memory(&res).unwrap().to_rgba();
    let radius = 4 as i32;
    let intensity = 55.0;
    let width = img.width();
    let height = img.height();
    let mut target = image::RgbaImage::new(width, height);
    let mut pixel_intensity_count: HashMap<usize, Intensity>;
    let mut intensity_lut = vec![vec![0; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let current_val = img.get_pixel(x, y).channels();
            let avg = (current_val[0] as i32 + current_val[1] as i32 + current_val[2] as i32)
                as f64
                / 3.0;
            let val = (avg * intensity) / 255.0;
            intensity_lut[y as usize][x as usize] = val.round() as usize;
        }
    }

    for y in 0..height {
        for x in 0..width {
            pixel_intensity_count = HashMap::new();
            for yy in -radius..radius {
                let yyy = (y as i32) + yy;
                for xx in -radius..radius {
                    let xxx = (x as i32) + xx;
                    if yyy > 0 && yyy < (height as i32) && xxx > 0 && xxx < (width as i32) {
                        let idx_x = xxx as usize;
                        let idx_y = yyy as usize;
                        let intensity_val = intensity_lut[idx_y][idx_x];
                        let pix = img.get_pixel(idx_x as u32, idx_y as u32).channels();
                        match pixel_intensity_count.get_mut(&(intensity_val as usize)) {
                            Some(val) => {
                                val.val = val.val + 1;
                                val.r = val.r + pix[0];
                                val.g = val.g + pix[1];
                                val.b = val.b + pix[2];
                            }
                            None => {
                                pixel_intensity_count.insert(
                                    intensity_val as usize,
                                    Intensity {
                                        val: 1,
                                        r: pix[0],
                                        g: pix[1],
                                        b: pix[2],
                                    },
                                );
                            }
                        }
                    }
                }
            }

            let mut map_vec: Vec<_> = pixel_intensity_count.iter().collect();
            map_vec.sort_by(|a, b| (b.1.val - a.1.val).cmp(&0));

            let cur_max = map_vec[0].1;
            target.put_pixel(
                x,
                y,
                Rgba::<u8>([
                    (cur_max.r / cur_max.val),
                    (cur_max.g / cur_max.val),
                    (cur_max.b / cur_max.val),
                    255,
                ]),
            )
        }
    }
    let final_image = encode_png(&target).unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}

#[post("/api/genprofile")]
async fn genprofile(body: web::Json<ProfileJson>) -> HttpResponse {
    let image_url = &body.image;
    // Load or download their background image
    let mut img: RgbaImage;
    if image_url == "0" {
        img = DEFAULT_PROFILE.clone();
    } else {
        let res = fetch(&image_url).await;
        img = image::load_from_memory(&res).unwrap().to_rgba();
    }
    let color = body.color.as_array().unwrap();
    let classes = body.classes.as_array().unwrap();
    let classes = [classes[0].as_str().unwrap(), classes[1].as_str().unwrap()];
    let r = color[0].as_i64().unwrap() as u8;
    let g = color[1].as_i64().unwrap() as u8;
    let b = color[2].as_i64().unwrap() as u8;
    let a = (color[3].as_f64().unwrap() * 255.0) as u8;
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
        &classes[0],
    );
    draw_text_mut(
        &mut img,
        color,
        228,
        259,
        scale,
        &TRAVITIA_FONT,
        &classes[1],
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
    let icons = body.icons.as_array().unwrap();
    let icon_1 = icons[0].as_str().unwrap();
    let icon_2 = icons[1].as_str().unwrap();
    if icon_1 != "none" {
        overlay(&mut img, &CASTS[icon_1], 205, 232);
    }
    if icon_2 != "none" {
        overlay(&mut img, &CASTS[icon_2], 205, 254);
    }
    let final_image = encode_png(&img).unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}

#[post("/api/genoverlay")]
async fn genoverlay(body: web::Json<OverlayJson>) -> HttpResponse {
    let url = &body.url;
    let res = fetch(&url).await;
    // Lanczos3 is best, but has slow speed
    let mut img = resize(
        &image::load_from_memory(&res).unwrap().to_rgba(),
        800,
        650,
        FilterType::Lanczos3,
    );
    let img2 = PROFILE.clone();
    overlay(&mut img, &img2, 0, 0);
    let final_image = encode_png(&img).unwrap();
    let buf = base64::encode(&final_image);
    HttpResponse::Ok().content_type("text/plain").body(buf)
}

#[post("/api/genchess")]
async fn genchess(body: web::Json<ChessJson>) -> HttpResponse {
    let xml = &body.xml;
    let opts = resvg::Options {
        background: None,
        fit_to: resvg::FitTo::Width(390),
        usvg: resvg::Options::default().usvg,
    };
    let tree = usvg::Tree::from_str(&xml, &opts.usvg).unwrap();
    let mut img = resvg::backend_raqote::render_to_image(&tree, &opts).unwrap();
    let vect = img.make_vec();
    let final_image = encode_png(
        &image::RgbaImage::from_vec(img.height() as u32, img.width() as u32, vect).unwrap(),
    )
    .unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(final_image)
}

#[post("/api/genadventures")]
async fn genadventures(body: web::Json<AdventuresJson>) -> HttpResponse {
    // body is the parsed JSON
    let chances = body.percentages.as_array().unwrap();
    let mut images = Vec::new();
    for idx in 0..29 {
        let current_chances = &chances[idx].as_array().unwrap();
        let chance_min = &current_chances[0];
        let chance_max = &current_chances[1];
        let mut new_image = ADVENTURES[idx].clone();
        let white = Rgb([0u8, 0u8, 0u8]);
        let scale = Scale { x: 20.0, y: 20.0 };
        if idx > 9 && idx < 20 {
            draw_text_mut(
                &mut new_image,
                white,
                314,
                168,
                scale,
                &TRAVITIA_FONT,
                &format!("{}% to", chance_min),
            );
            draw_text_mut(
                &mut new_image,
                white,
                314,
                188,
                scale,
                &TRAVITIA_FONT,
                &format!("{}%", chance_max),
            );
        } else {
            draw_text_mut(
                &mut new_image,
                white,
                314,
                148,
                scale,
                &TRAVITIA_FONT,
                &format!("{}% to", chance_min),
            );
            draw_text_mut(
                &mut new_image,
                white,
                314,
                168,
                scale,
                &TRAVITIA_FONT,
                &format!("{}%", chance_max),
            );
        }
        let final_image = encode_png(&new_image).unwrap();
        let buf = format!("data:image/png;base64,{}", base64::encode(&final_image));
        images.push(buf);
    }
    HttpResponse::Ok().json(images)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(index)
            .service(genadventures)
            .service(genchess)
            .service(genoverlay)
            .service(genprofile)
            .service(pixelate)
            .service(invert_endpoint)
            .service(edges_endpoint)
            .service(oil_endpoint)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
