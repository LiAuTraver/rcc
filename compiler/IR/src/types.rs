use ::rcc_adt::{Alignment, FloatFormat, SizeBit};

use crate::DataLayout;

/// IR Type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type<'ir> {
  Void(),
  Label(),
  Floating(FloatFormat),

  Pointer(),
  Integer(SizeBit),
  Array(Array<'ir>),
  Function(Function<'ir>),
  // TODO: complete it later, placeholder now vvv
  Struct(Struct<'ir>),
}

impl<'ir> Type<'ir> {
  pub fn size_bits(&self, data_layout: &DataLayout) -> SizeBit {
    use Type::*;
    match self {
      Void() => SizeBit::U0,
      Label() => SizeBit::U0,
      Function(_) => SizeBit::U0, // function type itself does not occupy space.
      Pointer() => data_layout.pointer_specs.size_bits(),
      Floating(format) => format.size_bits(),
      Integer(width) => *width,
      Array(array) => array.element_type.size_bits(data_layout) * array.length,
      Struct(_) => unimplemented!(),
    }
  }

  pub fn alignment(&self, data_layout: &DataLayout) -> Alignment {
    use Type::*;
    match self {
      Void() => panic!("invalid call: void type has no alignment"),
      Label() => panic!("invalid call: label has no alignment"),
      Function(_) => panic!("invalid call: function type has no alignment"),
      Floating(format) => data_layout.float_specs(*format).align(),
      Pointer() => data_layout.pointer_specs.align(),
      Integer(size_bit) => data_layout.integer_specs(*size_bit).align(),
      Array(array) => array.element_type.alignment(data_layout),
      Struct(_) => todo!(),
    }
  }

  pub fn extent(&self) -> usize {
    use Type::*;
    match self {
      Void() => 0,
      Label() => 0,
      Floating(_) => 1,
      Pointer() => 1,
      Integer(_) => 1,
      Function(_) => 0,
      Struct(_) => 1,
      Array(array) => 1 + array.element_type.extent(),
    }
  }
}

pub type TypeRef<'ir> = &'ir Type<'ir>;
pub type TypeRefMut<'ir> = &'ir mut Type<'ir>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct Array<'ir> {
  pub(super) element_type: TypeRef<'ir>,
  pub(super) length: usize,
}

impl<'ir> Array<'ir> {
  pub fn new(element_type: TypeRef<'ir>, length: usize) -> Self {
    Self {
      element_type,
      length,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function<'ir> {
  pub return_type: TypeRef<'ir>,
  pub params: &'ir [TypeRef<'ir>],
  pub is_variadic: bool,
}

impl<'ir> Function<'ir> {
  pub fn new(
    return_type: TypeRef<'ir>,
    params: &'ir [TypeRef<'ir>],
    is_variadic: bool,
  ) -> Self {
    Self {
      return_type,
      params,
      is_variadic,
    }
  }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Struct<'ir> {
  _placeholder: &'ir ::std::marker::PhantomData<i8>,
}

mod cvt {

  use ::rcc_utils::{interconvert, make_trio_for, make_trio_for_unit_tuple};

  use super::*;

  interconvert!(Array, Type, 'ir);
  interconvert!(Function, Type, 'ir);
  interconvert!(Struct, Type, 'ir);
  interconvert!(SizeBit, Type<'ir>, Integer);

  make_trio_for_unit_tuple!(Void, Type<'ir>);
  make_trio_for_unit_tuple!(Label, Type<'ir>);
  make_trio_for_unit_tuple!(Pointer, Type<'ir>);

  make_trio_for!(SizeBit, Type<'ir>, Integer);
  make_trio_for!(FloatFormat, Type<'ir>, Floating);
  make_trio_for!(Array, Type, 'ir);
  make_trio_for!(Function, Type, 'ir);
  make_trio_for!(Struct, Type, 'ir);
}

mod fmt {
  use ::std::fmt::Display;

  use super::*;
  impl Display for Function<'_> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      todo!()
    }
  }
  impl Display for Array<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "[{} x {}]", self.length, self.element_type)
    }
  }
  impl Display for Struct<'_> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      todo!()
    }
  }

  impl Display for Type<'_> {
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
}
