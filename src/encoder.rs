use image::{
    codecs::png::{CompressionType, FilterType, PngEncoder},
    ImageBuffer, ImageEncoder, ImageError, Pixel, PixelWithColorType,
};

use std::ops::Deref;

pub fn encode_png<P, Container>(img: &ImageBuffer<P, Container>) -> Result<Vec<u8>, ImageError>
where
    P: Pixel<Subpixel = u8> + PixelWithColorType + 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut buf = Vec::new();
    let encoder = PngEncoder::new_with_quality(&mut buf, CompressionType::Fast, FilterType::Sub);
    encoder.write_image(img, img.width(), img.height(), P::COLOR_TYPE)?;
    Ok(buf)
}
