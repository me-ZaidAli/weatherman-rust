use chrono::NaiveDate;
use serde::Deserialize;
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]

pub struct DailyTemperatureReading {
    #[serde(rename = "PKT")]
    pub date: NaiveDate,
    #[serde(deserialize_with = "csv::invalid_option", rename = "Max TemperatureC")]
    pub max_temperature: Option<i8>,
    #[serde(deserialize_with = "csv::invalid_option", rename = "Mean TemperatureC")]
    pub mean_temperature: Option<i8>,
    #[serde(deserialize_with = "csv::invalid_option", rename = "Min TemperatureC")]
    pub min_temperature: Option<i8>,
    #[serde(deserialize_with = "csv::invalid_option", rename = "Max Humidity")]
    pub max_humidity: Option<u8>,
    #[serde(deserialize_with = "csv::invalid_option", rename = "Mean Humidity")]
    pub mean_humidity: Option<u8>,
    #[serde(deserialize_with = "csv::invalid_option", rename = "Min Humidity")]
    pub min_humidity: Option<u8>,
}
