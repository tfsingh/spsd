use crate::cli::io;
use crate::utils::machines::get_instances;
use std::error::Error;

pub fn list_instances() -> Result<String, Box<dyn Error>> {
    let result = get_instances()?;
    io::display_instances(result);
    Ok(String::new())
}
