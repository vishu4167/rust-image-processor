#Rust Image Processor

A high-performance image processing tool built in Rust with both CLI and REST API support.
Supports grayscale, invert, brightness adjustment, and rotation with parallel processing.

#Features

CLI-based image processing

REST API for real-time image transformation

Parallel image processing using Rayon

Async request handling using Tokio + Axum

Supports multiple image operations

#Requirements

Rust installed

Cargo (comes with Rust)

Check installation:

rustc --version
cargo --version

If not installed: https://rust-lang.org

#Setup
Clone the repository
git clone https://github.com/vishu4167/rust-image-processor.git
cd rust-image-processor

CLI Usage

Run the project:

cargo run -- <input_image> <output_image> [OPTIONS]

For optimized performance:

cargo run --release -- <input_image> <output_image> [OPTIONS]
Example
cargo run -- input.jpg output.jpg --grayscale --brightness 40 --rotate 90

#API Usage
Start the server
cargo run -- --api

Server runs at:

http://127.0.0.1:3000
Send request (Linux / macOS)
curl -X POST "http://127.0.0.1:3000/process?grayscale=true&rotate=90" \
-F "file=@input.jpg" \
-o result.jpg
Send request (Windows PowerShell)
curl.exe -X POST "http://127.0.0.1:3000/process?grayscale=true&rotate=90" -F "file=@input.jpg" -o result.jpg

#Options
CLI Options

--grayscale → Convert image to grayscale

--invert → Invert image colors

--brightness <value> → Adjust brightness

--rotate <90|180|270> → Rotate image

API Query Parameters

grayscale=true

invert=true

brightness=<value>

rotate=<90|180|270>



