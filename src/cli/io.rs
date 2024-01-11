use crate::utils::types::{Instance, InstanceState};
use colored::Colorize;
use std::error::Error;
extern crate prettytable;
use prettytable::{cell, row, Cell, Row, Table};

pub fn display_error(err: Box<dyn Error>) {
    println!("{}: {}", "ERROR".red(), err);
}

pub fn display_success(message: &str) {
    if message != String::new() {
        println!("{}: {}", "SUCCESS".blue(), message)
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
            Cell::new(&format!("{:?}", instance.specs.cpu_count)),
            Cell::new(&format!("{:?} mb", instance.specs.memory_mb)),
            Cell::new(&format!("{:?} gb", instance.specs.volume_gb)),
            Cell::new(&instance.region),
            Cell::new(&format!("{:?}", instance.port.unwrap())),
            Cell::new(&format!(
                "{}",
                match instance.state {
                    InstanceState::Running => "Running".green(),
                    InstanceState::Stopped => "Stopped".red(),
                }
            )),
        ]));
    }

    table.printstd();
}
