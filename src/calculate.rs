use crate::reader::reading::DailyTemperatureReading;
use chrono::{Datelike, NaiveDate};
use colored::Colorize;
use std::fmt;

pub struct YearlyCalculation {
    highest_temperature_with_date: (NaiveDate, i8),
    lowest_temperature_with_date: (NaiveDate, i8),
    max_humidity_with_date: (NaiveDate, u8),
}

impl YearlyCalculation {
    pub fn calculate(monthly_readings: &Vec<DailyTemperatureReading>) -> Self {
        Self {
            highest_temperature_with_date: monthly_readings
                .iter()
                .map(|reading| (reading.date, reading.max_temperature))
                .max_by_key(|reading| reading.1)
                .unwrap(),
            lowest_temperature_with_date: monthly_readings
                .iter()
                .map(|reading| (reading.date, reading.min_temperature))
                .max_by_key(|reading| reading.1)
                .unwrap(),
            max_humidity_with_date: monthly_readings
                .iter()
                .map(|reading| (reading.date, reading.max_humidity))
                .max_by_key(|reading| reading.1)
                .unwrap(),
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
            date_for_max_humidity.format(date_format),
        );

        write!(f, "{}", result)
    }
}

pub struct MonthlyCalculation {
    highest_mean_temperature: i8,
    lowest_mean_temperature: i8,
    average_humidity: u8,
    readings_for_chart: Vec<DailyTemperatureReading>, // (day, min temp, max temp),
}

impl MonthlyCalculation {
    pub fn calculate(daily_readings_for_month: &Vec<DailyTemperatureReading>) -> Self {
        Self {
            highest_mean_temperature: daily_readings_for_month
                .iter()
                .max_by_key(|reading| reading.mean_temperature)
                .unwrap()
                .mean_temperature,
            lowest_mean_temperature: daily_readings_for_month
                .iter()
                .min_by_key(|reading| reading.mean_temperature)
                .unwrap()
                .mean_temperature,
            average_humidity: (daily_readings_for_month
                .iter()
                .map(|reading| reading.mean_humidity as u16)
                .sum::<u16>()
                / daily_readings_for_month.len() as u16) as u8,
            readings_for_chart: daily_readings_for_month
                .iter()
                .cloned()
                .collect::<Vec<DailyTemperatureReading>>(),
        }
    }

    pub fn print_chart(&self) {
        println!(
            "{}",
            self.readings_for_chart.get(0).unwrap().date.format("%B %Y")
        );

        for reading in self.readings_for_chart.iter() {
            let day_number = reading.date.day0() + 1;

            Self::print_temperature_bar(reading.max_temperature, day_number, "red");
            Self::print_temperature_bar(reading.min_temperature, day_number, "blue");
        }
    }

    fn print_temperature_bar(temperature: i8, day_number: u32, color: &str) {
        let temperature_bar = "+".repeat(temperature as usize);

        println!(
            "{day_number} {bar} {temperature}C",
            bar = if color == "red" {
                temperature_bar.red()
            } else {
                temperature_bar.blue()
            }
        );
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
