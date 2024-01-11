use crate::utils::types::Instance;
use colored::Colorize;
use std::error::Error;

pub fn display_error(err: Box<dyn Error>) {
    println!("{}: {}", "ERROR".red(), err);
}

pub fn display_success(message: &str) {
    if message != String::new() {
        println!("{}: {}", "SUCCESS".blue(), message)
    }
}

pub fn display_instances(instances: Vec<Instance>) {
    println!("{:?}", instances);
}
