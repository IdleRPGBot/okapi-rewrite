use image::{ImageBuffer, Pixel};

use std::ops::Deref;

pub fn encode_webp<P, Container>(img: &ImageBuffer<P, Container>) -> Vec<u8>
where
    P: Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    let encoder = webp::Encoder::from_image(img).unwrap();
    encoder.encode(83.0).to_vec()
}
