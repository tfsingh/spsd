use clap::ArgMatches;

mod cli;
mod cmd;
mod lib;

fn main() {
    let command = cli::arg_parsing::read_input();

    match command.subcommand() {
        Some(("new", args)) => {
            let name = args.try_get_one::<String>("name");
            let size = args.try_get_one::<String>("size");
            let region = args.try_get_one::<String>("region");

            match (name, size, region) {
                (Ok(Some(name)), Ok(Some(size)), Ok(Some(region))) => {
                    cmd::new::create_new_instance(name, size, region)
                }
                _ => eprintln!("Error in argument parsing"),
            }
        }
        Some(("modify", args)) => {
            let name = args.try_get_one::<String>("name");
            let size = args.try_get_one::<String>("size");

            match (name, size) {
                (Ok(Some(name)), Ok(Some(size))) => cmd::modify::modify_instance(name, size),
                _ => eprintln!("Error in argument parsing"),
            }
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
        Some(("list", _)) => cmd::list::list_instances(),
        Some(("profile", _args)) => cmd::profile::modify_profile(),
        _ => {
            eprintln!("Subcommand invalid")
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
