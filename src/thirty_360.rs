use crate::{get_last_day_of_month, is_last_day_of_feb, DayCountFraction, DayCounter};

#[cfg(not(feature = "hifitime"))]
use chrono::{Datelike, NaiveDate};
#[cfg(feature = "hifitime")]
use hifitime::Epoch;

/// 30/360
///
/// Let $Y_1$ be the year of $d_1$, $M_1$ be the month of $d_1$,
/// and $D_1$ be the day of $d_1$. Similarly define these for $d_2$.
///
/// Apply the following two rules:
/// 1. If $D_1$ is 31, then change $D_1$ to 30.
/// 2. If $D_2$ is 31 and $D_1$ is either 30 or 31, then change $D_2$ to 30.
///
/// Now plug into the following to get the day count fraction.
///
/// $$
/// \frac{360 (Y_2 - Y_1) + 30 (M_2 - M_1) + (D_2 - D_1)}{360}
/// $$

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Thirty360;

impl DayCounter for Thirty360 {
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        let y1 = start.year();
        let m1 = start.month() as i32;
        let mut d1 = start.day() as i32;

        let y2 = end.year();
        let m2 = end.month() as i32;
        let mut d2 = end.day() as i32;

        if d1 == 31 {
            d1 = 30;
        }
        if d2 == 31 && d1 >= 30 {
            d2 = 30;
        }

        let numerator = 360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1);
        DayCountFraction::new(f64::from(numerator) / 360.0)
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, start: &Epoch, end: &Epoch) -> DayCountFraction<Self> {
        let (y1, m1, mut d1, _, _, _, _) = start.to_gregorian_utc();
        let (y2, m2, mut d2, _, _, _, _) = end.to_gregorian_utc();

        if d1 == 31 {
            d1 = 30;
        }
        if d2 == 31 && d1 >= 30 {
            d2 = 30;
        }

        let numerator = (360 * (y2 - y1)) + (30 * i32::from(m2 - m1)) + i32::from(d2 - d1);
        DayCountFraction::new(f64::from(numerator) / 360.0)
    }
}

impl std::fmt::Display for Thirty360 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "30/360")
    }
}

/// 30E/360
///
/// Let $Y_1$ be the year of $d_1$, $M_1$ be the month of $d_1$,
/// and $D_1$ be the day of $d_1$. Similarly define these for $d_2$.
///
/// Apply the following two rules:
/// 1. If $D_1$ is 31, then change $D_1$ to 30.
/// 2. If $D_2$ is 31, then change $D_2$ to 30.
///
/// Now plug into the following to get the day count fraction.
///
/// $$
/// \frac{360 (Y_2 - Y_1) + 30 (M_2 - M_1) + (D_2 - D_1)}{360}
/// $$

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct ThirtyE360;

impl DayCounter for ThirtyE360 {
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        let y1 = start.year();
        let m1 = start.month() as i32;
        let mut d1 = start.day() as i32;

        let y2 = end.year();
        let m2 = end.month() as i32;
        let mut d2 = end.day() as i32;

        if d1 == 31 {
            d1 = 30;
        }
        if d2 == 31 {
            d2 = 30;
        }

        let numerator = 360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1);
        DayCountFraction::new(f64::from(numerator) / 360.0)
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, start: &Epoch, end: &Epoch) -> DayCountFraction<Self> {
        let (y1, m1, mut d1, _, _, _, _) = start.to_gregorian_utc();
        let (y2, m2, mut d2, _, _, _, _) = end.to_gregorian_utc();

        if d1 == 31 {
            d1 = 30;
        }
        if d2 == 31 {
            d2 = 30;
        }

        let numerator = (360 * (y2 - y1)) + (30 * i32::from(m2 - m1)) + i32::from(d2 - d1);
        DayCountFraction::new(f64::from(numerator) / 360.0)
    }
}

impl std::fmt::Display for ThirtyE360 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "30E/360")
    }
}

/// 30E/360 (ISDA)
///
/// Let $Y_1$ be the year of $d_1$, $M_1$ be the month of $d_1$,
/// and $D_1$ be the day of $d_1$. Similarly define these for $d_2$.
///
/// Apply the following two rules:
/// 1. If $D_1$ is the last day of the month, then change $D_1$ to 30.
/// 2. If $D_2$ is the last day of February but not the **termination date**
///    or $D_2$ is 31, then change $D_2$ to 30.
///
/// Now plug into the following to get the day count fraction.
///
/// $$
/// \frac{360 (Y_2 - Y_1) + 30 (M_2 - M_1) + (D_2 - D_1)}{360}
/// $$

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct ThirtyE360ISDA {
    /// The termination date is the last date on which **new** obligations arise under the swap contract.
    ///
    /// Source: [2006 ISDA Definitions Chapter 3 Section 3](https://jollycontrarian.com/index.php?title=Termination_Date_-_ISDA_Definition)
    #[cfg(not(feature = "hifitime"))]
    pub termination_date: NaiveDate,
    /// The termination date is the last date on which **new** obligations arise under the swap contract.
    ///
    /// Source: [2006 ISDA Definitions Chapter 3 Section 3](https://jollycontrarian.com/index.php?title=Termination_Date_-_ISDA_Definition)
    #[cfg(feature = "hifitime")]
    pub termination_date: Epoch,
}

