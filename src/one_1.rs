use crate::{DayCountFraction, DayCounter};

#[cfg(not(feature = "hifitime"))]
use chrono::NaiveDate;
#[cfg(feature = "hifitime")]
use hifitime::Epoch;

/// 1/1
///
/// This is always equal to 1.

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct OneOne;

impl DayCounter for OneOne {
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(&self, _start: &NaiveDate, _end: &NaiveDate) -> DayCountFraction<Self> {
        DayCountFraction::new(1.0)
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, _start: &Epoch, _end: &Epoch) -> DayCountFraction<Self> {
        DayCountFraction::new(1.0)
    }
}

impl std::fmt::Display for OneOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1/1")
    }
}
