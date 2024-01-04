use crate::lib::instances::update_instance;
pub fn modify_instance(name: &String, size: &String) {
    println!("{:?}", update_instance(name, size));
}
