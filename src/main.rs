use chrono::NaiveDate;
use clap::Parser;
use colored::Colorize;
use std::process;
use weatherman::{run, Arguments};

fn validate_format_for(date: &str) {
    let full_date = format!("{}/01", date);

    NaiveDate::parse_from_str(&full_date, "%Y/%m/%d")
        .expect(&"Provided date should be in YYYY/MM format".red().to_string());
}

pub fn validate(input: &Arguments) {
    if let Some(year) = &input.year {
        let year_str = year.to_string();

        for ch in year_str.chars() {
            if !ch.is_numeric() || year_str.len() < 4 {
                println!("{}", "Please provide year in format YYYY".red());
                process::exit(0)
            }
        }
    }
    if let Some(year_month) = &input.year_with_month {
        validate_format_for(&year_month);
    }

    if let Some(year_month) = &input.year_with_month_for_chart {
        validate_format_for(&year_month);
    }
}

fn main() {
    let args: Arguments = Arguments::parse();

    validate(&args);

    if let Err(e) = run(&args) {
        eprintln!("Application error: {}", e)
    }
}
