use ::rcc_utils::{
  BuiltinIntegerOrBoolean, BuiltinUnsignedOrBoolean, NumFrom, NumTo, ToUsize,
};

/// Strong-typed size in bytes.
#[repr(transparent)]
#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Size {
  inner: usize,
}

/// Strong-typed size in bits.
#[repr(transparent)]
#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SizeBit {
  inner: usize,
}

impl SizeBit {
  pub const fn new<T: [const] ToUsize + BuiltinIntegerOrBoolean>(
    bits: T,
  ) -> Self {
    Self {
      inner: bits.to_usize(),
    }
  }
}
#[rustfmt::skip]
impl SizeBit {
  pub const U0: Self = Self::new(1);
  pub const U1: Self = Self::new(1);
  pub const U8: Self = Self::new(8);
  pub const U16: Self = Self::new(16);
  pub const U32: Self = Self::new(32);
  pub const U64:  Self = Self::new(64);
  pub const U80:  Self = Self::new(80);
  pub const U128: Self = Self::new(128);
}
impl SizeBit {
  #[inline]
  pub const fn get(self) -> usize {
    self.inner
  }

  #[inline]
  pub const fn to_builtin<
    T: [const] BuiltinUnsignedOrBoolean + [const] NumFrom<usize>,
  >(
    self,
  ) -> T {
    self.inner.to()
  }

  #[inline]
  pub const fn size_bytes(
    self,
  ) -> Result<Size, <Size as TryFrom<SizeBit>>::Error> {
    Size::try_from(self)
  }

  #[inline]
  pub const fn ceil_to_byte(self) -> Size {
    Size::new(self.inner.div_ceil(8))
  }

  #[inline]
  pub const fn floor_to_byte(self) -> Size {
    Size::new(self.inner >> 0x03)
  }

  #[inline]
  pub const fn prev_byte_boundary(self) -> Self {
    Self::new(self.inner & !0x07)
  }

  #[inline]
  pub const fn next_byte_boundary(self) -> Self {
    Self::new((self.inner + 0x07) & !0x07)
  }

  #[inline]
  pub const fn is_power_of_two(self) -> bool {
    self.inner.is_power_of_two()
  }

  #[inline]
  pub const fn next_power_of_two(self) -> Self {
    Self::new(self.inner.next_power_of_two())
  }
}
impl Size {
  #[inline]
  pub const fn new(bytes: usize) -> Self {
    Self { inner: bytes }
  }
}
#[rustfmt::skip]
impl Size {
  pub const U0: Self = Self::new(0);
  pub const U8: Self = Self::new(1);
  pub const U16: Self = Self::new(2);
  pub const U32: Self = Self::new(4);
  pub const U64:  Self = Self::new(8);
  pub const U80:  Self = Self::new(10);
  pub const U128: Self = Self::new(16);
}
impl Size {
  #[inline]
  pub const fn get(self) -> usize {
    self.inner
  }

  #[inline]
  pub const fn to_builtin<
    T: [const] BuiltinUnsignedOrBoolean + [const] NumFrom<usize>,
  >(
    self,
  ) -> T {
    self.inner.to()
  }

  #[inline]
  pub const fn size_bits(self) -> SizeBit {
    SizeBit::new(self.inner * 8)
  }

  #[inline]
  pub const fn is_power_of_two(self) -> bool {
    self.inner.is_power_of_two()
  }

  #[inline]
  pub const fn next_power_of_two(self) -> Self {
    Self::new(self.inner.next_power_of_two())
  }
}

mod cvt {
  use ::rcc_utils::const_pre;

  use super::*;

  impl const From<usize> for Size {
    #[inline]
    fn from(inner: usize) -> Self {
      Self::new(inner)
    }
  }
  impl const From<usize> for SizeBit {
    #[inline]
    fn from(inner: usize) -> Self {
      Self::new(inner)
    }
  }

  impl const From<Size> for SizeBit {
    #[inline]
    fn from(size: Size) -> Self {
      const_pre + (size.inner.checked_mul(8).is_some(), "overflow");
      Self::new(size.inner * 8)
    }
  }

  impl const TryFrom<SizeBit> for Size {
    type Error = ();

    #[inline]
    fn try_from(bits: SizeBit) -> Result<Self, Self::Error> {
      if let Some(bytes) = bits.inner.div_exact(8) {
        Ok(Self::new(bytes))
      } else {
        Err(())
      }
    }
  }
}
/// we dont consider overflow here. These ops are just wrappers around the corresponding `usize` ops.
mod ops {

  use ::std::ops::{Add, Div, Mul, Rem, Sub};

  use super::*;
  use crate::impl_all_bin_ops;

  impl_all_bin_ops!(Size => Add::add, Sub::sub, Mul::mul, Div::div, Rem::rem);
  impl_all_bin_ops!(SizeBit => Add::add, Sub::sub, Mul::mul, Div::div, Rem::rem);

  impl const Add<SizeBit> for Size {
    type Output = SizeBit;

