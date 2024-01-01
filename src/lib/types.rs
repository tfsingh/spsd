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
