use easy_upnp::{Ipv4Cidr, PortMappingProtocol, UpnpConfig};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use crate::packages::Files;

pub fn prepare_requested_files(diff_pkg: HashSet<PathBuf>) -> Files {
    let mut files = Files::new();

    for file_path in diff_pkg {
        let content =
            fs::read(&file_path).unwrap_or_else(|_| panic!("Could not read file {:?}", file_path));
        files.add_record(file_path, content);
    }

    files
}

pub fn get_port_config(port: u16, duration: u32) -> UpnpConfig {
    UpnpConfig {
        address: Some(Ipv4Cidr::from_str("192.168.0").unwrap()),
        port,
        protocol: PortMappingProtocol::TCP,
        duration,
        comment: "Webcrane server".to_string(),
    }
}
