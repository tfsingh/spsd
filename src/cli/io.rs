use super::value_parsers::{
    parse_cpu, parse_image, parse_memory, parse_port, parse_region, parse_volume,
};
use crate::utils::types::{Instance, InstanceInput, InstanceState};
use colored::Colorize;
use std::{
    error::Error,
    io::{self, Write},
};
extern crate prettytable;
use prettytable::{row, Cell, Row, Table};

pub fn display_error(err: Box<dyn Error>) {
    println!("\n{}: {}", "ERROR".red(), err);
}

pub fn display_success(message: &str) {
    if message != String::new() {
        println!("\n{}: {}", "SUCCESS".blue(), message)
    }
}

pub fn display_instances(instances: Vec<Instance>) {
    let mut table = Table::new();

    table.add_row(row![
        "Name".blue(),
        "Image".blue(),
        "CPUs".blue(),
        "Memory".blue(),
        "Volume".blue(),
        "Region".blue(),
        "Port".blue(),
        "State".blue()
    ]);
    for instance in instances {
        table.add_row(Row::new(vec![
            Cell::new(&instance.name),
            Cell::new(&instance.image),
            Cell::new(&format!("{}", instance.specs.cpu_count)),
            Cell::new(&format!("{} mb", instance.specs.memory_mb)),
            Cell::new(&format!("{} gb", instance.specs.volume_gb)),
            Cell::new(&instance.region),
            Cell::new(&format!(
                "{}",
                match instance.port {
                    Some(port) => port.to_string(),
                    None => String::new(),
                }
            )),
            Cell::new(&format!(
                "{}",
                match instance.state {
                    InstanceState::Running => "Running".green(),
                    InstanceState::Stopped => "Stopped".red(),
                }
            )),
        ]));
    }
    println!("");
    table.printstd();
}

fn get_user_input(prompt: &str) -> Option<String> {
    let mut input = String::new();
    print!("{}", prompt.blue());
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();

    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

pub fn prompt_instance_creation(mut instance: InstanceInput) -> InstanceInput {
    println!("");
    instance.name = match &instance.name {
        Some(_) => instance.name,
        None => get_user_input("Instance name: "),
    };
    instance.image = match &instance.image {
        Some(_) => instance.image,
        None => get_user_input("Image url: ").and_then(|input| parse_image(&input).ok()),
    };
    instance.cpus = match &instance.cpus {
        Some(_) => instance.cpus,
        None => get_user_input("Number of CPUs: ").and_then(|input| parse_cpu(&input).ok()),
    };
    instance.memory = match &instance.memory {
        Some(_) => instance.memory,
        None => get_user_input("Memory (mb): ").and_then(|input| parse_memory(&input).ok()),
    };
    instance.volume = match &instance.volume {
        Some(_) => instance.volume,
        None => get_user_input("Volume (gb): ").and_then(|input| parse_volume(&input).ok()),
    };
    instance.region = match &instance.region {
        Some(_) => instance.region,
        None => get_user_input("Region: ").and_then(|input| parse_region(&input).ok()),
    };
    instance.port = match &instance.port {
        Some(_) => instance.port,
        None => get_user_input("Port (optional, enter to continue): ").and_then(|input| {
            if input.is_empty() {
                None
            } else {
                parse_port(&input).ok()
            }
        }),
    };
    instance
}
