use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

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
