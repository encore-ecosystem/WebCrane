use easy_upnp::{add_ports, delete_ports};
use file_management::send_requested_files;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};

mod file_management;
mod utils;

use crate::modes::shared::{build_local_hash_package, encode_addr};
use crate::packages::RequestedFiles;
use crate::{config::load_config, modes::push::utils::get_port_config};

pub async fn push(_args: &[String], _shift: usize) {
    // Load config
    let cfg = load_config();

    // Open port
    let port = cfg.server.port;
    println!("[INFO]: Starting predeploy procedures");
    println!("[INFO]: Opening port {}...", port);
    let duration = 3600;
    for result in add_ports([get_port_config(port, duration)]) {
        if let Err(err) = result {
            println!("[ERROR]: {}", err);
            std::process::exit(-1);
        }
    }
    println!("[INFO]: Success. Port is opened for {duration} seconds.");
    println!("[INFO]: Deploying bootstrap");

    // Start listening
    let addr = cfg.server.ip.to_string() + ":" + &cfg.server.port.to_string();
    let encoded_addr = encode_addr(&addr);

    let listener = TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Bootstrap is unable to listen to {}", addr));
    println!("[INFO]: Bootstrap listens to {}", addr);
    println!("[INFO]: Credentials: {}", encoded_addr);
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("Connected streams should have a peer address");
        println!("[INFO]: Accepted new peer: {}", peer);
        tokio::task::spawn(accept_connection(peer, stream));
    }

    // Close port
    println!("[INFO]: Closing port 5432...");
    for result in delete_ports([get_port_config(port, duration)]) {
        if let Err(err) = result {
            println!("[ERROR]: {}", err);
            std::process::exit(-1);
        }
    }
    println!("[INFO]: Success. Port is closed");
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = push_procedure(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}

async fn push_procedure(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    println!("[INFO]: New websocket connection: {}", peer);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // 1. Form hash package
    let hash_pkg = build_local_hash_package();

    // 2. Send hash package
    let serialized_hash_pkg =
        serde_json::to_string(&hash_pkg).expect("Failed to serialize package");
    if let Err(e) = ws_sender.send(Message::text(serialized_hash_pkg)).await {
        println!("[Error] Failed to send hash package: {}", e);
    } else {
        println!("[INFO]: Hash package was sent successfully!");
    }

    // 3. Receive diff
    let data = ws_receiver.next().await.unwrap().unwrap();
    let requested_files: RequestedFiles = serde_json::from_str(&data.into_text().unwrap()).unwrap();
    println!("[INFO]: Diff package received successfully!");

    // 4. Send requested files
    send_requested_files(&mut ws_sender, requested_files).await;

    // 5. Close connection
    ws_sender.close().await.unwrap();

    println!("[INFO]: Success!");

    Ok(())
}
