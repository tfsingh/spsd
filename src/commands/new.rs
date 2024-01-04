use crate::utils;

pub fn create_new_instance(name: &String, cpus: u32, memory: u32, region: &String) {
    // todo: figure out volumes and state persistence
    println!(
        "{:?}",
        utils::instances::create_instance(name, cpus, memory, region)
    );
}
