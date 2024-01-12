use crate::utils;
use std::{error::Error, process::Command};

pub fn start_instance(name: &str) -> Result<String, Box<dyn Error>> {
    let instance_id = utils::machines::start_machine(name)?;
    let mut child = Command::new("flyctl")
        .arg("ssh")
        .arg("console")
        .arg("--machine")
        .arg(instance_id)
        .arg("--quiet")
        .spawn()?;
    child.wait().unwrap();
    Ok(String::new())
}
