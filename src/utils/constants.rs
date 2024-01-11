use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::env;
use std::error::Error;

use super::types::InstanceState;

pub const POSSIBLE_REGIONS: [&str; 17] = [
    "ams", "bom", "cdg", "dfw", "fra", "hkg", "iad", "lax", "lhr", "nrt", "ord", "scl", "sea",
    "sin", "sjc", "syd", "yyz",
];

pub fn validate_cpu(count: &str) -> Result<u32, String> {
    let value: u32 = count.parse().map_err(|_| "Invalid number of CPUs")?;

    if value >= 1 && value <= 16 {
        Ok(value)
    } else {
        Err("Number of CPUs must be between 1 and 16".to_string())
    }
}

pub fn validate_memory(amount: &str) -> Result<u32, String> {
    let value: u32 = amount.parse().map_err(|_| "Invalid amount of memory")?;

    let rounded_value = (value + 128) / 256 * 256;

    if rounded_value >= 256 && rounded_value <= 32768 {
        Ok(rounded_value)
    } else {
        Err("Memory must be between 256 and 32768".to_string())
    }
}

pub fn validate_volume(size: &str) -> Result<u32, String> {
    let value: u32 = size.parse().map_err(|_| "Invalid volume size")?;

    if value >= 1 && value <= 50 {
        Ok(value)
    } else {
        Err("Size of volume must be between 1 and 500 gb".to_string())
    }
}

pub fn validate_port(port: &str) -> Result<u16, String> {
    let value: u16 = port.parse().map_err(|_| "Invalid port")?;

    if value >= 1024 && value <= 65535 {
        Ok(value)
    } else {
        Err("Port must be between 1024 and 65535".to_string())
    }
}

fn get_api_key() -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    match env::var("FLY_API_KEY") {
        Ok(value) => Ok(value),
        Err(_) => Err("FLY_API_KEY not set".to_string().into()),
    }
}

fn get_app_name() -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    match env::var("FLY_APP_NAME") {
        Ok(value) => Ok(value),
        Err(_) => Err("FLY_APP_NAME not set".to_string().into()),
    }
}

pub fn get_headers() -> Result<HeaderMap, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let authorization_value = HeaderValue::from_str(&format!("Bearer {}", get_api_key()?))?;
    headers.insert(AUTHORIZATION, authorization_value);
    Ok(headers)
}

pub fn get_hostname() -> Result<String, Box<dyn Error>> {
    let app_name = get_app_name();
    match app_name {
        Ok(app_name) => Ok(format!("https://api.machines.dev/v1/apps/{}", app_name)),
        Err(error) => Err(error),
    }
}

pub fn parse_state(state: &str) -> InstanceState {
    match state {
        "starting" | "started" => InstanceState::Running,
        _ => InstanceState::Stopped,
    }
}
