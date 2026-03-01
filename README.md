#Rust Image Processor

A CLI-based image processing tool built in Rust.
Supports grayscale, invert, brightness adjustment, and rotation.

## Requirements

- Rust installed  
- Cargo (comes with Rust)

Check installation:

```bash
rustc --version
cargo --version
```

If not installed, download Rust from: https://rust-lang.org

##  Setup & Run

### 1️.Clone the repository

```bash
git clone https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git
cd YOUR_REPO_NAME
```

### 2. Run the project

```bash
cargo run -- <input_image> <output_image> [OPTIONS]
```

For optimized performance:

```bash
cargo run --release -- <input_image> <output_image> [OPTIONS]
```

---

##  Available Options

- `--grayscale` → Convert image to grayscale  
- `--invert` → Invert image colors  
- `--brightness <value>` → Adjust brightness (e.g. 50 or -70)  
- `--rotate <90|180|270>` → Rotate image  

---

##  Example

```bash
cargo run -- input.jpg output.jpg --grayscale --brightness 40 --rotate 90
```

The processed image will be saved with the output file name you provide.
