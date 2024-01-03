use crate::lib::instances::update_instance;
pub fn modify_instance(name: &String, size: &String, region: &String) {
    update_instance(name, size, region);
}
