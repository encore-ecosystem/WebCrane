use easy_upnp::{Ipv4Cidr, PortMappingProtocol, UpnpConfig};

pub fn get_port_config(port: u16, duration: u32) -> UpnpConfig {
    UpnpConfig {
        address: Some(Ipv4Cidr::from_str("192.168.0").unwrap()),
        port,
        protocol: PortMappingProtocol::TCP,
        duration,
        comment: "Webcrane server".to_string(),
    }
}
