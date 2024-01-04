use crate::utils;
pub fn destroy_instance(name: &String) {
    println!("{:?}", utils::machines::delete_machine(name));
}
