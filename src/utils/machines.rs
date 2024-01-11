use super::constants::{get_headers, get_hostname, parse_state};
use super::types::{Instance, InstanceSpecs, Volume};
use crate::utils::types::{Machine, Machines};
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::error::Error;
use tokio;


pub fn stop_machine(name: &String) -> Result<String, Box<dyn Error>> {
    let instance_id = get_instance_from_name(&name)?.machine_id;
    let hostname = get_hostname()? + "/machines/" + &instance_id + "/stop";
    match make_request::<Value>(Method::POST, hostname, None) {
        Ok(_) => Ok(instance_id),
        Err(error) => Err(error),
    }
}

pub fn start_machine(name: &String) -> Result<String, Box<dyn Error>> {
    let instance_id = get_instance_from_name(&name)?.machine_id;
    let hostname = get_hostname()? + "/machines/" + &instance_id + "/start";
    match make_request::<Value>(Method::POST, hostname, None) {
        Ok(_) => {
            poll(&instance_id)?;
            Ok(instance_id)
        }
        Err(error) => Err(error),
    }
}

pub fn create_volume(
    name: &String,
    volume_gb: u32,
    region: &String,
) -> Result<String, Box<dyn Error>> {
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

pub fn delete_volume(volume_id: &String) -> Result<String, Box<dyn Error>> {
    let hostname = get_hostname()? + "/volumes/" + volume_id;
    match make_request::<Value>(Method::DELETE, hostname, None) {
        Ok(_) => Ok(String::from("Volume deleted")),
        Err(error) => Err(error),
    }
}

pub fn create_machine(
    name: &String,
    cpu_count: u32,
    memory_mb: u32,
    volume_gb: u32,
    region: &String,
) -> Result<Instance, Box<dyn Error>> {
    let hostname = get_hostname()? + "/machines";
    let volume_id = create_volume(name, volume_gb, region)?;
    let body = create_body_from_specs(
        name,
        InstanceSpecs {
            cpu_count,
            memory_mb,
            volume_gb,
        },
        Some(region),
        Some(&volume_id),
    )?;
    let machine = make_request::<Machine>(Method::POST, hostname, Some(body))?;

    if let Some(instance) = machine {
        let instance = parse_response_body(vec![instance])?.remove(0);
        Ok(instance)
    } else {
        delete_volume(&volume_id)?;
        Err("Error in instance creation".into())
    }
}

// todo: fly api is not returning mounts on update, this will work when we make it
pub fn update_machine(
    name: &String,
    cpu_count: u32,
    memory_mb: u32,
) -> Result<Instance, Box<dyn Error>> {
    let hostname = get_hostname()? + "/machines/" + &get_instance_from_name(&name)?.machine_id;
    let body = create_body_from_specs(
        name,
        InstanceSpecs {
            cpu_count,
            memory_mb,
            volume_gb: 0,
        },
        None,
        None,
    )?;
    let machine = make_request::<Machine>(Method::POST, hostname, Some(body))?;
    let machine = machine.unwrap();
    let instance = parse_response_body(vec![machine])?.remove(0);
    poll(&instance.machine_id)?;
    stop_machine(name)?;
    Ok(instance)
}

pub fn delete_machine(name: &String) -> Result<String, Box<dyn Error>> {
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
    let instances = parse_response_body(machines.unwrap());
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

    if success {
        let parsed_response: T = serde_json::from_str(&response_body)?;
        Ok(Some(parsed_response))
    } else {
        Err(response_body.into())
    }
}

fn poll(instance_id: &String) -> Result<String, Box<dyn Error>> {
    let hostname = get_hostname()? + "/machines/" + instance_id + "/wait";
    let body = serde_json::json!({"state" : "started"}).to_string();
    if let Ok(_) = make_request::<Value>(Method::GET, hostname, Some(body)) {
        Ok(String::from("Instance started"))
    } else {
        Err("Instance was not started".into())
    }
}

fn parse_response_body(machines: Machines) -> Result<Vec<Instance>, Box<dyn Error>> {
    let mut instances = Vec::new();
    for machine in machines.iter() {
        instances.push(Instance {
            machine_id: machine.id.clone(),
            volume_id: machine
                .config
                .mounts
                .get(0)
                .map(|mount| mount.volume.clone())
                .unwrap_or_default(),
            name: machine.name.clone(),
            specs: match &machine.config.guest {
                Some(guest) => InstanceSpecs {
                    cpu_count: guest.cpus,
                    memory_mb: guest.memory_mb,
                    volume_gb: machine
                        .config
                        .mounts
                        .get(0)
                        .map(|mount| mount.size_gb)
                        .unwrap_or_default(),
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
    name: &str,
    specs: InstanceSpecs,
    region: Option<&String>,
    volume_id: Option<&String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut body = serde_json::json!({
        "name": name,
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
                "cpus": specs.cpu_count,
                "memory_mb": specs.memory_mb
            }
        }
    });

    if let Some(reg) = region {
        body["region"] = serde_json::json!(reg);
    }

    if let Some(v_id) = volume_id {
        let mounts = serde_json::json!([{
            "encrypted": true,
            "name": name,
            "path": "/data",
            "size_gb": specs.volume_gb,
            "size_gb_limit": 500,
            "volume": v_id
        }]);
        body["config"]["mounts"] = mounts;
    }

    Ok(serde_json::to_string(&body)?)
}

fn get_instance_from_name(name: &str) -> Result<Instance, Box<dyn Error>> {
    let instances = get_instances()?;
    let instance = instances
        .iter()
        .find(|instance| instance.name == name)
        .cloned();

    instance.ok_or_else(|| "Instance not found".into())
}
