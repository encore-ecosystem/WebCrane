use crate::modes::shared::decode_addr;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod utils;

use crate::modes::pull::utils::{
    delete_files, group_files, merge_files, move_files, write_new_files,
};
use crate::modes::shared::build_local_hash_package;
use crate::packages::{FileTransfer, Files, GroupedFiles, HashPackage};

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
    let file_transfer = FileTransfer::new(
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

    // 5. Receive missing files
    let data = ws_receiver.next().await.unwrap().unwrap();
    let new_files: Files =
        serde_json::from_str(&data.into_text().unwrap()).expect("Failed to receive new files");

    let data = ws_receiver.next().await.unwrap().unwrap();
    let files_to_update: Files =
        serde_json::from_str(&data.into_text().unwrap()).expect("Failed to receive updated files");

    println!("[INFO]: New and updated files were received successfully!");

    // println!("{:?}\n{:?}", new_files, files_to_update);

    // 6. Write nonconflict data (new / move / delete)
    delete_files(grouped_files.files_to_delete);
    move_files(grouped_files.files_to_move);
    write_new_files(new_files);

    // 7. Resolve conflicts

    // 8. Write the rest of data
    merge_files(files_to_update);

    println!("[INFO]: Success!");
}
