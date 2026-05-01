use ::rcc_adt::{Alignment, Floating, Integral, Signedness, Size, SizeBit};
use Signedness::*;

use super::{
  Array, ArraySize, Enum, FunctionProto, Pointer,
  Primitive::{self, *},
  QualifiedType, Record, Type, Union,
};
use crate::{Constant, TargetInfo};

pub const trait TypeInfo<'c> {
  #[must_use]
  fn size(&self, target_info: &TargetInfo) -> Size;
  #[must_use]
  fn size_bits(&self, target_info: &TargetInfo) -> SizeBit;
  #[must_use]
  fn default_value(&self, target_info: &TargetInfo) -> Constant<'c>;
  #[must_use]
  fn extent(&self) -> usize;
  #[must_use]
  fn alignment(&self, target_info: &TargetInfo) -> Alignment;
  #[must_use]
  fn signedness(&self, target_info: &TargetInfo) -> Option<Signedness>;
  #[inline(always)]
  #[must_use]
  fn is_complete(&self, target_info: &TargetInfo) -> bool {
    self.size(target_info).get() != 0
  }
  #[inline(always)]
  #[must_use]
  fn is_scalar(&self) -> bool {
    self.extent() == 1
  }
}

impl<'c> TypeInfo<'c> for QualifiedType<'c> {
  #[inline(always)]
  fn size(&self, target_info: &TargetInfo) -> Size {
    self.unqualified_type.size(target_info)
  }

  #[inline(always)]
  fn size_bits(&self, target_info: &TargetInfo) -> SizeBit {
    self.unqualified_type.size_bits(target_info)
  }

  #[inline(always)]
  fn default_value(&self, target_info: &TargetInfo) -> Constant<'c> {
    self.unqualified_type.default_value(target_info)
  }

  #[inline(always)]
  fn extent(&self) -> usize {
    self.unqualified_type.extent()
  }

  #[inline(always)]
  fn signedness(&self, target_info: &TargetInfo) -> Option<Signedness> {
    self.unqualified_type.signedness(target_info)
  }

  #[inline(always)]
  fn alignment(&self, target_info: &TargetInfo) -> Alignment {
    self.unqualified_type.alignment(target_info)
  }
}
impl<'c> TypeInfo<'c> for Type<'c> {
  #[inline]
  fn size(&self, target_info: &TargetInfo) -> Size {
    ::rcc_utils::static_dispatch!(
      self,
      |variant| variant.size(target_info) =>
      Primitive Array Pointer FunctionProto Enum Record Union
    )
  }

  #[inline]
  fn size_bits(&self, target_info: &TargetInfo) -> SizeBit {
    ::rcc_utils::static_dispatch!(
      self,
      |variant| variant.size_bits(target_info) =>
      Primitive Array Pointer FunctionProto Enum Record Union
    )
  }

  #[inline]
  fn default_value(&self, target_info: &TargetInfo) -> Constant<'c> {
    ::rcc_utils::static_dispatch!(
      self,
      |variant| variant.default_value(target_info) =>
      Primitive Array Pointer FunctionProto Enum Record Union
    )
  }

  #[inline]
  fn extent(&self) -> usize {
    ::rcc_utils::static_dispatch!(
      self,
      |variant| variant.extent() =>
      Primitive Array Pointer FunctionProto Enum Record Union
    )
  }

  #[inline]
  fn alignment(&self, target_info: &TargetInfo) -> Alignment {
    ::rcc_utils::static_dispatch!(
      self,
      |variant| variant.alignment(target_info) =>
      Primitive Array Pointer FunctionProto Enum Record Union
    )
  }

  #[inline]
  fn signedness(&self, target_info: &TargetInfo) -> Option<Signedness> {
    ::rcc_utils::static_dispatch!(
      self,
      |variant| variant.signedness(target_info) =>
      Primitive Array Pointer FunctionProto Enum Record Union
    )
  }
}
impl<'c> const TypeInfo<'c> for Primitive {
  /// integral size should be aligned with method `Primitive::integer_width()`.
  fn size(&self, target_info: &TargetInfo) -> Size {
    // x86_64 sizes
    match self {
      Nullptr => target_info.pointer.size,
      Void => Size::U0,
      Bool => target_info.boolean.size,
      Char | SChar | UChar => target_info.character.size,
      UShort | Short => target_info.short.size,
      Int | UInt => target_info.int.size,
      Long => target_info.long.size,
      LongLong => target_info.long_long.size,
      ULong => target_info.long.size,
      ULongLong => target_info.long_long.size,
      Float => target_info.float.size,
      Double => target_info.double.size,
      LongDouble => target_info.long_double.size,
      ComplexFloat => (target_info.float.size.get() * 2).into(),
      ComplexDouble => (target_info.double.size.get() * 2).into(),
      ComplexLongDouble => (target_info.long_double.size.get() * 2).into(),
      __IRBit => panic!("invalid call"),
    }
  }

