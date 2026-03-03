mod processor;
mod cli;
mod api;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--api".to_string()) {
        api::start_server().await;
    } else {
        cli::run_cli();
    }
}