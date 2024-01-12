use crate::utils::{self, config::get_app_name};
use std::{error::Error, process::Command};

pub fn start_instance(name: &str) -> Result<String, Box<dyn Error>> {
    let instance_id = utils::machines::start_machine(name)?;
    let mut child = Command::new("flyctl")
        .arg("ssh")
        .arg("console")
        .arg("--machine")
        .arg(instance_id)
        .arg("--quiet")
        .arg("-a")
        .arg(get_app_name()?)
        .spawn()?;
    child.wait().unwrap();
    Ok(String::new())
}
