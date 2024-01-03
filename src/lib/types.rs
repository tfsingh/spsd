use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub struct InstanceData {
    pub size: i32,
    pub disk: i32,
}

#[derive(Debug)]
pub struct Instance {
    pub machine_id: String,
    pub name: String,
    pub size: String,
    pub region: String,
}

// structs for parsing fly's api responses from fade (https://github.com/nebulatgs/fade/)
pub type Machines = Vec<Machine>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Machine {
    pub id: String,
    pub name: String,
    pub state: String,
    pub region: String,
    pub instance_id: String,
    pub private_ip: String,
    pub config: MachineConfig,
    pub image_ref: ImageRef,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageRef {
    pub registry: String,
    pub repository: String,
    pub tag: String,
    pub digest: String,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineConfig {
    pub env: Option<HashMap<String, String>>,
    pub init: Init,
    pub image: String,
    pub services: Option<Vec<Service>>,
    pub metadata: Option<HashMap<String, String>>,
    pub restart: Option<Restart>,
    pub guest: Option<Guest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub internal_port: u16,
    pub ports: Vec<Port>,
    pub protocol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Port {
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Guest {
    pub cpu_kind: String,
    pub cpus: i64,
    pub memory_mb: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Init {
    pub exec: Option<Value>,
    pub entrypoint: Option<Vec<String>>,
    pub cmd: Option<Vec<String>>,
    pub tty: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Restart {
    pub policy: String,
}
