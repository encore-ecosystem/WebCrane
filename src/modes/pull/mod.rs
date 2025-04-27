use file_handlers::received::process_requested_files;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use utils::request_missing_files;

mod file_handlers;
mod utils;

use crate::modes::pull::file_handlers::local::{delete_files, move_files};
use crate::modes::pull::utils::group_files;
use crate::modes::shared::build_local_hash_package;
use crate::modes::shared::decode_addr;
use crate::packages::{GroupedFiles, HashPackage, RequestedFiles};

pub async fn pull(args: &[String], shift: usize) {
    if shift >= args.len() {
        println!("[Error] Enter pusher's address");
        std::process::exit(-1);
    }

    // 1. Decode address
    let address = "ws://".to_owned() + &decode_addr(&args[shift]);

    let (ws_stream, _) = connect_async(address).await.expect("Failed to connect");
    println!("[INFO]: Connection established!");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // 2. Receive hashes
    let data = ws_receiver.next().await.unwrap().unwrap();
    let remote_hash_pkg: HashPackage = serde_json::from_str(&data.into_text().unwrap()).unwrap();

    // 3. Get hashed of local files
    let local_hash_pkg = build_local_hash_package();

    // 3. Compare local and remote files, group files
    let grouped_files: GroupedFiles = group_files(local_hash_pkg, remote_hash_pkg);

    // 4. Request missing files
    let missing_files = RequestedFiles::new(
        grouped_files.new_files.clone(),
        grouped_files.files_to_update.clone(),
    );
    request_missing_files(&mut ws_sender, missing_files).await;
    println!("[INFO]: Missing file were requested successfully!");

    // 5. Receive and process missing files
    process_requested_files(&mut ws_receiver).await;
    println!("[INFO]: Missing were received successfully!");

    // 6. Process locally stored files
    delete_files(grouped_files.files_to_delete);
    move_files(grouped_files.files_to_move);

    println!("[INFO]: Done!");
}
