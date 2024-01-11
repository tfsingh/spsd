use super::{start::start_instance, stop::stop_instance};
use std::time::Instant;

pub fn ephemeral(name: &str) {
    let start_time = Instant::now();

    start_instance(name);
    stop_instance(name);

    let duration = start_time.elapsed();

    println!("{:?}", duration);
}
