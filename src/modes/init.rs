use std::env;
use std::fs;
use std::io::Write;

use crate::config::create_config;

pub async fn enter_init(_args: &[String]) {
    let project_root = env::current_dir().unwrap();
    let webcrane_path = project_root.join(".webcrane");
    let webcraneignore_path = webcrane_path.join(".webcraneignore");
    let webcraneconfig_path = webcrane_path.join("config.toml");

    // Check if the webcrane folder exists
    if webcrane_path.exists() & webcraneignore_path.exists() & webcraneconfig_path.exists() {
        println!("Project is already initialized!");
        return;
    }

    // Create folder and files
    if !webcrane_path.exists() {
        fs::create_dir(webcrane_path).unwrap();
    }
    if !webcraneignore_path.exists() {
        let mut file = fs::File::create(webcraneignore_path).unwrap();
        let text = ".webcrane".to_string();
        file.write_all(text.as_bytes()).unwrap()
    }
    if !webcraneconfig_path.exists() {
        create_config(webcraneconfig_path);
    }
}
