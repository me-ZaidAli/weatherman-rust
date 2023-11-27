use crate::utils::date_from;
use reading::DailyReading;
use std::{collections::HashMap, error::Error, fs, path::PathBuf};

pub mod reading;

pub fn read_dir(path: &str) -> Result<HashMap<(i32, i32), Vec<DailyReading>>, Box<dyn Error>> {
    let mut yearly_readings: HashMap<(i32, i32), Vec<DailyReading>> = HashMap::new();

    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        let (year, month, daily_readings_for_month) = read_file_at(&path)?;

        yearly_readings.insert((year, month), daily_readings_for_month);
    }

    Ok(yearly_readings)
}

fn read_file_at(path: &PathBuf) -> Result<(i32, i32, Vec<DailyReading>), Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let file_lines = file_content.lines();

    let mut daily_readings: Vec<DailyReading> = vec![];

    let mut month = 0;
    let mut year = 0;

    for (index, line) in file_lines.enumerate() {
        if index != 0 {
            let line_split: Vec<&str> = line.split(',').collect();

            let date_split: Vec<i32> = line_split[0]
                .split('-')
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            year = date_split[0];
            month = date_split[1];

            if !is_empty(&line_split) {
                let date = date_from(line_split[0]);
                let max_temperature = line_split[1].parse::<u32>().ok();
                let min_temperature = line_split[3].parse::<u32>().ok();
                let mean_temperature = line_split[2].parse::<u32>().ok();
                let max_humidity = line_split[7].parse::<i32>().ok();
                let min_humidity = line_split[9].parse::<i32>().ok();
                let mean_humidity = line_split[8].parse::<i32>().ok();

                let daily_reading = DailyReading::new(
                    date,
                    max_temperature,
                    min_temperature,
                    mean_temperature,
                    max_humidity,
                    min_humidity,
                    mean_humidity,
                );

                daily_readings.push(daily_reading);
            }
        }
    }

    Ok((year, month, daily_readings))
}

fn is_empty(reading_line_split: &Vec<&str>) -> bool {
    reading_line_split[1].is_empty()
        && reading_line_split[2].is_empty()
        && reading_line_split[3].is_empty()
        && reading_line_split[7].is_empty()
        && reading_line_split[8].is_empty()
        && reading_line_split[9].is_empty()
}
