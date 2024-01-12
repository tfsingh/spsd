use super::value_parsers;
use clap::{arg, ArgMatches, Command};

// todo: let region be set to a default, -q flag on new
pub fn read_input() -> ArgMatches {
    Command::new("spec")
        .author("Tej Singh, tejfsingh@gmail.com")
        .version("0.1")
        .about("An interface for managing state persistent ephermeral compute")
        .subcommand_required(true)
        .subcommand(
            Command::new("new")
                .about("Create a new instance")
                .arg(arg!(<name> "Name of instance").required(false))
                .arg(arg!(<image> "Url of image (\"base\" for docker-python)")
                        .value_parser(value_parsers::parse_image)
                        .required(false))
                .arg(
                    arg!(<cpus> "Number of CPUs (1-16)")
                        .value_parser(value_parsers::parse_cpu)
                        .required(false),
                )
                .arg(
                    arg!(<memory> "Amount of memory (256 - 32768 mb)")
                        .value_parser(value_parsers::parse_memory)
                        .required(false),
                ).arg(arg!(<volume> "Size of volume (1-500 gb)").value_parser(value_parsers::parse_volume).required(false))
                .arg(
                    arg!(<region> "Region of instance")
                        .value_parser(value_parsers::parse_region)
                        .required(false),
                ).arg(arg!(<port> "Local port to expose (optional)").value_parser(value_parsers::parse_port).required(false))
                .after_help("Please note fly enforces cpu/memory ratios that render your configuration invalid"),
        )
        .subcommand(
            Command::new("start")
                .about("Start and connect to an instance")
                .arg(arg!(<name> "Name of instance").required(true))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("stop")
                .about("Stop an instance")
                .arg(arg!(<name> "Name of instance").required(true))
                .arg_required_else_help(true),
        ).subcommand(
            Command::new("ss")
            .about("Run an instance ephemerally")
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
