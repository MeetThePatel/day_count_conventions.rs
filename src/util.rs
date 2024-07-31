use chrono::{Datelike, NaiveDate};

pub fn get_last_day_of_month(year: i32, month: u32) -> i32 {
    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };

    next_month.pred_opt().unwrap().day() as i32
}

pub fn is_last_day_of_feb(date: NaiveDate) -> bool {
    if date.month() == 2 {
        get_last_day_of_month(date.year(), date.month()) == date.day() as i32
    } else {
        false
    }
}

pub fn is_feb29_between_exc_inc(date1: NaiveDate, date2: NaiveDate) -> bool {
    let mut current_year = date1.year();

    while current_year <= date2.year() {
        if NaiveDate::from_ymd_opt(current_year, 2, 29).is_some() {
            let feb_29 = NaiveDate::from_ymd_opt(current_year, 2, 29).unwrap();
            if feb_29 > date1 && feb_29 <= date2 {
                return true;
            }
        }
        current_year += 1;
    }
    false
}
