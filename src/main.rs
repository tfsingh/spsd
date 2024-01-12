use clap::ArgMatches;
use cli::io::prompt_instance_creation;
use std::error::Error;
use utils::types::InstanceInput;

mod cli;
mod commands;
mod utils;

fn main() {
    let command = cli::arg_parsing::read_input();

    let result: Result<String, Box<dyn Error>> = match command.subcommand() {
        Some(("new", args)) => {
            let name = args.try_get_one::<String>("name").unwrap().cloned();
            let image = args.try_get_one::<String>("image").unwrap().cloned();
            let cpus = args.try_get_one::<u32>("cpus").unwrap().cloned();
            let memory = args.try_get_one::<u32>("memory").unwrap().cloned();
            let volume = args.try_get_one::<u32>("volume").unwrap().cloned();
            let region = args.try_get_one::<String>("region").unwrap().cloned();
            let port = args.try_get_one::<u16>("port").unwrap().cloned();

            let instance: InstanceInput = InstanceInput {
                name,
                image,
                cpus,
                memory,
                volume,
                region,
                port,
            };

            let instance = prompt_instance_creation(instance);

            match (
                instance.name,
                instance.image,
                instance.cpus,
                instance.memory,
                instance.volume,
                instance.region,
                instance.port,
            ) {
                (
                    Some(name),
                    Some(image),
                    Some(cpus),
                    Some(memory),
                    Some(volume),
                    Some(region),
                    port,
                ) => commands::new::create_new_instance(
                    &name, &image, cpus, memory, volume, &region, port,
                ),
                _ => Err("Error in argument parsing, use -h to see valid values".into()),
            }
        }

        Some(("start", args)) => {
            handle_command_with_name(args, |name| commands::start::start_instance(name))
        }

        Some(("stop", args)) => {
            handle_command_with_name(args, |name| commands::stop::stop_instance(name))
        }

        Some(("ss", args)) => handle_command_with_name(args, |name| commands::ss::ephemeral(name)),

        Some(("destroy", args)) => {
            handle_command_with_name(args, |name| commands::destroy::destroy_instance(name))
        }

        Some(("profile", args)) => {
            let api_key = args.try_get_one::<String>("api_key");
            let allocate_ip = args.try_get_one::<String>("allocate_ip");

            match (api_key, allocate_ip) {
                (Ok(Some(app_name)), Ok(Some(allocate_ip))) => {
                    let should_allocate = allocate_ip == "y";
                    commands::profile::modify_profile(app_name, should_allocate)
                }
                _ => Err("Error in argument parsing, use -h to see valid values".into()),
            }
        }

        Some(("list", args)) => {
            let list_ips = args.try_get_one::<String>("ip");
            match list_ips {
                Ok(Some(_)) => commands::list::list_instances(true),
                Ok(None) => commands::list::list_instances(false),
                _ => Err("Error in argument parsing, use -h to see valid values".into()),
            }
        }

        _ => Err("Subcommand invalid".into()),
    };

    match result {
        Ok(message) => cli::io::display_success(&message),
        Err(message) => cli::io::display_error(message),
    };
}

fn handle_command_with_name<F>(args: &ArgMatches, function: F) -> Result<String, Box<dyn Error>>
where
    F: Fn(&str) -> Result<String, Box<dyn Error>>,
{
    if let Ok(Some(name)) = args.try_get_one::<String>("name") {
        function(&name)
    } else {
        Err("Please provide the name of the instance".into())
    }
}
