use dotenv::dotenv;
use std::env;
use std::error::Error;

pub const POSSIBLE_SIZES: [&str; 5] = ["micro", "small", "med", "large", "xl"];
pub const POSSIBLE_REGIONS: [&str; 17] = [
    "ams", "bom", "cdg", "dfw", "fra", "hkg", "iad", "lax", "lhr", "nrt", "ord", "scl", "sea",
    "sin", "sjc", "syd", "yyz",
];

pub fn get_hostname() -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    match env::var("FLY_API_HOSTNAME") {
        Ok(value) => Ok(value),
        Err(_) => Err("FLY_API_HOSTNAME not set".to_string().into()),
    }
}

pub fn get_api_key() -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    match env::var("FLY_API_KEY") {
        Ok(value) => Ok(value),
        Err(_) => Err("FLY_API_KEY not set".to_string().into()),
    }
}

pub fn get_app_name() -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    match env::var("FLY_APP_NAME") {
        Ok(value) => Ok(value),
        Err(_) => Err("FLY_APP_NAME not set".to_string().into()),
    }
}
