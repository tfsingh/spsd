use crate::utils;

pub fn create_new_instance(name: &String, cpus: u32, memory: u32, volume: u32, region: &String) {
    println!(
        "{:?}",
        utils::machines::create_machine(name, cpus, memory, volume, region)
    );
}
