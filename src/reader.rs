use csv;
use reading::DailyTemperatureReading;
use std::{error::Error, fs, path::PathBuf, vec};

pub mod reading;

pub fn read_dir(path: &str) -> Result<Vec<DailyTemperatureReading>, Box<dyn Error>> {
    let mut readings: Vec<DailyTemperatureReading> = vec![];

    for entry in fs::read_dir(path)? {
        let path = entry?.path();

        let daily_readings_for_month = read_file(&path)?;

        readings.extend(daily_readings_for_month.into_iter());
    }

    Ok(readings)
}

fn read_file(path: &PathBuf) -> Result<Vec<DailyTemperatureReading>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut iter: csv::DeserializeRecordsIter<'_, fs::File, DailyTemperatureReading> =
        rdr.deserialize();

    let mut daily_readings: Vec<DailyTemperatureReading> = vec![];

    if let Some(record) = iter.next() {
        if record.is_ok() {
            let result = record.unwrap();

            daily_readings.push(result);
        }
    }

    for record in iter {
        if record.is_ok() {
            daily_readings.push(record.unwrap());
        }
    }

    Ok(daily_readings)
}
