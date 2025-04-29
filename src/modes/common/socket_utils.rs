use base64::engine::general_purpose;
use base64::Engine;

pub async fn encode_addr(ip: &str, port: &str) -> String {
    general_purpose::STANDARD.encode(ip.to_string() + ":" + port)
}

pub fn decode_addr(encoded: &str) -> String {
    String::from_utf8(
        general_purpose::STANDARD
            .decode(encoded)
            .expect("Could not decode ip"),
    )
    .expect("Could not decode ip")
}
