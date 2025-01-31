use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Manifest {
    project: Project
}

#[derive(Deserialize)]
pub struct Project {
    name   : String,
    ignore : Vec<String>
}

pub fn get_manifest(path: PathBuf) -> Manifest {
    if !path.is_file() {
        println!("Path to manifest should be a file");
    }
    
    let mut file : File = File::open(path).expect("Unable to open manifest");
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    toml::from_str(text.as_str()).unwrap()
}

pub fn save_manifest(manifest: &Manifest, path: PathBuf) {
    
}