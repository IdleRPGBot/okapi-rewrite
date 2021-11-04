use image::{ImageBuffer, Pixel};

use std::{hint::unreachable_unchecked, ops::Deref};

use crate::webp::{encode_webp_rgb, encode_webp_rgba};

pub fn encode_webp<P, Container>(img: &ImageBuffer<P, Container>) -> Vec<u8>
where
    P: Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    match P::CHANNEL_COUNT {
        3 => encode_webp_rgb(img.as_ref(), img.width(), img.height(), 64),
        4 => encode_webp_rgba(img.as_ref(), img.width(), img.height(), 64),
        _ => unsafe { unreachable_unchecked() },
    }
}
