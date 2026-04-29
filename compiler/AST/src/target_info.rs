use ::rcc_adt::{Alignment, Size, SizeBit};
use ::rcc_shared::{DataModel, Triple};
use ::std::ops::Deref;
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
    Self::new(size, Alignment::from_align(size.get()))
  }
}
impl TypeMeta {
  pub const fn size_bits(&self) -> SizeBit {
    self.size.size_bits()
  }
}
#[derive(Debug)]
pub struct TargetInfoMixin {
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
  pub is_char_signed: bool,
  pub min_array_align: Alignment,
}
#[derive(Debug)]

pub struct TargetInfo {
  triple: Triple,
  infos: TargetInfoMixin,
}
impl const Deref for TargetInfo {
  type Target = TargetInfoMixin;

  fn deref(&self) -> &Self::Target {
    &self.infos
  }
}

impl TargetInfo {
  pub fn triple(&self) -> &Triple {
    &self.triple
  }
}

impl TargetInfo {
  #[allow(non_upper_case_globals)]
  pub fn host() -> Self {
    const triple: Triple = Triple::HOST;
    const data_model: DataModel =
      triple.data_model().expect("no layout for host!");
    const pointer: TypeMeta = TypeMeta::from_size(data_model.pointer_width());
    // FIXME: lang dependent.
    const boolean: TypeMeta = TypeMeta::from_size(Size::U8);
    const short: TypeMeta = TypeMeta::from_size(data_model.short_size());
    const int: TypeMeta = TypeMeta::from_size(data_model.int_size());
    const long: TypeMeta = TypeMeta::from_size(data_model.long_size());
    const long_long: TypeMeta =
      TypeMeta::from_size(data_model.long_long_size());
    const float: TypeMeta = TypeMeta::from_size(data_model.float_size());
    const double: TypeMeta = TypeMeta::from_size(data_model.double_size());
    // FIXME: target dependent.
    const long_double: TypeMeta = double;
    // stack is always kept 16-byte aligned on Linux x86_64.
    const min_array_align: Alignment = Alignment::from_align(16);
    const character: TypeMeta = TypeMeta::from_size(data_model.char_size());
    const is_char_signed: bool = data_model.char_signess().is_signed();

    Self {
      triple,
      infos: TargetInfoMixin {
        pointer,
        boolean,
        short,
        int,
        long,
        long_long,
        float,
        double,
        long_double,
        min_array_align,
        character,
        is_char_signed,
      },
    }
  }
}
