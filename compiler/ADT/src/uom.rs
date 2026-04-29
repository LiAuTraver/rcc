//! stands for *Unit of Measurement*. Type-Safe wrappers.
//!
//! Type is all you need.

mod alignment;
mod size;

pub use self::{
  alignment::Alignment,
  size::{Size, SizeBit},
};

#[macro_use]
mod macros {
  #[macro_export]
  macro_rules! impl_fmt {
    ($trait:ident, $class:ident) => {
      impl ::std::fmt::$trait for $class {
        #[inline]
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
          self.inner.fmt(f)
        }
      }
    };
  }
  #[macro_export]
  macro_rules! impl_all_fmt {
      ($class:ident => $($trait:ident),*) => {
          $($crate::impl_fmt!($trait, $class);)*
      };
  }

  #[macro_export]
  macro_rules! impl_bin_ops {
    ($trait:ident:: $method:ident, $class:ident) => {
      impl const ::std::ops::$trait for $class {
        type Output = Self;

        #[inline]
        fn $method(self, rhs: Self) -> Self::Output {
          Self::new(::std::ops::$trait::$method(self.inner, rhs.inner))
        }
      }
    };
  }

  #[macro_export]
  macro_rules! impl_all_bin_ops {
    ($class:ident => $($trait:ident:: $method:ident),*) => {
      $($crate::impl_bin_ops!($trait::$method, $class);)*
    };
  }
}
