use crate::utils::machines::stop_machine;

pub fn stop_instance(name: &str) {
    println!("{:?}", stop_machine(name));
}
