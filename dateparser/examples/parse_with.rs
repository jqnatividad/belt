use chrono::{
    naive::NaiveTime,
    offset::{Local, Utc},
};
use qsv_dateparser::parse_with;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed_in_local = parse_with("2021-10-09", &Local, NaiveTime::from_hms(0, 0, 0))?;
    println!("{:#?}", parsed_in_local);

    let parsed_in_utc = parse_with("2021-10-09", &Utc, NaiveTime::from_hms(0, 0, 0))?;
    println!("{:#?}", parsed_in_utc);

    Ok(())
}
