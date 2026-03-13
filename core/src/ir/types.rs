/// IR Type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type<'ir> {
  Void,
  Label,
  Float,
  Double,

  Pointer,
  Integer(u8),
  Array(Array<'ir>),
  Function(Function<'ir>),
  // TODO: complete it later, placeholder now vvv
  Struct(Struct<'ir>),
}

pub type TypeRef<'ir> = &'ir Type<'ir>;
pub type TypeRefMut<'ir> = &'ir mut Type<'ir>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct Array<'ir> {
  element_type: TypeRef<'ir>,
  length: usize,
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
    result_type: TypeRef<'ir>,
    params: &'ir [TypeRef<'ir>],
    is_variadic: bool,
  ) -> Self {
    Self {
      return_type: result_type,
      params,
      is_variadic,
    }
  }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Struct<'ir> {
  _placeholder: &'ir ::std::marker::PhantomData<i8>,
}
use ::rcc_utils::{interconvert, make_trio_for};

interconvert!(Array, Type, 'ir);
interconvert!(Function, Type, 'ir);
interconvert!(Struct, Type, 'ir);

make_trio_for!(Array, Type, 'ir);
make_trio_for!(Function, Type, 'ir);
make_trio_for!(Struct, Type, 'ir);

impl<'ir> Type<'ir> {
  #[inline]
  pub fn ref_eq(lhs: TypeRef<'ir>, rhs: TypeRef<'ir>) -> bool {
    if cfg!(debug_assertions) && !::std::ptr::eq(lhs, rhs) && lhs == rhs {
      eprintln!(
        "INTERNAL INVARIANT: comparing types by pointer but they are actually \
         the same: {:p}: {:?} and {:p}: {:?}.",
        lhs, lhs, rhs, rhs
      );
      return true;
    }
    ::std::ptr::eq(lhs, rhs)
  }
}
