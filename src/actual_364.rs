use crate::{DayCountFraction, DayCounter};

#[cfg(not(feature = "hifitime"))]
use chrono::NaiveDate;
#[cfg(feature = "hifitime")]
use hifitime::{Epoch, Unit};

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
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        DayCountFraction::new((*end - *start).num_days() as f64 / 364.0)
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, start: &Epoch, end: &Epoch) -> DayCountFraction<Self>
    where
        Self: Sized,
    {
        DayCountFraction::new((*end - *start).to_unit(Unit::Day) / 364.0)
    }
}

impl std::fmt::Display for Actual364 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actual/364")
    }
}
