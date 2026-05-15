/// delibreately write wrappers rather than using [`std::ops::Deref`].
#[allow(clippy::mut_from_ref, reason = "allocator is meant to do this.")]
pub trait Bumper {
  /// Allocates uninitialized memory for a value of type `T` and returns a mutable reference to it.
  ///
  /// # Safety
  /// If the returned reference is casted to a pointer, **be aware of the provenance**
  /// and ensure there only exists one (mut) pointer to the same memory.
  ///
  /// *always use `cargo miri test` to test code that uses this function to ensure safety!*
  ///
  /// Unsafe operations w.r.t. the pointer would easily violate
  /// [Stacked Borrows](https://plv.mpi-sws.org/rustbelt/stacked-borrows/).
  ///
  /// Note: `cargo miri` would sometimes fail on Windows since File APIs are unsupported.
  #[must_use]
  fn alloc_uninit<T>(&self) -> *mut T;

  /// If the returned reference is casted to a pointer, **beaware of the provenance**.
  #[must_use]
  fn alloc<T>(&self, val: T) -> &mut T
  where
    T: ::std::fmt::Debug;

  #[must_use]
  fn alloc_slice<T>(&self, values: &[T]) -> &mut [T]
  where
    T: Copy;

  #[must_use]
  fn alloc_str(&self, src: &str) -> &mut str;

  #[must_use]
  fn alloc_slice_fill_iter<T, I>(&self, iter: I) -> &mut [T]
  where
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator;
}
use ::bumpalo::{Bump, collections as bc};

use super::Arena;

/// See the doc of [`CollectIn`](::bumpalo::collections::CollectIn). This is a re-export.
pub trait CollectIn<'bump>: Iterator + Sized {
  /// See the doc of [`collect_in`](::bumpalo::collections::CollectIn::collect_in). This is a re-export.
  #[inline(always)]
  fn collect_in<C: bc::FromIteratorIn<Self::Item, Alloc = &'bump Bump>>(
    self,
    alloc: &'bump Arena,
  ) -> C {
    C::from_iter_in(self, alloc.raw_bump())
  }
}
impl<T> CollectIn<'_> for T where T: bc::CollectIn {}
