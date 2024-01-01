use csv::ReaderBuilder;
use std::error::Error;

pub fn start_instance(name: &String) {
    let result = get_instance(name);
    let instance = match result {
        Ok(instance) => instance,
        Err(error) => return eprintln!("{}", error),
    };
    println!("{:?}", instance);
}

struct InstanceData {
    size: i32,
    disk: i32,
}

#[derive(Debug)]
struct Instance {
    machine_id: String,
    name: String,
    instance_type: String,
    region: String,
}

fn get_instance(name: &String) -> Result<Instance, Box<dyn Error>> {
    let file_path = "instances.csv";

    let mut reader = ReaderBuilder::new().from_path(file_path)?;

    for result in reader.records() {
        let record = result?;
        if record.get(1).unwrap() == name {
            return {
                Ok(Instance {
                    machine_id: record.get(0).map(String::from).ok_or("Invalid CSV")?,
                    name: record.get(1).map(String::from).ok_or("Invalid CSV")?,
                    instance_type: record.get(2).map(String::from).ok_or("Invalid CSV")?,
                    region: record.get(3).map(String::from).ok_or("Invalid CSV")?,
                })
            };
        }
    }

    Err("Instance with given name does not exist".into())
}
