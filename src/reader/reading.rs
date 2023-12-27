use chrono::NaiveDate;
use serde::Deserialize;
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]

pub struct DailyTemperatureReading {
    #[serde(rename = "PKT")]
    pub date: NaiveDate,
    #[serde(rename = "Max TemperatureC")]
    pub max_temperature: i8,
    #[serde(rename = "Mean TemperatureC")]
    pub mean_temperature: i8,
    #[serde(rename = "Min TemperatureC")]
    pub min_temperature: i8,
    #[serde(rename = "Max Humidity")]
    pub max_humidity: u8,
    #[serde(rename = "Mean Humidity")]
    pub mean_humidity: u8,
    #[serde(rename = "Min Humidity")]
    pub min_humidity: u8,
}
