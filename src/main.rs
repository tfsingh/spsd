use clap::ArgMatches;

mod cli;
mod commands;
mod utils;

fn main() {
    let command = cli::arg_parsing::read_input();

    match command.subcommand() {
        Some(("new", args)) => {
            let name = args.try_get_one::<String>("name");
            let cpus = args.try_get_one::<u32>("cpus");
            let memory = args.try_get_one::<u32>("memory");
            let region = args.try_get_one::<String>("region");

            match (name, cpus, memory, region) {
                (Ok(Some(name)), Ok(Some(cpus)), Ok(Some(memory)), Ok(Some(region))) => {
                    commands::new::create_new_instance(name, *cpus, *memory, region)
                }
                (name, cpus, memory, region) => {
                    eprintln!(
                        "Error in argument parsing: name={:?}, cpus={:?}, memory={:?}, region={:?}",
                        name, cpus, memory, region
                    );
                }
            }
        }

        Some(("modify", args)) => {
            let name = args.try_get_one::<String>("name");
            let cpus = args.try_get_one::<u32>("cpus");
            let memory = args.try_get_one::<u32>("memory");

            match (name, cpus, memory) {
                (Ok(Some(name)), Ok(Some(cpus)), Ok(Some(memory))) => {
                    commands::modify::modify_instance(name, *cpus, *memory)
                }
                _ => eprintln!("Error in argument parsing"),
            }
        }
        Some(("start", args)) => {
            handle_command_with_name(args, |name| commands::start::start_instance(name))
        }
        Some(("stop", args)) => {
            handle_command_with_name(args, |name| commands::stop::stop_instance(name))
        }
        Some(("destroy", args)) => {
            handle_command_with_name(args, |name| commands::destroy::destroy_instance(name))
        }
        Some(("list", _)) => commands::list::list_instances(),
        Some(("profile", _args)) => commands::profile::modify_profile(),
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