  #[inline]
  fn size_bits(&self, target_info: &TargetInfo) -> SizeBit {
    match self {
      __IRBit => SizeBit::U1,
      _ => SizeBit::from(self.size(target_info)),
    }
  }

  #[inline]
  fn default_value(&self, target_info: &TargetInfo) -> Constant<'c> {
    match self {
      Nullptr => Constant::Nullptr(),
      Void => panic!("void type has no value"),
      _ if self.is_integer() => Constant::Integral(Integral::new(
        0,
        self.size_bits(target_info),
        self
          .signedness(target_info)
          .expect("type has no signedness"),
      )),
      _ if self.is_floating_point() =>
        Constant::Floating(Floating::zero(self.floating_format())),
      _ => unreachable!(),
    }
  }

  #[inline(always)]
  fn extent(&self) -> usize {
    use Primitive::*;
    match self {
      Void => 0,
      _ => 1,
    }
  }

  fn alignment(&self, target_info: &TargetInfo) -> Alignment {
    match self {
      Nullptr => target_info.pointer.alignment,
      Void => Alignment::MIN,
      Bool => target_info.boolean.alignment,
      Char | SChar | UChar => target_info.character.alignment,
      UShort | Short => target_info.short.alignment,
      Int | UInt => target_info.int.alignment,
      Long | ULong => target_info.long.alignment,
      LongLong | ULongLong => target_info.long_long.alignment,
      Float => target_info.float.alignment,
      Double => target_info.double.alignment,
      LongDouble => target_info.long_double.alignment,
      ComplexFloat => target_info.float.alignment, // FIXME: may be different
      ComplexDouble => target_info.double.alignment, // FIXME: may be different
      ComplexLongDouble => target_info.long_double.alignment, // FIXME: may be different
      __IRBit => Alignment::O0,
    }
  }

  fn signedness(&self, target_info: &TargetInfo) -> Option<Signedness> {
    use Primitive::*;
    match self {
      Void => None,
      Nullptr => Some(Unsigned),
      Char => Some(target_info.char_signess),
      Bool | UChar | UShort | UInt | ULong | ULongLong => Some(Unsigned),
      SChar | Short | Int | Long | LongLong | Float | Double | LongDouble
      | ComplexFloat | ComplexDouble | ComplexLongDouble => Some(Signed),
      __IRBit =>
        panic!("ir bit i1 is unsigned, but you probably should not call this"),
    }
  }
}

impl<'c> TypeInfo<'c> for Array<'c> {
  fn size(&self, target_info: &TargetInfo) -> Size {
    match self.size {
      ArraySize::Constant(sz) =>
        self.element_type.unqualified_type.size(target_info) * sz,
      ArraySize::Incomplete => Size::U0,
      ArraySize::Variable(_id) => Size::U0, // ignore for now
    }
  }

  #[inline(always)]
  fn size_bits(&self, target_info: &TargetInfo) -> SizeBit {
    self.size(target_info).into()
  }

  #[inline]
  fn default_value(&self, _target_info: &TargetInfo) -> Constant<'c> {
    panic!("default value for non-scalar type should not be requested");
  }

  fn extent(&self) -> usize {
    self.element_type.extent() + 1
  }

  fn alignment(&self, target_info: &TargetInfo) -> Alignment {
    self.element_type.alignment(target_info)
  }

  fn signedness(&self, _target_info: &TargetInfo) -> Option<Signedness> {
    None
  }
}

