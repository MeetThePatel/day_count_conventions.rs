use std::fmt::Display;

use crate::DayCounter;

use chrono::NaiveDate;

/// 1/1
///
/// This is always equal to 1.

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct OneOne;

impl DayCounter for OneOne {
    fn day_count_fraction(&self, _start: &NaiveDate, _end: &NaiveDate) -> f64 {
        1.0
    }
}

impl Display for OneOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1/1")
    }
}
