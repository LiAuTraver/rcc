//! stands for *Unit of Measurement*. Type-Safe wrappers. All Units are *immutable* and *trivially-copyable*.
//!
//! Type is all you need.
#[macro_use]
mod macros {
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
  macro_rules! impl_all_fmt {
      ($class:ident => $($trait:ident),*) => {
          $(impl_fmt!($trait, $class);)*
      };
  }

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

  macro_rules! impl_all_bin_ops {
    ($class:ident => $($trait:ident:: $method:ident),*) => {
      $(impl_bin_ops!($trait::$method, $class);)*
    };
  }
}

mod alignment;
mod size;

pub use self::{
  alignment::Alignment,
  size::{Size, SizeBit},
};
