use std::fmt::Display;

use crate::{DayCountFraction, DayCounter};

use chrono::NaiveDate;

/// Actual/365.25
///
/// $$
/// \frac{d_2 - d_1}{365.25}
/// $$
///
/// where $d_2 - d_1$ is the number of days between the two dates (excluding
/// the last day).

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Actual36525;

impl DayCounter for Actual36525 {
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        DayCountFraction::new((*end - *start).num_days() as f64 / 365.25)
    }
}

impl Display for Actual36525 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actual/365.25")
    }
}

/// Actual/365.25 (inc)
///
/// Includes the last day of the period.
///
/// $$
/// \frac{d_2 - d_1 + 1}{365.25}
/// $$
///
/// where $d_2 - d_1$ is the number of days between the two dates.

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Actual36525Inc;

impl DayCounter for Actual36525Inc {
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        DayCountFraction::new(((*end - *start).num_days() + 1) as f64 / 365.25)
    }
}

impl Display for Actual36525Inc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actual/365.25 (inc)")
    }
}
