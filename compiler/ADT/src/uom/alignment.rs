use ::rcc_utils::const_pre;
use ::std::num::NonZeroU8;

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
  pub const fn new(flag: u8) -> Option<Self> {
    if flag != 0 {
      // SAFETY: safe.
      Some(Self::new_unchecked(flag))
    } else {
      None
    }
  }

  #[inline]
  pub const fn new_unchecked(flag: u8) -> Self {
    const_pre + (flag != 0, "UB");
    Self {
      flag: unsafe { NonZeroU8::new_unchecked(flag) },
    }
  }

  #[inline]
  pub const fn fixed<const FLAG: u8>() -> Self {
    const_pre + (FLAG != 0, "UB");
    Self {
      flag: unsafe { NonZeroU8::new_unchecked(FLAG) },
    }
  }

  /// constructs an alignment which is the smallest power of two greater than or equal to `align`.
  ///
  /// if the alignment is zero, it will be treated as one.
  #[inline]
  pub const fn from_align_roundup(align: usize) -> Self {
    Self::new_unchecked(Self::usize2u8flag(align.next_power_of_two()))
  }

  #[inline]
  pub const fn from_align_fixed<const ALIGN: usize>() -> Self {
    const_pre + (ALIGN.is_power_of_two(), "not a power of two");
    Self::new_unchecked(Self::usize2u8flag(ALIGN))
  }

  #[inline]
  pub const fn from_align(align: usize) -> Self {
    const_pre + (align.is_power_of_two(), "not a power of two");
    Self::new_unchecked(Self::usize2u8flag(align))
  }

  #[inline]
  pub const fn from_maybe_align(align: usize) -> Option<Self> {
    if align.is_power_of_two() {
      Some(Self::new_unchecked(Self::usize2u8flag(align)))
    } else {
      None
    }
  }

  #[inline(always)]
  const fn usize2u8flag(usize: usize) -> u8 {
    (usize.trailing_zeros()) as u8 + 1
  }
}

impl Alignment {
  #[inline]
  pub const fn align_bytes(self) -> usize {
    1usize << (self.flag.get() - 1)
  }

  #[inline]
  pub const fn align_bits(self) -> usize {
    self.align_bytes() * 8
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
      Self::new_unchecked(self.flag.checked_add(rhs).expect("overflow!").get())
    }
  }

  #[allow(clippy::suspicious_arithmetic_impl)]
  impl const Shr<u8> for Alignment {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u8) -> Self::Output {
      Self::new(self.flag.get() - rhs).expect("downflow!")
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
          self.align_bytes().fmt(f)
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

  #[allow(unused)]
  use super::*;

  #[test]
  #[should_panic]
  const fn invalid_fixed() {
    _ = Alignment::fixed::<0>();
  }

  #[test]
  #[should_panic]
  const fn invalid_new() {
    _ = Alignment::new(0).unwrap();
  }
  #[test]
  #[should_panic]
  const fn invalid_new_unchecked() {
    _ = Alignment::new_unchecked(0);
  }
  #[test]
  #[should_panic]
  const fn invalid_from_fixed() {
    _ = Alignment::fixed::<0>();
  }
  #[test]
  #[should_panic]
  const fn invalid_from_align_fixed() {
    _ = Alignment::from_align_fixed::<3>();
  }
  #[test]
  #[should_panic]
  const fn invalid_from_zero_align() {
    _ = Alignment::from_align_fixed::<0>();
  }
  #[test]
  const fn ctors_fixed() {
    static_assert_eq!(Alignment::fixed::<1>().align_bytes(), 1);
    static_assert_eq!(Alignment::fixed::<2>().align_bytes(), 2);
    static_assert_eq!(Alignment::fixed::<3>().align_bytes(), 4);
    static_assert_eq!(Alignment::fixed::<4>().align_bytes(), 8);
    static_assert_eq!(Alignment::fixed::<5>().align_bytes(), 16);
    static_assert_eq!(Alignment::fixed::<6>().align_bytes(), 32);
    static_assert_eq!(Alignment::fixed::<7>().align_bytes(), 64);
    static_assert_eq!(Alignment::fixed::<8>().align_bytes(), 128);
    static_assert_eq!(Alignment::fixed::<9>().align_bytes(), 256);
    static_assert_eq!(Alignment::fixed::<10>().align_bytes(), 512);

    static_assert_eq!(Alignment::from_align_fixed::<1>().power(), 0);
    static_assert_eq!(Alignment::from_align_fixed::<2>().power(), 1);
    static_assert_eq!(Alignment::from_align_fixed::<4>().power(), 2);
    static_assert_eq!(Alignment::from_align_fixed::<8>().power(), 3);

    static_assert_eq!(Alignment::from_maybe_align(1).unwrap().align_bytes(), 1);
    static_assert_eq!(Alignment::from_maybe_align(2).unwrap().align_bytes(), 2);
    static_assert_eq!(Alignment::from_maybe_align(3), None);
    static_assert_eq!(Alignment::from_maybe_align(4).unwrap().align_bytes(), 4);
    static_assert_eq!(Alignment::from_maybe_align(5), None);
    static_assert_eq!(Alignment::from_maybe_align(6), None);
    static_assert_eq!(Alignment::from_maybe_align(7), None);
    static_assert_eq!(Alignment::from_maybe_align(8).unwrap().align_bytes(), 8);
  }
  #[test]
  const fn ctors_align() {
    static_assert_eq!(Alignment::from_align_roundup(0).align_bytes(), 1);
    static_assert_eq!(Alignment::from_align_roundup(1).align_bytes(), 1);
    static_assert_eq!(Alignment::from_align_roundup(2).align_bytes(), 2);
    static_assert_eq!(Alignment::from_align_roundup(3).align_bytes(), 4);
    static_assert_eq!(Alignment::from_align_roundup(4).align_bytes(), 4);
    static_assert_eq!(Alignment::from_align_roundup(5).align_bytes(), 8);
    static_assert_eq!(Alignment::from_align_roundup(6).align_bytes(), 8);
    static_assert_eq!(Alignment::from_align_roundup(7).align_bytes(), 8);
    static_assert_eq!(Alignment::from_align_roundup(8).align_bytes(), 8);
    static_assert_eq!(Alignment::from_align_roundup(9).align_bytes(), 16);
    static_assert_eq!(Alignment::from_align_roundup(15).align_bytes(), 16);
    static_assert_eq!(Alignment::from_align_roundup(16).align_bytes(), 16);
    static_assert_eq!(Alignment::from_align_roundup(17).align_bytes(), 32);
  }

  #[test]
  const fn ops() {
    const a: Alignment = Alignment::fixed::<3>();
    static_assert_eq!((a << 2).align_bytes(), 16);
    static_assert_eq!((a >> 2).align_bytes(), 1);
  }
}