impl ThirtyE360ISDA {
    #[cfg(not(feature = "hifitime"))]
    /// Create a new [`ThirtyE360ISDA`] with a given termination date.
    #[must_use]
    pub const fn new(termination_date: NaiveDate) -> Self {
        Self { termination_date }
    }
    #[cfg(feature = "hifitime")]
    /// Create a new [`ThirtyE360ISDA`] with a given termination date.
    #[must_use]
    pub const fn new(termination_date: Epoch) -> Self {
        Self { termination_date }
    }
}

impl DayCounter for ThirtyE360ISDA {
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        let y1 = start.year();
        let m1 = start.month();
        let mut d1 = start.day() as i32;

        let y2 = end.year();
        let m2 = end.month();
        let mut d2 = end.day() as i32;

        if get_last_day_of_month(y1, m1) == d1 {
            d1 = 30;
        }

        if is_last_day_of_feb(*end) && (self.termination_date != *end || d2 == 31) {
            d2 = 30;
        }

        let numerator = 360 * (y2 - y1) + 30 * ((m2 - m1) as i32) + (d2 - d1);
        DayCountFraction::new(f64::from(numerator) / 360.0)
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, start: &Epoch, end: &Epoch) -> DayCountFraction<Self> {
        let (y1, m1, mut d1, _, _, _, _) = start.to_gregorian_utc();
        let (y2, m2, mut d2, _, _, _, _) = end.to_gregorian_utc();

        if get_last_day_of_month(y1, u32::from(m1)) == i32::from(d1) {
            d1 = 30;
        }

        if is_last_day_of_feb(*end) && (self.termination_date != *end || d2 == 31) {
            d2 = 30;
        }

        let numerator = (360 * (y2 - y1)) + (30 * i32::from(m2 - m1)) + i32::from(d2 - d1);
        DayCountFraction::new(f64::from(numerator) / 360.0)
    }
}

impl std::fmt::Display for ThirtyE360ISDA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "30E/360 (ISDA)")
    }
}

/// 30E+/360 (ISDA)
///
/// Let $Y_1$ be the year of $d_1$, $M_1$ be the month of $d_1$,
/// and $D_1$ be the day of $d_1$. Similarly define these for $d_2$.
///
/// Apply the following two rules:
/// 1. If $D_1$ 31, then change $D_1$ to 30.
/// 2. If $D_2$ is 31, then change $D_2$ to 1 and $M_2$ to $M_2 + 1$.
///
/// Now plug into the following to get the day count fraction.
///
/// $$
/// \frac{360 (Y_2 - Y_1) + 30 (M_2 - M_1) + (D_2 - D_1)}{360}
/// $$

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct ThirtyEPlus360ISDA;

impl DayCounter for ThirtyEPlus360ISDA {
    #[cfg(not(feature = "hifitime"))]
    fn day_count_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> DayCountFraction<Self> {
        let y1 = start.year();
        let m1 = start.month() as i32;
        let mut d1 = start.day() as i32;

        let y2 = end.year();
        let mut m2 = end.month() as i32;
        let mut d2 = end.day() as i32;

        if d1 == 31 {
            d1 = 30;
        }
        if d2 == 31 {
            d2 = 1;
            m2 += 1;
        }

        let numerator = 360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1);
        DayCountFraction::new(f64::from(numerator) / 360.0)
    }
    #[cfg(feature = "hifitime")]
    fn day_count_fraction(&self, start: &Epoch, end: &Epoch) -> DayCountFraction<Self> {
        let (y1, m1, mut d1, _, _, _, _) = start.to_gregorian_utc();
        let (y2, mut m2, mut d2, _, _, _, _) = end.to_gregorian_utc();

        if d1 == 31 {
            d1 = 30;
        }
        if d2 == 31 {
            d2 = 1;
            m2 += 1;
        }

        let numerator = (360 * (y2 - y1)) + (30 * i32::from(m2 - m1)) + i32::from(d2 - d1);
        DayCountFraction::new(f64::from(numerator) / 360.0)
    }
}

impl std::fmt::Display for ThirtyEPlus360ISDA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "30E+/360 (ISDA)")
    }
}
