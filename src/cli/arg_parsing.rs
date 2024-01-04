use crate::lib::constants::{POSSIBLE_REGIONS, POSSIBLE_SIZES};
use clap::{arg, ArgMatches, Command};

// changes -> switch to cpu/memory, let region be set to a default
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
                    arg!(<size> "Size of instance")
                        .value_parser(POSSIBLE_SIZES)
                        .required(true),
                )
                .arg(
                    arg!(<region> "Region of instance (immutable)")
                        .value_parser(POSSIBLE_REGIONS)
                        .required(true),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("modify")
                .about("Modify the configuration of an instance")
                .arg(arg!(<name> "Name of instance").required(true))
                .arg(
                    arg!(<size> "Size of instance")
                        .value_parser(POSSIBLE_SIZES)
                        .required(true),
                )
                .arg_required_else_help(true),
        )
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
