use ::bumpalo::Bump;
use ::std::{cell::RefCell, collections::HashSet};

use super::{
  Type, TypeRef,
  types::{Array, Function},
};
/// Although the lifetime speficier here is `'ir`, but it should actually be the same as `'context` in [`crate::session::Session`] who owns it.
#[derive(Debug)]
pub struct Context<'ir> {
  void_type: TypeRef<'ir>,
  label_type: TypeRef<'ir>,
  float32_type: TypeRef<'ir>,
  float64_type: TypeRef<'ir>,
  pointer_type: TypeRef<'ir>,
  common_integer_types: [TypeRef<'ir>; 6],

  type_interner: RefCell<HashSet<TypeRef<'ir>>>,
  arena: &'ir Bump,
}

impl<'ir> Context<'ir> {
  pub fn new(arena: &'ir Bump) -> Self {
    let this = Self {
      void_type: arena.alloc(Type::Void),
      label_type: arena.alloc(Type::Label),
      float32_type: arena.alloc(Type::Float),
      float64_type: arena.alloc(Type::Double),
      pointer_type: arena.alloc(Type::Pointer),
      common_integer_types: [
        arena.alloc(Type::Integer(1)),
        arena.alloc(Type::Integer(8)),
        arena.alloc(Type::Integer(16)),
        arena.alloc(Type::Integer(32)),
        arena.alloc(Type::Integer(64)),
        arena.alloc(Type::Integer(128)),
      ],

      type_interner: Default::default(),
      arena,
    };
    {
      let mut refmut = this.type_interner.borrow_mut();
      refmut.insert(this.void_type);
      refmut.insert(this.label_type);
      refmut.insert(this.float32_type);
      refmut.insert(this.float64_type);
      refmut.insert(this.pointer_type);
      this.common_integer_types.iter().for_each(|&t| {
        refmut.insert(t);
      });
    }
    this
  }
}
impl<'ir> Context<'ir> {
  pub fn void_type(&self) -> TypeRef<'ir> {
    self.void_type
  }

  pub fn label_type(&self) -> TypeRef<'ir> {
    self.label_type
  }

  pub fn float32_type(&self) -> TypeRef<'ir> {
    self.float32_type
  }

  pub fn float64_type(&self) -> TypeRef<'ir> {
    self.float64_type
  }

  pub fn pointer_type(&self) -> TypeRef<'ir> {
    self.pointer_type
  }

  fn do_intern(&self, value: Type<'ir>) -> TypeRef<'ir> {
    if let Some(existing) = self.type_interner.borrow().get(&value) {
      existing
    } else {
      let interned = self.arena.alloc(value);
      self.type_interner.borrow_mut().insert(interned);
      interned
    }
  }

  pub fn intern<T: Into<Type<'ir>>>(&self, value: T) -> TypeRef<'ir> {
    self.do_intern(value.into())
  }

  pub fn make_integer(&self, bits: u8) -> TypeRef<'ir> {
    match bits {
      1 => self.common_integer_types[0],
      8 => self.common_integer_types[1],
      16 => self.common_integer_types[2],
      32 => self.common_integer_types[3],
      64 => self.common_integer_types[4],
      128 => self.common_integer_types[5],
      _ => self.intern(Type::Integer(bits)),
    }
  }

  pub fn make_array(
    &self,
    element_type: TypeRef<'ir>,
    length: usize,
  ) -> TypeRef<'ir> {
    self.intern(Array::new(element_type, length))
  }

  pub fn make_function(
    &self,
    result_type: TypeRef<'ir>,
    params: &'ir [TypeRef<'ir>],
    is_variadic: bool,
  ) -> TypeRef<'ir> {
    self.intern(Function::new(result_type, params, is_variadic))
  }
}

use crate::types;
impl<'ir> Context<'ir> {
  pub fn ir_type(
    &self,
    qualified_type: &types::QualifiedType<'ir>,
  ) -> TypeRef<'ir> {
    use Primitive::*;
    use types::{Primitive, TypeInfo};
    match qualified_type.unqualified_type {
      types::Type::Primitive(primitive) => match primitive {
        Float => self.float32_type,
        Double => self.float64_type,
        Void => self.void_type,
        Nullptr => self.pointer_type,
        integer @ (Bool | Char | SChar | Short | Int | Long | LongLong
        | UChar | UShort | UInt | ULong | ULongLong) =>
          self.make_integer(integer.size_bits() as u8),
        placeholder @ (LongDouble | ComplexFloat | ComplexDouble
        | ComplexLongDouble) => todo!("{placeholder:#?} not implemented"),
      },
      types::Type::Pointer(pointer) => self.pointer_type,
      types::Type::Array(array) => self.make_array(
        self.ir_type(&array.element_type),
        match array.size {
          types::ArraySize::Constant(c) => c,
          types::ArraySize::Incomplete | types::ArraySize::Variable(_) => 0,
        },
      ),
      types::Type::FunctionProto(function_proto) => self.make_function(
        self.ir_type(&function_proto.return_type),
        self.arena.alloc_slice_fill_iter(
          function_proto
            .parameter_types
            .iter()
            .map(|t| self.ir_type(t)),
        ),
        function_proto.is_variadic,
      ),
      types::Type::Enum(_) => todo!(),
      types::Type::Record(_) => todo!(),
      types::Type::Union(_) => todo!(),
    }
  }
}
