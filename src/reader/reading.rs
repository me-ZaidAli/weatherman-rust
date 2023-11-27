use chrono::NaiveDate;

#[derive(Debug)]
pub struct DailyReading {
    pub date: NaiveDate,
    pub max_temperature: Option<u32>,
    pub min_temperature: Option<u32>,
    pub mean_temperature: Option<u32>,
    pub max_humidity: Option<i32>,
    pub min_humidity: Option<i32>,
    pub mean_humidity: Option<i32>,
}

impl DailyReading {
    pub fn new(
        date: NaiveDate,
        max_temperature: Option<u32>,
        min_temperature: Option<u32>,
        mean_temperature: Option<u32>,
        max_humidity: Option<i32>,
        min_humidity: Option<i32>,
        mean_humidity: Option<i32>,
    ) -> Self {
        Self {
            date,
            max_temperature,
            min_temperature,
            mean_temperature,
            max_humidity,
            min_humidity,
            mean_humidity,
        }
    }
}
