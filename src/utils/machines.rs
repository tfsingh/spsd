use crate::utils::types::{Machine, Machines};

use super::constants::{get_headers, get_hostname, parse_state};
use super::types::{Instance, InstanceSpecs};
use serde_json::Value;
use std::error::Error;
use tokio;

fn get_instance_id_from_name(name: &String) -> Result<String, Box<dyn Error>> {
    let id = get_instances()?
        .iter()
        .find(|instance| &instance.name == name)
        .map(|instance| instance.machine_id.clone());

    Ok(id.ok_or("Instance not found")?)
}

pub fn stop_machine(name: &String) -> Result<Instance, Box<dyn Error>> {
    let hostname = get_hostname()? + "/" + &get_instance_id_from_name(&name)? + "/stop";
    post_request(hostname, String::from(""), false)
}

pub fn start_machine(name: &String) -> Result<String, Box<dyn Error>> {
    let instance_id = get_instance_id_from_name(&name)?;
    let hostname = get_hostname()? + "/" + &instance_id + "/start";
    match post_request(hostname, String::from(""), false) {
        Ok(_) => Ok(instance_id),
        Err(error) => Err(error),
    }
}

pub fn create_machine(
    name: &String,
    cpus: u32,
    memory: u32,
    volume: u32,
    region: &String,
) -> Result<Instance, Box<dyn Error>> {
    let hostname = get_hostname()?;
    let body_to_send = create_body_from_specs(name, InstanceSpecs { cpus, memory }, region)?;
    let instance = post_request(hostname, body_to_send, true)?;
    stop_machine(name);
    Ok(instance)
}

pub fn update_machine(name: &String, cpus: u32, memory: u32) -> Result<Instance, Box<dyn Error>> {
    let hostname = get_hostname()? + "/" + &get_instance_id_from_name(&name)?;
    let body_to_send =
        create_body_from_specs(name, InstanceSpecs { cpus, memory }, &String::from(""))?;
    let instance = post_request(hostname, body_to_send, true)?;
    stop_machine(name);
    Ok(instance)
}

pub fn delete_machine(name: &String) -> Result<String, Box<dyn Error>> {
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
        let error: Value = serde_json::from_str(&body)?;
        Err(error.to_string().into())
    }
}

#[tokio::main]
async fn post_request(
    hostname: String,
    body_to_send: String,
    expect_instance: bool,
) -> Result<Instance, Box<dyn Error>> {
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
        if expect_instance {
            let machine: Machine = serde_json::from_str(&body)?;
            let mut instance = parse_response_body(vec![machine])?;
            Ok(instance.remove(0))
        } else {
            Ok(Instance::phony())
        }
    } else {
        let error: Value = serde_json::from_str(&body)?;
        Err(error.to_string().into())
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

// todo: add in params for state persistence, request for volume,
// and implement a better way to keep the machine alive on startup
fn create_body_from_specs(
    name: &String,
    specs: InstanceSpecs,
    region: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let body = serde_json::json!({
        "name": name,
        "region" : region,
        "config": {
            "init": {
                "exec": [
                    "/bin/sleep",
                    "inf"
                ]
            },
            "image": "registry-1.docker.io/library/ubuntu:latest",
            "guest": {
                "cpu_kind": "shared",
                "cpus": specs.cpus,
                "memory_mb": specs.memory
            },
            "mounts": [
                {
                    "encrypted": true,
                    "name": "test_volume",
                    "path": "/data",
                    "size_gb": 50,
                    "size_gb_limit": 500,
                    "volume": "vol_p4mmzwx7md9958d4"
                }
            ]
        }
    });

    Ok(serde_json::to_string(&body)?)
}
