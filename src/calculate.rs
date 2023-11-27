use crate::reader::reading::DailyReading;
use chrono::{Datelike, NaiveDate};
use colored::Colorize;
use std::{collections::HashMap, fmt, vec};

enum ReadingType {
    Temperature((NaiveDate, u32)),
    Humidity((NaiveDate, i32)),
}

impl ReadingType {
    fn temperature(&self) -> Option<(NaiveDate, u32)> {
        if let ReadingType::Temperature(temp) = self {
            Some(*temp)
        } else {
            None
        }
    }

    fn humidity(&self) -> Option<(NaiveDate, i32)> {
        if let ReadingType::Humidity(hum) = self {
            Some(*hum)
        } else {
            None
        }
    }
}
enum YearlyCalculationTypes {
    MaxTemperature,
    MinTemperature,
    MaxHumidity,
}

#[derive(PartialEq)]
enum MonthlyCalculationTypes {
    MaxMeanTemperature,
    MinMeanTemperature,
    MeanHumidity,
}

pub struct YearlyCalculation {
    highest_temperature_with_date: (NaiveDate, u32),
    lowest_temperature_with_date: (NaiveDate, u32),
    max_humidity_with_date: (NaiveDate, i32),
}

impl YearlyCalculation {
    pub fn calculate(
        year: i32,
        yearly_readings_map: &HashMap<(i32, i32), Vec<DailyReading>>,
    ) -> Result<Self, &str> {
        let highest_temperature_with_date = Self::find(
            YearlyCalculationTypes::MaxTemperature,
            year,
            yearly_readings_map,
        )?
        .temperature()
        .unwrap();
        let lowest_temperature_with_date = Self::find(
            YearlyCalculationTypes::MinTemperature,
            year,
            yearly_readings_map,
        )?
        .temperature()
        .unwrap();
        let max_humidity_with_date = Self::find(
            YearlyCalculationTypes::MaxHumidity,
            year,
            yearly_readings_map,
        )?
        .humidity()
        .unwrap();

        Ok(Self {
            highest_temperature_with_date,
            lowest_temperature_with_date,
            max_humidity_with_date,
        })
    }

    fn find(
        action: YearlyCalculationTypes,
        year: i32,
        yearly_readings_map: &HashMap<(i32, i32), Vec<DailyReading>>,
    ) -> Result<ReadingType, &str> {
        let mut readings: Vec<ReadingType> = vec![];

        (1..13).for_each(|month| {
            if let Some(reading) = yearly_readings_map.get(&(year, month)) {
                let reading_iter = reading.iter();

                readings.push(match action {
                    YearlyCalculationTypes::MaxTemperature => ReadingType::Temperature(
                        reading_iter
                            .filter(|reading| reading.max_temperature.is_some())
                            .map(|reading| (reading.date.clone(), reading.max_temperature.unwrap()))
                            .max_by_key(|date_temperature| date_temperature.1)
                            .unwrap(),
                    ),
                    YearlyCalculationTypes::MinTemperature => ReadingType::Temperature(
                        reading_iter
                            .filter(|reading| reading.min_temperature.is_some())
                            .map(|reading| (reading.date.clone(), reading.min_temperature.unwrap()))
                            .min_by_key(|date_temperature| date_temperature.1)
                            .unwrap(),
                    ),
                    YearlyCalculationTypes::MaxHumidity => ReadingType::Humidity(
                        reading
                            .iter()
                            .filter(|reading| reading.max_humidity.is_some())
                            .map(|reading| (reading.date.clone(), reading.max_humidity.unwrap()))
                            .max_by_key(|date_temperature| date_temperature.1)
                            .unwrap(),
                    ),
                })
            }
        });

        let readings_iter = readings.into_iter();

        if readings_iter.len() == 0 {
            return Err("No reading found against the provided year");
        }

        match action {
            YearlyCalculationTypes::MinTemperature => Ok(readings_iter
                .min_by_key(|reading| reading.temperature().unwrap().1)
                .unwrap()),
            YearlyCalculationTypes::MaxTemperature => Ok(readings_iter
                .max_by_key(|reading| reading.temperature().unwrap().1)
                .unwrap()),
            YearlyCalculationTypes::MaxHumidity => Ok(readings_iter
                .max_by_key(|reading| reading.humidity().unwrap().1)
                .unwrap()),
        }
    }
}

