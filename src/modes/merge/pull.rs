use easy_upnp::{add_ports, delete_ports, Ipv4Cidr, PortMappingProtocol, UpnpConfig};
use tokio::net::{TcpListener, TcpStream};
use futures_util::{SinkExt, StreamExt};
use std::{net::SocketAddr, time::Duration};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};


fn get_port_config(duration: u32) -> UpnpConfig {
    UpnpConfig {
        address: Some(Ipv4Cidr::from_str("192.168.0").unwrap()),
        port: 5432,
        protocol: PortMappingProtocol::TCP,
        duration,
        comment: "Webcrane server".to_string(),
    }
}


pub async fn merge_pull(_args: &[String], _shift: usize) {
    // Open port
    println!("[INFO]: Starting predeploy procedures");
    println!("[INFO]: Opening port 5432...");
    let duration = 3600;
    for result in add_ports([get_port_config(duration)]) {
        if let Err(err) = result {
            println!("[ERROR]: {}", err);
            std::process::exit(-1);
        }
    }
    println!("[INFO]: Successful. Port is opened for {duration} seconds.");
    println!("[INFO]: Deploying bootstrap");

    let addr = "127.0.0.1:5432";
    
    let listener = TcpListener::bind(addr).await.expect("Can't listen");
    println!("[INFO]: Bootstrap listen on {}", addr);
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        println!("[INFO] Accepted new peer address: {}", peer);
        tokio::task::spawn(accept_connection(peer, stream));
    }

    println!("[INFO]: Closing port 5432...");
    for result in delete_ports([get_port_config(duration)]) {
        if let Err(err) = result {
            println!("[ERROR]: {}", err);
            std::process::exit(-1);
        }
    }
    println!("[INFO]: Successful. Port is closed");
}


async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = pull_procedure(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}

async fn pull_procedure(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let websocket_stream = accept_async(stream).await.expect("Failed to accept");
    println!("[INFO]: New websocket connection: {}", peer);

    let mut interval = tokio::time::interval(Duration::from_millis(1000));
    let (mut ws_sender, mut ws_receiver) = websocket_stream.split();
    
    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() ||msg.is_binary() {
                            ws_sender.send(msg).await?;
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
                ws_sender.send(Message::text("tick")).await?;
            }
        }
    }

    Ok(())
}
