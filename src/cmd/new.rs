use crate::lib;

pub fn create_new_instance(name: &String, size: &String, region: &String) {
    // api call to create new instance
    let machine_id = String::from("1");
    lib::instances::create_instance(&machine_id, name, size, region);
}
