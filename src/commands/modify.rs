use crate::utils::instances::update_instance;
pub fn modify_instance(name: &String, cpus: u32, memory: u32) {
    println!("{:?}", update_instance(name, cpus, memory));
}
