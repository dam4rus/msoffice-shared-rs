use super::error::PatternRestrictionError;
use regex::Regex;
use std::{marker::PhantomData, str::FromStr};

pub trait PatternRestricted {
    fn restriction_pattern() -> &'static str;
}
#[derive(Debug, Clone, PartialEq)]
pub struct Signed;
#[derive(Debug, Clone, PartialEq)]
pub struct Unsigned;

pub type OnOff = bool;

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum UniversalMeasureUnit {
    #[strum(serialize = "mm")]
    Millimeter,
    #[strum(serialize = "cm")]
    Centimeter,
    #[strum(serialize = "in")]
    Inch,
    #[strum(serialize = "pt")]
    Point,
    #[strum(serialize = "pc")]
    Pica,
    #[strum(serialize = "pi")]
    Pitch,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UniversalMeasure<T = Signed> {
    pub value: f64,
    pub unit: UniversalMeasureUnit,
    pub _phantom: PhantomData<T>,
}

pub type PositiveUniversalMeasure = UniversalMeasure<Unsigned>;

impl<T> UniversalMeasure<T> {
    pub fn new(value: f64, unit: UniversalMeasureUnit) -> Self {
        Self {
            value,
            unit,
            _phantom: PhantomData,
        }
    }
}

impl PatternRestricted for UniversalMeasure<Signed> {
    fn restriction_pattern() -> &'static str {
        r#"^-?[0-9]+(\.[0-9]+)?(mm|cm|in|pt|pc|pi)$"#
    }
}

impl PatternRestricted for UniversalMeasure<Unsigned> {
    fn restriction_pattern() -> &'static str {
        r#"^[0-9]+(\.[0-9]+)?(mm|cm|in|pt|pc|pi)$"#
    }
}

impl<T> FromStr for UniversalMeasure<T>
where
    UniversalMeasure<T>: PatternRestricted,
{
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(Self::restriction_pattern()).expect("valid regexp should be provided");
        let captures = re
            .captures(s)
            .ok_or_else(|| Box::new(PatternRestrictionError::NoMatch))?;
        // Group 0 and 2 can't be empty if the match succeeds
        let unit_slice = captures.get(2).unwrap();
        let value_slice = &s[captures.get(0).unwrap().start()..unit_slice.start()];
        Ok(Self::new(value_slice.parse()?, unit_slice.as_str().parse()?))
    }
}

#[cfg(test)]
#[test]
pub fn test_universal_measure_from_str() {
    assert_eq!(
        "123.4567mm".parse::<UniversalMeasure>().unwrap(),
        UniversalMeasure::new(123.4567, UniversalMeasureUnit::Millimeter),
    );
    assert_eq!(
        "123cm".parse::<UniversalMeasure>().unwrap(),
        UniversalMeasure::new(123.0, UniversalMeasureUnit::Centimeter),
    );
    assert_eq!(
        "-123in".parse::<UniversalMeasure>().unwrap(),
        UniversalMeasure::new(-123.0, UniversalMeasureUnit::Inch),
    );
}
