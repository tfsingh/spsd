use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::env;
use std::error::Error;

fn get_api_key() -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    match env::var("FLY_API_KEY") {
        Ok(value) => Ok(value),
        Err(_) => Err("FLY_API_KEY not set, please run spsd profile"
            .to_string()
            .into()),
    }
}

pub fn get_app_name() -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    match env::var("FLY_APP_NAME") {
        Ok(value) => Ok(value),
        Err(_) => Err("FLY_APP_NAME not set, please run spsd profile"
            .to_string()
            .into()),
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
