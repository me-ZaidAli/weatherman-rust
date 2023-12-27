use crate::{reader::reading::DailyTemperatureReading, utils::parse_date_from};
use chrono::{Datelike, NaiveDate};
use clap::Parser;
use colored::Colorize;
use reader::read_dir;
use std::error::Error;

mod calculate;
mod reader;
mod utils;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[arg(short = 'e',value_parser = validate_year )]
    /// for a given year display the highest temperature and day, lowest temperature and day, most humid day and humidity
    pub year: Option<u16>,

    #[arg(short = 'a', value_parser = validate_date)]
    /// for a given month display the average highest temperature, average lowest temperature, average humidity
    pub year_with_month: Option<String>,

    #[arg(short = 'c', value_parser = validate_date)]
    /// for a given month draw two horizontal bar charts on the console
    /// for the highest and lowest temperature on each day.
    /// Highest in red and lowest in blue.
    pub year_with_month_for_chart: Option<String>,

    #[arg(last = true)]
    path: String,
}

fn validate_year(year_str: &str) -> Result<u16, String> {
    for ch in year_str.to_string().chars() {
        if !ch.is_numeric() || year_str.len() != 4 {
            return Err("Please provide year in format YYYY".red().to_string());
        }
    }

    Ok(year_str.parse::<u16>().unwrap())
}

pub fn validate_date(year_with_month: &str) -> Result<String, String> {
    let full_date = format!("{}/01", year_with_month);

    if NaiveDate::parse_from_str(&full_date, "%Y/%m/%d").is_ok() {
        Ok(year_with_month.to_string())
    } else {
        Err("Provided date should be in YYYY/MM format"
            .red()
            .to_string())
    }
}

pub fn run(args: &Arguments) -> Result<(), Box<dyn Error>> {
    let readings = read_dir(&args.path)?;

    if let Some(year) = args.year {
        let yearly_readings = readings
            .iter()
            .filter(|reading| reading.date.year() as u16 == year)
            .cloned()
            .collect::<Vec<DailyTemperatureReading>>();

        println!(
            "{}",
            calculate::YearlyCalculation::calculate(&yearly_readings)
        )
    }

    if let Some(year_with_month) = &args.year_with_month {
        let (year, month) = parse_date_from(year_with_month);

        let monthly_readings = readings
            .iter()
            .filter(|reading| {
                reading.date.year() as u16 == year && reading.date.month() as u16 == month
            })
            .cloned()
            .collect::<Vec<DailyTemperatureReading>>();

        println!(
            "{}",
            calculate::MonthlyCalculation::calculate(&monthly_readings)
        );
    }

    if let Some(year_with_month_for_chart) = &args.year_with_month_for_chart {
        let (year, month) = parse_date_from(year_with_month_for_chart);

        let monthly_readings = readings
            .iter()
            .filter(|reading| {
                reading.date.year() as u16 == year && reading.date.month() as u16 == month
            })
            .cloned()
            .collect::<Vec<DailyTemperatureReading>>();

        let monthly_calculations = calculate::MonthlyCalculation::calculate(&monthly_readings);

        monthly_calculations.print_chart();
    }

    Ok(())
}
