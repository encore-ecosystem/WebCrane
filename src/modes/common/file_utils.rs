use blake3::Hasher;
use ignore::WalkBuilder;
use std::io::{Read, Seek, SeekFrom};
use std::{env, fs::File};

use crate::modes::common::error::Error;
use crate::packages::HashPackage;
use crate::shared::constants::FILE_READ_CHUNK_SIZE;

pub fn build_local_hash_package() -> Result<HashPackage, Error> {
    let project_root = env::current_dir()?;
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
                let rel_path = entry.path().strip_prefix(env::current_dir()?)?;

                let mut hasher = Hasher::new();
                let mut file_buffer = vec![0u8; FILE_READ_CHUNK_SIZE];

                let mut file = File::open(entry.path())?;
                let file_size = file.metadata()?.len();

                if file_size > 3 * FILE_READ_CHUNK_SIZE as u64 {
                    // Read beginning
                    file.read_exact(&mut file_buffer)?;
                    hasher.update(&file_buffer);
                    // Read middle
                    let middle = file_size / 2;
                    file.seek(SeekFrom::Start(middle - (FILE_READ_CHUNK_SIZE as u64 / 2)))?;
                    file.read_exact(&mut file_buffer)?;
                    hasher.update(&file_buffer);
                    // Read end
                    file.seek(SeekFrom::End(-(FILE_READ_CHUNK_SIZE as i64)))?;
                    file.read_exact(&mut file_buffer)?;
                    hasher.update(&file_buffer);
                } else {
                    file.read_to_end(&mut file_buffer)?;
                }

                hasher.update(file_size.to_string().as_bytes());
                hasher.update(rel_path.to_str().unwrap().as_bytes());
                package.add_record(
                    rel_path.to_path_buf(),
                    hasher.finalize().to_hex().to_string().as_bytes().to_vec(),
                );
            }
            Err(err) => {
                println!("[WARN]: {:?}", err);
            }
        }
    }

    Ok(package)
}
