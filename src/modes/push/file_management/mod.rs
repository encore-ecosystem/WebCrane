use futures_util::{stream::SplitSink, SinkExt};
use std::{fs::File, io::Read, path::PathBuf};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::packages::{
    EndOfFile, EndOfFileTransfer, FileGroup, RequestedFiles, StartOfFIleTransfer, StartOfFile,
};
use utils::{count_chunks, count_total_size, CHUNK_SIZE};

mod utils;

pub async fn send_requested_files(
    ws_sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    requested_files: RequestedFiles,
) {
    // send start of transfer signal
    let num_files = requested_files.files_to_update.len() + requested_files.new_files.len();
    let total_size = count_total_size(&requested_files).unwrap();
    let soft = StartOfFIleTransfer::new(num_files, total_size);
    let serialized_soft = serde_json::to_string(&soft).expect("Failed to serialize EOT package");
    ws_sender
        .send(Message::text(serialized_soft))
        .await
        .unwrap();

    // send files
    for file_path in requested_files.new_files {
        send_file(ws_sender, file_path, FileGroup::New).await;
    }

    for file_path in requested_files.files_to_update {
        send_file(ws_sender, file_path, FileGroup::New).await;
    }

    // send end of transfer signal
    let eoft = EndOfFileTransfer {};
    let serialized_eoft = serde_json::to_string(&eoft).expect("Failed to serialize EOT package");
    ws_sender
        .send(Message::text(serialized_eoft))
        .await
        .unwrap();

    println!("[INFO]: Requested files were sent successfully!");
}

pub async fn send_file(
    ws_sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    file_path: PathBuf,
    file_group: FileGroup,
) {
    // send start of file signal
    let num_chunks = count_chunks(&file_path).unwrap();
    let sot = StartOfFile::new(file_path.clone(), file_group, num_chunks);
    let serialized_sot = serde_json::to_string(&sot).expect("Failed to serialize EOT package");
    ws_sender.send(Message::text(serialized_sot)).await.unwrap();

    // send content by chunks
    let mut file = File::open(&file_path).unwrap();
    let mut buffer = vec![0u8; CHUNK_SIZE];
    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        ws_sender
            .send(Message::binary(buffer.to_vec()))
            .await
            .unwrap();
    }

    // send end of file signal
    let eof = EndOfFile {};
    let serialized_eof = serde_json::to_string(&eof).expect("Failed to serialize EOF package");
    ws_sender.send(Message::text(serialized_eof)).await.unwrap();
}
