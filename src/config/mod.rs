use error::{CreateConfigError, LoadConfigError};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::Ipv4Addr;
use std::{env, io};
use std::{fs, path::PathBuf};

mod error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub project: ProjectConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub ip: Option<Ipv4Addr>,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
}

pub fn create_config(path: PathBuf) -> Result<(), CreateConfigError> {
    let server_config = ServerConfig {
        ip: None,
        port: 5432,
    };

    print!("Enter project name: ");
    io::stdout().flush()?;
    let mut project_name = String::new();
    io::stdin().read_line(&mut project_name)?;
    let project_config = ProjectConfig {
        name: project_name.trim().to_string(),
    };

    let config = Config {
        server: server_config,
        project: project_config,
    };
    let toml_string = toml::to_string(&config)?;
    fs::write(path, toml_string)?;

    Ok(())
}

pub fn load_config() -> Result<Config, LoadConfigError> {
    let path = env::current_dir()?.join(".webcrane").join("config.toml");
    let content = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
