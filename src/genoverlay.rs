use base64::encode;
use image::{
    imageops::{overlay, resize, FilterType},
    io::Reader,
    png::PNGEncoder,
    ImageBuffer, ImageError, Pixel,
};
use nix::sys::resource::{setrlimit, Resource};
use std::env;
use std::io::{self, Read};
use std::ops::Deref;
use std::process::exit;

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
    let mut write_target: Vec<u8> = Vec::new();
    stdin.read_to_end(&mut write_target).unwrap();
    let b = io::Cursor::new(write_target.clone());
    let reader = Reader::new(b).with_guessed_format().unwrap();
    let dimensions = reader.into_dimensions().unwrap();
    if dimensions.0 > 2000 || dimensions.1 > 2000 {
        exit(1);
    }
    let c = io::Cursor::new(write_target);
    let new_reader = Reader::new(c).with_guessed_format().unwrap();
    let img = new_reader.decode().unwrap().to_rgba();
    // Lanczos3 is best, but has slow speed
    let mut img = resize(&img, 800, 650, FilterType::Lanczos3);
    let mut base = env::current_dir().unwrap();
    base.push("assets");
    base.push("images");
    base.push("ProfileOverlayNew.png");
    let img2 = image::open(base).unwrap().to_rgba();
    overlay(&mut img, &img2, 0, 0);
    let final_image = encode(encode_png(&img).unwrap());
    print!("{:?}", final_image);
}
