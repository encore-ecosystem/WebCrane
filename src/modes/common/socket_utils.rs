use base64::engine::general_purpose;
use base64::Engine;
use external_ip;

pub async fn encode_addr(ip: &str, port: &str) -> String {
    if ip == "127.0.0.1" {
        general_purpose::STANDARD.encode(ip.to_owned() + ":" + port)
    } else {
        let external_addr = external_ip::get_ipv4().await.unwrap();
        general_purpose::STANDARD.encode(external_addr.to_string() + ":" + port)
    }
}

pub fn decode_addr(encoded: &str) -> String {
    String::from_utf8(
        general_purpose::STANDARD
            .decode(encoded)
            .expect("Could not decode ip"),
    )
    .expect("Could not decode ip")
}
