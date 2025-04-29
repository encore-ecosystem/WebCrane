use std::{fs, io, path::Path};

use crate::{packages::RequestedFiles, shared::constants::FILE_TRANSFER_CHUNK_SIZE};

pub async fn count_chunks<P: AsRef<Path>>(path: P) -> io::Result<usize> {
    let metadata = fs::metadata(path)?;
    let file_size = metadata.len() as usize;

    let chunks = if file_size == 0 {
        0
    } else {
        file_size.div_ceil(FILE_TRANSFER_CHUNK_SIZE)
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
        let metadata = fs::metadata(path)?;
        if metadata.is_file() {
            total_bytes += metadata.len();
        }
    }

    let total_gb = total_bytes;

    Ok(total_gb)
}
