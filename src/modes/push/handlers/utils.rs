use std::{
    io::{self},
    path::Path,
};
use tokio::fs;

use crate::packages::RequestedFiles;

pub const CHUNK_SIZE: usize = 4 * 1024;

pub async fn count_chunks<P: AsRef<Path>>(path: P) -> io::Result<usize> {
    let metadata = fs::metadata(path).await?;
    let file_size = metadata.len() as usize;

    let chunks = if file_size == 0 {
        0
    } else {
        file_size.div_ceil(CHUNK_SIZE)
    };

    Ok(chunks)
}

pub async fn count_total_size(requested_files: &RequestedFiles) -> io::Result<u64> {
    let mut total_bytes: u64 = 0;

    for path in requested_files
        .new_files
        .iter()
        .chain(requested_files.files_to_update.iter())
    {
        let metadata = fs::metadata(path).await?;
        if metadata.is_file() {
            total_bytes += metadata.len();
        }
    }

    let total_gb = total_bytes;

    Ok(total_gb)
}
