use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::{collections::HashMap, path::PathBuf};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::packages::{EndOfTransfer, FilePackage, FileType, GroupedFiles, HashPackage};

pub fn group_files(local_hash_pkg: HashPackage, remote_hash_pkg: HashPackage) -> GroupedFiles {
    let remote_path2hash: HashMap<&PathBuf, &Vec<u8>> = remote_hash_pkg
        .file_paths
        .iter()
        .zip(remote_hash_pkg.file_hashes.iter())
        .collect();

    let remote_hash2path: HashMap<&Vec<u8>, &PathBuf> = remote_hash_pkg
        .file_hashes
        .iter()
        .zip(remote_hash_pkg.file_paths.iter())
        .collect();

    let local_path2hash: HashMap<&PathBuf, &Vec<u8>> = local_hash_pkg
        .file_paths
        .iter()
        .zip(local_hash_pkg.file_hashes.iter())
        .collect();

    let local_hash2path: HashMap<&Vec<u8>, &PathBuf> = local_hash_pkg
        .file_hashes
        .iter()
        .zip(local_hash_pkg.file_paths.iter())
        .collect();

    let mut new_files: HashSet<PathBuf> = HashSet::new();
    let mut files_to_update: HashSet<PathBuf> = HashSet::new();
    let mut files_to_delete: HashSet<PathBuf> = HashSet::new();
    let mut files_to_move: HashSet<(PathBuf, PathBuf)> = HashSet::new();

    for (remote_path, remote_hash) in &remote_path2hash {
        if let Some(local_hash) = local_path2hash.get(remote_path) {
            if local_hash != remote_hash {
                files_to_update.insert((*remote_path).clone());
            }
        } else {
            new_files.insert((*remote_path).clone());
        }
    }

    for local_path in local_path2hash.keys() {
        if !remote_path2hash.contains_key(local_path) {
            files_to_delete.insert((*local_path).clone());
        }
    }

    for (local_hash, local_path) in &local_hash2path {
        if let Some(remote_path) = remote_hash2path.get(local_hash) {
            if remote_path != local_path {
                files_to_move.insert(((*local_path).clone(), (*remote_path).clone()));
            }
        }
    }

    GroupedFiles::new(new_files, files_to_update, files_to_move, files_to_delete)
}

pub async fn process_requested_files(
    ws_receiver: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) {
    loop {
        let msg = ws_receiver.next().await.unwrap().unwrap();

        if let Ok(file_pkg) = serde_json::from_str::<FilePackage>(&msg.clone().into_text().unwrap())
        {
            process_file_package(file_pkg);
            continue;
        }
        if serde_json::from_str::<EndOfTransfer>(&msg.clone().into_text().unwrap()).is_ok() {
            println!("Received EndPackage. Stopping.");
            break;
        }

        panic!("Received unknown or invalid message: {}", msg);
    }
}

pub fn process_file_package(file_pkg: FilePackage) {
    match file_pkg.file_type {
        FileType::New => write_new_file(file_pkg),
        FileType::Update => write_updated_file(file_pkg),
        _ => {}
    }
}

pub fn delete_files(files_to_delete: HashSet<PathBuf>) {
    for path in files_to_delete {
        if path.exists() {
            fs::remove_file(&path).unwrap_or_else(|_| panic!("Could not delete file {:?}", path));
        }
    }
}
pub fn move_files(files_to_move: HashSet<(PathBuf, PathBuf)>) {
    for (from, to) in files_to_move {
        if from.exists() {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)
                    .unwrap_or_else(|_| panic!("Could not create new directory for {:?}", to));
            }
            fs::rename(&from, &to)
                .unwrap_or_else(|_| panic!("Could move file from {:?} to {:?}", from, to));
        }
    }
}

pub fn write_new_file(new_file: FilePackage) {
    if let Some(parent) = new_file.path.parent() {
        fs::create_dir_all(parent)
            .unwrap_or_else(|_| panic!("Could not create new directory for {:?}", parent));
    }
    let mut file = fs::File::create(&new_file.path)
        .unwrap_or_else(|_| panic!("Could not create file {:?}", &new_file.path));
    file.write_all(&new_file.content)
        .unwrap_or_else(|_| panic!("Could not write file {:?}", &new_file.path));
}

pub fn write_updated_file(updated_file: FilePackage) {
    write_new_file(updated_file);
}
