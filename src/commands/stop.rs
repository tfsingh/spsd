use crate::utils::machines::stop_machine;

pub fn stop_instance(name: &String) {
    stop_machine(name);
}
