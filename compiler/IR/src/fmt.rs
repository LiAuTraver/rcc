use ::rcc_adt::FloatFormat;
use ::std::fmt::Display;

use super::types;

impl Display for types::Function<'_> {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for types::Array<'_> {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for types::Struct<'_> {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl Display for types::Type<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Void() => write!(f, "void"),
      Self::Label() => write!(f, "label"),
      Self::Floating(FloatFormat::IEEE32) => write!(f, "float"),
      Self::Floating(FloatFormat::IEEE64) => write!(f, "double"),
      Self::Pointer() => write!(f, "ptr"),
      Self::Integer(bit_width) => write!(f, "i{bit_width}"),
      Self::Array(array) => array.fmt(f),
      Self::Function(function) => function.fmt(f),
      Self::Struct(s) => s.fmt(f),
    }
  }
}
