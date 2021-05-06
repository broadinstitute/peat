use std::fmt::{Display, Formatter};
use std::fmt;
use crate::error::Error;
use std::ops::Range;

#[derive(Copy, Clone)]
pub(crate) struct UIntRange {
    from: u64,
    until: u64,
}

#[derive(Copy, Clone)]
pub(crate) struct UIntRangeRange {
    dividend: UIntRange,
    divisor: UIntRange,
}

#[derive(Copy, Clone)]
pub(crate) enum Value {
    UIntValue(u64),
    UIntRangeValue(UIntRange),
    UIntRangeRangeValue(UIntRangeRange),
}

impl UIntRange {
    pub(crate) fn new(from: u64, until: u64) -> UIntRange { UIntRange { from, until } }
    pub(crate) fn len(&self) -> u64 { self.until - self.from }
    pub(crate) fn contains(&self, i: u64) -> bool { i >= self.from && i < self.until }
    pub(crate) fn to_range(&self) -> Range<u64> { self.from .. self.until }
}

fn ceil_div(dividend: u64, divisor: u64) -> Result<u64, Error> {
    if divisor == 0 {
        return Err(Error::from("Division by zero"));
    }
    Ok((dividend + divisor - 1) / divisor)
}

impl UIntRangeRange {
    pub(crate) fn new(dividend: UIntRange, divisor: UIntRange) -> UIntRangeRange {
        UIntRangeRange { dividend, divisor }
    }
    pub(crate) fn pick(&self, g: u64) -> Result<UIntRange, Error> {
        if !self.divisor.contains(g) {
            return Err(Error::from(format!("Pick {} is not in range {}.", g, self.divisor)));
        }
        let l = self.dividend.from;
        let n = self.dividend.len();
        let m = self.divisor.len();
        let lg = self.divisor.from;
        let from = l + ceil_div((g - lg) * n, m)?;
        let until = l + ceil_div((g - lg + 1) * n, m)?;
        Ok(UIntRange::new(from, until))
    }
}

impl Value {
    pub(crate) fn as_int(&self) -> Result<u64, Error> {
        match self {
            Value::UIntValue(ui) => Ok(*ui),
            Value::UIntRangeValue(ui_rng) =>
                Err(Error::from(
                    format!("Expected integer, but got range {}", ui_rng)
                )),
            Value::UIntRangeRangeValue(ui_rng_rng) =>
                Err(Error::from(
                    format!("Expected integer, but got range of ranges {}.", ui_rng_rng)
                ))
        }
    }

    pub(crate) fn as_range(&self) -> Result<UIntRange, Error> {
        match self {
            Value::UIntValue(ui) =>
                Err(Error::from(
                    format!("Expected range, but got integer {}", ui)
                )),
            Value::UIntRangeValue(ui_rng) => Ok(*ui_rng),
            Value::UIntRangeRangeValue(ui_rng_rng) =>
                Err(Error::from(
                    format!("Expected range, but got range of ranges {}.", ui_rng_rng)
                ))
        }
    }

    pub(crate) fn as_range_range(&self) -> Result<UIntRangeRange, Error> {
        match self {
            Value::UIntValue(ui) =>
                Err(Error::from(
                    format!("Expected range of ranges, but got integer {}", ui)
                )),
            Value::UIntRangeValue(ui_rng) =>
                Err(Error::from(
                    format!("Expected range of ranges, but got range {}", ui_rng)
                )),
            Value::UIntRangeRangeValue(ui_rng_rng) => Ok(*ui_rng_rng)
        }
    }

    pub(crate) fn new_int(ui: u64) -> Value { Value::UIntValue(ui) }

    pub(crate) fn new_range(from: u64, until: u64) -> Value {
        Value::UIntRangeValue(UIntRange { from, until })
    }

    pub(crate) fn new_range_range(dividend: UIntRange, divisor: UIntRange) -> Value {
        Value::UIntRangeRangeValue(UIntRangeRange { dividend, divisor })
    }
}

impl Display for UIntRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(format!("{} .. {}", self.from, self.until).as_str(), f)
    }
}

impl Display for UIntRangeRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(format!("{} / {}", self.divisor, self.dividend).as_str(), f)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::UIntValue(ui) => { Display::fmt(ui, f) }
            Value::UIntRangeValue(uint_range) => { Display::fmt(uint_range, f) }
            Value::UIntRangeRangeValue(uint_range_range) => {
                Display::fmt(uint_range_range, f)
            }
        }
    }
}

