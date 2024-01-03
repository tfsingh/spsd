use crate::lib::types::{Machine, Machines};

use super::constants::{get_headers, get_hostname, get_size_from_cpu_count, get_specs_from_size};
use super::types::{Instance, InstanceSpecs};
use std::error::Error;
use tokio;

// todo: switch to cpu/memory instead of size
pub fn get_instance_id_from_name(name: &String) -> Result<String, Box<dyn Error>> {
    let id = get_instances()?
        .iter()
        .find(|instance| &instance.name == name)
        .map(|instance| instance.machine_id.clone());

    Ok(id.ok_or("Instance not found")?)
}

pub fn create_instance(
    name: &String,
    size: &String,
    region: &String,
) -> Result<Instance, Box<dyn Error>> {
    let hostname = get_hostname()?;
    let specs = get_specs_from_size(size)?;
    let body_to_send = create_body_from_specs(name, specs, region)?;
    make_request(hostname, body_to_send)
}

pub fn update_instance(
    name: &String,
    size: &String,
    region: &String,
) -> Result<Instance, Box<dyn Error>> {
    let hostname = get_hostname()? + &get_instance_id_from_name(&name)?;
    let specs = get_specs_from_size(size)?;
    let body_to_send = create_body_from_specs(name, specs, region)?;
    make_request(hostname, body_to_send)
}

pub fn delete_instance(name: &String) -> Result<Instance, Box<dyn Error>> {
    unimplemented!()
}

#[tokio::main]
pub async fn get_instances() -> Result<Vec<Instance>, Box<dyn Error>> {
    let headers = get_headers()?;
    let client = reqwest::Client::new();

    let response = client.get(get_hostname()?).headers(headers).send().await?;
    let success = response.status().is_success();
    let body = response.text().await?;

    if success {
        println!("{:?}", body);
        let machines: Machines = serde_json::from_str(&body)?;
        let instances = parse_response_body(machines)?;
        Ok(instances)
    } else {
        Err(body.into())
    }
}

#[tokio::main]
async fn make_request(hostname: String, body_to_send: String) -> Result<Instance, Box<dyn Error>> {
    let headers = get_headers()?;
    let client = reqwest::Client::new();

    let response = client
        .get(hostname)
        .headers(headers)
        .body(body_to_send)
        .send()
        .await?;
    let success = response.status().is_success();
    let body = response.text().await?;

    if success {
        println!("{:?}", body);
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
            size: match &machine.config.guest {
                Some(guest) => get_size_from_cpu_count(guest.cpus)?,
                None => String::from("none"),
            },
            region: machine.region.clone(),
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
