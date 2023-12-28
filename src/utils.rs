pub fn parse_date_from(date_str: &str) -> (i32, u32) {
    let mut split_date = date_str.split("/");
    (
        split_date.next().unwrap().parse::<i32>().unwrap(),
        split_date.next().unwrap().parse::<u32>().unwrap(),
    )
}
