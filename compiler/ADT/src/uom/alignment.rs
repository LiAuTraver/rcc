use ::rcc_utils::const_pre;
use ::std::num::NonZeroU8;

use super::{Size, SizeBit};

/// 6.2.8p4:  Every valid alignment value shall be a nonnegative integral power of two.
///
/// A tiny alignment type that can represent alignments from 1 to 256
/// (normally only up to 64 is used, nobody in sane would use more than 64 alignment).
///
/// The [`Self::flag`] acts like `onehot`, i.e., `log2(align) + 1` is stored in the flag.
/// It is represented as the exponent of two, e.g:
/// - the alignment of `1` is represented as `flag = 1`,
/// - the alignment of `4` is represented as `flag = 3`, and so on.
#[repr(transparent)]
#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Alignment {
  flag: NonZeroU8,
}

impl Alignment {
  #[inline]
  pub const fn from_align_fixed<const ALIGN: usize>() -> Self {
    const_pre + (ALIGN.is_power_of_two(), "ALIGN is not a power of two");
    unsafe { Self::from_flag_unchecked(Self::usize2u8flag(ALIGN)) }
  }

  #[inline]
  pub const fn from_size(size: Size) -> Option<Self> {
    if size.is_power_of_two() {
      Some(unsafe { Self::from_flag_unchecked(Self::usize2u8flag(size.get())) })
    } else {
      None
    }
  }

  #[inline]
  pub const fn from_size_unchecked(size: Size) -> Self {
    Self::from_align_unchecked(size.get())
  }

  /// constructs an alignment which is the smallest power of two greater than or equal to `align`.
  ///
  /// if the alignment is zero, it will be treated as one.
  #[inline]
  pub const fn from_size_ceil(size: Size) -> Self {
    unsafe {
      Self::from_flag_unchecked(Self::usize2u8flag(
        size.get().next_power_of_two(),
      ))
    }
  }

  #[inline]
  pub const fn from_size_bits(size_bits: SizeBit) -> Option<Self> {
    match Size::try_from(size_bits) {
      Ok(size) => Self::from_size(size),
      Err(_) => None,
    }
  }

  #[inline]
  pub const fn from_size_bits_unchecked(size_bits: SizeBit) -> Self {
    Self::from_align_unchecked(size_bits.get() / 8)
  }

  #[inline]
  pub const fn from_size_bits_ceil(size_bits: SizeBit) -> Self {
    Self::from_size_ceil(size_bits.ceil_to_byte())
  }
}
impl Alignment {
  #[inline(always)]
  const fn usize2u8flag(usize: usize) -> u8 {
    (usize.trailing_zeros()) as u8 + 1
  }

  #[inline]
  const unsafe fn from_flag_unchecked(flag: u8) -> Self {
    const_pre + (flag != 0, "flag should not be 0");
    Self {
      // SAFETY: safe.
      flag: unsafe { NonZeroU8::new_unchecked(flag) },
    }
  }

  #[inline]
  const fn from_align_unchecked(align: usize) -> Self {
    const_pre + (align.is_power_of_two(), "not a power of two");
    unsafe { Self::from_flag_unchecked(Self::usize2u8flag(align)) }
  }
}
#[rustfmt::skip]
impl Alignment {
  /// the smallest possible [`alignment`](Alignment), i.e., 1 [byte](Size).
  pub const MIN: Alignment = Self::O0;

  /// Order-0 [`alignment`](Alignment), i.e., `2^0 = 1` [byte](Size).
  pub const O0: Alignment = Self::from_align_fixed::<1>();

  /// Order-1 [`alignment`](Alignment), i.e., `2^1 = 2` [bytes](Size).
  pub const O1: Alignment = Self::from_align_fixed::<2>();

  /// Order-2 [`alignment`](Alignment), i.e., `2^2 = 4` [bytes](Size).
  pub const O2: Alignment = Self::from_align_fixed::<4>();

  /// Order-3 [`alignment`](Alignment), i.e., `2^3 = 8` [bytes](Size).
  pub const O3: Alignment = Self::from_align_fixed::<8>();

  /// Order-4 [`alignment`](Alignment), i.e., `2^4 = 16` [bytes](Size).
  pub const O4: Alignment = Self::from_align_fixed::<16>();

  /// Order-255 [`alignment`](Alignment), i.e., `2^255` [bytes](Size).
  pub const O255: Alignment = unsafe { Self::from_flag_unchecked(255) };
  
  /// the largest possible [`alignment`](Alignment) that [`this struct`](Alignment) can represent.
  pub const MAX: Alignment = Self::O255;
}
impl Alignment {
  #[inline]
  pub const fn size(self) -> Size {
    Size::from(1usize << (self.flag.get() - 1))
  }

  #[inline]
  pub const fn size_bits(self) -> SizeBit {
    SizeBit::from(self.size())
  }

  #[inline]
  pub const fn power(self) -> u8 {
    self.flag.get() - 1
  }
}

mod ops {
  use ::std::ops::{Shl, Shr};

  use super::*;

  impl const Shl<u8> for Alignment {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
      unsafe {
        Self::from_flag_unchecked(
          self.flag.checked_add(rhs).expect("overflow!").get(),
        )
      }
    }
  }

  #[allow(clippy::suspicious_arithmetic_impl)]
  impl const Shr<u8> for Alignment {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u8) -> Self::Output {
      if self.flag.get() > rhs {
        unsafe { Self::from_flag_unchecked(self.flag.get() - rhs) }
      } else {
        panic!("downflow!")
      }
    }
  }
}
mod fmt {

  use super::*;

