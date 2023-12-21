use chrono::Datelike;
use csv;
use reading::DailyTemperatureReading;
use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    fs,
    path::PathBuf,
    vec,
};

pub mod reading;

pub fn read_dir(
    path: &str,
) -> Result<HashMap<u16, HashMap<u16, Vec<DailyTemperatureReading>>>, Box<dyn Error>> {
    let mut yearly_readings: HashMap<u16, HashMap<u16, Vec<DailyTemperatureReading>>> =
        HashMap::new();

    for entry in fs::read_dir(path)? {
        let path = entry?.path();

        let (year, month, daily_readings_for_month) = read_file(&path)?;

        match yearly_readings.entry(year) {
            Entry::Vacant(e) => {
                let mut monthly_readings_hash_map = HashMap::new();
                monthly_readings_hash_map.insert(month, daily_readings_for_month);
                e.insert(monthly_readings_hash_map);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().insert(month, daily_readings_for_month);
            }
        }
    }

    Ok(yearly_readings)
}

fn read_file(path: &PathBuf) -> Result<(u16, u16, Vec<DailyTemperatureReading>), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut iter: csv::DeserializeRecordsIter<'_, fs::File, DailyTemperatureReading> =
        rdr.deserialize();

    let mut daily_readings: Vec<DailyTemperatureReading> = vec![];

    let mut month = 0;
    let mut year = 0;

    if let Some(record) = iter.next() {
        let result = record.unwrap();

        year = result.date.year() as u16;
        month = result.date.month() as u16;

        daily_readings.push(result);
    }

    for record in iter {
        if record.is_ok() {
            daily_readings.push(record.unwrap());
        }
    }

    Ok((year, month, daily_readings))
}
