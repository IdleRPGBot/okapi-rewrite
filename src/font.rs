use conv::ValueInto;
use image::Pixel;
use imageproc::definitions::Clamp;
use imageproc::drawing::Canvas;
use imageproc::pixelops::weighted_sum;
use rusttype::{point, Font, PositionedGlyph, Scale};

/// Draws colored text on an image in place. `scale` is augmented font scaling on both the x and y axis (in pixels). Note that this function *does not* support newlines, you must do this manually
pub fn draw_text_mut<'a, C>(
    canvas: &'a mut C,
    color: C::Pixel,
    x: u32,
    y: u32,
    scale: Scale,
    font: &'a Font<'a>,
    text: &'a str,
    max_width: i32,
) where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    let glyphs: Vec<PositionedGlyph<'_>> = font.layout(text, scale, offset).collect();

    // Calculate the X offset of the last character's right edge
    // TODO: Do not unwrap, get the last one that is Some(T)
    let last_glyph = glyphs.last().unwrap().pixel_bounding_box().unwrap().max.x;

    let glyphs = match last_glyph > max_width {
        true => {
            // It is too big, adjust our size
            let shrink_factor = last_glyph as f32 / max_width as f32;
            let new_scale = Scale {
                x: scale.x / shrink_factor,
                y: scale.y,
            };
            font.layout(text, new_scale, offset).collect()
        }
        false => glyphs,
    };

    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|gx, gy, gv| {
                let gx = gx as i32 + bb.min.x;
                let gy = gy as i32 + bb.min.y;

                let image_x = gx + x as i32;
                let image_y = gy + y as i32;

                let image_width = canvas.width() as i32;
                let image_height = canvas.height() as i32;

                if image_x >= 0 && image_x < image_width && image_y >= 0 && image_y < image_height {
                    let pixel = canvas.get_pixel(image_x as u32, image_y as u32);
                    let weighted_color = weighted_sum(pixel, color, 1.0 - gv, gv);
                    canvas.draw_pixel(image_x as u32, image_y as u32, weighted_color);
                }
            })
        }
    }
}
