use super::constants::{get_headers, get_hostname};
use super::json_handling;
use super::types::{Instance, InstanceSpecs, Volume};
use crate::utils::types::{Machine, Machines};
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::error::Error;
use tokio;

pub fn stop_machine(name: &str) -> Result<String, Box<dyn Error>> {
    let instance_id = get_instance_from_name(&name)?.machine_id;
    let hostname = get_hostname()? + "/machines/" + &instance_id + "/stop";
    match make_request::<Value>(Method::POST, hostname, None) {
        Ok(_) => Ok(instance_id),
        Err(error) => Err(error),
    }
}

pub fn start_machine(name: &str) -> Result<String, Box<dyn Error>> {
    let instance_id = get_instance_from_name(&name)?.machine_id;
    let hostname = get_hostname()? + "/machines/" + &instance_id + "/start";
    match make_request::<Value>(Method::POST, hostname, None) {
        Ok(_) => {
            poll_machine(&instance_id)?;
            Ok(instance_id)
        }
        Err(error) => Err(error),
    }
}

pub fn create_volume(name: &str, volume_gb: u32, region: &str) -> Result<String, Box<dyn Error>> {
    let hostname = get_hostname()? + "/volumes";
    let body = serde_json::json!({"name" : name,
                                      "region": region, 
                                      "size_gb": volume_gb});
    let volume = make_request::<Volume>(Method::POST, hostname, Some(body.to_string()))?;
    if let Some(volume) = volume {
        Ok(volume.id)
    } else {
        Err("Error in creation of volume".into())
    }
}

pub fn delete_volume(volume_id: &str) -> Result<String, Box<dyn Error>> {
    let hostname = get_hostname()? + "/volumes/" + volume_id;
    match make_request::<Value>(Method::DELETE, hostname, None) {
        Ok(_) => Ok(String::from("Volume deleted")),
        Err(error) => Err(error),
    }
}

// can make optimizations here to reduce number of requests
pub fn create_machine(
    name: &str,
    image: &str,
    cpu_count: u32,
    memory_mb: u32,
    volume_gb: u32,
    region: &str,
    port: Option<u16>,
) -> Result<Instance, Box<dyn Error>> {
    if port.is_some() {
        ensure_port_is_unique(port)?;
    }
    let hostname = get_hostname()? + "/machines";
    let volume_id = create_volume(name, volume_gb, region)?;
    let body = json_handling::create_body_from_specs(
        name,
        image,
        InstanceSpecs {
            cpu_count,
            memory_mb,
            volume_gb,
        },
        region,
        &volume_id,
        port,
    )?;
    let machine = make_request::<Machine>(Method::POST, hostname, Some(body))?;
    if let Some(instance) = machine {
        let instance = json_handling::parse_response_body(vec![instance])?.remove(0);
        poll_machine(&instance.machine_id)?;
        stop_machine(&instance.name)?;
        Ok(instance)
    } else {
        delete_volume(&volume_id)?;
        Err("Error in instance creation".into())
    }
}

pub fn delete_machine(name: &str) -> Result<String, Box<dyn Error>> {
    let instance = get_instance_from_name(&name)?;
    let hostname = get_hostname()? + "/machines/" + &instance.machine_id;
    let result = make_request::<Value>(Method::DELETE, hostname, None)?;
    if let Some(_) = result {
        delete_volume(&instance.volume_id)?;
        Ok(String::from("Deleted"))
    } else {
        Err("Did not delete".into())
    }
}

pub fn get_instances() -> Result<Vec<Instance>, Box<dyn Error>> {
    let hostname = get_hostname()? + "/machines";
    let machines = make_request::<Machines>(Method::GET, hostname, None)?;
    let instances = json_handling::parse_response_body(machines.unwrap());
    instances
}

#[tokio::main]
async fn make_request<T: DeserializeOwned>(
    method: Method,
    hostname: String,
    body: Option<String>,
) -> Result<Option<T>, Box<dyn Error>> {
    let headers = get_headers()?;
    let client = Client::new();
    let mut request = client.request(method, &hostname).headers(headers);

    if let Some(b) = body {
        request = request.body(b);
    }

    let response = request.send().await?;
    let success = response.status().is_success();
    let response_body = response.text().await?;
    //println!("{:?}", response_body);
    if success {
        let parsed_response: T = serde_json::from_str(&response_body)?;
        Ok(Some(parsed_response))
    } else {
        Err(response_body.into())
    }
}

fn poll_machine(machine_id: &str) -> Result<String, Box<dyn Error>> {
    let hostname = get_hostname()? + "/machines/" + machine_id + "/wait";
    let body = serde_json::json!({"state" : "started"}).to_string();
    if let Ok(_) = make_request::<Value>(Method::GET, hostname, Some(body)) {
        Ok(String::from("Instance started"))
    } else {
        Err("Instance was not started".into())
    }
}

fn get_instance_from_name(name: &str) -> Result<Instance, Box<dyn Error>> {
    let instances = get_instances()?;
    let instance = instances
        .iter()
        .find(|instance| instance.name == name)
        .cloned();

    instance.ok_or_else(|| "Instance not found".into())
}

fn ensure_port_is_unique(port: Option<u16>) -> Result<(), Box<dyn Error>> {
    let instances = get_instances()?;
    if instances.iter().any(|instance| instance.port == port) {
        return Err("Instance port is not unique".into());
    }
    Ok(())
}
