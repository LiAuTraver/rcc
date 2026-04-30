use ::rcc_adt::{Alignment, FloatFormat, Size, SizeBit};
use ::rcc_shared::{Endianess, ObjectFormat, Triple};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq)]
pub enum SymbolDecoration {
  Unknown,

  ELF,
  COFF,
}

impl From<ObjectFormat> for SymbolDecoration {
  fn from(format: ObjectFormat) -> Self {
    match format {
      ObjectFormat::Unknown => Self::Unknown,
      ObjectFormat::ELF => Self::ELF,
      ObjectFormat::COFF => Self::COFF,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeSpecs {
  width: SizeBit,
  alignment: Alignment,
  /// This is not needed in my pipeline, but in order to get the llvm-compatible target layout...
  preferred_alignment: Alignment,
}
impl TypeSpecs {
  /// constructs a [`TypeSpecs`] from a given [`Size`],
  /// rounding up the alignment.
  pub fn from_size(size: Size) -> Self {
    Self::new(size.into(), Alignment::from_size_ceil(size))
  }

  /// constructs a [`TypeSpecs`] from a given [`SizeBit`],
  /// rounding up the alignment.
  pub fn from_size_bits(size_bits: SizeBit) -> Self {
    Self::new(size_bits, Alignment::from_size_bits_ceil(size_bits))
  }

  /// constructs a [`TypeSpecs`] from a given [`SizeBit`] and [`Alignment`].
  pub const fn new(width: SizeBit, alignment: Alignment) -> Self {
    Self {
      width,
      alignment,
      preferred_alignment: alignment,
    }
  }

  /// manually specified all three fields.
  pub const fn new_preferred(
    width: SizeBit,
    alignment: Alignment,
    preferred_alignment: Alignment,
  ) -> Self {
    Self {
      width,
      alignment,
      preferred_alignment,
    }
  }
}
impl TypeSpecs {
  #[inline]
  pub const fn size_bits(&self) -> SizeBit {
    self.width
  }

  #[inline]
  pub fn size(&self) -> Option<Size> {
    Size::try_from(self.width).ok()
  }

  #[inline]
  pub const fn align(&self) -> Alignment {
    self.alignment
  }

  #[inline]
  pub const fn preferred_align(&self) -> Alignment {
    self.preferred_alignment
  }
}
#[derive(Debug)]
pub struct DataLayout {
  pub endianess: Endianess,
  pub symbol_decoration: SymbolDecoration,

  pub pointer_specs: TypeSpecs,

  pub integer_specs: [TypeSpecs; 6],
  pub float_specs: [TypeSpecs; 2],

  pub stack_align: Alignment,

  _this_struct_is_not_designed_to_be_pod: Vec<TypeSpecs>,
  // TODO: struct specs
}
impl DataLayout {
  pub fn host() -> Self {
    Self::new(Triple::HOST)
  }

  pub fn new(triple: Triple) -> Self {
    let endianess = triple.endianness().unwrap_or_default();
    let symbol_decoration = triple.object_format.into();

    let data_model =
      triple.data_model().expect("failed to construct datamodel!");

    let pointer_specs = TypeSpecs::from_size(data_model.pointer_width());

    let integer_specs = [
      TypeSpecs::from_size_bits(SizeBit::U1),
      TypeSpecs::from_size(Size::U8),
      TypeSpecs::from_size(Size::U16),
      TypeSpecs::from_size(Size::U32),
      TypeSpecs::from_size(Size::U64),
      TypeSpecs::from_size(Size::U128),
    ];
    let float_specs = [
      TypeSpecs::from_size(Size::U32),
      TypeSpecs::from_size(Size::U64),
    ];

    fn strict_le(lhs: &TypeSpecs, rhs: &TypeSpecs) -> bool {
      lhs.size_bits() < rhs.size_bits()
    }

    debug_assert!(
      integer_specs.is_sorted_by(strict_le)
        && float_specs.is_sorted_by(strict_le)
    );

    Self {
      endianess,
      symbol_decoration,
      pointer_specs,
      integer_specs,
      float_specs,
      stack_align: Alignment::from_align_fixed::<128>(),
      // just a placeholder to disable `no drop`, we may make integer_specs a vec later, who knows.
      _this_struct_is_not_designed_to_be_pod: Vec::with_capacity(0),
    }
  }
}
impl DataLayout {
  fn float32_specs(&self) -> TypeSpecs {
    self.float_specs[0]
  }

  fn float64_specs(&self) -> TypeSpecs {
    self.float_specs[1]
  }

  pub fn float_specs(&self, format: FloatFormat) -> TypeSpecs {
    use FloatFormat::*;
    match format {
      IEEE32 => self.float32_specs(),
      IEEE64 => self.float64_specs(),
    }
  }

  pub fn integer_specs(&self, size_bit: SizeBit) -> TypeSpecs {
    for specs in self.integer_specs.iter() {
      if specs.size_bits() >= size_bit {
        return *specs;
      }
    }
    *self.integer_specs.last().expect("no specs!")
  }
}
mod fmt {
  use ::std::fmt;

  use super::*;

  impl fmt::Display for SymbolDecoration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      use SymbolDecoration::*;
      write!(
        f,
        "{}",
        match self {
          ELF => "e",
          COFF => "w",
          Unknown => "<didnt know the ABI>",
        }
      )
    }
  }
  #[inline]
  fn fmt_endianess(endianess: Endianess) -> &'static str {
    use Endianess::*;

    match endianess {
      Little => "e",
      Big => "E",
    }
  }

  impl fmt::Display for TypeSpecs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}:{}", self.size_bits(), self.align().size_bits())?;

      if self.preferred_align() != self.align() {
        write!(f, ":{}", self.preferred_align().size_bits())?;
      }
      Ok(())
    }
  }

  impl fmt::Display for DataLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(
        f,
        "{}-m:{}-p0:{}-",
        fmt_endianess(self.endianess),
        self.symbol_decoration,
        self.pointer_specs
      )?;

      let needs_print = |specs: &&TypeSpecs| {
        !specs.size_bits().get().is_multiple_of(8)
        // ^^^ / vvv size is a power of 2.
          || specs.size_bits() >= self.pointer_specs.size_bits()
          || specs.align()
            != Alignment::from_size_bits_unchecked(specs.size_bits())
          || specs.preferred_align() != specs.align()
      };

      // only print the integer specs that are eq/gt the pointer size
      self
        .integer_specs
        .iter()
        .skip(1) //  the first is always i1.
        .filter(needs_print)
        .try_for_each(|spec| write!(f, "i{}-", spec))?;

      self
        .float_specs
        .iter()
        .filter(needs_print)
        .try_for_each(|spec| write!(f, "f{}-", spec))?;

      // FIXME: currently just assume the legal integer width is the power of 2 and also >= 8, <= pointer_width.
      write!(
        f,
        "n{}-",
        (0..self.pointer_specs.size_bits().get().trailing_zeros() + 1)
          .map(|pow| 1usize << pow)
          .filter(|&width| width >= 8)
          .map(|width| width.to_string())
          .collect::<Vec<_>>()
          .join(":")
      )?;

      write!(f, "S{}", self.stack_align)
    }
  }
}
