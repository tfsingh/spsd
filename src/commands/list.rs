use crate::utils::machines::get_instances;

pub fn list_instances() {
    println!("{:?}", get_instances().unwrap());
}
