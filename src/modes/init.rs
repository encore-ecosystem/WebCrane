use std::env;
use std::fs;
use std::io::Write;

pub fn enter_init(_args: &[String]) {
    let project_root = env::current_dir().unwrap();
    let webcrane_path = project_root.join("webcrane");
    let manifest_path = webcrane_path.join("manifest.toml");
    let webcraneignore_path = webcrane_path.join(".webcraneignore");

    // Check existence of the webcrane folder
    if !(webcrane_path.exists() & webcraneignore_path.exists()) {
        println!("Project already initialized!");
        return;
    }

    // Create folder and files
    if !webcrane_path.exists() {
        fs::create_dir(webcrane_path).unwrap();
    }
    if !manifest_path.exists() {
        let mut file = fs::File::create(manifest_path).unwrap();
        let text = format!("[project]\nname={:?}\nignore=[\n\t{},\n]", project_root.file_stem().unwrap(), ".webcraneignore");
        file.write_all(text.as_bytes()).unwrap();
    }
    if !webcraneignore_path.exists() {
        let mut file = fs::File::create(webcraneignore_path).unwrap();
        let text = "webcrane".to_string();
        file.write_all(text.as_bytes()).unwrap()
    }
}