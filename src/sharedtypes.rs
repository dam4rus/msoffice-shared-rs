use super::error::PatternRestrictionError;
use regex::Regex;
use std::{marker::PhantomData, str::FromStr};

pub type OnOff = bool;
pub type Lang = String;

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum CalendarType {
    #[strum(serialize = "gregorian")]
    Gregorian,
    #[strum(serialize = "gregorianUs")]
    GregorianUs,
    #[strum(serialize = "gregorianMeFrench")]
    GregorianMeFrench,
    #[strum(serialize = "gregorianArabic")]
    GregorianArabic,
    #[strum(serialize = "hijri")]
    Hijri,
    #[strum(serialize = "hebrew")]
    Hebrew,
    #[strum(serialize = "taiwan")]
    Taiwan,
    #[strum(serialize = "japan")]
    Japan,
    #[strum(serialize = "thai")]
    Thai,
    #[strum(serialize = "korea")]
    Korea,
    #[strum(serialize = "saka")]
    Saka,
    #[strum(serialize = "gregorianXlitEnglish")]
    GregorianXlitEnglish,
    #[strum(serialize = "gregorianXlitFrench")]
    GregorianXlitFrench,
    #[strum(serialize = "none")]
    None,
}

/// Trait indicating that a data type is restricted by a string pattern. A pattern is basically a regular expression.
pub trait PatternRestricted {
    fn restriction_pattern() -> &'static str;
}

/// Empty struct used to tag a data type implying that the stored value is signed.
#[derive(Debug, Clone, PartialEq)]
pub struct Signed;

/// Empty struct used to tag a data type implying that the stored value is unsigned.
#[derive(Debug, Clone, PartialEq)]
pub struct Unsigned;

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

pub type PositiveUniversalMeasure = UniversalMeasure<Unsigned>;

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

#[derive(Debug, Clone, PartialEq)]
pub enum TwipsMeasure {
    Decimal(u64),
    UniversalMeasure(PositiveUniversalMeasure),
}

impl FromStr for TwipsMeasure {
    // TODO custom error type
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<u64>() {
            Ok(TwipsMeasure::Decimal(value))
        } else {
            Ok(TwipsMeasure::UniversalMeasure(s.parse()?))
        }
    }
}

#[cfg(test)]
#[test]
pub fn test_twips_measure_from_str() {
    assert_eq!("123".parse::<TwipsMeasure>().unwrap(), TwipsMeasure::Decimal(123));
    assert_eq!(
        "123.456mm".parse::<TwipsMeasure>().unwrap(),
        TwipsMeasure::UniversalMeasure(PositiveUniversalMeasure::new(123.456, UniversalMeasureUnit::Millimeter)),
    );
}

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum VerticalAlignRun {
    #[strum(serialize = "baseline")]
    Baseline,
    #[strum(serialize = "superscript")]
    Superscript,
    #[strum(serialize = "subscript")]
    Subscript,
}
