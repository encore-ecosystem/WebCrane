use easy_upnp::{Ipv4Cidr, PortMappingProtocol, UpnpConfig};
use futures_util::{stream::SplitSink, SinkExt};
use std::fs;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::packages::{EndOfTransfer, FilePackage, FileType, RequestedFiles};

pub fn get_port_config(port: u16, duration: u32) -> UpnpConfig {
    UpnpConfig {
        address: Some(Ipv4Cidr::from_str("192.168.0").unwrap()),
        port,
        protocol: PortMappingProtocol::TCP,
        duration,
        comment: "Webcrane server".to_string(),
    }
}

pub async fn send_requested_files(
    ws_sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    requested_files: RequestedFiles,
) {
    for file_path in requested_files.new_files {
        let content =
            fs::read(&file_path).unwrap_or_else(|_| panic!("Could not read file {:?}", file_path));
        let file_pkg = FilePackage::new(file_path, FileType::New, content);
        send_file(ws_sender, file_pkg).await;
    }

    for file_path in requested_files.files_to_update {
        let content =
            fs::read(&file_path).unwrap_or_else(|_| panic!("Could not read file {:?}", file_path));
        let file_pkg = FilePackage::new(file_path, FileType::Update, content);
        send_file(ws_sender, file_pkg).await;
    }

    let eot = EndOfTransfer {};
    let serialized_end_of_transfer =
        serde_json::to_string(&eot).expect("Failed to serialize EOT package");
    ws_sender
        .send(Message::text(serialized_end_of_transfer))
        .await
        .unwrap();

    println!("[INFO]: Requested files were sent successfully!");
}

pub async fn send_file(
    ws_sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    file: FilePackage,
) {
    let serialized_file = serde_json::to_string(&file).expect("Failed to serialize new files");
    if let Err(e) = ws_sender.send(Message::text(serialized_file)).await {
        println!("[Error] Failed to send new files: {}", e);
    }
}
