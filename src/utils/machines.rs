use super::constants::{get_headers, get_hostname, parse_state};
use super::types::{Instance, InstanceSpecs, Volume};
use crate::utils::types::{Machine, Machines};
use reqwest::Method;
use std::error::Error;
use std::{thread, time};
use tokio;

fn get_instance_from_name(name: &str) -> Result<Instance, Box<dyn Error>> {
    let instances = get_instances()?;
    let instance = instances
        .iter()
        .find(|instance| instance.name == name)
        .cloned();

    instance.ok_or_else(|| "Instance not found".into())
}

pub fn stop_machine(name: &String) -> Result<String, Box<dyn Error>> {
    let instance_id = get_instance_from_name(&name)?.machine_id;
    let hostname = get_hostname()? + "/machines/" + &instance_id + "/stop";
    match make_request(Method::POST, hostname, None, false) {
        Ok(_) => Ok(instance_id),
        Err(error) => Err(error),
    }
}

pub fn start_machine(name: &String) -> Result<String, Box<dyn Error>> {
    let instance_id = get_instance_from_name(&name)?.machine_id;
    let hostname = get_hostname()? + "/machines/" + &instance_id + "/start";
    match make_request(Method::POST, hostname, None, false) {
        Ok(_) => Ok(instance_id),
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
    request_volume(hostname, body.to_string())
}

pub fn delete_volume(volume_id: &String) -> Result<String, Box<dyn Error>> {
    let hostname = get_hostname()? + "/volumes/" + volume_id;
    match make_request(Method::DELETE, hostname, None, false) {
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
    let result = make_request(Method::POST, hostname, Some(body), true)?;

    if let Some(instance) = result {
        let three_seconds = time::Duration::from_secs(10); // todo: remove this hack, figure out how to keep machine up
        thread::sleep(three_seconds);
        stop_machine(name)?; // todo: fix this, make it properly stop
        Ok(instance)
    } else {
        delete_volume(&volume_id)?;
        Err("Error in instance creation".into())
    }
}

// todo: figure out how to make api return mounts everytime
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
    let instance = make_request(Method::POST, hostname, Some(body), true)?;
    stop_machine(name)?;
    Ok(instance.expect("Error in updating of machine"))
}

pub fn delete_machine(name: &String) -> Result<String, Box<dyn Error>> {
    let instance = get_instance_from_name(&name)?;
    let hostname = get_hostname()? + "/" + &instance.machine_id;
    if let Ok(Some(_)) = make_request(Method::DELETE, hostname, None, false) {
        delete_volume(&instance.volume_id)?;
        Ok(String::from("Deleted"))
    } else {
        Err("Did not delete".into())
    }
}

#[tokio::main]
pub async fn request_volume(hostname: String, body: String) -> Result<String, Box<dyn Error>> {
    let headers = get_headers()?;
    let client = reqwest::Client::new();

    let request = client.post(hostname).body(body).headers(headers);

    let response = request.send().await?;
    let success = response.status().is_success();
    let body = response.text().await?;
    if success {
        let volume: Volume = serde_json::from_str(&body)?;
        Ok(volume.id)
    } else {
        Err(body.to_string().into())
    }
}

#[tokio::main]
pub async fn get_instances() -> Result<Vec<Instance>, Box<dyn Error>> {
    let headers = get_headers()?;
    let client = reqwest::Client::new();
    let hostname = get_hostname()? + "/machines";

    let response = client.get(hostname).headers(headers).send().await?;
    let success = response.status().is_success();
    let body = response.text().await?;

    if success {
        println!("{:?}", body);
        let machines: Machines = serde_json::from_str(&body)?;
        let instances = parse_response_body(machines)?;
        Ok(instances)
    } else {
        Err(body.to_string().into())
    }
}

#[tokio::main]
async fn make_request(
    method: Method,
    hostname: String,
    body: Option<String>,
    expect_parseable_result: bool,
) -> Result<Option<Instance>, Box<dyn Error>> {
    let headers = get_headers()?;
    let client = reqwest::Client::new();
    let request = if let Some(body) = body {
        client.request(method, hostname).body(body).headers(headers)
    } else {
        client.request(method, hostname).headers(headers)
    };
    let response = request.send().await?;
    let success = response.status().is_success();
    let body = response.text().await?;
    if success {
        if expect_parseable_result {
            let machine: Machine = serde_json::from_str(&body)?;
            let mut instance = parse_response_body(vec![machine])?;
            Ok(Some(instance.remove(0)))
        } else {
            Ok(None)
        }
    } else {
        Err(body.to_string().into())
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

// todo: implement a better way to keep the machine alive on startup
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
