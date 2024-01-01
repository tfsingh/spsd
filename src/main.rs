use clap::{arg, ArgMatches, Command};
mod cmd;
mod lib;
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
                .arg(arg!(<size> "Size of instance"))
                .arg(arg!(<region> "Region of instance"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("modify")
                .about("Modify the configuration of an instance")
                .arg(arg!(<name> "Name of instance"))
                .arg(arg!(<size> "Size of instance"))
                .arg(arg!(<region> "Region of instance")),
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
        .subcommand(Command::new("list").about("List all instances"))
        .subcommand(Command::new("profile").about("Set profile for infra provider"))
        .get_matches();

    match command.subcommand() {
        Some(("new", args)) => handle_command_with_all(args, |name, size, region| {
            cmd::new::create_new_instance(name, size, region)
        }),
        Some(("modify", args)) => handle_command_with_all(args, |name, size, region| {
            cmd::modify::modify_instance(name, size, region)
        }),
        Some(("start", args)) => {
            handle_command_with_name(args, |name| cmd::start::start_instance(name))
        }
        Some(("stop", args)) => {
            handle_command_with_name(args, |name| cmd::stop::stop_instance(name))
        }
        Some(("destroy", args)) => {
            handle_command_with_name(args, |name| cmd::destroy::destroy_instance(name))
        }
        Some(("list", _)) => cmd::list::list_instances(),
        Some(("profile", _args)) => cmd::profile::modify_profile(),
        _ => {
            eprintln!("ERROR: Subcommand invalid")
        }
    }
}

fn handle_command_with_all<F>(args: &ArgMatches, function: F)
where
    F: Fn(&String, &String, &String),
{
    let name = args.try_get_one::<String>("name");
    let size = args.try_get_one::<String>("size");
    let region = args.try_get_one::<String>("region");

    match (name, size, region) {
        (Ok(Some(name)), Ok(Some(size)), Ok(Some(region))) => function(&name, &size, &region),
        _ => eprintln!("Error in argument parsing"),
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
