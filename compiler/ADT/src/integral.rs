use ::rcc_utils::{
  BuiltinIntegerOrBoolean, BuiltinNumeric, NumFrom, NumTo, const_pre,
  ensure_is_pod, signed_type_of,
};

use crate::SizeBit;
mod private {
  pub trait Sealed {}
  impl Sealed for super::SizeBit {}
}
pub(crate) const trait ToU8: private::Sealed {
  fn u8(self) -> u8;
}
impl const ToU8 for SizeBit {
  fn u8(self) -> u8 {
    self.to_builtin()
  }
}
/// the reason these stuffs exists is that
/// i planned somedays after when the sizae is considered large,
/// change this to usize/u64 to be not that difficult.
type Underlying = u128;
type SignedUnderlying = signed_type_of!(u128);
use ::rcc_utils::ToU128 as ToUnderlying;
const __MAX_ZU: u8 = 128;

/// Signedness of an integer type.
#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq)]
pub enum Signedness {
  Unsigned,
  Signed,
}
use Signedness::*;

impl Signedness {
  #[inline(always)]
  pub const fn is_signed(self) -> bool {
    matches!(self, Signed)
  }

  #[inline(always)]
  pub const fn is_unsigned(self) -> bool {
    matches!(self, Unsigned)
  }
}

/// A width-aware integer that can represent any C integer type, inspired by
/// [LLVM/Clang's APInt](https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/ADT/APInt.h?=#L78),
/// this provides a unified representation for all integer constants.
///
/// The value is always stored in the lower [`Integral::width`] bits of [`Integral::bits`].
/// For signed interpretation, use [`Integral::as_signed`].
///
/// This class is designed to by *trivially copyable* and *const-evaluable*,
/// all methods are taking either `self` or `&self` and return a new `Integral`, no internal mutation.
///
/// Also, methods w.r.t. 2 or more [`Integral`]s (e.g. [`Integral::overflowing_add`]) needs to ensure
/// that the operands have the same width and signedness, otherwise panic.
///
/// It does not support 0-width integers, and the current maximum width isSelf::MAX_SUPPORTED_SIZE_BITSbits.
///
/// It's an oversight that the signedness does not matter at IR Level,
/// so at IR Level the [`Integral::signedness`] field should always be ignored.
#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq)]
pub struct Integral {
  /// The raw bits, stored in the lower `width` bits.
  ///
  /// The underlying storage type for all integer values.
  /// Using u128 allows us to represent all C integer types (up to 64-bit)
  /// with room for future extensions.
  bits: Underlying,
  /// The bit width of this integer (1-128).
  width: u8,
  /// Whether this integer should be interpreted as signed.
  ///
  /// # U should always ignore the [`Signedness`] after IRCodeGen pass
  ///
  /// **making them all [`Unsigned`]!**
  ///
  /// *Additional explanation:*
  /// The reason it's here is that as long as our maximun support [Self::width]
  /// is `128` bits, our struct alignment should at least aligned as `0x10`
  /// because 1. rust's memory model mandates alignment, so reference
  /// to the packed struct field must be aligned. 2. it actually slows down
  /// the CPU cuz the additional load inst if not aligned.
  ///
  /// Hence, the size remains unchanged for our custom [`Integral`] type
  /// compared to its corresponding type without [`Signedness`] information --
  /// there's no point of the goal is to reduce the size -- too lazy is also a reason :)
  ///
  /// So we stores [`this`](Integral) struct -- that is with signedness in
  /// IR Constant value. **again, you should always IGNORE the [`Signedness`]!**
  signedness: Signedness,
}

ensure_is_pod!(Integral);

impl Integral {
  pub const MAX_SUPPORTED_SIZE_BITS: SizeBit =
    SizeBit::new(self::__MAX_ZU as usize);
}

impl Integral {
  #[inline]
  pub const fn new_unchecked<
    T: [const] ToUnderlying + BuiltinIntegerOrBoolean,
  >(
    value: T,
    width: u8,
    signedness: Signedness,
  ) -> Self {
    Self {
      bits: Self::mask(value.to_u128(), width),
      width,
      signedness,
    }
  }

