// C's built-in types
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  ::strum_macros::Display,
  ::strum_macros::IntoStaticStr,
  ::strum_macros::AsRefStr,
  ::strum_macros::EnumString,
)]

pub enum Primitive {
  #[strum(serialize = "bool")]
  #[strum(serialize = "_Bool")]
  Bool,
  #[strum(serialize = "char")]
  Char, // plain char
  #[strum(serialize = "signed char")]
  SChar, // signed char
  #[strum(serialize = "short")]
  Short,
  #[strum(serialize = "int")]
  Int,
  #[strum(serialize = "long")]
  Long,
  #[strum(serialize = "long long")]
  LongLong,
  #[strum(serialize = "unsigned char")]
  UChar,
  #[strum(serialize = "unsigned short")]
  UShort,
  #[strum(serialize = "unsigned int")]
  UInt,
  #[strum(serialize = "unsigned long")]
  ULong,
  #[strum(serialize = "unsigned long long")]
  ULongLong,
  #[strum(serialize = "float")]
  Float,
  #[strum(serialize = "double")]
  Double,
  #[strum(serialize = "long double")]
  LongDouble,
  /// 6.2.5p24: The void type comprises an empty set of values; it is an incomplete object type that cannot be completed.
  #[strum(serialize = "void")]
  Void,
  /// 6.5.5p4: `nullptr`. The type `nullptr_t` shall not be converted to any type other than `void`, `bool` or a pointer type.
  #[strum(serialize = "nullptr_t")]
  Nullptr,
  // ignore below for now: __STDC_NO_COMPLEX__
  #[strum(serialize = "_Complex float")]
  ComplexFloat,
  #[strum(serialize = "_Complex")]
  #[strum(serialize = "_Complex double")]
  ComplexDouble,
  #[strum(serialize = "_Complex long double")]
  ComplexLongDouble,

  /// FIXME.
  #[strum(serialize = "__auto_type")]
  __AutoType,
  /// This represent a bit -- 1/8 of byte for IR's `i1` type -- merely a workaround to fix my design.
  ///
  /// # Warning:
  /// **Any attempt to access, create or call to this primitive type using AST-type related function would immediately result in panic.**
  ///
  /// The **ONLY** valid member function is [`TypeInfo::size_bits`], which [`Self::Bool`] returns 8 and [`Self::__IRBit`] returns 1.
  #[strum(disabled)]
  __IRBit,
}
::rcc_utils::ensure_is_pod!(Primitive);

use super::{
  CastType::{self, *},
  TypeInfo,
};
use crate::TargetInfo;
type CommonTypeResult = (
  Primitive, /* the common type. */
  CastType,  /* the cast type which shall be applied to lhs. */
  CastType,  /* the cast type which shall be applied to rhs. */
);
impl Primitive {
  #[must_use]
  pub fn common_type(
    lhs: Self,
    rhs: Self,
    target_info: &TargetInfo,
  ) -> CommonTypeResult {
    // If both operands have the same type, then no further conversion is needed.
    // first: _Decimal types ignored
    // also, complex types ignored
    if lhs == rhs {
      return (lhs, Noop, Noop);
    }
    if matches!(lhs, Self::Void | Self::Nullptr)
      || matches!(rhs, Self::Void | Self::Nullptr)
    {
      panic!("Invalid types for common type: {:?}, {:?}", lhs, rhs);
    }
    // otherwise, if either operand is of some floating type, the other operand is converted to it.
    // Otherwise, if any of the two types is an enumeration, it is converted to its underlying type. - handled upstream
    match (lhs.is_floating_point(), rhs.is_floating_point()) {
      (true, false) => (lhs, Noop, IntegralToFloating),
      (false, true) => (rhs, IntegralToFloating, Noop),
      (true, true) => Self::common_floating_rank(lhs, rhs),
      (false, false) => Self::common_integer_rank(lhs, rhs, target_info),
    }
  }

  #[must_use]
  fn common_floating_rank(lhs: Self, rhs: Self) -> CommonTypeResult {
    assert!(lhs.is_floating_point() && rhs.is_floating_point());
    if lhs.floating_rank() > rhs.floating_rank() {
      (lhs, Noop, FloatingCast)
    } else {
      (rhs, FloatingCast, Noop)
    }
  }

  #[must_use]
  fn common_integer_rank(
    lhs: Self,
    rhs: Self,
    target_info: &TargetInfo,
  ) -> CommonTypeResult {
    assert!(lhs.is_integer() && rhs.is_integer());

    // shall be promoted upstream
    debug_assert_eq!(lhs, lhs.integer_promotion().0);
    debug_assert_eq!(rhs, rhs.integer_promotion().0);

    if lhs == rhs {
      // done
      return (lhs, Noop, Noop);
    }
    if lhs.is_unsigned(target_info) == rhs.is_unsigned(target_info) {
      return if lhs.integer_rank() > rhs.integer_rank() {
        (lhs, Noop, IntegralCast)
      } else {
        (rhs, IntegralCast, Noop)
      };
    }
    fn signed_and_unsigned(
      signed: Primitive,
      unsigned: Primitive,
      target_info: &TargetInfo,
    ) -> (
      Primitive, /* common type */
      CastType,  /* from signed */
      CastType,  /* from unsigned */
    ) {
      debug_assert!(!signed.is_unsigned(target_info));
      debug_assert!(unsigned.is_unsigned(target_info));
      if signed.integer_rank() >= unsigned.integer_rank() {
        (signed, Noop, IntegralCast)
      } else if unsigned.size(target_info) > signed.size(target_info) {
        (unsigned, IntegralCast, Noop)
      } else {
        // if the signed type cannot represent all values of the unsigned type, return the unsigned version of the signed type
        // the unsigned type is always larger/equal than the corresponding unsigned type.
        // so this branch is extremely unlikely to be taken
        (signed.into_unsigned(), IntegralCast, IntegralCast)
      }
    }
    const trait TupleExt {
      #[must_use]
      fn swap2nd3rd(self) -> Self;
    }
    impl const TupleExt for CommonTypeResult {
      #[inline(always)]
      fn swap2nd3rd(self) -> Self {
        let (t, l, r) = self;
        (t, r, l)
      }
    }

    if lhs.is_unsigned(target_info) {
      signed_and_unsigned(rhs, lhs, target_info).swap2nd3rd()
    } else {
      signed_and_unsigned(lhs, rhs, target_info)
    }
  }

  pub const fn is_character_type(&self) -> bool {
    use Primitive::*;
    matches!(self, Char | UChar | SChar)
  }
}
use ::rcc_adt::FloatFormat;

impl From<Primitive> for FloatFormat {
  fn from(value: Primitive) -> Self {
    use FloatFormat::*;
    use Primitive::*;
    match value {
      Float => IEEE32,
      Double => IEEE64,
      _ => panic!("Invalid primitive type for float format: {:?}", value),
    }
  }
}
