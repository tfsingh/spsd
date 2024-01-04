use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Instance {
    pub machine_id: String,
    pub name: String,
    pub specs: InstanceSpecs,
    pub region: String,
    pub state: InstanceState,
}

#[derive(Debug)]
pub struct InstanceSpecs {
    pub cpus: u32,
    pub memory: u32,
}

#[derive(Debug)]
pub enum InstanceState {
    Running,
    Stopped,
}

impl InstanceSpecs {
    pub fn phony() -> Self {
        Self { cpus: 0, memory: 0 }
    }
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
    pub cpus: u32,
    pub memory_mb: u32,
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
    pub policy: Option<String>,
}