  /// Create a new integral value, automatically masking to the specified width.
  #[inline]
  pub const fn new<T: [const] ToUnderlying + BuiltinIntegerOrBoolean>(
    value: T,
    width: SizeBit,
    signedness: Signedness,
  ) -> Self {
    const_pre
      + (
        width.u8() > 0,
        /* and */
        width <= Self::MAX_SUPPORTED_SIZE_BITS,
        "invalid width!",
      );
    Self::new_unchecked(value, width.to_builtin(), signedness)
  }

  #[inline]
  pub const fn from_signed<
    T: [const] ToUnderlying + BuiltinIntegerOrBoolean,
  >(
    value: T,
    width: SizeBit,
  ) -> Self {
    Self::new(value, width, Signed)
  }

  #[inline]
  pub const fn from_unsigned<
    T: [const] ToUnderlying + BuiltinIntegerOrBoolean,
  >(
    value: T,
    width: SizeBit,
  ) -> Self {
    Self::new(value, width, Unsigned)
  }

  /// give a i1 bool
  #[inline]
  pub const fn from_bool(value: bool) -> Self {
    Self::new_unchecked(value, 1, Unsigned)
  }

  #[inline]
  pub const fn bitmask(width: SizeBit) -> Self {
    const_pre
      + (
        width.u8() > 0,
        width <= Self::MAX_SUPPORTED_SIZE_BITS,
        "invalid width!",
      );
    Self::new(Self::mask(Underlying::MAX, width.u8()), width, Unsigned)
  }

  #[inline]
  pub const fn i1_true() -> Self {
    Self::new_unchecked(1, 1, Unsigned)
  }

  #[inline]
  pub const fn i1_false() -> Self {
    Self::new_unchecked(0, 1, Unsigned)
  }
}
impl Integral {
  #[inline(always)]
  pub const fn bits(&self) -> Underlying {
    self.bits
  }

  #[inline(always)]
  pub const fn width(&self) -> SizeBit {
    SizeBit::new(self.width)
  }

  #[inline(always)]
  pub const fn signedness(&self) -> Signedness {
    self.signedness
  }

  #[inline(always)]
  pub const fn is_signed(&self) -> bool {
    self.signedness.is_signed()
  }

  #[inline(always)]
  pub const fn is_unsigned(&self) -> bool {
    self.signedness.is_unsigned()
  }

  /// sign-extended to i128.
  #[inline]
  pub const fn as_signed(&self) -> SignedUnderlying {
    let shift = Self::MAX_SUPPORTED_SIZE_BITS.u8() - self.width;
    ((self.bits as SignedUnderlying) << shift) >> shift
  }

  /// get the bits as unsigned, same as [`Integral::bits()`].
  #[inline]
  pub const fn as_unsigned(&self) -> Underlying {
    self.bits
  }

  #[inline]
  pub const fn is_zero(&self) -> bool {
    self.bits == 0
  }

  #[inline]
  pub const fn is_one(&self) -> bool {
    self.bits == 1
  }

  /// Check if the sign bit is set.
  #[inline]
  pub const fn sign_bit(&self) -> bool {
    if self.width == Self::MAX_SUPPORTED_SIZE_BITS.u8() {
      (self.bits as SignedUnderlying) < 0
    } else {
      (self.bits >> (self.width - 1)) & 1 != 0
    }
  }

  /// Check if value is negative **(only meaningful for signed)**.
  #[inline]
  pub const fn is_negative(&self) -> bool {
    self.is_signed() && self.sign_bit()
  }

  /// Check if value is positive (> 0).
  #[inline]
  pub const fn is_positive(&self) -> bool {
    !self.is_zero() && !self.is_negative()
  }

  /// Get the minimum value for this width and signedness.
  pub const fn min_value(width: SizeBit, signedness: Signedness) -> Self {
    match signedness {
      Unsigned => Self::new(0, width, signedness),
      Signed => {
        let min = (1 as Underlying) << (width.u8() - 1);
        Self::new(min, width, signedness)
      },
    }
  }

  /// Get the maximum value for this width and signedness.
  pub const fn max_value(width: SizeBit, signedness: Signedness) -> Self {
    match signedness {
      Unsigned => Self::new(Self::max_unsigned(width), width, signedness),
      Signed => Self::new(
        ((1 as Underlying) << (width.u8() - 1)) - 1,
        width,
        signedness,
      ),
    }
  }

