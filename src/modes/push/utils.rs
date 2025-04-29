use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    str::FromStr,
};

use easy_upnp::{Ipv4Cidr, PortMappingProtocol, UpnpConfig};

use crate::config::load_config;

pub fn get_port_config(port: u16, duration: u32) -> UpnpConfig {
    UpnpConfig {
        address: Some(Ipv4Cidr::from_str("192.168.0").unwrap()),
        port,
        protocol: PortMappingProtocol::TCP,
        duration,
        comment: "Webcrane server".to_string(),
    }
}

pub async fn get_ip_from_config() -> (Ipv4Addr, Option<Ipv4Addr>) {
    let cfg = load_config().unwrap();

    let local_ip = match cfg.server.ip {
        Some(ip) => ip,
        None => {
            let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
            socket.connect("255.255.255.0:0").unwrap();
            let local_addr: SocketAddr = socket.local_addr().unwrap();
            Ipv4Addr::from_str(&local_addr.ip().to_string()).unwrap()
        }
    };

    let external_ip: Option<Ipv4Addr> = if local_ip != Ipv4Addr::new(127, 0, 0, 1) {
        Some(external_ip::get_ipv4().await.unwrap())
    } else {
        None
    };

    (local_ip, external_ip)
}
