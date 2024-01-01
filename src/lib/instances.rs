use super::types;
use csv::ReaderBuilder;
use std::error::Error;

pub fn get_instance(name: &String) -> Result<types::Instance, Box<dyn Error>> {
    let file_path = "instances.csv";

    let mut reader = ReaderBuilder::new().from_path(file_path)?;

    for result in reader.records() {
        let record = result?;
        if record.get(1).unwrap() == name {
            return {
                Ok(types::Instance {
                    machine_id: record.get(0).map(String::from).ok_or("Invalid CSV")?,
                    name: record.get(1).map(String::from).ok_or("Invalid CSV")?,
                    size: record.get(2).map(String::from).ok_or("Invalid CSV")?,
                    region: record.get(3).map(String::from).ok_or("Invalid CSV")?,
                })
            };
        }
    }

    Err("ERROR: Instance with given name does not exist".into())
}

pub fn create_instance(machine_id: &String, name: &String, size: &String, region: &String) {
    // take an optional parameter that is "index"
}

pub fn delete_instance(name: &String) {}
