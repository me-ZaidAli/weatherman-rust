use clap::Parser;
use reader::read_dir;
use std::error::Error;

use crate::utils::parse_date_from;

mod calculate;
mod reader;
mod utils;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[arg(short = 'e')]
    /// for a given year display the highest temperature and day, lowest temperature and day, most humid day and humidity
    pub year: Option<String>,

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
        let parsed_year = year.parse::<i32>()?;
        println!(
            "{}",
            calculate::YearlyCalculation::calculate(parsed_year, &readings)?
        )
    }

    if let Some(year_with_month) = &args.year_with_month {
        let (year, month) = parse_date_from(year_with_month);

        println!(
            "{}",
            calculate::MonthlyCalculation::calculate(year, month, &readings)?
        );
    }

    if let Some(year_with_month_for_chart) = &args.year_with_month_for_chart {
        let (year, month) = parse_date_from(year_with_month_for_chart);

        let monthly_calculations =
            calculate::MonthlyCalculation::calculate(year, month, &readings)?;

        monthly_calculations.print_chart();
    }

    Ok(())
}
