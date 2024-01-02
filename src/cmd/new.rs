use crate::lib;

pub fn create_new_instance(name: &String, size: &String, region: &String) {
    // api call to create new instance
    // process of creation will help us refine the instance data struct
    // in types we'll define sizes -> associated data (cpu, disk, etc)
    // note — figure out volumes and machine persistence
    let machine_id = String::from("1");
    lib::instances::create_instance(&machine_id, name, size, region);
}
