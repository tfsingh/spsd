use crate::cli::io;
use crate::utils::constants::get_app_name;
use crate::utils::machines::get_instances;
use std::error::Error;
use std::process::Command;

pub fn list_instances(list_ips: bool) -> Result<String, Box<dyn Error>> {
    if list_ips {
        println!("");
        let mut child = Command::new("flyctl")
            .arg("ip")
            .arg("list")
            .arg("-a")
            .arg(get_app_name()?)
            .spawn()?;
        child.wait().unwrap();
    } else {
        let result = get_instances()?;
        io::display_instances(result);
    }

    Ok(String::new())
}
