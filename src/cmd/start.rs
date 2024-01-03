use crate::lib;

pub fn start_instance(name: &String) {
    let result = lib::instances::get_instance_id_from_name(name);
    let instance_id = match result {
        Ok(instance_id) => instance_id,
        Err(error) => return eprintln!("{}", error),
    };
    println!("{:?}", instance_id);
}
