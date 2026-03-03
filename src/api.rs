use axum::{
    extract::{Multipart, Query},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use image::load_from_memory;
use std::net::SocketAddr;

use crate::processor::{process_image, ProcessOptions};

#[derive(Deserialize)]
struct ImageParams {
    grayscale: Option<bool>,
    invert: Option<bool>,
    brightness: Option<i32>,
    rotate: Option<u32>,
}

pub async fn start_server() {
    let app = Router::new()
        .route("/", get(root))
        .route("/process", post(process));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("API running at http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn root() -> &'static str {
    "Server running successfully "
}

async fn process(
    Query(params): Query<ImageParams>,
    mut multipart: Multipart,
) -> Vec<u8> {
    let mut image_bytes = vec![];

    while let Some(field) = multipart.next_field().await.unwrap() {
        image_bytes = field.bytes().await.unwrap().to_vec();
    }

    let img = load_from_memory(&image_bytes)
        .unwrap()
        .to_rgb8();

    let options = ProcessOptions {
        grayscale: params.grayscale.unwrap_or(false),
        invert: params.invert.unwrap_or(false),
        brightness: params.brightness.unwrap_or(0),
        rotate: params.rotate.unwrap_or(0),
    };

    let result = process_image(img, &options);

    let mut buffer = Vec::new();
    result
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageOutputFormat::Jpeg(80),
        )
        .unwrap();

    buffer
}