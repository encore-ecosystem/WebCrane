use std::env;
use futures_util::{Sink, StreamExt};
use tokio_tungstenite::connect_async;
use ignore::WalkBuilder;
use sha2::Digest;
use std::fs::File;
use std::io::Read;
use crate::packages::HashPackage;

pub async fn merge_push(args: &[String], shift: usize) {
    if shift >= args.len() {
        println!("[Error] Enter address for merge push context");
        std::process::exit(-1);
    }

    let address = &args[shift];
    let (ws_stream, _) = connect_async(address).await.expect("Failed to connect");
    println!("[INFO]: Connection established!");
    
    let (write, _read) = ws_stream.split();

    // 1. Form ignore environment
    let project_root = env::current_dir().unwrap();
    let webcrane_path = project_root.join(".webcrane");
    let webcraneignore_path = webcrane_path.join(".webcraneignore");

    let mut walk = WalkBuilder::new(project_root);
    walk.hidden(false);
    if let Some(err) = walk.add_ignore(webcraneignore_path) {
        println!("[WARN]: {:?}", err);
    };
    
    println!("[INFO]: Forming hash package");
    let mut package = HashPackage::new(vec![], vec![]);
    for result in walk.build() {
        match result {
            Ok(entry) => {
                if !entry.path().is_file() {
                    continue;
                }
                let rel_path = entry.path().strip_prefix(env::current_dir().unwrap()).unwrap();

                let mut hasher = sha2::Sha256::new();
                let mut file_buffer = Vec::new();
                File::open(entry.path()).unwrap().read_exact(&mut file_buffer).unwrap();

                hasher.update(rel_path.to_str().unwrap());
                hasher.update(&file_buffer);
                package.add_record(rel_path.to_path_buf(), hasher.finalize().to_vec());
            }
            Err(err) => {
                println!("[WARN]: {:?}", err);
            }
        }
    }
    
    // 2. Send hash package
}
