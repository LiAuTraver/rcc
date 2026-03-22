#![feature(negative_impls)]
#![feature(const_trait_impl)]
mod macros;
mod num_traits;

use ::std::{
  cell::RefCell,
  rc::{Rc, Weak},
};

pub use self::num_traits::*;

pub type SmallString = ::compact_str::CompactString;
/// as someone who came from C++, I'd more prefer to call it shared_ptr rather than Rc/RefCell or whatever. :p
#[allow(non_camel_case_types)]
pub type shared_ptr<T> = Rc<RefCell<T>>;
#[allow(non_camel_case_types)]
pub type weak_ptr<T> = Weak<RefCell<T>>;

/// A handy trait for converting between types with additional context.
pub trait IntoWith<With, To> {
  fn into_with(self, with: With) -> To;
}
pub trait DisplayWith<'a, With, To: ::std::fmt::Display> {
  fn display_with(&'a self, with: &'a With) -> To;
}
pub trait FromWith<With, From>: Sized {
  fn from_with(from: From, with: With) -> Self;
}
pub trait TryFromWith<With, From>: Sized {
  type Error;
  fn try_from_with(from: From, with: With) -> Result<Self, Self::Error>;
}
/// A trait for creating dummy instances of types during testing.
///
/// This is useful for situations where a placeholder value is needed,
/// such as during testing or when initializing data structures,
/// but their actual values do not matter.
///
/// The difference between this and the [`Default`] trait is that Dummy
/// instances are often invalid or nonsensical in a real context,
/// whereas [`Default`] instances are expected to be valid and meaningful.
///
/// In other words, [`Dummy`] targets for ppl who read and write the code.
///
/// Why not use [`Option<T>`] or [`Result<T, E>`]? -- there's no point
/// to wrap every single type in Option or Result just cater for unittest.
#[cfg(debug_assertions)]
pub trait Dummy {
  fn dummy() -> Self;
}
#[cfg(debug_assertions)]
impl<T: Dummy> Dummy for shared_ptr<T> {
  fn dummy() -> Self {
    Rc::new(RefCell::new(T::dummy()))
  }
}

#[cfg(debug_assertions)]
impl Dummy for u32 {
  #[inline]
  fn dummy() -> Self {
    u32::MAX
  }
}
#[cfg(debug_assertions)]
impl Dummy for usize {
  #[inline]
  fn dummy() -> Self {
    usize::MAX
  }
}

pub type StrRef<'c> = &'c str;

pub trait RefEq {
  fn ref_eq(lhs: Self, rhs: Self) -> bool
  where
    Self: PartialEq + Sized;
}

impl RefEq for StrRef<'_> {
  fn ref_eq(lhs: Self, rhs: Self) -> bool
  where
    Self: PartialEq + Sized,
  {
    let ref_eq = ::std::ptr::eq(lhs, rhs);
    if const { cfg!(debug_assertions) } {
      let actual_eq = lhs == rhs;
      if ref_eq != actual_eq {
        eprintln!(
          "INTERNAL ERROR: comparing by pointer address result did not match 
          the actual result: {:p}: {:?} and {:p}: {:?}
        ",
          lhs, lhs, rhs, rhs
        );
      }
      return actual_eq;
    }
    ref_eq
  }
}

pub trait Unbox {
  type Output;
  #[must_use]
  fn unbox(self) -> Self::Output
  where
    Self::Output: Unbox<Output = Self::Output>;
}
// impl<T> Unbox for T {
//   default type Output = T;

//   #[inline(always)]
//   default fn unbox(self) -> Self::Output {
//     self
//     // unsafe {
//     //   let result = ::std::ptr::read
//     //     (&self as *const _ as *const Self::Output);
//     //   ::std::mem::forget(self);
//     //   result
//     // }
//   }
// }
impl<T> Unbox for Box<T> {
  type Output = T;

  #[inline(always)]
  fn unbox(self) -> Self::Output
  where
    Self::Output: Unbox<Output = Self::Output>,
  {
    *self
  }
}
