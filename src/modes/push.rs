use std::env;
use crate::manifest::get_manifest;

pub fn enter_push(args: &[String]) {
    let project_root = env::current_dir().unwrap();
    let webcrane_path = project_root.join("webcrane");
    let manifest_path = webcrane_path.join("Cargo.toml");

    let mut manifest = get_manifest(manifest_path);
}