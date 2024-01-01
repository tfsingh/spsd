use crate::lib;

pub fn start_instance(name: &String) {
    let result = lib::instances::get_instance(name);
    let instance = match result {
        Ok(instance) => instance,
        Err(error) => return eprintln!("{}", error),
    };
    println!("{:?}", instance);
}