  /// Cast to a different width and/or signedness.
  /// This performs truncation or extension as appropriate.
  pub const fn cast(
    self,
    new_width: SizeBit,
    new_signedness: Signedness,
  ) -> Self {
    let new_bits = if new_width.u8() >= self.width {
      // extension
      if self.is_signed() && self.sign_bit() {
        // signed
        let extension_mask = Self::max_unsigned(new_width)
          ^ Self::max_unsigned(SizeBit::new(self.width as usize));
        self.bits | extension_mask
      } else {
        // zero
        self.bits
      }
    } else {
      // truncation
      self.bits
    };

    Self::new(new_bits, new_width, new_signedness)
  }

  /// Change signedness without changing the bits.
  #[inline]
  pub const fn reinterpret(self, signedness: Signedness) -> Self {
    Self { signedness, ..self }
  }

  /// Zero-extend to a wider type.
  #[inline]
  pub const fn zext(self, new_width: SizeBit) -> Self {
    const_pre + (new_width.u8() >= self.width);
    Self::new(self.bits, new_width, Unsigned)
  }

  /// Sign-extend to a wider type.
  #[inline]
  pub const fn sext(self, new_width: SizeBit) -> Self {
    const_pre + (new_width.u8() >= self.width);
    self.cast(new_width, Signed)
  }

  /// Truncate to a narrower type.
  #[inline]
  pub const fn trunc(self, new_width: SizeBit, signedness: Signedness) -> Self {
    const_pre + (new_width.u8() <= self.width);
    Self::new(self.bits, new_width, signedness)
  }

  #[inline]
  pub const fn to_builtin<
    T: [const] BuiltinNumeric
      + [const] NumFrom<Underlying>
      + [const] NumFrom<SignedUnderlying>,
  >(
    self,
  ) -> T {
    if self.is_signed() {
      self.as_signed().to()
    } else {
      self.bits.to()
    }
  }
}
impl Integral {
  /// Add with overflow detection.
  pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
    const_pre + (self.signedness, rhs.signedness);
    const_pre + (self.width, rhs.width, "width mismatch");

    let sum = self.bits.wrapping_add(rhs.bits);
    let result = Self::new_unchecked(sum, self.width, self.signedness);

    let overflow = if self.is_signed() {
      // signed overflow: signs of operands are same, but result sign differs
      let a_neg = self.sign_bit();
      let b_neg = rhs.sign_bit();
      let r_neg = result.sign_bit();
      (a_neg == b_neg) && (a_neg != r_neg)
    } else {
      // unsigned overflow: result is smaller than either operand
      result.bits < self.bits
    };

