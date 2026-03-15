use ::std::fmt::Display;

use super::{
  Module, Type, Value, ValueData, ValueID,
  instruction::{self as inst, Instruction},
  module, types,
};

impl Display for types::Function<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for types::Array<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for types::Struct<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl Display for Type<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Void => write!(f, "void"),
      Self::Label => write!(f, "label"),
      Self::Float => write!(f, "float"),
      Self::Double => write!(f, "double"),
      Self::Pointer => write!(f, "ptr"),
      Self::Integer(bit_width) => write!(f, "i{bit_width}"),
      Self::Array(array) => array.fmt(f),
      Self::Function(function) => function.fmt(f),
      Self::Struct(s) => s.fmt(f),
    }
  }
}
