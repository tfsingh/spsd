use crate::utils;

pub fn create_new_instance(name: &String, cpus: u32, memory: u32, region: &String) {
    // todo: figure out volumes and state persistence
    println!(
        "{:?}",
        utils::machines::create_machine(name, cpus, memory, region)
    );
}