impl fmt::Display for YearlyCalculation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (date_for_highest_temperature, highest_temperature) =
            &self.highest_temperature_with_date;
        let (date_for_lowest_temperature, lowest_temperature) = &self.lowest_temperature_with_date;
        let (date_for_max_humidity, max_humidity) = &self.max_humidity_with_date;

        let date_format = "%b %d";

        let result = format!(
            "Highest: {}C on {}\nLowest: {}C on {}\nHumid: {}% on {}",
            highest_temperature,
            date_for_highest_temperature.format(date_format),
            lowest_temperature,
            date_for_lowest_temperature.format(date_format),
            max_humidity,
            date_for_max_humidity.format(date_format)
        );

        write!(f, "{}", result)
    }
}

pub struct MonthlyCalculation {
    highest_mean_temperature: u32,
    lowest_mean_temperature: u32,
    average_humidity: i32,
    readings_for_chart: Vec<(NaiveDate, u32, u32)>, // (day, min temp, max temp),
}

impl MonthlyCalculation {
    pub fn calculate(
        year: i32,
        month: i32,
        yearly_readings_map: &HashMap<(i32, i32), Vec<DailyReading>>,
    ) -> Result<Self, &str> {
        if let Some(readings) = yearly_readings_map.get(&(year, month)) {
            let mut readings_for_chart: Vec<(NaiveDate, u32, u32)> = vec![];

            let highest_mean_temperature =
                Self::find(MonthlyCalculationTypes::MaxMeanTemperature, readings);

            let lowest_mean_temperature =
                Self::find(MonthlyCalculationTypes::MinMeanTemperature, readings);

            let average_humidity = Self::find(MonthlyCalculationTypes::MeanHumidity, readings);

            readings.iter().for_each(|reading| {
                if reading.max_temperature.is_some() && reading.min_temperature.is_some() {
                    readings_for_chart.push((
                        reading.date,
                        reading.max_temperature.unwrap(),
                        reading.min_temperature.unwrap(),
                    ))
                }
            });

            Ok(Self {
                highest_mean_temperature,
                lowest_mean_temperature,
                readings_for_chart,
                average_humidity: average_humidity as i32,
            })
        } else {
            return Err("No readings found against the provided date");
        }
    }

    fn find(action: MonthlyCalculationTypes, daily_readings_for_month: &Vec<DailyReading>) -> u32 {
        let daily_readings_for_month_iter = daily_readings_for_month.iter();
        let readings_count = daily_readings_for_month_iter.len() as i32;

        if MonthlyCalculationTypes::MeanHumidity == action {
            return (daily_readings_for_month_iter
                .filter(|reading| reading.mean_humidity.is_some())
                .map(|reading| reading.mean_humidity.unwrap())
                .sum::<i32>()
                / readings_count) as u32;
        } else {
            let mean_temperature_readings = daily_readings_for_month_iter
                .filter(|reading| reading.mean_temperature.is_some())
                .map(|reading| reading.mean_temperature.unwrap());

            if MonthlyCalculationTypes::MaxMeanTemperature == action {
                return mean_temperature_readings.max().unwrap();
            }

            if MonthlyCalculationTypes::MinMeanTemperature == action {
                return mean_temperature_readings.min().unwrap();
            }
        }

        return 0 as u32;
    }

    pub fn print_chart(&self) {
        for (index, (date, max_temp, min_temp)) in self.readings_for_chart.iter().enumerate() {
            if index == 0 {
                println!("{}", &date.format("%B %Y").to_string())
            }

            Self::print_temperature_bar(max_temp, date, "red");
            Self::print_temperature_bar(min_temp, date, "blue");
        }
    }

    fn print_temperature_bar(temperature: &u32, date: &NaiveDate, color: &str) {
        for i in 0..(*temperature as i32) {
            if i == 0 {
                print!("{} ", date.day0() + 1);
            }

            if color == "red" {
                print!("{}", "+".red());
            } else {
                print!("{}", "+".blue());
            }
        }

        println!(" {}C", temperature);
    }
}

impl fmt::Display for MonthlyCalculation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = format!(
            "Highest Average: {}C\nLowest Average: {}C\nAverage Humidity: {}%",
            &self.highest_mean_temperature, &self.lowest_mean_temperature, &self.average_humidity,
        );
        write!(f, "{}", result)
    }
}
