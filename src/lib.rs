use clap::Parser;
use reader::read_dir;
use std::error::Error;

use crate::{reader::reading::DailyTemperatureReading, utils::parse_date_from};

mod calculate;
mod reader;
mod utils;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[arg(short = 'e')]
    /// for a given year display the highest temperature and day, lowest temperature and day, most humid day and humidity
    pub year: Option<u16>,

    #[arg(short = 'a')]
    /// for a given month display the average highest temperature, average lowest temperature, average humidity
    pub year_with_month: Option<String>,

    #[arg(short = 'c')]
    /// for a given month draw two horizontal bar charts on the console
    /// for the highest and lowest temperature on each day.
    /// Highest in red and lowest in blue.
    pub year_with_month_for_chart: Option<String>,

    #[arg(last = true)]
    path: String,
}

pub fn run(args: &Arguments) -> Result<(), Box<dyn Error>> {
    let readings = read_dir(&args.path)?;

    if let Some(year) = &args.year {
        let monthly_readings_map = readings.get(year).unwrap().to_owned();

        let monthly_readings = monthly_readings_map
            .into_values()
            .into_iter()
            .flatten()
            .collect::<Vec<DailyTemperatureReading>>();

        println!(
            "{}",
            calculate::YearlyCalculation::calculate(monthly_readings)
        )
    }

    if let Some(year_with_month) = &args.year_with_month {
        let (year, month) = parse_date_from(year_with_month);

        let daily_readings_for_month = readings.get(&year).unwrap().get(&month).unwrap().to_owned();

        println!(
            "{}",
            calculate::MonthlyCalculation::calculate(daily_readings_for_month)
        );
    }

    if let Some(year_with_month_for_chart) = &args.year_with_month_for_chart {
        let (year, month) = parse_date_from(year_with_month_for_chart);

        let daily_readings_for_month = readings.get(&year).unwrap().get(&month).unwrap().to_owned();

        let monthly_calculations =
            calculate::MonthlyCalculation::calculate(daily_readings_for_month);

        monthly_calculations.print_chart();
    }

    Ok(())
}
