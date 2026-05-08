use ::rcc_adt::{FloatFormat, Floating, Integral, Signedness, SizeBit};

use crate::{DiagData, DiagMeta, Severity};

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
  Integral(Integral),
  Floating(Floating),
}
::rcc_utils::interconvert!(Integral, Number);
::rcc_utils::interconvert!(Floating, Number);
impl Number {
  pub const FLOATING_SUFFIXES: &'static [&'static str] = &[
    "f", "F", // float
    "l", "L", // long double
    // unsupported
    "df", "DF", // _Decimal32
    "dd", "DD", // _Decimal64
    "dl", "DL", // _Decimal128
  ];
  // literal suffixes
  pub const INTEGER_SUFFIXES: &'static [&'static str] = &[
    "u", "U", // unsigned
    "l", "L", // long
    "ll", "LL", // long long
    "ul", "uL", "Ul", "UL", "lu", "lU", "Lu", "LU", // unsigned long
    "ull", "uLL", "Ull", "ULL", "llu", "llU", "LLu",
    "LLU", // unsigned long long
    "uz", "uZ", "Uz", "UZ", "zu", "zU", "Zu", "ZU", // size_t
    "z", "Z", // size_t's signed version
    // msvc extensions
    "i8", "i16", "i32", "i64", // signed int
    "ui8", "ui16", "ui32", "ui64", // unsigned int
    // unsupported
    "wb", "WB", // _BitInt
    "uwb", "uWB", "Uwb", "UWB", // unsigned _BitInt
  ];

  /// parse a numeric literal with optional suffix, if fails, return an error message and the default value of the Constant
  pub fn parse<'c>(
    num: &str,
    base: u32,
    suffix: Option<&str>,
    is_floating: bool,
  ) -> (Self, Option<DiagMeta<'c>>) {
    let num = num.strip_suffix('.').unwrap_or(num);

    use DiagData::*;

    macro_rules! int_conv {
      ($t:ty, $signess:ident) => {
        match <$t>::from_str_radix(num, base) {
          Ok(v) => (Integral::from(v).into(), None),
          Err(e) => (
            Integral::new(
              <$t>::default(),
              SizeBit::new(<$t>::BITS),
              Signedness::$signess,
            )
            .into(),
            Some(
              InvalidNumberFormat(format!("{e} in number '{num}'"))
                + Severity::Error,
            ),
          ),
        }
      };
    }
    macro_rules! float_conv {
      ($t:ty, $format:ident) => {
        match num.parse::<$t>() {
          Ok(v) => (Floating::from(v).into(), None),
          Err(e) => (
            Floating::new(<$t>::default().to_bits(), FloatFormat::$format)
              .into(),
            Some(
              InvalidNumberFormat(format!("{e} in number '{num}'"))
                + Severity::Error,
            ),
          ),
        }
      };
    }
    match (suffix, is_floating) {
      // default to int
      // FIXME: the size is not determined to be `int`.
      (None, false) => int_conv!(i32, Signed),
      (_, true) if base != 10 => (
        Floating::default().into(),
        Some(
          InvalidNumberFormat(format!(
            "currently floating point literals only support decimal format, \
             got {num}"
          )) + Severity::Error,
        ),
      ),
      // default to double
      (None, true) => float_conv!(f64, IEEE64),
      // integer with suffix
      (Some(suf), false) => match suf {
        // FIXME: `u`/`U` only indicates unsigned, and provide no information about the size.
        "u" | "U" => int_conv!(u32, Unsigned),
        "l" | "L" => int_conv!(i64, Signed),
        "ll" | "LL" => int_conv!(i64, Signed),
        "ul" | "uL" | "Ul" | "UL" | "lu" | "lU" | "Lu" | "LU" =>
          int_conv!(u64, Unsigned),
        "ull" | "uLL" | "Ull" | "ULL" | "llu" | "llU" | "LLu" | "LLU" =>
          int_conv!(u64, Unsigned),
        "z" | "Z" => int_conv!(isize, Signed),
        "uz" | "uZ" | "Uz" | "UZ" | "zu" | "zU" | "Zu" | "ZU" =>
          int_conv!(usize, Unsigned),
        "i8" => int_conv!(i8, Signed),
        "i16" => int_conv!(i16, Signed),
        "i32" => int_conv!(i32, Signed),
        "i64" => int_conv!(i64, Signed),
        "ui8" => int_conv!(u8, Unsigned),
        "ui16" => int_conv!(u16, Unsigned),
        "ui32" => int_conv!(u32, Unsigned),
        "ui64" => int_conv!(u64, Unsigned),
        _ => (
          Integral::default().into(),
          Some(
            InvalidNumberFormat(format!(
              "invalid or unsupported integer literal suffix: {suf}"
            )) + Severity::Error,
          ),
        ),
      },
      // floating with suffix
      (Some(suf), true) => match suf {
        "f" | "F" => float_conv!(f32, IEEE32),
        "l" | "L" => float_conv!(f64, IEEE64),
        _ => (
          Floating::default().into(),
          Some(
            InvalidNumberFormat(format!(
              "invalid or unsupported floating literal suffix: {suf}"
            )) + Severity::Error,
          ),
        ),
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn integer() {
    assert_eq!(
      Number::parse("42", 10, None, false).0,
      Integral::from(42i32).into()
    );
    assert_eq!(
      Number::parse("42", 10, Some("u"), false).0,
      Integral::from(42u32).into()
    );
    assert_eq!(
      Number::parse("42", 10, Some("l"), false).0,
      Integral::from(42i64).into()
    );
    assert_eq!(
      Number::parse("42", 10, Some("ul"), false).0,
      Integral::from(42u64).into()
    );
    assert_eq!(
      Number::parse("42", 10, Some("ull"), false).0,
      Integral::from(42u64).into()
    );
    assert_eq!(
      Number::parse("42", 10, Some("z"), false).0,
      Integral::from(42isize).into()
    );
    assert_eq!(
      Number::parse("42", 10, Some("uz"), false).0,
      Integral::from(42usize).into()
    );

    assert_eq!(
      Number::parse("-42", 10, None, false).0,
      Integral::from(-42i32).into()
    );
    assert_eq!(
      Number::parse("-42", 10, Some("ll"), false).0,
      Integral::from(-42i64).into()
    );
  }
  #[test]
  fn floating() {
    assert_eq!(
      Number::parse("42.60", 10, None, true).0,
      Floating::from(42.60f64).into()
    );
    assert_eq!(
      Number::parse("42.60", 10, Some("f"), true).0,
      Floating::from(42.60f32).into()
    );
    assert_eq!(
      Number::parse("42.60", 10, Some("l"), true).0,
      Floating::from(42.60f64).into()
    );
    assert_eq!(
      Number::parse(".42", 10, None, true).0,
      Floating::from(0.42f64).into()
    );
    assert!(Number::parse(".0", 10, None, true).1.is_none());
    assert!(Number::parse("0.", 10, None, true).1.is_none());
    assert!(Number::parse("0", 10, None, true).1.is_none());

    assert_eq!(
      Number::parse("-42.60", 10, None, true).0,
      Floating::from(-42.60f64).into()
    );
    assert_eq!(
      Number::parse("-42.60", 10, Some("f"), true).0,
      Floating::from(-42.60f32).into()
    );
    assert!(Number::parse("-.42", 10, None, true).1.is_none());
    assert!(Number::parse("-0.", 10, None, true).1.is_none());
    assert!(Number::parse("-0", 10, None, true).1.is_none());
  }
  #[test]
  fn invalid() {
    // out of range.
    assert!(
      Number::parse("4294967296", 10, Some("u"), false)
        .1
        .is_some()
    );
    // invalid suffix
    assert!(Number::parse("42", 10, Some("x"), false).1.is_some());
    assert!(Number::parse("42.60", 10, Some("x"), true).1.is_some());
    // pass negative number to unsigned type
    assert!(Number::parse("-42", 10, Some("u"), false).1.is_some());
  }
  #[test]
  fn exponent() {
    assert_eq!(
      Number::parse("1e10", 10, None, true).0,
      Floating::from(1e10f64).into()
    );
    assert_eq!(
      Number::parse("1e-10", 10, None, true).0,
      Floating::from(1e-10f64).into()
    );
    assert_eq!(
      Number::parse("-1e10", 10, None, true).0,
      Floating::from(-1e10f64).into()
    );
    assert_eq!(
      Number::parse("-1e-10", 10, None, true).0,
      Floating::from(-1e-10f64).into()
    );
  }
}
