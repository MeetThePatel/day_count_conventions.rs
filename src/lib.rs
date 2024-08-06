//! # Day Count Conventions
//!
//! This library supplies common day count conventions for financial applications.
//!
//! Currently supported day count conventions are:
//! - [Actual/360](Actual360)
//! - [Actual/360 (inc)](Actual360Inc)
//! - [Actual/364](Actual364)
//! - [Actual/365 (A)](Actual365A)
//! - [Actual/365 (Fixed)](Actual365Fixed)
//! - [Actual/366](Actual366)
//! - [Actual/366 (inc)](Actual366Inc)
//! - [Actual/365.25](Actual36525)
//! - [Actual/365.25 (inc)](Actual36525Inc)
//! - [NL/365](NL365)
//! - [1/1](OneOne)
//! - [30/360](Thirty360)
//! - [30E/360](ThirtyE360)
//! - [30E/360 (ISDA)](ThirtyE360ISDA)
//! - [30E+/360 (ISDA)](ThirtyEPlus360ISDA)
//!
//! If there are any conventions that you would like implemented, don't
//! hestitate to submit a PR or raise in issue on [GitHub](https://github.com/MeetThePatel/day_count_conventions.rs)!
//!
//! ### References:
//! **Note:** The following sources may have slightly different definitions. As
//! a precaution, please see the documentations for the particular definitions
//! used in this package.
//!
//! - 1. [OpenGamma (Chapter 3)](https://quant.opengamma.io/Interest-Rate-Instruments-and-Market-Conventions.pdf)
//! - 2. [Wikipedia](https://en.wikipedia.org/wiki/Day_count_convention)
//! - 3. [2006 ISDA Definitions](https://www.isda.org/book/2006-isda-definitions/)
//! - 4. [QuantLib](https://github.com/lballabio/QuantLib/tree/master/ql/time/daycounters)
//! - 5. [DeltaQuants](http://www.deltaquants.com/day-count-conventions)

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

/// The [`DayCounter`] trait represents any day count conventions. Only one method
/// is required: [`day_count_function`](DayCounter::day_count_fraction).
pub trait DayCounter: std::fmt::Display + Copy + Clone {
    /// Given a two dates, calculate the day-count-fraction between the two dates.
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(
        &self,
        start: &chrono::NaiveDate,
        end: &chrono::NaiveDate,
    ) -> DayCountFraction<Self>
    where
        Self: Sized;
    /// Given a two dates, calculate the day-count-fraction between the two dates.
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(
        &self,
        start: &hifitime::Epoch,
        end: &hifitime::Epoch,
    ) -> DayCountFraction<Self>
    where
        Self: Sized;
}

/// Wrapper type for a day count fraction that contains information on which
/// counter was used to generate the fraction.
pub struct DayCountFraction<D>
where
    D: DayCounter,
{
    fraction: f64,
    _marker: std::marker::PhantomData<D>,
}
impl<D> DayCountFraction<D>
where
    D: DayCounter,
{
    /// Create a new DayCountFraction with convention D.
    #[must_use]
    #[inline]
    pub const fn new(fraction: f64) -> Self {
        Self {
            fraction,
            _marker: std::marker::PhantomData,
        }
    }

    /// Access the fraction
    #[must_use]
    #[inline]
    pub const fn get_fraction(&self) -> f64 {
        self.fraction
    }
}
impl<D> PartialEq for DayCountFraction<D>
where
    D: DayCounter,
{
    fn eq(&self, other: &Self) -> bool {
        self.fraction == other.fraction
    }
}

mod actual_360;
pub use actual_360::{Actual360, Actual360Inc};

mod actual_364;
pub use actual_364::Actual364;

mod actual_365;
pub use actual_365::{Actual365A, Actual365Fixed};

mod actual_366;
pub use actual_366::{Actual366, Actual366Inc};

mod actual_365_25;
pub use actual_365_25::{Actual36525, Actual36525Inc};

mod nl_365;
pub use nl_365::NL365;

mod one_1;
pub use one_1::OneOne;

mod thirty_360;
pub use thirty_360::{Thirty360, ThirtyE360, ThirtyE360ISDA, ThirtyEPlus360ISDA};

mod util;
pub(crate) use util::{get_last_day_of_month, is_feb29_between_exc_inc, is_last_day_of_feb};

#[cfg(test)]
mod tests {
    use crate::{DayCounter, OneOne, NL365};

    #[cfg(not(feature = "hifitime"))]
    use chrono::NaiveDate;
    #[cfg(feature = "hifitime")]
    use hifitime::Epoch;

    #[cfg(not(feature = "hifitime"))]
    #[test]
    fn comparison() {
        let day1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let day2 = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();

        let dc1 = NL365::default();
        let dc2 = OneOne::default();

        let yf1 = dc1.day_count_fraction(&day1, &day2);
        let yf2 = dc2.day_count_fraction(&day1, &day2);

        // The following line of code should not compile, as they come from
        // two different day count conventions.
        // yf1.eq(yf2);

        // However, should it be the case that you *really* need to compare two
        // year fractions of different conventions, you can always get the
        // fraction.
        assert_ne!(yf1.fraction, yf2.fraction);
    }
    #[cfg(feature = "hifitime")]
    #[test]
    fn comparison() {
        let day1 = Epoch::from_gregorian_utc_at_midnight(2024, 1, 1);
        let day2 = Epoch::from_gregorian_utc_at_midnight(2024, 7, 1);

        let dc1 = NL365;
        let dc2 = OneOne;

        let yf1 = dc1.day_count_fraction(&day1, &day2);
        let yf2 = dc2.day_count_fraction(&day1, &day2);

        // The following line of code should not compile, as they come from
        // two different day count conventions.
        // yf1.eq(yf2);

        // However, should it be the case that you *really* need to compare two
        // year fractions of different conventions, you can always get the
        // fraction.
        assert_ne!(yf1.fraction, yf2.fraction);
    }
}
