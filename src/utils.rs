pub fn parse_date_from(date_str: &str) -> (u16, u16) {
    let mut split_date = date_str.split("/");
    (
        split_date.next().unwrap().parse::<u16>().unwrap(),
        split_date.next().unwrap().parse::<u16>().unwrap(),
    )
}
