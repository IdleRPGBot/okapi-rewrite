use image::{DynamicImage, ImageBuffer};
use libwebp_sys::{
    VP8StatusCode, WebPBitstreamFeatures, WebPDecodeRGB, WebPDecodeRGBA, WebPEncodeRGB,
    WebPEncodeRGBA, WebPGetFeatures, WebPGetInfo,
};

pub fn encode_webp_rgba(input_image: &[u8], width: u32, height: u32, quality: i32) -> Vec<u8> {
    unsafe {
        let mut out_buf = std::ptr::null_mut();
        let stride = width as i32 * 4;
        let len = WebPEncodeRGBA(
            input_image.as_ptr(),
            width as i32,
            height as i32,
            stride,
            quality as f32,
            &mut out_buf,
        );
        std::slice::from_raw_parts(out_buf, len as usize).into()
    }
}

pub fn decode_webp_rgba(buf: &[u8]) -> Vec<u8> {
    let mut width = 0;
    let mut height = 0;
    let len = buf.len();
    unsafe {
        WebPGetInfo(buf.as_ptr(), len, &mut width, &mut height);
        let out_buf = WebPDecodeRGBA(buf.as_ptr(), len, &mut width, &mut height);

        std::slice::from_raw_parts(out_buf, (width * height * 4) as usize).into()
    }
}

pub fn encode_webp_rgb(input_image: &[u8], width: u32, height: u32, quality: i32) -> Vec<u8> {
    unsafe {
        let mut out_buf = std::ptr::null_mut();
        let stride = width as i32 * 4;
        let len = WebPEncodeRGB(
            input_image.as_ptr(),
            width as i32,
            height as i32,
            stride,
            quality as f32,
            &mut out_buf,
        );
        std::slice::from_raw_parts(out_buf, len as usize).into()
    }
}

pub fn decode_webp_rgb(buf: &[u8]) -> Vec<u8> {
    let mut width = 0;
    let mut height = 0;
    let len = buf.len();
    unsafe {
        WebPGetInfo(buf.as_ptr(), len, &mut width, &mut height);
        let out_buf = WebPDecodeRGB(buf.as_ptr(), len, &mut width, &mut height);

        std::slice::from_raw_parts(out_buf, (width * height * 3) as usize).into()
    }
}

pub fn decode(buf: &[u8]) -> Option<DynamicImage> {
    let features = unsafe {
        let mut features: WebPBitstreamFeatures = std::mem::zeroed();

        let result = WebPGetFeatures(buf.as_ptr(), buf.len(), &mut features as *mut _);

        if result == VP8StatusCode::VP8_STATUS_OK {
            features
        } else {
            return None;
        }
    };

    if features.has_animation == 1 {
        return None;
    }

    let mut width = features.width;
    let mut height = features.height;

    let image_ptr = unsafe {
        if features.has_alpha == 1 {
            WebPDecodeRGBA(
                buf.as_ptr(),
                buf.len(),
                &mut width as *mut _,
                &mut height as *mut _,
            )
        } else {
            WebPDecodeRGB(
                buf.as_ptr(),
                buf.len(),
                &mut width as *mut _,
                &mut height as *mut _,
            )
        }
    };

    if image_ptr.is_null() {
        return None;
    }

    if features.has_alpha == 1 {
        let image =
            unsafe { std::slice::from_raw_parts(image_ptr, (width * height * 4) as usize).into() };
        let buffer = ImageBuffer::from_raw(width as u32, height as u32, image)?;

        Some(DynamicImage::ImageRgba8(buffer))
    } else {
        let image =
            unsafe { std::slice::from_raw_parts(image_ptr, (width * height * 3) as usize).into() };
        let buffer = ImageBuffer::from_raw(width as u32, height as u32, image)?;

        Some(DynamicImage::ImageRgb8(buffer))
    }
}
