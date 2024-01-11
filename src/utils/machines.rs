use super::constants::{get_headers, get_hostname, parse_state};
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
            poll(&instance_id)?;
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

pub fn create_machine(
    name: &str,
    image: &str,
    cpu_count: u32,
    memory_mb: u32,
    volume_gb: u32,
    region: &str,
    port: Option<u16>,
) -> Result<Instance, Box<dyn Error>> {
    let hostname = get_hostname()? + "/machines";
    let volume_id = create_volume(name, volume_gb, region)?;
    let body = create_body_from_specs(
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
        let instance = parse_response_body(vec![instance])?.remove(0);
        poll(&instance.machine_id)?;
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
    //println!("{:?}", response_body);
    if success {
        let parsed_response: T = serde_json::from_str(&response_body)?;
        Ok(Some(parsed_response))
    } else {
        Err(response_body.into())
    }
}

fn poll(instance_id: &str) -> Result<String, Box<dyn Error>> {
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
            image: machine.config.image.clone(),
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
            port: match &machine.config.services {
                Some(services) => Some(services.get(0).unwrap().internal_port),
                None => None,
            },
            state: parse_state(&machine.state),
        })
    }
    Ok(instances)
}

fn create_body_from_specs(
    name: &str,
    image: &str,
    specs: InstanceSpecs,
    region: &str,
    volume_id: &str,
    port: Option<u16>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut body = serde_json::json!({
        "name": name,
        "region": region,
        "config": {
            "init": {
                "exec": [
                    "/bin/sleep",
                    "inf"
                ]
            },
            "image": image,
            "guest": {
                "cpu_kind": "shared",
                "cpus": specs.cpu_count,
                "memory_mb": specs.memory_mb
            },
            "mounts": [{
                "encrypted": true,
                "name": name,
                "path": "/data",
                "size_gb": specs.volume_gb,
                "size_gb_limit": 500,
                "volume": volume_id
            }],
        }
    });

    if let Some(port) = port {
        body["config"]["services"] = serde_json::json!([{
            "ports": [
                {
                    "port": port,
                    "handlers": [
                    "http"
                    ]
                }
            ],
            "protocol": "tcp",
            "internal_port": port
        }]);
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
