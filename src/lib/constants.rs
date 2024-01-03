use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::env;
use std::error::Error;

use super::types::InstanceSpecs;

pub const POSSIBLE_SIZES: [&str; 5] = ["micro", "small", "med", "large", "xl"];
pub const POSSIBLE_REGIONS: [&str; 17] = [
    "ams", "bom", "cdg", "dfw", "fra", "hkg", "iad", "lax", "lhr", "nrt", "ord", "scl", "sea",
    "sin", "sjc", "syd", "yyz",
];

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
        Ok(app_name) => Ok(format!(
            "https://api.machines.dev/v1/apps/{}/machines",
            app_name
        )),
        Err(error) => Err(error),
    }
}

pub fn get_size_from_cpu_count(cpus: u32) -> Result<String, String> {
    match cpus {
        1 => Ok(String::from("micro")),
        2 => Ok(String::from("small")),
        4 => Ok(String::from("med")),
        8 => Ok(String::from("large")),
        16 => Ok(String::from("xl")),
        _ => Err(String::from("unknown")),
    }
}

// haven't verified if these ratios make sense, will probably end up switching to cpus/memory
pub fn get_specs_from_size(size: &str) -> Result<InstanceSpecs, String> {
    match size {
        "micro" => Ok(InstanceSpecs {
            cpus: 1,
            memory: 512,
        }),
        "small" => Ok(InstanceSpecs {
            cpus: 2,
            memory: 512,
        }),
        "med" => Ok(InstanceSpecs {
            cpus: 4,
            memory: 1024,
        }),
        "large" => Ok(InstanceSpecs {
            cpus: 8,
            memory: 2048,
        }),
        "xl" => Ok(InstanceSpecs {
            cpus: 16,
            memory: 4096,
        }),
        _ => Err(String::from("unknown size")),
    }
}
