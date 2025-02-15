use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HashPackage {
    file_paths  : Vec<PathBuf>,
    file_hashes : Vec<Vec<u8>>,
}

impl HashPackage {
    pub fn new(file_paths: Vec<PathBuf>, file_hashes : Vec<Vec<u8>>) -> Self {
        Self { file_paths, file_hashes }
    }
    
    pub fn add_record(&mut self, path: PathBuf, hash: Vec<u8>) {
        self.file_paths.push(path);
        self.file_hashes.push(hash);
    }
}
