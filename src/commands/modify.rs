use crate::utils::machines::update_machine;
pub fn modify_instance(name: &String, cpus: u32, memory: u32) {
    println!("{:?}", update_machine(name, cpus, memory));
}
