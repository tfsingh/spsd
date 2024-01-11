use std::error::Error;

use crate::utils;

pub fn create_new_instance<'a>(
    name: &str,
    image: &str,
    cpus: u32,
    memory: u32,
    volume: u32,
    region: &str,
    port: Option<u16>,
) -> Result<String, Box<dyn Error>> {
    let result = utils::machines::create_machine(name, image, cpus, memory, volume, region, port);
    match result {
        Ok(_) => Ok(String::from(format!("Created instance {}", name))),
        Err(contents) => Err(contents),
    }
}
