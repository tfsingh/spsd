use super::types::Instance;
use csv::ReaderBuilder;
use csv::StringRecord;
use csv::WriterBuilder;
use std::error::Error;

const PATH: &str = "instances.csv";

pub fn get_instance(name: &String) -> Result<Instance, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(PATH)?;

    for result in reader.records() {
        let record = result?;
        if record.get(1).unwrap() == name {
            return get_instance_from_record(record);
        }
    }

    Err("Instance with given name does not exist".into())
}

pub fn create_instance(
    machine_id: &String,
    name: &String,
    size: &String,
    region: &String,
) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new().from_path(PATH)?;

    let name_verification = get_instance(name);

    if let Err(_) = name_verification {
        writer.write_record(&[machine_id, name, size, region])?;
        writer.flush()?;

        Ok(())
    } else {
        Err("Instance with given name already exists".into())
    }
}

pub fn delete_instance(name: &str) -> Result<Option<Instance>, Box<dyn Error>> {
    let mut rows = get_instances()?;

    let index = rows.iter().position(|instance| instance.name == name);

    let deleted_instance = match index {
        Some(i) => Some(rows.remove(i)),
        None => None,
    };

    let mut wtr = WriterBuilder::new().has_headers(true).from_path(PATH)?;

    for instance in &rows {
        wtr.write_record(&[
            &instance.machine_id,
            &instance.name,
            &instance.size,
            &instance.region,
        ])?;
    }

    wtr.flush()?;

    Ok(deleted_instance)
}

pub fn get_instances() -> Result<Vec<Instance>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(PATH)?;

    let rows: Result<Vec<Instance>, Box<dyn Error>> = rdr
        .records()
        .map(|record| {
            let record = record.map_err(|e| Box::new(e) as Box<dyn Error>)?;
            get_instance_from_record(record)
        })
        .collect();

    rows
}

fn get_instance_from_record(record: StringRecord) -> Result<Instance, Box<dyn Error>> {
    let machine_id = record
        .get(0)
        .map(String::from)
        .ok_or("machine_id corrupted")?;
    let name = record.get(1).map(String::from).ok_or("name corrupted")?;
    let size = record.get(2).map(String::from).ok_or("size corrupted")?;
    let region = record.get(3).map(String::from).ok_or("region corrupted")?;
    Ok(Instance {
        machine_id,
        name,
        size,
        region,
    })
}
