use clap::{arg, ArgMatches, Command};
mod cmd;

fn main() {
    let command = Command::new("spec")
        .author("Tej Singh, tejfsingh@gmail.com")
        .version("0.1")
        .about("An interface for managing state persistent ephermeral compute")
        .subcommand_required(true)
        .subcommand(
            Command::new("new")
                .about("Create a new instance")
                .arg(arg!(<name> "Name of instance"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("start")
                .about("Start an instance")
                .arg(arg!(<name> "Name of instance")),
        )
        .subcommand(
            Command::new("stop")
                .about("Stop an instance")
                .arg(arg!(<name> "Name of instance")),
        )
        .subcommand(
            Command::new("destroy")
                .about("Destroy an instance")
                .arg(arg!(<name> "Name of instance")),
        )
        .subcommand(
            Command::new("modify")
                .about("Modify the configuration of an instance")
                .arg(arg!(<name> "Name of instance")),
        )
        .subcommand(Command::new("list").about("List all instances"))
        .subcommand(Command::new("profile").about("Set profile for infra provider"))
        .get_matches();

    match command.subcommand() {
        Some(("new", args)) => {
            handle_command_with_name(args, |name| cmd::new::create_new_instance(name))
        }
        Some(("start", args)) => {
            handle_command_with_name(args, |name| cmd::start::start_instance(name))
        }
        Some(("stop", args)) => {
            handle_command_with_name(args, |name| cmd::stop::stop_instance(name))
        }
        Some(("destroy", args)) => {
            handle_command_with_name(args, |name| cmd::destroy::destroy_instance(name))
        }
        Some(("modify", args)) => {
            handle_command_with_name(args, |name| cmd::modify::modify_instance(name))
        }
        Some(("list", _)) => cmd::list::list_instances(),
        Some(("profile", _subm)) => cmd::profile::modify_profile(),
        _ => {
            eprintln!("ERROR: Subcommand invalid")
        }
    }
}

fn handle_command_with_name<F>(args: &ArgMatches, function: F)
where
    F: Fn(&String),
{
    if let Ok(Some(name)) = args.try_get_one::<String>("name") {
        function(&name);
    } else {
        eprintln!("Please provide the name of the instance");
    }
}
