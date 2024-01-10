use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Instance {
    pub machine_id: String,
    pub volume_id: String,
    pub name: String,
    pub specs: InstanceSpecs,
    pub region: String,
    pub state: InstanceState,
}

#[derive(Debug, Clone)]
pub struct InstanceSpecs {
    pub cpu_count: u32,
    pub memory_mb: u32,
    pub volume_gb: u32,
}

#[derive(Debug, Clone)]
pub enum InstanceState {
    Running,
    Stopped,
}

impl InstanceSpecs {
    pub fn phony() -> Self {
        Self {
            cpu_count: 0,
            memory_mb: 0,
            volume_gb: 0,
        }
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
    pub mounts: Vec<Mount>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Mount {
    pub encrypted: bool,
    pub path: String,
    pub size_gb: u32,
    pub volume: String,
    pub name: String,
    pub size_gb_limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    pub id: String,
    pub name: String,
    pub state: String,
    pub size_gb: u32,
    pub region: String,
    pub zone: String,
    pub encrypted: bool,
    pub attached_machine_id: Option<String>,
    pub attached_alloc_id: Option<String>,
    pub created_at: String,
    pub blocks: u32,
    pub block_size: u32,
    pub blocks_free: u32,
    pub blocks_avail: u32,
    pub fstype: String,
    pub snapshot_retention: u32,
    pub host_dedication_key: String,
}
