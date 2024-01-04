use crate::utils;
pub fn destroy_instance(name: &String) {
    println!("{:?}", utils::instances::delete_instance(name));
}