  macro_rules! impl_fmt {
    ($trait:ident) => {
      impl ::std::fmt::$trait for Alignment {
        #[inline]
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
          self.size().fmt(f)
        }
      }
    };
  }
  impl ::std::fmt::Debug for Alignment {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
      write!(f, "2^{}", self.power())
    }
  }
  impl_fmt!(Display);
  impl_fmt!(Binary);
  impl_fmt!(Octal);
  impl_fmt!(LowerHex);
  impl_fmt!(UpperHex);
  impl_fmt!(LowerExp);
  impl_fmt!(UpperExp);
}
#[cfg(test)]
#[allow(clippy::unnecessary_cast)]
#[allow(non_upper_case_globals)]
mod tests {
  use ::rcc_utils::static_assert_eq;
  use Alignment as A;

  #[allow(unused)]
  use super::*;

  /// only panic when debug_assertion is enabled at runtime.
  #[test]
  #[should_panic(expected = "flag should not be 0")]
  const fn invalid_new_unchecked() {
    _ = unsafe { A::from_flag_unchecked(0) };
  }
  /// always panic if the given ALIGN is not a valid align at compile-time.
  #[test]
  #[should_panic(expected = "ALIGN is not a power of two")]
  const fn invalid_from_zero_align() {
    _ = A::from_align_fixed::<0>();
  }
  /// ditto.
  #[test]
  #[should_panic(expected = "ALIGN is not a power of two")]
  const fn invalid_from_align_fixed() {
    _ = A::from_align_fixed::<3>();
  }
  #[test]
  const fn ctors_fixed() {
    static_assert_eq!(A::from_align_fixed::<1>().power(), 0);
    static_assert_eq!(A::from_align_fixed::<2>().power(), 1);
    static_assert_eq!(A::from_align_fixed::<4>().power(), 2);
    static_assert_eq!(A::from_align_fixed::<8>().power(), 3);

    static_assert_eq!(A::from_size(0.into()), None);
    static_assert_eq!(A::from_size(1.into()).unwrap().size().get(), 1);
    static_assert_eq!(A::from_size(2.into()).unwrap().size().get(), 2);
    static_assert_eq!(A::from_size(3.into()), None);
    static_assert_eq!(A::from_size(4.into()).unwrap().size().get(), 4);
    static_assert_eq!(A::from_size(5.into()), None);
    static_assert_eq!(A::from_size(6.into()), None);
    static_assert_eq!(A::from_size(7.into()), None);
    static_assert_eq!(A::from_size(8.into()).unwrap().size().get(), 8);
  }
  #[test]
  const fn ctors_align() {
    static_assert_eq!(A::from_size_ceil(0.into()).size().get(), 1);
    static_assert_eq!(A::from_size_ceil(1.into()).size().get(), 1);
    static_assert_eq!(A::from_size_ceil(2.into()).size().get(), 2);
    static_assert_eq!(A::from_size_ceil(3.into()).size().get(), 4);
    static_assert_eq!(A::from_size_ceil(4.into()).size().get(), 4);
    static_assert_eq!(A::from_size_ceil(5.into()).size().get(), 8);
    static_assert_eq!(A::from_size_ceil(6.into()).size().get(), 8);
    static_assert_eq!(A::from_size_ceil(7.into()).size().get(), 8);
    static_assert_eq!(A::from_size_ceil(8.into()).size().get(), 8);
    static_assert_eq!(A::from_size_ceil(9.into()).size().get(), 16);
    static_assert_eq!(A::from_size_ceil(15.into()).size().get(), 16);
    static_assert_eq!(A::from_size_ceil(16.into()).size().get(), 16);
    static_assert_eq!(A::from_size_ceil(17.into()).size().get(), 32);

    static_assert_eq!(A::from_size_bits_ceil(0.into()).size().get(), 1);
    static_assert_eq!(A::from_size_bits_ceil(1.into()).size().get(), 1);
    static_assert_eq!(A::from_size_bits_ceil(2.into()).size().get(), 1);
    static_assert_eq!(A::from_size_bits_ceil(7.into()).size().get(), 1);
    static_assert_eq!(A::from_size_bits_ceil(8.into()).size().get(), 1);
    static_assert_eq!(A::from_size_bits_ceil(9.into()).size().get(), 2);
    static_assert_eq!(A::from_size_bits_ceil(15.into()).size().get(), 2);
    static_assert_eq!(A::from_size_bits_ceil(16.into()).size().get(), 2);
    static_assert_eq!(A::from_size_bits_ceil(17.into()).size().get(), 4);
    static_assert_eq!(A::from_size_bits_ceil(31.into()).size().get(), 4);
    static_assert_eq!(A::from_size_bits_ceil(32.into()).size().get(), 4);
    static_assert_eq!(A::from_size_bits_ceil(33.into()).size().get(), 8);
  }

  #[test]
  const fn ops() {
    const a: Alignment = A::from_align_fixed::<4>();
    static_assert_eq!((a << 2).size().get(), 16);
    static_assert_eq!((a >> 2).size().get(), 1);
  }
  #[test]
  #[should_panic(expected = "downflow!")]
  const fn ops_invalid() {
    const p: u8 = 2;
    const s: u8 = 2;
    const a: Alignment = A::from_align_fixed::<{ 2usize.pow(p as u32) }>();
    static_assert_eq!(a >> s, Alignment::O0);
    _ = a >> (s + 1);
  }

  #[test]
  #[should_panic(expected = "overflow!")]
  const fn ops_invalid_overflow() {
    const p: u8 = 2;
    const s: u8 = u8::MAX - p;
    const a: Alignment = A::from_align_fixed::<{ 2usize.pow(p as u32) }>();
    static_assert_eq!(a << (s - 1), Alignment::MAX);
    _ = a << s;
  }
}
