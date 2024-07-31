# Day Count Conventions

This library supplies common day count conventions for financial applications.

Currently supported day count conventions are:
- [Actual/360](Actual360)
- [Actual/360 (inc)](Actual360Inc)
- [Actual/364](Actual364)
- [Actual/365 (A)](Actual365A)
- [Actual/365 (Fixed)](Actual365Fixed)
- [Actual/366](Actual366)
- [Actual/366 (inc)](Actual366Inc)
- [Actual/365.25](Actual36525)
- [Actual/365.25 (inc)](Actual36525Inc)
- [NL/365](NL365)
- [1/1](OneOne)
- [30/360](Thirty360)
- [30E/360](ThirtyE360)
- [30E/360 (ISDA)](ThirtyE360ISDA)
- [30E+/360 (ISDA)](ThirtyEPlus360ISDA)

If there are any conventions that you would like implemented, don't
hestitate to submit a PR or raise in issue on [GitHub](https://github.com/MeetThePatel/day_count_conventions.rs)!

### References:
**Note:** The following sources may have slightly different definitions. As
a precaution, please see the documentations for the particular definitions
used in this package.

- 1. [OpenGamma (Chapter 3)](https://quant.opengamma.io/Interest-Rate-Instruments-and-Market-Conventions.pdf)
- 2. [Wikipedia](https://en.wikipedia.org/wiki/Day_count_convention)
- 3. [2006 ISDA Definitions](https://www.isda.org/book/2006-isda-definitions/)
- 4. [QuantLib](https://github.com/lballabio/QuantLib/tree/master/ql/time/daycounters)
- 5. [DeltaQuants](http://www.deltaquants.com/day-count-conventions)
