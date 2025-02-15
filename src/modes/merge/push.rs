use futures_util::StreamExt;
use tokio_tungstenite::connect_async;

pub async fn merge_push(args: &[String], shift: usize) {
    if shift >= args.len() {
        println!("[Error] Enter address for merge push context");
        std::process::exit(-1);
    }

    let address = &args[shift];
    let (ws_stream, _) = connect_async(address).await.expect("Failed to connect");
    println!("[INFO]: Connection established!");

    let (_write, _read) = ws_stream.split();
}
