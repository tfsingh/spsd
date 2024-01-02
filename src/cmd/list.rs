use crate::lib::instances::get_instances;

pub fn list_instances() {
    println!("{:?}", get_instances().unwrap());
}
