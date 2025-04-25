use base64::engine::general_purpose;
use base64::Engine;
use ignore::WalkBuilder;
use sha2::Digest;
use std::{env, fs::File, io::Read};

use crate::packages::HashPackage;

pub fn build_local_hash_package() -> HashPackage {
    let project_root = env::current_dir().unwrap();
    let webcrane_path = project_root.join(".webcrane");
    let webcraneignore_path = webcrane_path.join(".webcraneignore");

    let mut walk = WalkBuilder::new(&project_root);
    walk.hidden(false);
    if let Some(err) = walk.add_ignore(webcraneignore_path) {
        println!("[WARN]: {:?}", err);
    };

    let mut package = HashPackage::new(vec![], vec![]);
    for result in walk.build() {
        match result {
            Ok(entry) => {
                if !entry.path().is_file() {
                    continue;
                }
                let rel_path = entry
                    .path()
                    .strip_prefix(env::current_dir().unwrap())
                    .unwrap();

                let mut hasher = sha2::Sha256::new();
                let mut file_buffer = [0; 1024];

                File::open(entry.path())
                    .unwrap()
                    .read_exact(&mut file_buffer)
                    .unwrap_or(());

                hasher.update(rel_path.to_str().unwrap());
                hasher.update(file_buffer);
                package.add_record(rel_path.to_path_buf(), hasher.finalize().to_vec());
            }
            Err(err) => {
                println!("[WARN]: {:?}", err);
            }
        }
    }

    package
}

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
