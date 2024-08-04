use std::fmt::Display;

use crate::{DayCountFraction, DayCounter};

use chrono::NaiveDate;

/// Actual/366
///
/// $$
/// \frac{d_2 - d_1}{366}
/// $$
///
/// where $d_2 - d_1$ is the number of days between the two dates (excluding
/// the last day).

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Actual366;

impl DayCounter for Actual366 {
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        DayCountFraction::new((*end - *start).num_days() as f64 / 366.0)
    }
}

impl Display for Actual366 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actual/366")
    }
}

/// Actual/366 (inc)
///
/// Includes the last day of the period.
///
/// $$
/// \frac{d_2 - d_1 + 1}{366}
/// $$
///
/// where $d_2 - d_1$ is the number of days between the two dates.

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Actual366Inc;

impl DayCounter for Actual366Inc {
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        DayCountFraction::new(((*end - *start).num_days() + 1) as f64 / 366.0)
    }
}

impl Display for Actual366Inc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actual/366 (inc)")
    }
}
