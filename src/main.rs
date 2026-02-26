use clap::Parser;
use image::{ImageBuffer, Rgb};
use rayon::prelude::*;
use std::path::Path;

#[derive(Parser)]
struct Args {
    input: String,
    output: String,

    #[arg(long)]
    grayscale: bool,

    #[arg(long)]
    invert: bool,

    #[arg(long, default_value_t = 0)]
    brightness: i32,

    #[arg(long, default_value_t = 0)]
    rotate: u32,
}

fn main() {
    let args = Args::parse();

    if !Path::new(&args.input).exists() {
        eprintln!("Input file does not exist!");
        return;
    }

    println!("Loading image...");
    println!("Processing image...");

    let img = image::open(&args.input)
        .expect("Failed to open image")
        .to_rgb8();

    let (width, height) = img.dimensions();

    let pixels: Vec<(u32, u32, [u8; 3])> = img
        .enumerate_pixels()
        .par_bridge()
        .map(|(x, y, pixel)| {
            let mut r = pixel[0] as i32;
            let mut g = pixel[1] as i32;
            let mut b = pixel[2] as i32;

            if args.grayscale {
                let gray =
                    (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as i32;
                r = gray;
                g = gray;
                b = gray;
            }

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

    let mut final_image = match args.rotate {
        90 => image::imageops::rotate90(&output),
        180 => image::imageops::rotate180(&output),
        270 => image::imageops::rotate270(&output),
        _ => output,
    };

    final_image
        .enumerate_pixels_mut()
        .for_each(|(_, _, pixel)| {
            let mut r = pixel[0] as i32;
            let mut g = pixel[1] as i32;
            let mut b = pixel[2] as i32;

            if args.invert {
                r = 255 - r;
                g = 255 - g;
                b = 255 - b;
            }

            r += args.brightness;
            g += args.brightness;
            b += args.brightness;

            pixel[0] = r.clamp(0, 255) as u8;
            pixel[1] = g.clamp(0, 255) as u8;
            pixel[2] = b.clamp(0, 255) as u8;
        });

    println!("Saving image...");
    final_image
        .save(&args.output)
        .expect("Failed to save image");

    println!("Done! Image saved as {}", args.output);
}