mod modes;
mod packages;
mod config;

use crate::modes::enter_webcrane_context;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    enter_webcrane_context(&args, 1).await;
}
