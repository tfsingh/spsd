use super::value_parsers;
use clap::{arg, ArgMatches, Command};

pub fn read_input() -> ArgMatches {
    Command::new("spsd")
        .author("Tej Singh, tejfsingh@gmail.com")
        .version("0.1")
        .about("A utility for managing state persistent serverless devboxes")
        .subcommand_required(true)
        .subcommand(
            Command::new("new")
                .about("Create a new instance")
                .arg(arg!(<name> "Name of instance").required(false))
                .arg(arg!(<image> "Url of image (\"base\" for ubuntu)")
                        .value_parser(value_parsers::parse_image)
                        .required(false))
                .arg(
                    arg!(<cpus> "Number of CPUs (1, 2, 4, 8, 12, 16)")
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
                ).arg(arg!(<port> "Port to expose (optional)").value_parser(value_parsers::parse_port).required(false))
                .after_help("Please note fly enforces cpu/memory ratios that may render your configuration invalid"),
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
            Command::new("sl")
            .about("Run an instance serverlessly")
            .arg(arg!(<name> "Name of instance").required(true))
            .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("destroy")
                .about("Destroy an instance")
                .arg(arg!(<name> "Name of instance").required(true))
                .arg_required_else_help(true),
        ).subcommand(
            Command::new("profile")
            .about("Set fly.io profile")
            .arg(arg!(<api_key> "Fly api key (overwrites existing)").required(true))
            .arg(arg!(<allocate_ip> "Allocate a dedicated IPv4 address (optional)").required(true).value_parser(["y", "n"])))
        .subcommand(
            Command::new("list")
            .about("List instances and attached IPs")
            .arg(arg!(<ip> "List attached IPv4 addresses").required(false)))
        .get_matches()
}
