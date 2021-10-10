use image::{
    png::{CompressionType, FilterType, PngEncoder},
    ImageBuffer, ImageError, Pixel,
};
use std::ops::Deref;

pub fn encode_png<P, Container>(img: &ImageBuffer<P, Container>) -> Result<Vec<u8>, ImageError>
where
    P: Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut buf = Vec::new();
    let encoder = PngEncoder::new_with_quality(&mut buf, CompressionType::Fast, FilterType::Sub);
    encoder.encode(img, img.width(), img.height(), P::COLOR_TYPE)?;
    Ok(buf)
}
