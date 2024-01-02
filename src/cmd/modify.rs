use crate::lib::instances::{create_instance, delete_instance};
pub fn modify_instance(name: &String, region: &String, size: &String) {
    if let Ok(Some(instance)) = delete_instance(name) {
        create_instance(&instance.machine_id, name, size, region);
    }
}
