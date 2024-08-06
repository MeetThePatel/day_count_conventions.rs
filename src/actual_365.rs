use crate::{is_feb29_between_exc_inc, DayCountFraction, DayCounter};

#[cfg(not(feature = "hifitime"))]
use chrono::NaiveDate;
#[cfg(feature = "hifitime")]
use hifitime::{Epoch, Unit};

/// Actual/365 (Fixed)
///
/// $$
/// \frac{d_2 - d_1}{365}
/// $$
///
/// where $d_2 - d_1$ is the number of days between the two dates.

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Actual365Fixed;

impl DayCounter for Actual365Fixed {
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        DayCountFraction::new((*end - *start).num_days() as f64 / 365.0)
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, start: &Epoch, end: &Epoch) -> DayCountFraction<Self> {
        DayCountFraction::new((*end - *start).to_unit(Unit::Day) / 365.0)
    }
}

impl std::fmt::Display for Actual365Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actual/365 (Fixed)")
    }
}

/// Actual/365 (A)
///
/// $$
/// \frac{d_2 - d_1}{\text{Denominator}}
/// $$
///
/// where $d_2 - d_1$ is the number of days between the two dates
/// and $\text{Denominator}$ is 366 if February 29th is between
/// $d_1$ (exclusive) and $d_2$ (inclusive), and 365 otherwise.

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Actual365A;

impl DayCounter for Actual365A {
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(
        &self,
        start: &chrono::NaiveDate,
        end: &chrono::NaiveDate,
    ) -> DayCountFraction<Self> {
        let denominator = if is_feb29_between_exc_inc(*start, *end) {
            366
        } else {
            365
        };
        DayCountFraction::new((*end - *start).num_days() as f64 / f64::from(denominator))
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, start: &Epoch, end: &Epoch) -> DayCountFraction<Self> {
        let denominator = if is_feb29_between_exc_inc(*start, *end) {
            366
        } else {
            365
        };
        DayCountFraction::new((*end - *start).to_unit(Unit::Day) / f64::from(denominator))
    }
}

impl std::fmt::Display for Actual365A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actual/365 (A)")
    }
}
