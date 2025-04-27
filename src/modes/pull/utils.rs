use std::collections::HashSet;
use std::{collections::HashMap, path::PathBuf};

use crate::packages::{GroupedFiles, HashPackage};

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
