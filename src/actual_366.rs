use crate::{DayCountFraction, DayCounter};

#[cfg(not(feature = "hifitime"))]
use chrono::NaiveDate;
#[cfg(feature = "hifitime")]
use hifitime::{Epoch, Unit};

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
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        DayCountFraction::new((*end - *start).num_days() as f64 / 366.0)
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, start: &Epoch, end: &Epoch) -> DayCountFraction<Self> {
        DayCountFraction::new((*end - *start).to_unit(Unit::Day) / 366.0)
    }
}

impl std::fmt::Display for Actual366 {
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
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        DayCountFraction::new(((*end - *start).num_days() + 1) as f64 / 366.0)
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, start: &Epoch, end: &Epoch) -> DayCountFraction<Self> {
        DayCountFraction::new(((*end - *start).to_unit(Unit::Day) + 1.0) / 366.0)
    }
}

impl std::fmt::Display for Actual366Inc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actual/366 (inc)")
    }
}
