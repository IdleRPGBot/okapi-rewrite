use std::{hint::unreachable_unchecked, num::NonZeroU32};

use fast_image_resize::{Image, MulDiv, PixelType, ResizeAlg, Resizer};
use image::{DynamicImage, ImageBuffer, Rgba};

pub use fast_image_resize::FilterType;

pub fn resize(
    image: DynamicImage,
    nwidth: u32,
    nheight: u32,
    filter: FilterType,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let src_height = unsafe { NonZeroU32::new_unchecked(image.height()) };
    let src_width = unsafe { NonZeroU32::new_unchecked(image.width()) };

    let mut src_image = match image {
        DynamicImage::ImageRgba8(img) => {
            Image::from_vec_u8(src_width, src_height, img.into_raw(), PixelType::U8x4).unwrap()
        }
        DynamicImage::ImageRgb8(_) => {
            let img = image.into_rgba8();
            Image::from_vec_u8(src_width, src_height, img.into_raw(), PixelType::U8x4).unwrap()
        }
        _ => unsafe { unreachable_unchecked() },
    };

    let dst_width = unsafe { NonZeroU32::new_unchecked(nwidth) };
    let dst_height = unsafe { NonZeroU32::new_unchecked(nheight) };
    let mut dst_image = Image::new(dst_width, dst_height, src_image.pixel_type());

    let mut dst_view = dst_image.view_mut();

    let mut resizer = Resizer::new(ResizeAlg::Convolution(filter));
    let mul_div = MulDiv::default();

    mul_div
        .multiply_alpha_inplace(&mut src_image.view_mut())
        .unwrap();
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();
    mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

    ImageBuffer::from_raw(nwidth, nheight, dst_image.into_buffer()).unwrap()
}
