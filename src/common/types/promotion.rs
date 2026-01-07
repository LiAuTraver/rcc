use crate::{breakpoint, common::types::Primitive};

impl Primitive {
  pub fn is_integer(&self) -> bool {
    self.is_signed_integer() || self.is_unsigned()
  }
  pub fn is_signed_integer(&self) -> bool {
    matches!(
      self,
      Primitive::Char
        | Primitive::SChar
        | Primitive::Short
        | Primitive::Int
        | Primitive::Long
        | Primitive::LongLong
    )
  }
  pub fn is_signed(&self) -> bool {
    self.is_signed_integer() || self.is_floating_point()
  }
  pub fn is_unsigned(&self) -> bool {
    matches!(
      self,
      Primitive::Bool
        | Primitive::UChar
        | Primitive::UShort
        | Primitive::UInt
        | Primitive::ULong
        | Primitive::ULongLong
    )
  }
  pub fn integer_rank(&self) -> u8 {
    // bitmask has no use here, just a unique value for each rank
    match self {
      Primitive::Bool => 0x01,
      Primitive::Char | Primitive::SChar | Primitive::UChar => 0x02,
      Primitive::Short | Primitive::UShort => 0x04,
      Primitive::Int | Primitive::UInt => 0x08,
      Primitive::Long | Primitive::ULong => 0x10,
      Primitive::LongLong | Primitive::ULongLong => 0x20,
      _ => {
        breakpoint!();
        panic!("Not an integer type");
      }
    }
  }
  pub fn is_floating_point(&self) -> bool {
    matches!(
      self,
      Primitive::Float | Primitive::Double | Primitive::LongDouble
    ) || self.is_complex()
  }
  pub fn floating_rank(&self) -> u8 {
    match self {
      Primitive::Float | Primitive::ComplexFloat => 0x01,
      Primitive::Double | Primitive::ComplexDouble => 0x02,
      Primitive::LongDouble | Primitive::ComplexLongDouble => 0x04,
      _ => {
        breakpoint!();
        panic!("Not a floating point type");
      }
    }
  }
  pub fn is_complex(&self) -> bool {
    matches!(
      self,
      Primitive::ComplexFloat | Primitive::ComplexDouble | Primitive::ComplexLongDouble
    )
  }
  pub fn is_void(&self) -> bool {
    matches!(self, Primitive::Void)
  }
  pub fn integer_promotion(self) -> Primitive {
    if !self.is_integer() {
      breakpoint!();
      panic!("Type {:?} is not an integer type", self);
    } else if self.integer_rank() < Primitive::Int.integer_rank() {
      Primitive::Int
    } else {
      self
    }
  }
}
