use std::error::Error;

use crate::utils::machines::stop_machine;

pub fn stop_instance(name: &str) -> Result<String, Box<dyn Error>> {
    let result = stop_machine(name);
    match result {
        Ok(_) => Ok(String::from(format!("Stopped instance {}", name))),
        Err(contents) => Err(contents),
    }
}
