use csv;
use reading::DailyReading;
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
) -> Result<HashMap<u16, HashMap<u16, Vec<DailyReading>>>, Box<dyn Error>> {
    let mut yearly_readings: HashMap<u16, HashMap<u16, Vec<DailyReading>>> = HashMap::new();

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

fn read_file(path: &PathBuf) -> Result<(u16, u16, Vec<DailyReading>), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;

    let mut daily_readings: Vec<DailyReading> = vec![];

    let mut month = 0;
    let mut year = 0;

    for result in rdr.deserialize() {
        let record: DailyReading = result?;

        if let Some(date) = &record.date {
            let date_split = date
                .split('-')
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            year = date_split[0] as u16;
            month = date_split[1] as u16;
        };

        daily_readings.push(record);
    }

    Ok((year, month, daily_readings))
}
