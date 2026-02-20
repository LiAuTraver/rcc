//! this file would be furthur split into multiple files when more impls are added.

use super::{Primitive, QualifiedType, Type};
use crate::common::StrRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pointer<'context> {
  pub pointee: QualifiedType<'context>,
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpressionId {
  id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ::strum_macros::Display)]
pub enum ArraySize {
  Constant(usize),
  /// unspecified size
  Incomplete,
  /// unsupported dynamic size, but i kept it here for the `full` type category
  ///
  /// TODO: if this holds an expression -- it's a cyclic reference of mod `type` and mod `analyzer::expression`. may use `ExpressionId` as a workaround.
  Variable(ExpressionId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Array<'context> {
  /// Array itself cannot have qualifiers,
  /// hence the QualifiedType::qualifiers of the whole array should be empty,
  /// the actual element type's qualifiers are stored here.
  pub element_type: QualifiedType<'context>,
  pub size: ArraySize,
  // These are not elem's, but the arraysize's. static is a hint for optimization,etc. dont care it for now.
  // pub qualifiers: Qualifiers,
  // pub is_static: bool,
}

/// function types themselves don't have qualifiers, but pointers to them can.
/// so the functionproto's qualifiers must be dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionProto<'context> {
  pub return_type: QualifiedType<'context>,
  pub parameter_types: &'context [QualifiedType<'context>],
  pub is_variadic: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Field<'context> {
  pub name: StrRef<'context>,
  pub field_type: QualifiedType<'context>,
}

// ignore unnamed/anonymous structs/unions for now
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Record<'context> {
  pub name: Option<StrRef<'context>>,
  pub fields: &'context [Field<'context>],
}

// seems not so much difference between struct and union here, but for convenience we keep them separate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Union<'context> {
  pub name: Option<StrRef<'context>>,
  pub fields: &'context [Field<'context>],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnumConstant<'context> {
  pub name: StrRef<'context>,
  pub value: Option<isize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Enum<'context> {
  pub name: Option<StrRef<'context>>,
  pub constants: &'context [EnumConstant<'context>],
  pub underlying_type: Primitive, // must be integer type
}

impl<'context> Pointer<'context> {
  pub fn new(pointee: QualifiedType<'context>) -> Self {
    Self { pointee }
  }
}

impl<'context> Array<'context> {
  pub fn new(element_type: QualifiedType<'context>, size: ArraySize) -> Self {
    Self { element_type, size }
  }
}
impl<'context> FunctionProto<'context> {
  pub fn new(
    return_type: QualifiedType<'context>,
    parameter_types: &'context [QualifiedType<'context>],
    is_variadic: bool,
  ) -> Self {
    Self {
      return_type,
      parameter_types,
      is_variadic,
    }
  }
}
impl<'context> Enum<'context> {
  pub fn new(
    name: Option<StrRef<'context>>,
    constants: &'context [EnumConstant<'context>],
    underlying_type: Primitive,
  ) -> Self {
    assert!(underlying_type.is_integer());
    Self {
      name,
      constants,
      underlying_type,
    }
  }

  pub fn into_underlying_type(self) -> Primitive {
    self.underlying_type
  }
}

// macro_rules! to_qualified_type {
//   ($ty:ty) => {
//     impl<'context> From<$ty> for QualifiedType<'context> {
//       fn from(value: $ty) -> Self {
//         QualifiedType::new_unqualified(Type::from(value).into())
//       }
//     }

//     impl<'context> From<$ty> for Box<QualifiedType<'context>> {
//       fn from(value: $ty) -> Self {
//         Box::new(QualifiedType::from(value))
//       }
//     }
//   };
// }

// to_qualified_type!(Primitive);
// to_qualified_type!(Array<'context>);
// to_qualified_type!(Pointer<'context>);
// to_qualified_type!(FunctionProto<'context>);
// to_qualified_type!(Enum<'context>);
// to_qualified_type!(Record<'context>);
// to_qualified_type!(Union<'context>);

::rcc_utils::interconvert!(Primitive, Type<'context>);
::rcc_utils::interconvert!(Array, Type, 'context);
::rcc_utils::interconvert!(Pointer, Type, 'context);
::rcc_utils::interconvert!(FunctionProto, Type, 'context);
::rcc_utils::interconvert!(Enum, Type, 'context);
::rcc_utils::interconvert!(Record, Type, 'context);
::rcc_utils::interconvert!(Union, Type, 'context);

::rcc_utils::make_trio_for!(Primitive, Type<'context>);
::rcc_utils::make_trio_for!(Array, Type, 'context);
::rcc_utils::make_trio_for!(Pointer, Type, 'context);
::rcc_utils::make_trio_for!(FunctionProto, Type, 'context);
::rcc_utils::make_trio_for!(Enum, Type, 'context);
::rcc_utils::make_trio_for!(Record, Type, 'context);
::rcc_utils::make_trio_for!(Union, Type, 'context);
