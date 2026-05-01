use ::rcc_adt::{FloatFormat, Floating, Integral, Signedness};
use ::rcc_utils::{RefEq, ensure_is_pod};

use super::{
  Array, ArraySize, Enum, FunctionProto, Pointer, Primitive, Record, TypeInfo,
  Union,
};
use crate::{Constant, TargetInfo, context::Context};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Type<'c> {
  Primitive(Primitive),
  Pointer(Pointer<'c>),
  Array(Array<'c>),
  FunctionProto(FunctionProto<'c>),
  Enum(Enum<'c>),
  Record(Record<'c>),
  Union(Union<'c>),
}
/// Indicates a reference to [`Type`] which stores in the `'c`.
/// Call [`Type::ref_eq`] to check two [`Type`] are equal or not -- dont use [`Eq`]/`==`.
pub type TypeRef<'c> = &'c Type<'c>;
pub type TypeRefMut<'c> = &'c mut Type<'c>;

ensure_is_pod!(Type);
ensure_is_pod!(TypeRef);
impl<'c> Type<'c> {
  // fn is_unsigned(&self) -> bool {
  //   match self {
  //     Type::Primitive(p) => p.is_unsigned(),
  //     Type::Pointer(_) => true,
  //     Type::Enum(e) => e.underlying_type.is_unsigned(),
  //     _ => false,
  //   }
  // }

  // fn is_signed(&self) -> bool {
  //   match self {
  //     Type::Primitive(p) => p.is_signed(),
  //     Type::Enum(e) => e.underlying_type.is_signed(),
  //     _ => false,
  //   }
  // }

  // pub fn signedness(&self) -> Option<Signedness> {
  //   use Signedness::*;
  //   if self.is_signed() {
  //     Some(Signed)
  //   } else if self.is_unsigned() {
  //     Some(Unsigned)
  //   } else {
  //     None
  //   }
  // }
}
impl<'c> Type<'c> {
  pub fn is_modifiable(&self, target_info: &TargetInfo) -> bool {
    if self.size(target_info).get() == 0 {
      false
    } else {
      match self {
        Type::Array(_) => false,
        _ => true, // todo: struct/union with const member
      }
    }
  }

  pub fn is_void(&self) -> bool {
    matches!(self, Type::Primitive(Primitive::Void))
  }

  pub fn is_integer(&self) -> bool {
    match self {
      Type::Primitive(p) => p.is_integer(),
      _ => false,
    }
  }

  pub fn is_floating_point(&self) -> bool {
    match self {
      Type::Primitive(p) => p.is_floating_point(),
      _ => false,
    }
  }

  pub fn is_arithmetic(&self) -> bool {
    match self {
      Type::Primitive(p) => p.is_arithmetic(),
      _ => false,
    }
  }

  pub fn is_character_type(&self) -> bool {
    self
      .as_primitive()
      .is_some_and(Primitive::is_character_type)
  }

  pub fn lookup(self, context: &Context<'c>) -> TypeRef<'c> {
    context.intern(self)
  }
}
impl RefEq for Type<'_> {
  fn ref_eq(lhs: &Self, rhs: &Self) -> bool
  where
    Self: PartialEq + Sized + ::std::fmt::Debug,
  {
    Self::ref_eq_impl(
      lhs,
      rhs,
      "\nthis is a known bug for TypeRef -- 1. canonical type 2. array of \
       typeref would compare to false. and needs to be fixed.",
    )
  }
}
mod private {
  use super::{Constant, Floating, Integral};
  pub trait Sealed {}
  impl Sealed for Integral {}
  impl Sealed for Floating {}
  impl Sealed for Constant<'_> {}
}
pub trait UnqualExt<'c>: private::Sealed {
  fn unqualified_type(&self, context: &'c Context) -> TypeRef<'c>;
}

impl<'c> UnqualExt<'c> for Integral {
  fn unqualified_type(&self, context: &'c Context) -> TypeRef<'c> {
    use Signedness::*;
    let width = self.width().ceil_to_byte();
    match self.signedness() {
      Signed => {
        match width {
          _ if width == context.boolean.size => context.i8_bool_type(),
          _ if width == context.character.size => context.char_type(),
          _ if width == context.short.size => context.short_type(),
          _ if width == context.int.size => context.int_type(),
          _ if width == context.long.size => context.long_type(),
          _ if width == context.long_long.size => context.long_long_type(),
          _ => context.int_type(), // default
        }
      },
      Unsigned => {
        match width {
          _ if width == context.character.size => context.uchar_type(),
          _ if width == context.short.size => context.ushort_type(),
          _ if width == context.int.size => context.uint_type(),
          _ if width == context.long.size => context.ulong_type(),
          _ if width == context.long_long.size => context.ulong_long_type(),
          _ => context.uint_type(), // default
        }
      },
    }
  }
}

impl<'c> UnqualExt<'c> for Floating {
  fn unqualified_type(&self, context: &'c Context) -> TypeRef<'c> {
    use FloatFormat::*;
    match self.format() {
      IEEE32 => Context::float32_type(context),
      IEEE64 => Context::float64_type(context),
    }
  }
}

impl<'c> UnqualExt<'c> for Constant<'c> {
  fn unqualified_type(&self, context: &'c Context) -> TypeRef<'c> {
    match self {
      Self::Integral(integral) => integral.unqualified_type(context),
      Self::Floating(floating) => floating.unqualified_type(context),
      Self::String(str) => Context::make_array(
        context,
        context.char_type().into(),
        // '\0'.
        ArraySize::Constant(str.len().saturating_add(1)),
      ),
      Self::Nullptr() => Context::nullptr_type(context),
      Self::Address(_) => Context::voidptr_type(context),
      _ => todo!(),
    }
  }
}
