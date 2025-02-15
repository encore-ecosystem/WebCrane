use crate::modes::enter_webcrane_context;
mod modes;
mod packages;


#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = 
    enter_webcrane_context(&args, 1).await;
}