impl<'c> TypeInfo<'c> for Record<'c> {
  fn size(&self, _target_info: &TargetInfo) -> Size {
    todo!()
  }

  #[inline(always)]
  fn size_bits(&self, target_info: &TargetInfo) -> SizeBit {
    SizeBit::from(self.size(target_info))
  }

  #[inline(always)]
  fn default_value(&self, _target_info: &TargetInfo) -> Constant<'c> {
    panic!("default value for non-scalar type should not be requested");
  }

  fn extent(&self) -> usize {
    1
  }

  fn alignment(&self, _target_info: &TargetInfo) -> Alignment {
    todo!()
  }

  #[inline(always)]
  fn signedness(&self, _target_info: &TargetInfo) -> Option<Signedness> {
    None
  }
}

impl<'c> TypeInfo<'c> for Union<'c> {
  fn size(&self, _target_info: &TargetInfo) -> Size {
    todo!()
    // self
    //   .fields
    //   .iter()
    //   .map(|f| f.field_type.unqualified_type.size())
    //   .max()
    //   .unwrap_or(0) // ditto
  }

  #[inline(always)]
  fn size_bits(&self, target_info: &TargetInfo) -> SizeBit {
    SizeBit::from(self.size(target_info))
  }

  #[inline(always)]
  fn default_value(&self, _target_info: &TargetInfo) -> Constant<'c> {
    panic!("default value for non-scalar type should not be requested");
  }

  #[inline(always)]
  fn extent(&self) -> usize {
    1
  }

  fn alignment(&self, _target_info: &TargetInfo) -> Alignment {
    todo!()
  }

  #[inline(always)]
  fn signedness(&self, _target_info: &TargetInfo) -> Option<Signedness> {
    None
  }
}
impl<'c> const TypeInfo<'c> for Pointer<'c> {
  #[inline(always)]
  fn size(&self, target_info: &TargetInfo) -> Size {
    target_info.pointer.size
  }

  #[inline(always)]
  fn size_bits(&self, target_info: &TargetInfo) -> SizeBit {
    SizeBit::from(self.size(target_info))
  }

  #[inline(always)]
  fn default_value(&self, _target_info: &TargetInfo) -> Constant<'c> {
    Constant::Nullptr()
  }

  #[inline(always)]
  fn extent(&self) -> usize {
    1
  }

  #[inline(always)]
  fn alignment(&self, target_info: &TargetInfo) -> Alignment {
    target_info.pointer.alignment
  }

  #[inline(always)]
  fn signedness(&self, _target_info: &TargetInfo) -> Option<Signedness> {
    Some(Unsigned)
  }
}

impl<'c> TypeInfo<'c> for FunctionProto<'c> {
  /// function types have no size
  /// and it is invalid to use `sizeof` w.r.t. `void`, `function designator` and `incomplete type`
  ///
  /// Clang returns `1` -- and previously here it returns `0` but it causes me lots of trouble.
  /// So now it returns `1`.
  #[inline(always)]
  fn size(&self, _target_info: &TargetInfo) -> Size {
    Size::U8
  }

  #[inline(always)]
  fn size_bits(&self, target_info: &TargetInfo) -> SizeBit {
    SizeBit::from(self.size(target_info))
  }

  #[inline(always)]
  fn default_value(&self, _target_info: &TargetInfo) -> Constant<'c> {
    panic!("default value for non-scalar type should not be requested");
  }

  /// not meaningful.
  #[inline(always)]
  fn extent(&self) -> usize {
    0
  }

  /// not meaningful either.
  #[inline(always)]
  fn alignment(&self, _target_info: &TargetInfo) -> Alignment {
    Alignment::MIN
  }

  #[inline(always)]
  fn signedness(&self, _target_info: &TargetInfo) -> Option<Signedness> {
    None
  }
}
impl<'c> TypeInfo<'c> for Enum<'c> {
  #[inline(always)]
  fn size(&self, target_info: &TargetInfo) -> Size {
    self.underlying_type.size(target_info)
  }

  #[inline(always)]
  fn size_bits(&self, target_info: &TargetInfo) -> SizeBit {
    SizeBit::from(self.size(target_info))
  }

  #[inline(always)]
  fn default_value(&self, target_info: &TargetInfo) -> Constant<'c> {
    self.underlying_type.default_value(target_info)
  }

  #[inline(always)]
  fn extent(&self) -> usize {
    1
  }

  #[inline(always)]
  fn alignment(&self, target_info: &TargetInfo) -> Alignment {
    self.underlying_type.alignment(target_info)
  }

  #[inline(always)]
  fn signedness(&self, target_info: &TargetInfo) -> Option<Signedness> {
    self.underlying_type.signedness(target_info)
  }
}
