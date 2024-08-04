use std::fmt::Display;

use crate::{is_feb29_between_exc_inc, DayCountFraction, DayCounter};

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
    fn day_count_fraction(
        &self,
        start: &chrono::NaiveDate,
        end: &chrono::NaiveDate,
    ) -> DayCountFraction<Self> {
        let mut numerator = (*end - *start).num_days();

        if is_feb29_between_exc_inc(*start, *end) {
            numerator -= 1;
        }

        DayCountFraction::new(numerator as f64 / 365.0)
    }
}

impl Display for NL365 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NL/365")
    }
}