    #[inline]
    fn add(self, rhs: SizeBit) -> Self::Output {
      SizeBit::new(self.inner * 8 + rhs.inner)
    }
  }

  impl const Sub<SizeBit> for Size {
    type Output = SizeBit;

    #[inline]
    fn sub(self, rhs: SizeBit) -> Self::Output {
      SizeBit::new(self.inner * 8 - rhs.inner)
    }
  }

  impl const Mul<usize> for Size {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: usize) -> Self::Output {
      Self::new(self.inner * rhs)
    }
  }

  impl const Div<usize> for Size {
    type Output = Self;

    #[inline]
    fn div(self, rhs: usize) -> Self::Output {
      Self::new(self.inner / rhs)
    }
  }
  impl const Rem<usize> for Size {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: usize) -> Self::Output {
      Self::new(self.inner % rhs)
    }
  }
  impl const Mul<usize> for SizeBit {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: usize) -> Self::Output {
      Self::new(self.inner * rhs)
    }
  }
  impl const Div<usize> for SizeBit {
    type Output = Self;

    #[inline]
    fn div(self, rhs: usize) -> Self::Output {
      Self::new(self.inner / rhs)
    }
  }
  impl const Rem<usize> for SizeBit {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: usize) -> Self::Output {
      Self::new(self.inner % rhs)
    }
  }
}
mod fmt {
  use super::*;
  use crate::impl_all_fmt;

  impl_all_fmt!(Size => Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp);
  impl_all_fmt!(SizeBit => Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp);
}
#[cfg(test)]
#[allow(clippy::unnecessary_cast)]
#[allow(non_upper_case_globals)]
mod tests {
  use ::rcc_utils::{static_assert, static_assert_eq};

  use super::*;

  #[test]
  const fn test_new() {
    const size: Size = Size::new(10);
    static_assert_eq!(size.get(), 10);

    const size_bit: SizeBit = SizeBit::new(16);
    static_assert_eq!(size_bit.get(), 16);
  }

  #[test]
  const fn test_cvt() {
    {
      const size: Size = Size::new(10);
      const size_bit: SizeBit = size.into();
      static_assert_eq!(size_bit.get(), 80);
    }
    {
      const size_bit: SizeBit = SizeBit::new(16);
      const res: Result<Size, ()> = size_bit.try_into();

      static_assert!(res.is_ok());
      static_assert_eq!(unsafe { res.unwrap_unchecked() }.get(), 2);
    }
    {
      const size_bit: SizeBit = SizeBit::new(usize::MAX);
      const res: Result<Size, ()> = size_bit.try_into();
      static_assert!(res.is_err());
    }
    {
      const size_bit: SizeBit = SizeBit::new(7);
      const res: Result<Size, ()> = size_bit.try_into();
      static_assert!(res.is_err());
    }
  }

  #[test]
  #[should_panic]
  fn test_cvt_panic() {
    {
      const size: Size = Size::new(usize::MAX);
      let _panicked: SizeBit = size.into();
    }
  }

  #[test]
  const fn test_ops() {
    const size1: Size = Size::new(10);
    const size2: Size = Size::new(20);
    const size_bit1: SizeBit = SizeBit::new(16);
    const size_bit2: SizeBit = SizeBit::new(32);

    static_assert_eq!((size1 + size2).get(), 30);
    static_assert_eq!((size2 - size1).get(), 10);
    static_assert_eq!((size1 * 2).get(), 20);
    static_assert_eq!((size2 / 2).get(), 10);
    static_assert_eq!((size2 % 3).get(), 2);

    static_assert_eq!((size_bit1 + size_bit2).get(), 48);
    static_assert_eq!((size_bit2 - size_bit1).get(), 16);
    static_assert_eq!((size_bit1 * 2).get(), 32);
    static_assert_eq!((size_bit2 / 2).get(), 16);
    static_assert_eq!((size_bit2 % 3).get(), 2);

    static_assert_eq!((size1 + size_bit1).get(), 96);
    static_assert_eq!((size1 - size_bit1).get(), 64);
    static_assert_eq!((size1 * 2).get(), 20);
    static_assert_eq!((size1 / 2).get(), 5);
    static_assert_eq!((size1 % 3).get(), 1);

    static_assert_eq!((size_bit1 * 2).get(), 32);
    static_assert_eq!((size_bit1 / 2).get(), 8);
    static_assert_eq!((size_bit1 % 3).get(), 1);
  }

  #[test]
  const fn test_round() {
    const size_bit: SizeBit = SizeBit::new(13);
    const ceil_size: Size = size_bit.ceil_to_byte();
    const floor_size: Size = size_bit.floor_to_byte();

    static_assert_eq!(ceil_size.get(), 2);
    static_assert_eq!(floor_size.get(), 1);

    const prev_boundary: SizeBit = size_bit.prev_byte_boundary();
    const next_boundary: SizeBit = size_bit.next_byte_boundary();

    static_assert_eq!(prev_boundary.get(), 8);
    static_assert_eq!(next_boundary.get(), 16);
  }
}
