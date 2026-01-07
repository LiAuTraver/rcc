use crate::common::types::{
  Array, ArraySize, Primitive, QualifiedType, Record, Type, TypeInfo, Union,
};

impl TypeInfo for QualifiedType {
  fn size(&self) -> usize {
    self.unqualified_type.size()
  }
}

impl TypeInfo for Type {
  fn size(&self) -> usize {
    match self {
      Type::Primitive(p) => p.size(),
      Type::Pointer(_) => Primitive::ULongLong.size(),
      Type::Enum(_) => Primitive::LongLong.size(),
      Type::Record(r) => r.size(),
      Type::Union(u) => u.size(),
      Type::Array(a) => a.size(),
      Type::FunctionProto(_) => 0,
    }
  }
}

impl TypeInfo for Primitive {
  fn size(&self) -> usize {
    // x86_64 sizes
    match self {
      Primitive::Char => 1,
      Primitive::SChar => 1,
      Primitive::Short => 2,
      Primitive::Int => 4,
      Primitive::Long => 8,
      Primitive::LongLong => 8,
      Primitive::UChar => 1,
      Primitive::UShort => 2,
      Primitive::UInt => 4,
      Primitive::ULong => 8,
      Primitive::ULongLong => 8,
      Primitive::Float => 4,
      Primitive::Double => 8,
      Primitive::LongDouble => 8,
      Primitive::Void => 0,
      Primitive::Bool => 1,
      Primitive::ComplexFloat => 8,
      Primitive::ComplexDouble => 16,
      Primitive::ComplexLongDouble => 16,
    }
  }
}

impl TypeInfo for Array {
  fn size(&self) -> usize {
    match &self.size {
      ArraySize::Constant(sz) => sz * self.element_type.unqualified_type.size(),
      ArraySize::Incomplete => 0,
    }
  }
}

impl TypeInfo for Record {
  fn size(&self) -> usize {
    self
      .fields
      .iter()
      .map(|f| f.field_type.unqualified_type.size())
      .sum() // rough, padding and alignment not considered -- incomplete type has no members anyway so this handles it too
  }
}

impl TypeInfo for Union {
  fn size(&self) -> usize {
    self
      .fields
      .iter()
      .map(|f| f.field_type.unqualified_type.size())
      .max()
      .unwrap_or(0) // ditto
  }
}