    (result, overflow)
  }

  /// Subtract with overflow detection.
  pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
    const_pre + (self.signedness, rhs.signedness);
    const_pre + (self.width, rhs.width, "width mismatch");

    let diff = self.bits.wrapping_sub(rhs.bits);
    let result = Self::new_unchecked(diff, self.width, self.signedness);

    let overflow = if self.is_signed() {
      // signed overflow.
      // a - b overflows if a and b have different signs and the result has the same sign as b
      let a_neg = self.sign_bit();
      let b_neg = rhs.sign_bit();
      let r_neg = result.sign_bit();
      (a_neg != b_neg) && (b_neg == r_neg)
    } else {
      // unsigned underflow: rhs > self
      rhs.bits > self.bits
    };

    (result, overflow)
  }

  /// Multiply with overflow detection.
  pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
    const_pre + (self.signedness, rhs.signedness);
    const_pre + (self.width, rhs.width, "width mismatch");

    let (product, overflow) = if self.is_signed() {
      let a = self.as_signed();
      let b = rhs.as_signed();
      let (p, o) = a.overflowing_mul(b);
      (p as Underlying, o)
    } else {
      self.bits.overflowing_mul(rhs.bits)
    };

    let result = Self::new_unchecked(product, self.width, self.signedness);

    // check if truncation lost bits
    let truncation_overflow = if self.is_signed() {
      let extended = result.as_signed();
      extended != (product as SignedUnderlying)
    } else {
      result.bits != product
    };

    (result, overflow || truncation_overflow)
  }

  /// Divide, returns None on division by zero.
  pub const fn checked_div(self, rhs: Self) -> Option<Self> {
    const_pre + (self.signedness, rhs.signedness);
    const_pre + (self.width, rhs.width, "width mismatch");

    if rhs.is_zero() {
      None?
    }

    let quotient = if self.is_signed() {
      (self.as_signed() / rhs.as_signed()) as Underlying
    } else {
      self.bits / rhs.bits
    };

    Some(Self::new_unchecked(quotient, self.width, self.signedness))
  }

  /// Remainder, returns [`None`] on division by zero.
  pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
    const_pre + (self.signedness, rhs.signedness);
    const_pre + (self.width, rhs.width, "width mismatch");

    if rhs.is_zero() {
      None?
    }

    let remainder = if self.is_signed() {
      (self.as_signed() % rhs.as_signed()) as Underlying
    } else {
      self.bits % rhs.bits
    };

    Some(Self::new_unchecked(remainder, self.width, self.signedness))
  }

  /// Logical right shift, always zero-fill.
  pub const fn lshr(self, amount: u32) -> Self {
    let amount = amount.min(self.width.to());
    Self::new_unchecked(self.bits >> amount, self.width, self.signedness)
  }

  /// Arithmetic right shift, always sign-fill.
  pub const fn ashr(self, amount: u32) -> Self {
    Self::new_unchecked(
      (self.as_signed() >> Ord::min(amount, self.width.to())) as Underlying,
      self.width,
      self.signedness,
    )
  }

  #[inline]
  const fn mask(value: Underlying, width: u8) -> Underlying {
    value & (((1 as Underlying) << width) - 1)
  }

  #[inline]
  const fn max_unsigned(width: SizeBit) -> Underlying {
    if width >= Self::MAX_SUPPORTED_SIZE_BITS {
      Underlying::MAX
    } else {
      ((1 as Underlying) << width.u8()) - 1
    }
  }
}
mod ops {
  use ::std::ops::{
    Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub,
  };

  use super::*;

  impl const Add for Integral {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
      self.overflowing_add(rhs).0
    }
  }

  impl const Sub for Integral {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
      self.overflowing_sub(rhs).0
    }
  }

  impl const Mul for Integral {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
      self.overflowing_mul(rhs).0
    }
  }

  impl const Div for Integral {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
      self.checked_div(rhs).expect("division by zero")
    }
  }
  impl const Rem for Integral {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
      self.checked_rem(rhs).expect("mod by zero")
    }
  }
  impl const Neg for Integral {
    type Output = Self;

    /// Negate (two's complement).
    #[inline]
    fn neg(self) -> Self {
      Self::new_unchecked(
        (!self.bits).wrapping_add(1),
        self.width,
        self.signedness,
      )
    }
  }

  impl const Not for Integral {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
      Self::new_unchecked(!self.bits, self.width, self.signedness)
    }
  }

  impl const BitAnd for Integral {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
      {
        const_pre + (self.width, rhs.width, "width mismatch");
        Self::new_unchecked(self.bits & rhs.bits, self.width, self.signedness)
      }
    }
  }

  impl const BitOr for Integral {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
      const_pre + (self.width, rhs.width, "width mismatch");
      Self::new_unchecked(self.bits | rhs.bits, self.width, self.signedness)
    }
  }

  impl const BitXor for Integral {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
      const_pre + (self.width, rhs.width, "width mismatch");

      Self::new_unchecked(self.bits ^ rhs.bits, self.width, self.signedness)
    }
  }

  impl const Shl<u32> for Integral {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: u32) -> Self {
      let amount = rhs.min(self.width as u32);
      Self::new_unchecked(self.bits << amount, self.width, self.signedness)
    }
  }

  impl const Shr<u32> for Integral {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u32) -> Self {
      let amount = rhs.min(self.width as u32);
      let result = if self.is_signed() {
        (self.as_signed() >> amount) as Underlying
      } else {
        self.bits >> amount
      };
      Self::new_unchecked(result, self.width, self.signedness)
    }
  }

  impl const PartialOrd for Integral {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
      if self.width != other.width || self.signedness != other.signedness {
        None
      } else {
        const_pre + (self.signedness, other.signedness, "signedness mismatch");
        const_pre + (self.width, other.width, "width mismatch");

        if self.is_signed() {
          self.as_signed().cmp(&other.as_signed())
        } else {
          self.bits.cmp(&other.bits)
        }
        .into()
      }
    }
  }
}
mod fmt {
  use super::*;

