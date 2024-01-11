use crate::utils;
pub fn destroy_instance(name: &str) {
    println!("{:?}", utils::machines::delete_machine(name));
}
