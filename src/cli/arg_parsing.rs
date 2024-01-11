use crate::utils::constants::{validate_cpu, validate_memory, validate_volume, POSSIBLE_REGIONS};
use clap::{arg, ArgMatches, Command};

// todo: let region be set to a default, -q flag on new
// https://gist.github.com/cbzehner/dede31dc0b4617fd4abaff9e03b15f9e
// https://github.com/clap-rs/clap/issues/1634
pub fn read_input() -> ArgMatches {
    Command::new("spec")
        .author("Tej Singh, tejfsingh@gmail.com")
        .version("0.1")
        .about("An interface for managing state persistent ephermeral compute")
        .subcommand_required(true)
        .subcommand(
            Command::new("new")
                .about("Create a new instance")
                .arg(arg!(<name> "Name of instance").required(true))
                .arg(
                    arg!(<cpus> "Number of CPUs (1-16)")
                        .value_parser(validate_cpu)
                        .required(true),
                )
                .arg(
                    arg!(<memory> "Amount of memory (256 - 32768 mb)")
                        .value_parser(validate_memory)
                        .required(true),
                ).arg(arg!(<volume> "Size of volume (1-500gb, immutable)").value_parser(validate_volume).required(true))
                .arg(
                    arg!(<region> "Region of instance (immutable)")
                        .value_parser(POSSIBLE_REGIONS)
                        .required(true),
                )
                .arg_required_else_help(true).after_help("Please note fly enforces cpu/memory ratios that may make your configuration invalid"),
        )/*
        .subcommand(
            Command::new("modify")
                .about("Modify the configuration of an instance")
                .arg(arg!(<name> "Name of instance").required(true))
                .arg(
                    arg!(<cpus> "Number of CPUs")
                        .value_parser(validate_cpu)
                        .required(true),
                )
                .arg(
                    arg!(<memory> "Amount of memory (mb)")
                        .value_parser(validate_memory)
                        .required(true),
                )
                .arg_required_else_help(true),
        )*/
        .subcommand(
            Command::new("start")
                .about("Start an instance")
                .arg(arg!(<name> "Name of instance").required(true))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("stop")
                .about("Stop an instance")
                .arg(arg!(<name> "Name of instance").required(true))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("destroy")
                .about("Destroy an instance")
                .arg(arg!(<name> "Name of instance").required(true))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("list").about("List all instances"))
        .subcommand(Command::new("profile").about("Set profile for infra provider"))
        .get_matches()
}
