use clap::Parser;
use std::path::Path;
use image::open;

use crate::processor::{process_image, ProcessOptions};

#[derive(Parser)]
pub struct Args {
    pub input: String,
    pub output: String,

    #[arg(long)]
    pub grayscale: bool,

    #[arg(long)]
    pub invert: bool,

    #[arg(long, default_value_t = 0)]
    pub brightness: i32,

    #[arg(long, default_value_t = 0)]
    pub rotate: u32,
}

pub fn run_cli() {
    let args = Args::parse();

    if !Path::new(&args.input).exists() {
        eprintln!("Input file does not exist!");
        return;
    }

    println!("Loading image...");

    let img = open(&args.input)
        .expect("Failed to open image")
        .to_rgb8();

    let options = ProcessOptions {
        grayscale: args.grayscale,
        invert: args.invert,
        brightness: args.brightness,
        rotate: args.rotate,
    };

    println!("Processing image...");
    let result = process_image(img, &options);

    println!("Saving image...");
    result
        .save(&args.output)
        .expect("Failed to save image");

    println!("Done! Image saved as {}", args.output);
}