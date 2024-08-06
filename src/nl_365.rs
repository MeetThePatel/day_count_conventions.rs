use crate::{is_feb29_between_exc_inc, DayCountFraction, DayCounter};

#[cfg(not(feature = "hifitime"))]
use chrono::NaiveDate;
#[cfg(feature = "hifitime")]
use hifitime::{Epoch, Unit};

/// NL/365
///
/// $$
/// \frac{\text{Numerator}}{365}
/// $$
///
/// where $\text{Numerator}$ is $d_2 - d_1 - 1$ is February 29th
/// is between $d_1$ (exclusive) and $d_2$ (inclusive), and
/// $d_2 - d_1$ otherwise.

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct NL365;

impl DayCounter for NL365 {
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        let mut numerator = (*end - *start).num_days();

        if is_feb29_between_exc_inc(*start, *end) {
            numerator -= 1;
        }

        DayCountFraction::new(numerator as f64 / 365.0)
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, start: &Epoch, end: &Epoch) -> DayCountFraction<Self> {
        let mut numerator = (*end - *start).to_unit(Unit::Day);

        if is_feb29_between_exc_inc(*start, *end) {
            numerator -= 1.0;
        }

        DayCountFraction::new(numerator / 365.0)
    }
}

impl std::fmt::Display for NL365 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NL/365")
    }
}
