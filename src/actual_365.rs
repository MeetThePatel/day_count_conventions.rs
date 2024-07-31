use std::fmt::Display;

use crate::{is_feb29_between_exc_inc, DayCounter};

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
    fn day_count_fraction(&self, start: &chrono::NaiveDate, end: &chrono::NaiveDate) -> f64 {
        (*end - *start).num_days() as f64 / 365.0
    }
}

impl Display for Actual365Fixed {
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
    fn day_count_fraction(&self, start: &chrono::NaiveDate, end: &chrono::NaiveDate) -> f64 {
        let denominator = if is_feb29_between_exc_inc(*start, *end) {
            366
        } else {
            365
        };
        (*end - *start).num_days() as f64 / f64::from(denominator)
    }
}

impl Display for Actual365A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actual/365 (A)")
    }
}
