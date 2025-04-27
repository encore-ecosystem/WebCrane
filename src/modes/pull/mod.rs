use crate::modes::shared::decode_addr;
use file_management::received::process_requested_files;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod file_management;
mod utils;

use crate::modes::pull::file_management::local::{delete_files, move_files};
use crate::modes::pull::utils::group_files;
use crate::modes::shared::build_local_hash_package;
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

    // 3. Compare local and remote files
    let grouped_files: GroupedFiles = group_files(local_hash_pkg, remote_hash_pkg);

    // println!("[DEBUG] Grouped files: {:?}", grouped_files);

    // 4. Request missing files
    let file_transfer = RequestedFiles::new(
        grouped_files.new_files.clone(),
        grouped_files.files_to_update.clone(),
    );
    let serialized_diff =
        serde_json::to_string(&file_transfer).expect("Failed to serialize package");
    if let Err(e) = ws_sender.send(Message::text(serialized_diff)).await {
        println!("[Error] Failed to send diff package: {}", e);
    } else {
        println!("[INFO]: Diff package sent successfully!");
    }

    // 5. Receive and process missing files
    process_requested_files(&mut ws_receiver).await;
    println!("[INFO]: New and updated files were received successfully!");

    // println!("{:?}\n{:?}", new_files, files_to_update);

    // 6. Process locally stored files
    delete_files(grouped_files.files_to_delete);
    move_files(grouped_files.files_to_move);

    println!("[INFO]: Success!");
}
