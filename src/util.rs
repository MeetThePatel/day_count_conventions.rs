#[cfg(not(feature = "hifitime"))]
use chrono::{Datelike, NaiveDate};
#[cfg(feature = "hifitime")]
use hifitime::{Epoch, Unit};

#[cfg(not(feature = "hifitime"))]
pub fn get_last_day_of_month(year: i32, month: u32) -> i32 {
    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };

    next_month.pred_opt().unwrap().day() as i32
}
#[cfg(feature = "hifitime")]
pub fn get_last_day_of_month(year: i32, month: u32) -> i32 {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    #[allow(clippy::cast_possible_truncation)]
    let next_month_epoch = Epoch::from_gregorian_utc_at_midnight(next_year, next_month as u8, 1);
    let last_day_epoch = next_month_epoch - Unit::Day;
    let (_, _, last_day, _, _, _, _) = last_day_epoch.to_gregorian_utc();
    last_day.into()
}

#[cfg(not(feature = "hifitime"))]
pub fn is_last_day_of_feb(date: NaiveDate) -> bool {
    if date.month() == 2 {
        get_last_day_of_month(date.year(), date.month()) == date.day() as i32
    } else {
        false
    }
}
#[cfg(feature = "hifitime")]
pub fn is_last_day_of_feb(date: Epoch) -> bool {
    let (year, month, day, _, _, _, _) = date.to_gregorian_utc();
    if month == 2 {
        get_last_day_of_month(year, u32::from(month)) == i32::from(day)
    } else {
        false
    }
}

#[cfg(not(feature = "hifitime"))]
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
#[cfg(feature = "hifitime")]
pub fn is_feb29_between_exc_inc(date1: Epoch, date2: Epoch) -> bool {
    let (mut current_year, _, _, _, _, _, _) = date1.to_gregorian_utc();
    let (end_year, _, _, _, _, _, _) = date1.to_gregorian_utc();

    while current_year <= end_year {
        if let Ok(feb_29) = Epoch::maybe_from_gregorian_utc(current_year, 2, 29, 0, 0, 0, 0) {
            if feb_29 > date1 && feb_29 <= date2 {
                return true;
            }
        }
        current_year += 1;
    }
    false
}
