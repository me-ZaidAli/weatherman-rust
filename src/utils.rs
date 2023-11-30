use chrono::NaiveDate;

pub fn date_from(date_string: &str) -> NaiveDate {
    let date_split: Vec<i32> = date_string
        .split("-")
        .map(|word| word.parse::<i32>().unwrap())
        .collect();

    let year = *date_split.get(0).unwrap_or(&0);
    let month = *date_split.get(1).unwrap_or(&1) as u32;
    let day = *date_split.get(2).unwrap_or(&1) as u32;

    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

pub fn parse_date_from(date_str: &str) -> (u16, u16) {
    let mut split_date = date_str.split("/");
    (
        split_date.next().unwrap().parse::<u16>().unwrap(),
        split_date.next().unwrap().parse::<u16>().unwrap(),
    )
}
