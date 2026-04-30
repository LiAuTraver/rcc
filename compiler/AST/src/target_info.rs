use ::rcc_adt::{Alignment, Signedness, Size, SizeBit};
use ::rcc_shared::{DataModel, Triple};
#[derive(Debug)]
pub struct TypeMeta {
  pub size: Size,
  pub alignment: Alignment,
}
impl TypeMeta {
  const fn new(size: Size, alignment: Alignment) -> Self {
    Self { size, alignment }
  }

  const fn from_size(size: Size) -> Self {
    Self::new(size, Alignment::from_size_ceil(size))
  }
}
impl TypeMeta {
  pub const fn size_bits(&self) -> SizeBit {
    self.size.size_bits()
  }
}

#[derive(Debug)]

pub struct TargetInfo {
  pub pointer: TypeMeta,
  pub boolean: TypeMeta,
  pub short: TypeMeta,
  pub int: TypeMeta,
  pub long: TypeMeta,
  pub long_long: TypeMeta,
  pub float: TypeMeta,
  pub double: TypeMeta,
  pub long_double: TypeMeta,

  pub character: TypeMeta,
  pub char_signess: Signedness,
}

impl TargetInfo {
  pub const HOST: TargetInfo = Self::new(Triple::HOST);

  pub const fn new(triple: Triple) -> Self {
    let data_model: DataModel =
      triple.data_model().expect("no layout for host!");
    let pointer: TypeMeta = TypeMeta::from_size(data_model.pointer_width());
    let boolean: TypeMeta = TypeMeta::from_size(Size::U8);
    let short: TypeMeta = TypeMeta::from_size(data_model.short_size());
    let int: TypeMeta = TypeMeta::from_size(data_model.int_size());
    let long: TypeMeta = TypeMeta::from_size(data_model.long_size());
    let long_long: TypeMeta = TypeMeta::from_size(data_model.long_long_size());
    let float: TypeMeta = TypeMeta::from_size(data_model.float_size());
    let double: TypeMeta = TypeMeta::from_size(data_model.double_size());
    // FIXME: double here.
    let long_double: TypeMeta = TypeMeta::from_size(data_model.double_size());
    let character: TypeMeta = TypeMeta::from_size(data_model.char_size());
    let char_signess = data_model.char_signess();

    Self {
      pointer,
      boolean,
      short,
      int,
      long,
      long_long,
      float,
      double,
      long_double,

      character,
      char_signess,
    }
  }
}
