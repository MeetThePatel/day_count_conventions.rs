use std::fmt::Display;

use crate::DayCounter;

use chrono::NaiveDate;

/// Actual/364
///
/// $$
/// \frac{d_2 - d_1}{364}
/// $$
///
/// where $d_2 - d_1$ is the number of days between the two dates.

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Actual364;

impl DayCounter for Actual364 {
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> f64 {
        (*end - *start).num_days() as f64 / 364.0
    }
}

impl Display for Actual364 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actual/364")
    }
}
