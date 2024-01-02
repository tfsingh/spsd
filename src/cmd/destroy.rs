use crate::lib;
pub fn destroy_instance(name: &String) {
    lib::instances::delete_instance(name);
}
