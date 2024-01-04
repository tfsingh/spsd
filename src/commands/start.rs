use crate::utils;
use std::process::Command;

pub fn start_instance(name: &String) {
    let instance_id = utils::machines::start_machine(name).unwrap();
    // todo: need to wait for the instance to be ready
    let mut child = Command::new("flyctl")
        .arg("ssh")
        .arg("console")
        .arg("--machine")
        .arg(instance_id)
        .spawn()
        .unwrap();
    let asd = child.wait().unwrap();
}
