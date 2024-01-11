use std::error::Error;

use clap::ArgMatches;

mod cli;
mod commands;
mod utils;

fn main() {
    let command = cli::arg_parsing::read_input();

    let result: Result<String, Box<dyn Error>> = match command.subcommand() {
        Some(("new", args)) => {
            let name = args.try_get_one::<String>("name");
            let image = args.try_get_one::<String>("image");
            let cpus = args.try_get_one::<u32>("cpus");
            let memory = args.try_get_one::<u32>("memory");
            let volume = args.try_get_one::<u32>("volume");
            let region = args.try_get_one::<String>("region");
            let port = args.try_get_one::<u16>("port");

            match (name, image, cpus, memory, volume, region, port) {
                (
                    Ok(Some(name)),
                    Ok(Some(image)),
                    Ok(Some(cpus)),
                    Ok(Some(memory)),
                    Ok(Some(volume)),
                    Ok(Some(region)),
                    Ok(port),
                ) => commands::new::create_new_instance(
                    name,
                    image,
                    *cpus,
                    *memory,
                    *volume,
                    region,
                    port.copied(),
                ),
                (name, image, cpus, memory, volume, region, port) => {
                    Err(format!(
                        "Error in argument parsing: name={:?}, image={:?}, cpus={:?}, memory={:?}, volume={:?}, region={:?}, port={:?}",
                        name, image, cpus, memory, volume, region, port).into()
                    )
                }
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

        Some(("list", _)) => commands::list::list_instances(),

        Some(("profile", _args)) => commands::profile::modify_profile(),
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
