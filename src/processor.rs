use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

pub struct ProcessOptions {
    pub grayscale: bool,
    pub invert: bool,
    pub brightness: i32,
    pub rotate: u32,
}

pub fn process_image(
    img: image::RgbImage,
    options: &ProcessOptions,
) -> image::RgbImage {
    let (width, height) = img.dimensions();

    
    let pixels: Vec<(u32, u32, [u8; 3])> = img
        .enumerate_pixels()
        .par_bridge()
        .map(|(x, y, pixel)| {
            let mut r = pixel[0] as i32;
            let mut g = pixel[1] as i32;
            let mut b = pixel[2] as i32;

            if options.grayscale {
                let gray =
                    (0.299 * r as f32
                        + 0.587 * g as f32
                        + 0.114 * b as f32) as i32;
                r = gray;
                g = gray;
                b = gray;
            }

            if options.invert {
                r = 255 - r;
                g = 255 - g;
                b = 255 - b;
            }

            r += options.brightness;
            g += options.brightness;
            b += options.brightness;

            (
                x,
                y,
                [
                    r.clamp(0, 255) as u8,
                    g.clamp(0, 255) as u8,
                    b.clamp(0, 255) as u8,
                ],
            )
        })
        .collect();

    let mut output = ImageBuffer::new(width, height);
    for (x, y, rgb) in pixels {
        output.put_pixel(x, y, Rgb(rgb));
    }

    // Rotation
    match options.rotate {
        90 => image::imageops::rotate90(&output),
        180 => image::imageops::rotate180(&output),
        270 => image::imageops::rotate270(&output),
        _ => output,
    }
}