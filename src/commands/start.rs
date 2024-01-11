use crate::utils;
use std::process::Command;

pub fn start_instance(name: &str) {
    let instance_id = utils::machines::start_machine(name).unwrap();
    let mut child = Command::new("flyctl")
        .arg("ssh")
        .arg("console")
        .arg("--machine")
        .arg(instance_id)
        .spawn()
        .unwrap();
    let _asd = child.wait().unwrap();
}
