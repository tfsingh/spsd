use dirs;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use super::types::Config;

fn get_config_path() -> io::Result<PathBuf> {
    match dirs::config_dir() {
        Some(path) => Ok(path.join("spsd_config.json")),
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Config directory not found",
        )),
    }
}

fn read_config() -> io::Result<Config> {
    let path = get_config_path()?;
    if path.exists() {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        Ok(Config::new())
    }
}

pub fn write_config(config: &Config) -> io::Result<()> {
    let path = get_config_path()?;
    let mut file = File::create(path)?;
    let contents = serde_json::to_string(config)?;
    file.write_all(contents.as_bytes())
}

pub fn get_api_key() -> Result<String, Box<dyn Error>> {
    let config = read_config()?;
    config
        .fly_api_key
        .ok_or_else(|| "FLY_API_KEY not set in config file".into())
}

pub fn get_app_name() -> Result<String, Box<dyn Error>> {
    let config = read_config()?;
    config
        .fly_app_name
        .ok_or_else(|| "FLY_APP_NAME not set in config file".into())
}
