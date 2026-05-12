use crate::Arena;

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
  /// [Stacked Borrows](https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md).
  ///
  /// Note: `cargo miri` would sometimes fail on Windows since File APIs are unsupported.
  #[must_use]
  unsafe fn alloc_uninit<T>(&self) -> *mut T;

  /// If the returned reference is casted to a pointer, **beaware of the provenance**.
  #[must_use]
  fn alloc<T>(&self, val: T) -> &mut T
  where
    T: ::std::fmt::Debug;

  fn alloc_slice_copy<T>(&self, values: &[T]) -> &mut [T]
  where
    T: Copy;

  fn alloc_str(&self, src: &str) -> &mut str;

  fn alloc_slice_fill_iter<T, I>(&self, iter: I) -> &mut [T]
  where
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator;
}
use super::fwd;
pub trait CollectIn<'bump>: Iterator + Sized {
  fn collect_in<
    C: fwd::FromIteratorIn<Self::Item, Alloc = &'bump fwd::Bump>,
  >(
    self,
    alloc: &'bump Arena,
  ) -> C {
    C::from_iter_in(self, alloc.raw_bump())
  }
}
impl<'bump, T> CollectIn<'bump> for T where T: fwd::CollectIn {}
