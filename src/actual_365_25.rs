use crate::{DayCountFraction, DayCounter};

#[cfg(not(feature = "hifitime"))]
use chrono::NaiveDate;
#[cfg(feature = "hifitime")]
use hifitime::{Epoch, Unit};

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
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        DayCountFraction::new((*end - *start).num_days() as f64 / 365.25)
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, start: &Epoch, end: &Epoch) -> DayCountFraction<Self> {
        DayCountFraction::new((*end - *start).to_unit(Unit::Day) / 365.25)
    }
}

impl std::fmt::Display for Actual36525 {
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
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        DayCountFraction::new(((*end - *start).num_days() + 1) as f64 / 365.25)
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, start: &Epoch, end: &Epoch) -> DayCountFraction<Self> {
        DayCountFraction::new(((*end - *start).to_unit(Unit::Day) + 1.0) / 365.25)
    }
}

impl std::fmt::Display for Actual36525Inc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actual/365.25 (inc)")
    }
}
