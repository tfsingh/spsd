use crate::utils;

pub fn create_new_instance(
    name: &str,
    image: &str,
    cpus: u32,
    memory: u32,
    volume: u32,
    region: &str,
    port: Option<u16>,
) {
    println!(
        "{:?}",
        utils::machines::create_machine(name, image, cpus, memory, volume, region, port)
    );
}