  macro_rules! impl_fmt {
    ($trait:ident) => {
      impl ::std::fmt::$trait for Integral {
        #[inline]
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
          if self.is_signed() {
            self.as_signed().fmt(f)
          } else {
            self.bits.fmt(f)
          }
        }
      }
    };
  }
  impl ::std::fmt::Debug for Integral {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
      if self.is_signed() {
        write!(f, "{}i{}", self.as_signed(), self.width)
      } else {
        write!(f, "{}u{}", self.bits, self.width)
      }
    }
  }

  impl_fmt!(Display);
  impl_fmt!(Binary);
  impl_fmt!(Octal);
  impl_fmt!(LowerHex);
  impl_fmt!(UpperHex);
  impl_fmt!(LowerExp);
  impl_fmt!(UpperExp);

  impl const Default for Integral {
    #[inline]
    fn default() -> Self {
      Self::from(0)
    }
  }
}
mod cvt {
  use super::*;
  macro_rules! impl_from_integral {
    ($t:ty, $width:expr, $signedness:expr) => {
      impl const From<$t> for Integral {
        #[inline(always)]
        fn from(value: $t) -> Self {
          Integral::new(value, SizeBit::new($width), $signedness)
        }
      }
    };
  }
  impl_from_integral!(bool, 1, Unsigned);
  impl_from_integral!(i8, i8::BITS, Signed);
  impl_from_integral!(u8, u8::BITS, Unsigned);
  impl_from_integral!(i16, i16::BITS, Signed);
  impl_from_integral!(u16, u16::BITS, Unsigned);
  impl_from_integral!(i32, i32::BITS, Signed);
  impl_from_integral!(u32, u32::BITS, Unsigned);
  impl_from_integral!(i64, i64::BITS, Signed);
  impl_from_integral!(u64, u64::BITS, Unsigned);
  impl_from_integral!(i128, i128::BITS, Signed);
  impl_from_integral!(u128, u128::BITS, Unsigned);
  impl_from_integral!(isize, isize::BITS, Signed);
  impl_from_integral!(usize, usize::BITS, Unsigned);
}

/// Actually running the tests here is not required, since they are `consteval`ed.
/// The success of compilation implies that every single tests are guranteed to pass.
#[cfg(test)]
#[allow(clippy::unnecessary_cast)]
#[allow(non_upper_case_globals)]
mod tests {
  use ::rcc_utils::{static_assert, static_assert_eq};

  #[allow(unused)]
  use super::*;

  #[test]
  const fn test_sign_extension() {
    const neg_one_i8: Integral = Integral::from(-1 as i8);
    static_assert_eq!(neg_one_i8.as_signed(), -1);
    static_assert_eq!(neg_one_i8.bits(), 0xFF);

    const extended: Integral = neg_one_i8.sext(SizeBit::new(32));
    static_assert_eq!(extended.as_signed(), -1);
    static_assert_eq!(extended.bits(), 0xFFFFFFFF);
  }

  #[test]
  const fn test_truncation() {
    const big: Integral = Integral::from(0x12345678 as i32);
    const small: Integral = big.trunc(SizeBit::new(8), Unsigned);
    static_assert_eq!(small.bits(), 0x78);
  }

  #[test]
  const fn test_overflow_detection() {
    const max_i8: Integral = Integral::new(127, SizeBit::new(8), Signed);
    const one: Integral = Integral::new(1, SizeBit::new(8), Signed);
    const R: (Integral, bool) = max_i8.overflowing_add(one);
    static_assert!(R.1);
    static_assert_eq!(R.0.as_signed(), -128);
  }

  #[test]
  const fn test_signed_comparison() {
    const neg: Integral = Integral::from(-1 as i8);
    const pos: Integral = Integral::from(1 as i8);
    static_assert!(neg < pos);
  }

  #[test]
  const fn test_unsigned_comparison() {
    const a: Integral = Integral::from(255 as u8);
    const b: Integral = Integral::from(1 as u8);
    static_assert!(a > b);
  }
}
