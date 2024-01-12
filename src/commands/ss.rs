use super::{start::start_instance, stop::stop_instance};
use std::{error::Error, time::Instant};

pub fn serverless(name: &str) -> Result<String, Box<dyn Error>> {
    let start_time = Instant::now();

    start_instance(name)?;
    stop_instance(name)?;

    let _duration = start_time.elapsed();

    Ok(String::new())
}
