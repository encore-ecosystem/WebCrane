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
pub struct RequestedFiles {
    pub new_files: HashSet<PathBuf>,
    pub files_to_update: HashSet<PathBuf>,
}

impl RequestedFiles {
    pub fn new(new_files: HashSet<PathBuf>, files_to_update: HashSet<PathBuf>) -> Self {
        Self {
            new_files,
            files_to_update,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    New,
    Update,
    Delete,
    Move,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePackage {
    pub path: PathBuf,
    pub file_type: FileType,
    pub content: Vec<u8>,
}

impl FilePackage {
    pub fn new(path: PathBuf, file_type: FileType, content: Vec<u8>) -> Self {
        Self {
            path,
            file_type,
            content,
        }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfTransfer {}
