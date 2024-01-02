use crate::lib::instances::{create_instance, delete_instance};
pub fn modify_instance(name: &String, region: &String, size: &String) {
    delete_instance(name);
}
