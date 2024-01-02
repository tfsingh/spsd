use super::types;
use csv::ReaderBuilder;
use csv::StringRecord;
use csv::WriterBuilder;
use std::error::Error;

const PATH: &str = "instances.csv";

pub fn get_instance(name: &String) -> Result<types::Instance, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(PATH)?;

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

pub fn delete_instance(name: &String) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(PATH)?;

    let rows: Vec<_> = rdr.records().collect::<Result<Vec<StringRecord>, _>>()?;

    let modified_rows: Vec<_> = rows
        .into_iter()
        .filter(|record| record.iter().any(|field| field == name))
        .collect();

    let mut wtr = WriterBuilder::new().has_headers(true).from_path(PATH)?;
    for record in modified_rows {
        wtr.write_record(&record.iter().collect::<Vec<&str>>())?;
    }
    wtr.flush()?;

    Ok(())
}
