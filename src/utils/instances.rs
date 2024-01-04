use crate::utils::types::{Machine, Machines};

use super::constants::{get_headers, get_hostname, parse_state};
use super::types::{Instance, InstanceSpecs};
use std::error::Error;
use tokio;

fn get_instance_id_from_name(name: &String) -> Result<String, Box<dyn Error>> {
    let id = get_instances()?
        .iter()
        .find(|instance| &instance.name == name)
        .map(|instance| instance.machine_id.clone());

    Ok(id.ok_or("Instance not found")?)
}

pub fn create_instance(
    name: &String,
    cpus: u32,
    memory: u32,
    region: &String,
) -> Result<Instance, Box<dyn Error>> {
    let hostname = get_hostname()?;
    let body_to_send = create_body_from_specs(name, InstanceSpecs { cpus, memory }, region)?;
    cru_request(hostname, body_to_send)
}

pub fn update_instance(name: &String, cpus: u32, memory: u32) -> Result<Instance, Box<dyn Error>> {
    let hostname = get_hostname()? + "/" + &get_instance_id_from_name(&name)?;
    let body_to_send =
        create_body_from_specs(name, InstanceSpecs { cpus, memory }, &String::from(""))?;
    cru_request(hostname, body_to_send)
}

pub fn delete_instance(name: &String) -> Result<String, Box<dyn Error>> {
    let hostname = get_hostname()? + "/" + &get_instance_id_from_name(&name)?;
    request_deletion(name, &hostname)
}

#[tokio::main]
pub async fn request_deletion(name: &String, hostname: &String) -> Result<String, Box<dyn Error>> {
    let headers = get_headers()?;
    let client = reqwest::Client::new();

    let response = client.delete(hostname).headers(headers).send().await?;
    let success = response.status().is_success();
    let body = response.text().await?;

    if success {
        Ok(String::from(format!(
            "Instance {} deleted successfully",
            name
        )))
    } else {
        Err(body.into())
    }
}

#[tokio::main]
pub async fn get_instances() -> Result<Vec<Instance>, Box<dyn Error>> {
    let headers = get_headers()?;
    let client = reqwest::Client::new();
    let hostname = get_hostname()?;

    let response = client.get(hostname).headers(headers).send().await?;
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

#[tokio::main]
async fn cru_request(hostname: String, body_to_send: String) -> Result<Instance, Box<dyn Error>> {
    let headers = get_headers()?;
    let client = reqwest::Client::new();

    let response = client
        .post(hostname)
        .headers(headers)
        .body(body_to_send)
        .send()
        .await?;

    let success = response.status().is_success();
    let body = response.text().await?;

    if success {
        let machine: Machine = serde_json::from_str(&body)?;
        let mut instance = parse_response_body(vec![machine])?;
        Ok(instance.remove(0))
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
            specs: match &machine.config.guest {
                Some(guest) => InstanceSpecs {
                    cpus: guest.cpus,
                    memory: guest.memory_mb,
                },
                None => InstanceSpecs::phony(),
            },
            region: machine.region.clone(),
            state: parse_state(&machine.state),
        })
    }
    Ok(instances)
}

fn create_body_from_specs(
    name: &String,
    specs: InstanceSpecs,
    region: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let body = serde_json::json!({
        "name": name,
        "region" : region,
        "config": {
            "image": "registry-1.docker.io/library/ubuntu:latest",
            "guest": {
                "cpu_kind": "shared",
                "cpus": specs.cpus,
                "memory_mb": specs.memory
            }
        }
    });

    Ok(serde_json::to_string(&body)?)
}
