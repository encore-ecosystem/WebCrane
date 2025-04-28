use base64::engine::general_purpose;
use base64::Engine;

pub fn encode_addr(ip: &str) -> String {
    general_purpose::STANDARD.encode(ip)
}

pub fn decode_addr(encoded: &str) -> String {
    String::from_utf8(
        general_purpose::STANDARD
            .decode(encoded)
            .expect("Could not decode ip"),
    )
    .expect("Could not decode ip")
}
