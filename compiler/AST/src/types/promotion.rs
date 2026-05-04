//! applied during unary operations, including (implicit) unary inside binary operations

use ::rcc_adt::{FloatFormat, Signedness};
use CastType::*;
use Signedness::*;

use super::{
  CastType,
  Primitive::{self, *},
  Type, TypeInfo,
};
use crate::TargetInfo;

pub trait Promotion {
  #[must_use]
  fn promote(self) -> (Self, CastType)
  where
    Self: Sized;
}

impl Primitive {
  pub const fn is_integer(&self) -> bool {
    matches!(
      self,
      Bool
        | Char
        | SChar
        | UChar
        | Short
        | UShort
        | Int
        | UInt
        | Long
        | ULong
        | LongLong
        | ULongLong
    )
  }

  pub const fn is_signed_integer(&self, target_info: &TargetInfo) -> bool {
    matches!(self.signedness(target_info), Some(Signed))
  }

  pub const fn is_arithmetic(&self) -> bool {
    self.is_integer() || self.is_floating_point()
  }

  pub const fn is_signed(&self, target_info: &TargetInfo) -> bool {
    self.is_signed_integer(target_info) || self.is_floating_point()
  }

  pub const fn is_unsigned(&self, target_info: &TargetInfo) -> bool {
    matches!(self.signedness(target_info), Some(Unsigned))
  }

  pub const fn integer_rank(&self) -> u8 {
    // bitmask has no use here, just a unique value for each rank
    match self {
      Bool => 0x01,
      Char | SChar | UChar => 0x02,
      Short | UShort => 0x04,
      Int | UInt => 0x08,
      Long | ULong => 0x10,
      LongLong | ULongLong => 0x20,
      _ => panic!("Not an integer type"),
    }
  }

  pub const fn is_floating_point(&self) -> bool {
    matches!(self, Float | Double | LongDouble) || self.is_complex()
  }

  pub const fn floating_rank(&self) -> u8 {
    match self {
      Float | ComplexFloat => 0x01,
      Double | ComplexDouble => 0x02,
      LongDouble | ComplexLongDouble => 0x04,
      _ => panic!("Not a floating point type"),
    }
  }

  pub const fn floating_format(&self) -> FloatFormat {
    use FloatFormat::*;
    match self {
      Float | ComplexFloat => IEEE32,
      Double | ComplexDouble => IEEE64,
      LongDouble | ComplexLongDouble => IEEE64,
      _ => panic!("Not a floating point type"),
    }
  }

  pub const fn is_complex(&self) -> bool {
    matches!(self, ComplexFloat | ComplexDouble | ComplexLongDouble)
  }

  pub const fn is_void(&self) -> bool {
    matches!(self, Void)
  }

  pub const fn is_bool(&self) -> bool {
    matches!(self, Bool)
  }

  pub const fn is_contextual_bool(&self) -> bool {
    matches!(self, Int)
  }

  pub const fn is_nullptr(&self) -> bool {
    matches!(self, Nullptr)
  }

  #[must_use]
  pub fn integer_promotion(self) -> (Primitive, CastType) {
    assert!(self.is_integer(), "Type {:?} is not an integer type", self);

    if self.integer_rank() < Int.integer_rank() {
      (Int, IntegralCast)
    } else {
      (self, Noop)
    }
  }

  /// floating promotion is only happened during variadic function calls and the old `functionnoproto`, which i wont implement
  #[must_use]
  pub fn floating_promotion(self) -> (Primitive, CastType) {
    assert!(
      self.is_floating_point(),
      "Type {:?} is not a floating point type",
      self
    );
    if self.floating_rank() < Double.floating_rank() {
      (Double, FloatingCast)
    } else {
      (self, Noop)
    }
  }

  #[must_use]
  pub fn into_unsigned(self) -> Primitive {
    match self {
      Bool => Bool,
      Char => UChar,
      SChar => UChar,
      Short => UShort,
      Int => UInt,
      Long => ULong,
      LongLong => ULongLong,
      _ => panic!("Type {:?} is not a signed integer type", self),
    }
  }
}
impl Promotion for Primitive {
  fn promote(self) -> (Self, CastType) {
    if self.is_integer() {
      self.integer_promotion()
    } else {
      (self, Noop)
    }
  }
}
impl<'c> Promotion for Type<'c> {
  fn promote(self) -> (Self, CastType) {
    match self {
      Self::Primitive(p) => {
        let (promoted, cast_type) = p.promote();
        (Self::Primitive(promoted), cast_type)
      },
      _ => (self, Noop),
    }
  }
}
