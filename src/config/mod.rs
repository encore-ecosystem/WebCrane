use std::io::Write;
use std::net::Ipv4Addr;
use std::{env, io};
use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub project: ProjectConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub ip: Ipv4Addr,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
}

pub fn create_config(path: PathBuf) {
    let server_config = ServerConfig {
        ip: Ipv4Addr::new(127, 0, 0, 1),
        port: 5432,
    };

    print!("Enter project name: ");
    io::stdout().flush().unwrap();
    let mut project_name = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Unable to read user input");
    let project_config = ProjectConfig {
        name: project_name.trim().to_string(),
    };

    let config = Config {
        server: server_config,
        project: project_config,
    };
    let toml_string = toml::to_string(&config).expect("Could not encode config");
    fs::write(path, toml_string).expect("Could not write config to file!");
}

pub fn load_config() -> Config {
    let path = env::current_dir()
        .unwrap()
        .join(".webcrane")
        .join("config.toml");
    let content = fs::read_to_string(&path).expect("Could not read config. WebCrane seems to be bot initialized!");
    let config: Config = toml::from_str(&content).expect("Could not parse config");
    config
}
