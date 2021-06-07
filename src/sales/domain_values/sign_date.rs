use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct SignDate(DateTime<Utc>);

impl SignDate {
    pub fn from_date_time(date_time: DateTime<Utc>) -> Self {
        SignDate(date_time)
    }

    pub fn from_year_month_day(year: i32, month: u32, day_of_month: u32) -> Self {
        SignDate(DateTime::from_utc(NaiveDate::from_ymd(year, month, day_of_month).and_hms(0, 0, 0), Utc))
    }
}