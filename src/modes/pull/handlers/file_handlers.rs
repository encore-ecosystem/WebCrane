use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::packages::{EndOfFile, EndOfFileTransfer, FileGroup, StartOfFIleTransfer, StartOfFile};

pub async fn process_requested_files(
    ws_receiver: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) {
    // receive start of file transfer signal
    let msg = ws_receiver.next().await.unwrap().unwrap();
    let soft = serde_json::from_str::<StartOfFIleTransfer>(
        &msg.into_text()
            .expect("Received wrong package at the start of new files transfer"),
    )
    .unwrap();

    // receive file content
    let pb = ProgressBar::new(soft.total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
    for _ in 0..soft.num_files {
        receive_file_content(ws_receiver, &pb).await;
    }

    // receive end of file transfer signal
    let msg = ws_receiver.next().await.unwrap().unwrap();
    assert!(serde_json::from_str::<EndOfFileTransfer>(&msg.into_text().unwrap()).is_ok());
}

pub async fn receive_file_content(
    ws_receiver: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pb: &ProgressBar,
) {
    let msg = ws_receiver.next().await.unwrap().unwrap();
    let file_metadata = serde_json::from_str::<StartOfFile>(&msg.clone().into_text().unwrap())
        .expect("Received wrong package at the start of file transfer");

    match file_metadata.file_group {
        FileGroup::New => receive_new_file_content(ws_receiver, file_metadata, pb).await,
        FileGroup::Update => receive_updated_file_content(ws_receiver, file_metadata, pb).await,
        _ => {}
    }
}

pub async fn receive_new_file_content(
    ws_receiver: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    file_metadata: StartOfFile,
    pb: &ProgressBar,
) {
    if let Some(parent) = file_metadata.path.parent() {
        fs::create_dir_all(parent)
            .unwrap_or_else(|_| panic!("Could not create new directory for {:?}", parent));
    }
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&file_metadata.path)
        .unwrap_or_else(|_| panic!("Could not open file {:?}", &file_metadata.path));

    for _ in 0..file_metadata.num_chunks {
        let msg = ws_receiver.next().await.unwrap().unwrap();
        let file_chunk = &msg.into_data();
        pb.inc(file_chunk.len() as u64);
        file.write_all(file_chunk).unwrap();
    }

    let msg = ws_receiver.next().await.unwrap().unwrap();
    assert!(serde_json::from_str::<EndOfFile>(&msg.into_text().unwrap()).is_ok());
}

pub async fn receive_updated_file_content(
    ws_receiver: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    file_metadata: StartOfFile,
    pb: &ProgressBar,
) {
    receive_new_file_content(ws_receiver, file_metadata, pb).await;
}

pub fn delete_files(files_to_delete: HashSet<PathBuf>) {
    for path in files_to_delete {
        if path.exists() {
            fs::remove_file(&path).unwrap_or_else(|_| panic!("Could not delete file {:?}", path));
            clean_up_empty_parent_dirs(&path);
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
            clean_up_empty_parent_dirs(&from);
        }
    }
}

fn clean_up_empty_parent_dirs(path: &Path) {
    let mut curr_dir = path.parent();
    while let Some(dir) = curr_dir {
        match fs::remove_dir(dir) {
            Ok(_) => {
                curr_dir = dir.parent();
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::DirectoryNotEmpty
                    || e.kind() == std::io::ErrorKind::NotFound
                {
                    break;
                } else {
                    panic!("Could not remove directory {:?}: {:?}", dir, e);
                }
            }
        }
    }
}
