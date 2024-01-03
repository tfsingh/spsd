use crate::lib::types::Machines;

use super::constants::{get_api_key, get_app_name, get_hostname};
use super::types::{Instance, Machine};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::error::Error;
use tokio;

pub fn get_instance(name: &String) -> Result<Instance, Box<dyn Error>> {
    unimplemented!()
}

pub fn create_instance(
    machine_id: &String,
    name: &String,
    size: &String,
    region: &String,
) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}

pub fn delete_instance(name: &str) -> Result<Option<Instance>, Box<dyn Error>> {
    unimplemented!()
}

#[tokio::main]
pub async fn get_instances() -> Result<Vec<Instance>, Box<dyn Error>> {
    let hostname = get_hostname()? + "/v1/apps/spec/machines";
    let api_key = get_api_key()?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let authorization_value = HeaderValue::from_str(&format!("Bearer {}", api_key))?;
    headers.insert(AUTHORIZATION, authorization_value);

    let client = reqwest::Client::new();

    let response = client.get(&hostname).headers(headers).send().await?;
    let success = response.status().is_success();
    let body = response.text().await?;

    if success {
        let machines: Machines = serde_json::from_str(&body)?;
        let instances = parse_response_body(machines)?;
        Ok(instances)
    } else {
        Err(body.into())
    }
}

fn parse_response_body(machines: Machines) -> Result<Vec<Instance>, Box<dyn Error>> {
    let mut instances = Vec::new();
    for machine in machines.iter() {
        instances.push(Instance {
            machine_id: machine.id.clone(),
            name: machine.name.clone(),
            size: match &machine.config.guest {
                Some(guest) => match guest.cpus {
                    1 => String::from("micro"),
                    2 => String::from("small"),
                    4 => String::from("med"),
                    8 => String::from("large"),
                    16 => String::from("xl"),
                    _ => String::from("unknown"),
                },
                _ => String::from("none"),
            },
            region: machine.region.clone(),
        })
    }
    Ok(instances)
}
