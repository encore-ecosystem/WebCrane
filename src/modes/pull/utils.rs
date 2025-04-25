use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::{collections::HashMap, path::PathBuf};

use crate::packages::{Files, GroupedFiles, HashPackage};

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
pub fn write_new_files(new_files: Files) {
    for (path, content) in new_files.path.iter().zip(new_files.content.iter()) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|_| panic!("Could not create new directory for {:?}", parent));
        }
        let mut file =
            File::create(path).unwrap_or_else(|_| panic!("Could not create file {:?}", path));
        file.write_all(content)
            .unwrap_or_else(|_| panic!("Could not write file {:?}", path));
    }
}
pub fn merge_files(files_to_update: Files) {
    write_new_files(files_to_update);
}
