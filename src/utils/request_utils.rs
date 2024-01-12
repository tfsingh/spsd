use super::config::{get_api_key, get_app_name};
use super::types::{parse_state, Instance, InstanceSpecs};
use crate::utils::types::Machines;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::error::Error;

pub fn parse_response_body(machines: Machines) -> Result<Vec<Instance>, Box<dyn Error>> {
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

pub fn create_body_from_specs(
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
