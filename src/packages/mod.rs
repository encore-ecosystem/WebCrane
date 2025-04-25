use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct HashPackage {
    pub file_paths: Vec<PathBuf>,
    pub file_hashes: Vec<Vec<u8>>,
}

impl HashPackage {
    pub fn new(file_paths: Vec<PathBuf>, file_hashes: Vec<Vec<u8>>) -> Self {
        Self {
            file_paths,
            file_hashes,
        }
    }

    pub fn add_record(&mut self, path: PathBuf, hash: Vec<u8>) {
        self.file_paths.push(path);
        self.file_hashes.push(hash);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTransfer {
    pub new_files: HashSet<PathBuf>,
    pub files_to_update: HashSet<PathBuf>,
}

impl FileTransfer {
    pub fn new(new_files: HashSet<PathBuf>, files_to_update: HashSet<PathBuf>) -> Self {
        Self {
            new_files,
            files_to_update,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    pub path: Vec<PathBuf>,
    pub content: Vec<Vec<u8>>,
}

impl Files {
    pub fn new() -> Self {
        let path: Vec<PathBuf> = Vec::new();
        let content: Vec<Vec<u8>> = Vec::new();
        Self { path, content }
    }

    pub fn add_record(&mut self, path: PathBuf, content: Vec<u8>) {
        self.path.push(path);
        self.content.push(content);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupedFiles {
    pub new_files: HashSet<PathBuf>,
    pub files_to_update: HashSet<PathBuf>,
    pub files_to_move: HashSet<(PathBuf, PathBuf)>,
    pub files_to_delete: HashSet<PathBuf>,
}

impl GroupedFiles {
    pub fn new(
        new_files: HashSet<PathBuf>,
        files_to_update: HashSet<PathBuf>,
        files_to_move: HashSet<(PathBuf, PathBuf)>,
        files_to_delete: HashSet<PathBuf>,
    ) -> Self {
        Self {
            new_files,
            files_to_update,
            files_to_move,
            files_to_delete,
        }
    }
}
