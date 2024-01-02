use crate::lib;
pub fn destroy_instance(name: &String) {
    println!("{:?}", lib::instances::delete_instance(name));
}
