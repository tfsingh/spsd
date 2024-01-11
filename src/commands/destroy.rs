use std::error::Error;

use crate::utils;
pub fn destroy_instance(name: &str) -> Result<String, Box<dyn Error>> {
    let result = utils::machines::delete_machine(name);
    match result {
        Ok(_) => Ok(String::from(format!("Destroyed instance {}", name))),
        Err(contents) => Err(contents),
    }
}
