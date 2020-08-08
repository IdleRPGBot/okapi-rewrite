use image::{png::PNGEncoder, ImageBuffer, ImageError, Pixel};
use std::ops::Deref;

pub fn encode_png<P, Container>(img: &ImageBuffer<P, Container>) -> Result<Vec<u8>, ImageError>
where
    P: Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut buf = Vec::new();
    let encoder = PNGEncoder::new(&mut buf);
    encoder.encode(img, img.width(), img.height(), P::COLOR_TYPE)?;
    Ok(buf)
}
