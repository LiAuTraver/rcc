use ::rcc_adt::{FloatFormat, Floating, Integral, Signedness};
use ::rcc_utils::{RefEq, StrRef, ensure_is_pod};

/// discrepancy: string literals are not constant values in C `char[N]`
/// (but in C++, it is, though.)
///
/// TODO: named constants `constexpr` and constant aggregate
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constant<'c> {
  Nullptr(),
  Integral(Integral),
  Floating(Floating),
  String(StrRef<'c>),
  /// FIXME: this is mostly wrong but cant fix this until we put AST nodes into arena.
  Address(StrRef<'c>),
}
ensure_is_pod!(Constant);
pub type ConstantRef<'c> = &'c Constant<'c>;
pub type ConstantRefMut<'c> = &'c mut Constant<'c>;
impl RefEq for ConstantRef<'_> {
  fn ref_eq(lhs: Self, rhs: Self) -> bool
  where
    Self: PartialEq + Sized,
  {
    let ref_eq = ::std::ptr::eq(lhs, rhs);
    if const { cfg!(debug_assertions) } {
      let actual_eq = lhs == rhs;
      if ref_eq != actual_eq {
        eprintln!(
          "INTERNAL ERROR: comparing by pointer address result did not match 
          the actual result: {:p}: {:?} and {:p}: {:?}
        ",
          lhs, lhs, rhs, rhs
        );
      }
      return actual_eq;
    }
    ref_eq
  }
}
impl<'c> Constant<'c> {
  pub const fn is_char_array(&self) -> bool {
    matches!(self, Self::String(_))
  }

  pub fn is_zero(&self) -> bool {
    match self {
      Self::Integral(integral) => integral.is_zero(),
      Self::Floating(floating) => floating.is_zero(),
      Self::String(s) => s.is_empty(),
      Self::Nullptr() => true,
      Self::Address(_) => false,
    }
  }

  #[inline(always)]
  pub fn is_not_zero(&self) -> bool {
    !self.is_zero()
  }

  pub fn to_boolean(self) -> Self {
    match self {
      Self::Integral(integral) =>
        Constant::Integral(Integral::from_bool(!integral.is_zero())),
      Self::Floating(floating) =>
        Constant::Integral(Integral::from_bool(!floating.is_zero())),
      Self::String(s) => Constant::Integral(Integral::from_bool(s.is_empty())),
      Self::Nullptr() => Constant::Integral(Integral::from_bool(false)),
      Self::Address(_) => Constant::Integral(Integral::from_bool(true)),
    }
  }

  pub fn to_integral(self, width: u8, signedness: Signedness) -> Self {
    match self {
      Self::Integral(integral) =>
        Constant::Integral(integral.cast(width, signedness)),
      Self::Floating(floating) =>
        Constant::Integral(floating.to_integral(width, signedness)),
      _ => unreachable!("handled elsewhere"),
    }
  }

  pub fn to_floating(self, format: FloatFormat) -> Self {
    match self {
      Self::Integral(integral) => Self::Floating(integral.to_floating(format)),
      Self::Floating(floating) => Self::Floating(floating),
      _ => unreachable!("handled elsewhere"),
    }
  }

  pub fn is_address(&self) -> bool {
    matches!(self, Constant::Address(_))
  }
}
::rcc_utils::interconvert!(Integral, Constant<'c>);
::rcc_utils::interconvert!(Floating, Constant<'c>);
// ::rcc_utils::interconvert!(???, Constant, String);
// ::rcc_utils::interconvert!(???, Constant, Address);

::rcc_utils::make_trio_for!(Integral, Constant<'c>);
::rcc_utils::make_trio_for!(Floating, Constant<'c>);
::rcc_utils::make_trio_for_unit_tuple!(Nullptr, Constant<'c>);

// ::rcc_utils::make_trio_for!(???, Constant, String);
// ::rcc_utils::make_trio_for!(???, Constant, Address);

impl ::std::fmt::Display for Constant<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Constant::*;
    match self {
      Integral(i) => write!(f, "{i}"),
      Floating(d) => write!(f, "{d}"),
      String(s) | Address(s) => write!(f, "\"{}\"", s),
      Nullptr() => write!(f, "nullptr"),
    }
  }
}
